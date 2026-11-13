#!/usr/bin/env python3
"""Fail-closed index/working-tree/archive gate; --history checks reachable objects.

Exit 0: clean, 1: prohibited identity, 2: unreadable/unsafe/over-limit input.
Diagnostics deliberately omit input paths, content and subprocess output.
"""

import argparse
import gzip
import io
import os
from pathlib import Path, PurePosixPath
import selectors
import stat
import subprocess
import sys
import tarfile
import tempfile
import time
import zipfile
import zlib


WORD = bytes((99, 97, 116, 97, 110))
PATTERNS = tuple(WORD.decode().encode(e) for e in
                 ("ascii", "utf-16le", "utf-16be", "utf-32le", "utf-32be"))
MAX_FILE = 64 * 1024 * 1024
MAX_TOTAL = 512 * 1024 * 1024
MAX_ITEMS = 100_000
MAX_DEPTH = 8
GIT_TIMEOUT = 30


class Detection(Exception):
    pass


class ScanError(Exception):
    pass


def mention(data):
    folded = data.lower()
    if any(p in folded for p in PATTERNS):
        raise Detection


def safe_name(name):
    mention(name.encode("utf-8", "surrogateescape"))
    if (not name or "\0" in name or "\\" in name or ":" in name
            or name.startswith("/") or ".." in PurePosixPath(name).parts):
        raise ScanError


def git(root, *args, data=b"", limit=MAX_FILE):
    """Bound stdout while reading; suppress Git's potentially private diagnostics."""
    with tempfile.TemporaryFile() as source:
        source.write(data)
        source.seek(0)
        with subprocess.Popen(["git", "-C", str(root), *args], stdin=source,
                              stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                              env=dict(os.environ, GIT_NO_REPLACE_OBJECTS="1",
                                       GIT_NO_LAZY_FETCH="1")) as p:
            try:
                result = bytearray()
                deadline = time.monotonic() + GIT_TIMEOUT
                with selectors.DefaultSelector() as ready:
                    ready.register(p.stdout, selectors.EVENT_READ)
                    while ready.get_map():
                        remaining = deadline - time.monotonic()
                        if remaining <= 0:
                            raise ScanError
                        for key, _ in ready.select(remaining):
                            chunk = os.read(key.fd, 65536)
                            if not chunk:
                                ready.unregister(key.fileobj)
                            result.extend(chunk)
                            if len(result) > limit:
                                raise ScanError
                if p.wait(timeout=max(0.01, deadline - time.monotonic())):
                    raise ScanError
                return bytes(result)
            except BaseException:
                p.kill()
                p.wait()
                raise


