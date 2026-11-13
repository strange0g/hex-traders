# T36: Fail-closed branding gate

Input preservation: T35; preparation revision `eb4f324`.
Scanner SHA-256: `cf2da6f9442ae2291a14f43fb29cd9c4461fb5025476f0f999f542fe78cf1991`.

Fresh RED/GREEN evidence on 13 November 2026: disposable drivers failed before
implementation (exit 1), then `python3 scripts/check-branding.py --self-test`
passed 53 controls (exit 0) and the independent disposable contract driver passed
55 controls (exit 0). Python AST/literal checks and `git diff --check` exit 0.
Terra independently approved this exact hash with no open findings after fixing
and retesting staged-content masking, Git replacements, concatenated gzip/ZIP,
unindexed ZIP records and truncated ZIP names. Detector-disable mutation fails.
Ordinary and ZIP64 stored/deflated descriptors pass independent controls.

The gate scans index and working files, archive names/decompressed contents and
all reachable Git object/ref contents. No extraction, archived execution, network
or dependency was added. Read/malformed/unsafe/tool errors exit 2; identity hits
exit 1. Bounds: 64 MiB per input, 512 MiB aggregate, 100,000 items, depth 8 and
30 seconds per Git command. Unsupported ZIP compression and ambiguous shapes
fail closed. Original task controls and review reports are retained privately.

Preparation tree and history scans each exit 1: original historical material is
still present by design. This is unresolved release scope, not a clean-tree pass.
Unaffected Rust checks were not repeated for the Python-only task. C17 must assess
the complete clean export and fresh history.
