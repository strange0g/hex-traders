---
name: spec
description: Start spec-driven development — write a structured specification before
  writing code
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


Invoke the agent-skills:spec-driven-development skill.

Begin by understanding what the user wants to build. Ask clarifying questions about:
1. The objective and target users
2. Core features and acceptance criteria
3. Tech stack preferences and constraints
4. Known boundaries (what to always do, ask first about, and never do)

Then generate a structured spec covering all six core areas: objective, commands, project structure, code style, testing strategy, and boundaries.

If the request bundles several independently testable capabilities, first propose a capability map (module ids, dependency direction, build order) per the skill's Phase 0 and get it approved, then spec each module in dependency order.

Save the spec as SPEC.md in the project root and confirm with the user before proceeding.
