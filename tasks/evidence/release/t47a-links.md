# T47a: Package-document link correction

Independent review identified one remaining security-guide API-contract link to
an omitted Cargo file after T47 had been committed. This focused corrective task
records the omission honestly. SECURITY.md now uses the version-tagged repository
URL; CONTRIBUTING.md states that spec/evidence links require the full Git checkout.
No engine code, fixtures, dependencies, assertions or quality threshold changed.

All 11 rustdoc tests, naming tree/history and diff checking passed. Package links
were inspected for clear repository scope. Actual version URLs remain pending the
T53 published-tag verification. T49 depends on this correction.
