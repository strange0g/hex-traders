# Project-local Agent Skills installation

Historical project-local installation: 9 November 2026.

This public derivative retains the installation facts and validation limits.
The original document is privately preserved under SHA-256 ID
`79fb9ddf83cab3eb5566668ba458d46b8a0a089243158e3edf586a8d302f28f8`.
T35 verified inventoried original documents, archived members and recorded Git
blob copies byte-for-byte. The installation results below describe that date;
they are not a fresh validation of current product settings or documentation.

Source: [addyosmani/agent-skills](https://github.com/addyosmani/agent-skills/tree/301b02b12bff98e4688e6984af6556802322bbde), pinned to commit `301b02b12bff98e4688e6984af6556802322bbde`.

## Installed files

- [.agents/skills/](.agents/skills/): all 25 upstream skills, including their bundled scripts and references, plus nine command skills (34 total).
- [.agents/references/](.agents/references/): all seven shared checklists, preserving the upstream relative paths.
- [.agents/commands/](.agents/commands/): byte-for-byte copies of all nine original Claude command files.
- [.agents/agents/](.agents/agents/): byte-for-byte copies of all four original persona files.
- [.agents/docs/agents.md](.agents/docs/agents.md): the original persona composition guide referenced by those files.
- [.agents/LICENSE](.agents/LICENSE): the original MIT license.
- [.agents/SOURCE.json](.agents/SOURCE.json): source commit and installation inventory.
- [.codex/agents/](.codex/agents/): four standalone project-scoped custom agent TOMLs.

No global skills, plugin registrations, dependencies, or configuration were installed. Existing files were preserved. The project had no application files to change.

## Using the command skills

Use these skills in a new project turn. If the selector has not refreshed, reopen the project chat. You can invoke them by name, for example: `Use $plan to plan the feature described in SPEC.md`.

| Original Claude command | Codex skill | Purpose |
| --- | --- | --- |
| `/spec` | `$spec` | Write a specification |
| `/plan` | `$plan` | Plan tasks, dependencies, acceptance criteria, and verification |
| `/build` | `$build` | Implement the next task; `$build auto` runs the approved plan |
| `/test` | `$test` | Test-driven development and bug reproduction |
| `/constraints` | `$constraints` | Define and enforce the project's quality bar |
| `/review` | `$review` | Review correctness, readability, architecture, security, and performance |
| `/webperf` | `$webperf` | Run a web performance audit |
| `/code-simplify` | `$code-simplify` | Simplify code while preserving behavior |
| `/ship` | `$ship` | Coordinate specialist reviews and produce a ship decision |

Every converted `SKILL.md` contains the entire original command body verbatim, preceded by a small, separately labeled Codex compatibility section. The original files, including their original frontmatter, are also retained in `.agents/commands/`.

The compatibility section maps namespaced skill references to the installed local skills, slash commands to skill invocations, command arguments to the user's prompt, Claude's Agent tool to Codex subagent tools, and pack-relative paths to `.agents/`. When `CLAUDE.md` is absent, project-guidance instructions use `AGENTS.md`. Planning keeps implementation read-only while allowing the requested plan artifacts to be written. No upstream workflow steps or approval gates were removed.

The nine command skills retain the upstream explicit-invocation behavior through `agents/openai.yaml` with `allow_implicit_invocation: false`. The 25 original skills retain their upstream discovery behavior.

## Historical custom-agent setup

The [official OpenAI custom agent documentation](https://learn.chatgpt.com/docs/agent-configuration/subagents#custom-agents) specifies standalone `.codex/agents/*.toml` files with `name`, `description`, and `developer_instructions`. This installation follows that schema.

| Agent | Project TOML | Role |
| --- | --- | --- |
| `code-reviewer` | [.codex/agents/code-reviewer.toml](.codex/agents/code-reviewer.toml) | Staff engineer review across five dimensions |
| `security-auditor` | [.codex/agents/security-auditor.toml](.codex/agents/security-auditor.toml) | Vulnerability detection and threat analysis |
| `test-engineer` | [.codex/agents/test-engineer.toml](.codex/agents/test-engineer.toml) | Test design and coverage analysis |
| `web-performance-auditor` | [.codex/agents/web-performance-auditor.toml](.codex/agents/web-performance-auditor.toml) | Web performance audits with evidence-based metrics |

Each agent embeds its full original persona body, with a brief path and tool compatibility note. No model, reasoning, sandbox, or approval override was added; agents inherit the parent session's settings. The installation report records that no `.codex/config.toml` registration was needed for the documented standalone format and that subagent tools were enabled by default at that time.

Example invocation: `Use the code-reviewer subagent to review this change and return its findings.` The `$ship` and `$webperf` command skills also request their corresponding personas.

For local skill discovery, the [official OpenAI skill documentation](https://learn.chatgpt.com/docs/build-skills#where-to-save-skills) specifies the project's `.agents/skills/` directory.

## Historical verification and limits

- All 34 skill folders passed the bundled Codex skill-creator validator.
- All 44 files in the original skills, shared references, and personas matched upstream byte-for-byte.
- All nine archived Claude commands matched upstream byte-for-byte; all converted skill bodies preserved the original command body exactly.
- All four TOMLs parsed with Python's TOML parser; their names, descriptions, and embedded persona bodies matched upstream.
- All 56 checked skill reference paths resolved; the retained composition guide and license matched upstream.
- The same checks passed against the final installed project files.
- Installed CLI version checked: `codex-cli 0.166.0`.

Live agent spawning and execution of the workflows were not tested during
installation. Later autonomous engine and release tasks used subagents; that
separate execution evidence is recorded in [tasks/todo.md](tasks/todo.md).
It does not retroactively expand the installation checks. Browser performance capture still requires suitable artifacts or browser tooling, as described in the original skills. This is a pinned local copy; future upstream updates are not applied automatically.
