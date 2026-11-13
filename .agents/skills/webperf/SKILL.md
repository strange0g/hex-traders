---
name: webperf
description: Run a web performance audit via the web-performance-auditor persona
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


`/webperf` targets web applications specifically. Do not use it for utility libraries, CLIs, or server-only code with no browser-facing output.

## Determine the mode

**Deep mode** — activate when any of these is available:
- A Lighthouse JSON report file (e.g. `npx lighthouse <url> --output json --output-path ./report.json`, or `npx -p chrome-devtools-mcp chrome-devtools lighthouse_audit --output-format=json` from the Chrome DevTools MCP CLI)
- A PageSpeed Insights JSON response (includes Lighthouse + CrUX)
- A CrUX API response (requires `CRUX_API_KEY` or `GOOGLE_API_KEY`)
- A DevTools performance trace
- A live URL plus the `chrome-devtools` MCP server configured in the harness (the agent can capture metrics directly via `lighthouse_audit` and `performance_*` tools)
- The Chrome DevTools MCP CLI invoked locally (via `npx -p chrome-devtools-mcp chrome-devtools <tool>` or after `npm i -g chrome-devtools-mcp`) — the user runs commands like `chrome-devtools lighthouse_audit --output-format=json` and passes the JSON output to the agent

**Quick mode** — default when none of the above are available. The agent scans source code for structural anti-patterns and labels every finding as `potential impact`.

## Run the audit

Spawn the `web-performance-auditor` subagent. Pass it explicitly:

- The files, components, or diff under review
- Any artifact paths (Lighthouse JSON, PSI JSON, CrUX response, trace) or pasted JSON content
- The target URL or page name when known
- A note on which mode you expect (Quick or Deep), so the agent surfaces missing inputs if Deep was intended

The subagent returns a scorecard (only populated with sourced values), a ranked list of findings, positive observations, and proactive recommendations.

## Output

Return the full audit report to the user. No synthesis or merge step is needed — this is a single-persona command.
