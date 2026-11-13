# T35: Verified private preservation

Input original revision: `1713fd34d3c9186b3df16bf2a9587ca6dc28204e`.
Preparation revision: `169d518`.

A durable access-restricted private copy preserves all original reachable Git
history and 273 allowed working-tree files, including all six uncommitted planning
inputs. No original Git configuration or credentials were copied. Generated build
trees are excluded. Original active source remains recoverable.

Fresh verification on 13 November 2026:

- Actual `git bundle verify`, manifest `sha256sum --check`, restored saved ref/HEAD
  and `git fsck --full --no-reflogs`: exit 0; 1,147 reachable objects.
- All 273 restored files match byte SHA-256 and permissions; all 270 original
  tracked Git modes match. Parsed JSON fixtures and locked configuration match.
- Independent audit verifies 831 checkpoint and 160 oracle archive members,
  479 indexed reachable Git blobs, 87 historical source/config records and 103
  C11 historical inputs against their declared original revision.
- Original evidence/validation/ship/performance checksum manifests pass, including
  15 internal ship-native checks. Private directory permissions are 0700.
- `git diff --check`: exit 0. No engine code or quality policy changed.

Exact manifests, restore results and independent reports are retained privately.
No historical script, test or benchmark was rerun; scratch paths outside the
original evidence inventory are not claimed recoverable. This record contains no
private preservation path or original raw evidence.
