---
name: constraints
description: Define and enforce this project's quality bar — interview, sane defaults,
  CONSTRAINTS.md
---

## Codex compatibility

The upstream command body below is preserved verbatim. Run it as a Codex skill:
- Resolve `agent-skills:<skill-name>` to the installed skill at `.agents/skills/<skill-name>/SKILL.md`.
- Treat `/command` and `/agent-skills:command` as `$command` invocations; `$ARGUMENTS` means the user's arguments to this skill.
- Use Codex's subagent tools for Claude's Agent tool, with the matching custom agent name from `.codex/agents/`; keep the original orchestration and review gates.
- Resolve pack-root `references/`, `skills/`, and `docs/` paths under `.agents/`. Resolve persona-relative links from the original file under `.agents/agents/`.
- For project guidance, use the project's `CLAUDE.md` when present; otherwise use `AGENTS.md`, including when the command asks to add project guidance. Claude-specific persona discovery maps to Codex's project and personal custom agent directories.
- "Enter plan mode" means keep implementation read-only while producing the requested planning artifacts.

## Original command


Invoke the agent-skills:constraint-driven-development skill.

$ARGUMENTS

Default behaviour with no arguments: set up constraints for this repository.

1. **Detect first.** Read package.json / pyproject.toml / go.mod, the test runner, existing lint configs, current coverage output, CI workflows, and the agent harness in use. Report what you found in two lines. Never ask for anything you can read.

2. **Interview, at most four questions.** One at a time, each with your best guess and a usable default so "I don't know" still produces a working config:
   - Which dimensions beyond the floor (coverage, security, performance, accessibility, architecture)
   - Block or warn when a check fails mid-task
   - Target numbers, or measure today's values and hold them
   - Slowest check tolerated before handing work back

3. **Write CONSTRAINTS.md** at the repo root with a Floor section, enforced numbers, measured-only metrics with today's values, and an exceptions table with owners and expiry dates. Every number needs a stated reason.

4. **Install what each picked dimension needs.** A dimension with a number and no tool behind it is an aspiration. Use the de facto tool so existing config keeps working: Semgrep for code scanning, gitleaks (always `--redact`) for secrets, osv-scanner for dependencies, axe-core for accessibility, Lighthouse for web vitals, size-limit for bundles, dependency-cruiser for boundaries, Stryker for assertion quality. Record the exact command next to each rule in CONSTRAINTS.md. Accessibility and performance need a running URL; if the project has none, say so and drop the dimension rather than inventing a check. Add the commands to package.json as check:fast / check:task / check:full.

5. **Place each check by cost.** Types, lint and secrets in the edit loop (seconds). Related tests and changed-line coverage at task end (under 90s). Everything else at review or in CI. Scope checks to the diff, not the whole repo.

6. **Point the agent at it.** Add a line to CLAUDE.md telling agents to read CONSTRAINTS.md and never weaken it to make a change pass.

7. **Verify.** Run the constraints against the current branch. If anything fails that the user disagrees with, fix the constraint now rather than leaving a gate people will learn to ignore.

Sub-commands:
- `/constraints check` — run the current constraints against this branch and report
- `/constraints guard` — inspect the diff for a weakened bar: lowered thresholds, skipped or deleted tests, new suppression comments, unfinished stubs, new exceptions
- `/constraints ratchet` — record today's measured values as the floor that must not fall
