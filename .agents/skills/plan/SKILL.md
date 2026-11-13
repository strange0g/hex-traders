---
name: plan
description: Break work into small verifiable tasks with acceptance criteria and dependency
  ordering
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


Invoke the agent-skills:planning-and-task-breakdown skill.

Read the existing spec (SPEC.md or equivalent) and the relevant codebase sections. Then:

1. Enter plan mode — read only, no code changes
2. Identify the dependency graph between components
3. Slice work vertically (one complete path per task, not horizontal layers)
4. Write tasks with acceptance criteria and verification steps
5. Add checkpoints between phases
6. Present the plan for human review

Save the plan to tasks/plan.md and task list to tasks/todo.md.

If tasks/plan.md or tasks/todo.md already exists with unchecked tasks for different work, stop and ask before writing — never silently overwrite an incomplete plan.
