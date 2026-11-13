---
name: code-simplify
description: Simplify code for clarity and maintainability — reduce complexity without
  changing behavior
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


Invoke the agent-skills:code-simplification skill.

Simplify recently changed code (or the specified scope) while preserving exact behavior:

1. Read CLAUDE.md and study project conventions
2. Identify the target code — recent changes unless a broader scope is specified
3. Understand the code's purpose, callers, edge cases, and test coverage before touching it
4. Scan for simplification opportunities:
   - Deep nesting → guard clauses or extracted helpers
   - Long functions → split by responsibility
   - Nested ternaries → if/else or switch
   - Generic names → descriptive names
   - Duplicated logic → shared functions
   - Dead code → remove after confirming
5. Apply each simplification incrementally — run tests after each change
6. Verify all tests pass, the build succeeds, and the diff is clean

If tests fail after a simplification, revert that change and reconsider. Use `code-review-and-quality` to review the result.