class Scanner:
    def __init__(self):
        self.total = 0
        self.items = 0

    def account(self, size):
        self.items += 1
        self.total += size
        if size > MAX_FILE or self.total > MAX_TOTAL or self.items > MAX_ITEMS:
            raise ScanError

    def content(self, data, name="", depth=0):
        self.account(len(data))
        if depth > MAX_DEPTH:
            raise ScanError
        mention(data)
        suffix = name.lower()
        if data.startswith(b"\x1f\x8b") or suffix.endswith((".gz", ".gzip", ".tgz")):
            expanded = bytearray()
            remaining = data
            while remaining:
                self.account(0)
                # Validate original filenames in every concatenated gzip member.
                if not remaining.startswith(b"\x1f\x8b") or len(remaining) < 10 or remaining[3] & 0xe0:
                    raise ScanError
                cursor = 10
                if remaining[3] & 4:
                    if len(remaining) < cursor + 2:
                        raise ScanError
                    cursor += 2 + int.from_bytes(remaining[cursor:cursor + 2], "little")
                if remaining[3] & 8:
                    end = remaining.find(b"\0", cursor)
                    if end < 0:
                        raise ScanError
                    safe_name(remaining[cursor:end].decode("latin1"))
                decoder = zlib.decompressobj(31)
                expanded.extend(decoder.decompress(remaining, MAX_FILE + 1 - len(expanded)))
                if len(expanded) > MAX_FILE or not decoder.eof:
                    raise ScanError
                remaining = decoder.unused_data
            inner_name = name[:-3] if suffix.endswith(".gz") else ""
            if suffix.endswith(".tgz"):
                inner_name = name[:-4] + ".tar"
            self.content(bytes(expanded), inner_name, depth + 1)
        elif (data.startswith((b"PK\x03\x04", b"PK\x05\x06", b"PK\x07\x08"))
              or suffix.endswith(".zip")):
            with zipfile.ZipFile(io.BytesIO(data)) as archive:
                members = archive.infolist()
                end = data.rfind(b"PK\x05\x06")
                if (end < 0 or end + 22 > len(data)
                        or end + 22 + int.from_bytes(data[end + 20:end + 22], "little") != len(data)
                        or (min(m.header_offset for m in members) if members else archive.start_dir) != 0):
                    raise ScanError
                cursor = 0
                for member in sorted(members, key=lambda m: m.header_offset):
                    safe_name(member.orig_filename)
                    # Keep ZipFile's local/central filename and header validation.
                    with archive.open(member):
                        pass
                    self.account(0)
                    kind = stat.S_IFMT(member.external_attr >> 16)
                    if (member.flag_bits & 1 or kind not in (0, stat.S_IFREG, stat.S_IFDIR)
                            or member.file_size > MAX_FILE or member.header_offset != cursor
                            or data[cursor:cursor + 4] != b"PK\x03\x04"):
                        raise ScanError
                    name_size = int.from_bytes(data[cursor + 26:cursor + 28], "little")
                    extra_size = int.from_bytes(data[cursor + 28:cursor + 30], "little")
                    start = cursor + 30 + name_size + extra_size
                    finish = start + member.compress_size
                    if finish > archive.start_dir:
                        raise ScanError
                    packed = data[start:finish]
                    if member.compress_type == zipfile.ZIP_STORED:
                        expanded = packed
                    elif member.compress_type == zipfile.ZIP_DEFLATED:
                        decoder = zlib.decompressobj(-15)
                        expanded = decoder.decompress(packed, MAX_FILE + 1)
                        if not decoder.eof or decoder.unused_data or decoder.unconsumed_tail:
                            raise ScanError
                    else:
                        raise ScanError
                    if len(expanded) != member.file_size or zlib.crc32(expanded) != member.CRC:
                        raise ScanError
                    if member.flag_bits & 8:
                        if data[finish:finish + 4] == b"PK\x07\x08":
                            finish += 4
                        wide = (data[cursor + 18:cursor + 22] == b"\xff" * 4
                                or max(member.file_size, member.compress_size) >= 2**32)
                        width = 8 if wide else 4
                        descriptor = (member.CRC.to_bytes(4, "little")
                                      + member.compress_size.to_bytes(width, "little")
                                      + member.file_size.to_bytes(width, "little"))
                        if data[finish:finish + len(descriptor)] != descriptor:
                            raise ScanError
                        finish += len(descriptor)
                    cursor = finish
                    if member.is_dir():
                        if expanded:
                            raise ScanError
                        continue
                    self.content(expanded, member.filename, depth + 1)
                if cursor != archive.start_dir:
                    raise ScanError
        elif suffix.endswith(".tar") or data[257:262] == b"ustar" or self.tar_header(data):
            # Require complete records and end markers, not tarfile's tolerant EOF.
            if len(data) % 512 or len(data) < 1024 or data[-1024:] != bytes(1024):
                raise ScanError
            with tarfile.open(fileobj=io.BytesIO(data), mode="r:") as archive:
                for member in archive:
                    safe_name(member.name)
                    self.account(0)
                    for key, value in member.pax_headers.items():
                        mention((key + value).encode("utf-8", "surrogateescape"))
                    if member.isdir():
                        if member.size:
                            raise ScanError
                        continue
                    if not member.isfile() or member.size > MAX_FILE:
                        raise ScanError
                    end = member.offset_data + member.size
                    if any(data[end:(end + 511) // 512 * 512]):
                        raise ScanError
                    with archive.extractfile(member) as stream:
                        expanded = stream.read(MAX_FILE + 1)
                    if len(expanded) != member.size:
                        raise ScanError
                    self.content(expanded, member.name, depth + 1)
                if any(data[archive.offset:]):
                    raise ScanError

    @staticmethod
    def tar_header(data):
        if len(data) < 512:
            return False
        try:
            tarfile.TarInfo.frombuf(data[:512], "utf-8", "surrogateescape")
            return True
        except tarfile.HeaderError:
            return False

    def tree(self, root):
        entries = git(root, "ls-files", "--stage", "-z").split(b"\0")
        for entry in entries[:-1]:
            header, name = entry.split(b"\t", 1)
            mode, oid, stage = header.split()
            name = os.fsdecode(name)
            safe_name(name)
            if stage != b"0" or mode not in (b"100644", b"100755"):
                raise ScanError
            # Do not follow any tracked link or parent link outside the checkout.
            directory = os.open(root, os.O_RDONLY | os.O_DIRECTORY)
            try:
                parts = PurePosixPath(name).parts
                for part in parts[:-1]:
                    child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                                    dir_fd=directory)
                    os.close(directory)
                    directory = child
                fd = os.open(parts[-1], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=directory)
                with os.fdopen(fd, "rb") as stream:
                    info = os.fstat(stream.fileno())
                    if not stat.S_ISREG(info.st_mode) or info.st_size > MAX_FILE:
                        raise ScanError
                    self.content(stream.read(MAX_FILE + 1), name)
                self.content(git(root, "cat-file", "blob", oid.decode()), name)
            finally:
                os.close(directory)

    def history(self, root):
        if git(root, "rev-parse", "--is-shallow-repository").strip() != b"false":
            raise ScanError
        graft = Path(os.fsdecode(git(root, "rev-parse", "--git-path", "info/grafts").rstrip(b"\n")))
        if os.path.lexists(root / graft):
            raise ScanError
        refs = git(root, "for-each-ref", "--format=%(refname)")
        mention(refs)
        # --all includes refs; --reflog is deliberately excluded from reachable history.
        objects = git(root, "rev-list", "--objects", "--all", "--no-object-names")
        # Include a detached HEAD, while permitting a newly initialized repository.
        head = git(root, "rev-parse", "--verify", "--quiet", "HEAD") if (
            git(root, "rev-parse", "--revs-only", "HEAD").strip()) else b""
        if head:
            objects += git(root, "rev-list", "--objects", "--no-object-names", head.strip().decode())
        ids = list(dict.fromkeys(objects.splitlines()))
        if len(ids) > MAX_ITEMS:
            raise ScanError
        for oid in ids:
            if len(oid) not in (40, 64) or any(c not in b"0123456789abcdef" for c in oid):
                raise ScanError
        metadata = git(root, "cat-file", "--batch-check", data=b"\n".join(ids) + b"\n") if ids else b""
        records = [line.split() for line in metadata.splitlines()]
        if len(records) != len(ids) or any(len(r) != 3 for r in records):
            raise ScanError
        names = {}
        # Trees first preserve archive suffixes even when their blobs were deleted.
        for oid, expected_kind, expected_size in sorted(records, key=lambda r: r[1] != b"tree"):
            batch = git(root, "cat-file", "--batch", data=oid + b"\n", limit=MAX_FILE + 1024)
            header, body = batch.split(b"\n", 1)
            actual, kind, size = header.split()
            size = int(size)
            if (actual != oid or kind != expected_kind or size != int(expected_size)
                    or kind not in (b"blob", b"tree", b"commit", b"tag")):
                raise ScanError
            if len(body) != size + 1 or not body.endswith(b"\n"):
                raise ScanError
            # Raw tree objects contain every component name; commits/tags contain messages.
            body = body[:-1]
            if kind == b"tree":
                cursor = 0
                while cursor < len(body):
                    end = body.index(b"\0", cursor)
                    mode, name = body[cursor:end].split(b" ", 1)
                    name = os.fsdecode(name)
                    safe_name(name)
                    target = body[end + 1:end + 1 + len(oid) // 2].hex().encode()
                    cursor = end + 1 + len(oid) // 2
                    if cursor > len(body) or mode not in (b"40000", b"100644", b"100755", b"120000", b"160000"):
                        raise ScanError
                    if name.lower().endswith((".tar", ".gz", ".gzip", ".tgz", ".zip")):
                        names.setdefault(target, set()).add(name)
            for name in names.get(oid, ("",)) if kind == b"blob" else ("",):
                self.content(body, name)


def self_test():
    script = str(Path(__file__).resolve())
    env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    env.update(GIT_AUTHOR_NAME="Detector test", GIT_AUTHOR_EMAIL="test@example.invalid",
               GIT_COMMITTER_NAME="Detector test", GIT_COMMITTER_EMAIL="test@example.invalid",
               GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull)
    checks = 0

    def expect(root, code, history=False, environment=None):
        nonlocal checks
        result = subprocess.run([sys.executable, script] + (["--history"] if history else []),
                                cwd=root, env=environment or env, capture_output=True,
                                timeout=GIT_TIMEOUT + 5)
        if result.returncode != code or WORD in (result.stdout + result.stderr).lower():
            raise ScanError
        checks += 1

    def setup(root, name, data):
        root.mkdir()
        git(root, "init", "-q")
        (root / name).write_bytes(data)
        git(root, "add", "--", name)

    def commit(root, message="Fixture"):
        with tempfile.TemporaryFile() as output:
            result = subprocess.run(["git", "-C", str(root), "commit", "-qm", message],
                                    env=env, stdout=output, stderr=output, timeout=GIT_TIMEOUT)
            if result.returncode:
                raise ScanError

    def tar(name, data):
        output = io.BytesIO()
        with tarfile.open(fileobj=output, mode="w") as archive:
            member = tarfile.TarInfo(name)
            member.size = len(data)
            archive.addfile(member, io.BytesIO(data))
        return output.getvalue()

    def zipped(name, data):
        output = io.BytesIO()
        with zipfile.ZipFile(output, "w", zipfile.ZIP_DEFLATED) as archive:
            archive.writestr(name, data)
        return output.getvalue()

    def zip_gap():
        clean = zipped("safe", b"clean")
        hidden = zipped("hidden", WORD)
        with zipfile.ZipFile(io.BytesIO(clean)) as archive:
            offset = archive.start_dir
        with zipfile.ZipFile(io.BytesIO(hidden)) as archive:
            record = hidden[:archive.start_dir]
        result = bytearray(clean[:offset] + record + clean[offset:])
        end = result.rfind(b"PK\x05\x06")
        result[end + 16:end + 20] = (offset + len(record)).to_bytes(4, "little")
        return bytes(result)

    with tempfile.TemporaryDirectory(prefix="branding-self-test-") as temporary:
        base = Path(temporary)
        mixed = WORD.decode().title().swapcase()
        controls = [
            ("clean", "clean.txt", b"Hex Traders", 0),
            ("text", "text.txt", mixed.encode(), 1),
            ("path", mixed + ".txt", b"clean", 1),
            ("nested", "asset.zip", zipped("inner.tar.gz", gzip.compress(tar("text", mixed.encode()))), 1),
            ("member", "asset.tar", tar(mixed, b"clean"), 1),
            ("clean-archive", "asset.zip", zipped("inner.tar.gz", gzip.compress(tar("text", b"clean"))), 0),
            ("malformed", "asset.gz", gzip.compress(b"clean")[:-4], 2),
            ("invalid-deflate", "asset.gz", bytes.fromhex("1f8b08000000000000ff07") + bytes(8), 2),
            ("gzip-concat", "asset.gz", gzip.compress(b"clean") + gzip.compress(mixed.encode()), 1),
            ("gzip-unsafe-second", "asset.gz", gzip.compress(b"clean")
             + bytes.fromhex("1f8b08080000000000ff") + b"../escape\0" + gzip.compress(b"clean")[10:], 2),
            ("malformed-zip", "asset.zip", b"broken", 2),
            ("zip-concat", "asset.zip", zipped("hidden", mixed.encode()) + zipped("clean", b"clean"), 2),
            ("zip-gap", "asset.zip", zip_gap(), 2),
            ("zip-null-name", "asset.zip", zipped("safe-name-12", b"clean")
             .replace(b"safe-name-12", b"safe\0../evil"), 2),
            ("malformed-tar", "asset.tar", b"broken", 2),
            ("unsafe-tar", "asset.tar", tar("../escape", b"clean"), 2),
            ("unsafe-zip", "asset.zip", zipped("/escape", b"clean"), 2),
        ]
        for label, name, data, code in controls:
            root = base / label
            setup(root, name, data)
            expect(root, code)
            commit(root)
            expect(root, code, True)
        for label, name, data, message in [
            ("old-blob", "file", mixed.encode(), "Fixture"),
            ("old-path", mixed, b"clean", "Fixture"),
            ("old-message", "file", b"clean", mixed),
        ]:
            root = base / label
            setup(root, name, data)
            commit(root, message)
            git(root, "rm", "-q", "--", name)
            (root / "clean").write_bytes(b"clean")
            git(root, "add", "clean")
            commit(root, "Clean current tree")
            expect(root, 0)
            expect(root, 1, True)
        root = base / "staged"
        setup(root, "file", mixed.encode())
        (root / "file").write_bytes(b"clean")
        expect(root, 1)
        root = base / "deleted-archive"
        setup(root, "old.gz", b"malformed")
        commit(root)
        git(root, "rm", "-q", "old.gz")
        (root / "clean").write_bytes(b"clean")
        git(root, "add", "clean")
        commit(root)
        expect(root, 0)
        expect(root, 2, True)
        root = base / "replacement"
        setup(root, "file", b"clean")
        commit(root, mixed)
        old = git(root, "rev-parse", "HEAD").strip().decode()
        (root / "file").write_bytes(b"clean current tree")
        git(root, "add", "file")
        commit(root)
        new = git(root, "rev-parse", "HEAD").strip().decode()
        git(root, "replace", old, new)
        expect(root, 0)
        expect(root, 1, True)
        # A shallow boundary and legacy grafts cannot attest to complete history.
        shallow = Path(os.fsdecode(git(root, "rev-parse", "--git-path", "shallow").strip()))
        (root / shallow).write_text(new + "\n")
        expect(root, 2, True)
        (root / shallow).unlink()
        graft = Path(os.fsdecode(git(root, "rev-parse", "--git-path", "info/grafts").strip()))
        (root / graft).write_text(new + "\n")
        expect(root, 2, True)
        root = base / "missing"
        setup(root, "file", b"clean")
        (root / "file").unlink()
        expect(root, 2)
        os.mkfifo(root / "file")
        expect(root, 2)
        expect(base / "clean", 2, environment=dict(env, PATH=""))
        expect(base, 2)
        scanner = Scanner()
        for data, depth in [(b"clean", MAX_DEPTH + 1), (bytes(MAX_FILE + 1), 0)]:
            try:
                scanner.content(data, depth=depth)
            except ScanError:
                checks += 1
            else:
                raise ScanError
    print(f"Branding self-tests passed ({checks} controls).")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--history", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    try:
        if args.self_test:
            self_test()
        else:
            root = Path(os.fsdecode(git(Path.cwd(), "rev-parse", "--show-toplevel").rstrip(b"\n")))
            scanner = Scanner()
            scanner.tree(root)
            if args.history:
                scanner.history(root)
            print("Branding scan passed.")
        return 0
    except Detection:
        print("Branding scan failed: prohibited identity detected.", file=sys.stderr)
        return 1
    except Exception:
        print("Branding scan failed: unreadable, malformed, unsafe or over-limit input.", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
