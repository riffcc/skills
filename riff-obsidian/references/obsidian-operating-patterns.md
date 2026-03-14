---
created: "2026-03-03T07:36"
updated: "2026-03-03T07:36"
themes:
  - agentic-engineering
  - external-memory
related:
  - [[Palace]]
---
# Obsidian Operating Patterns

## Pattern A: Company Operating System

The vault is organised into plain-named folders with no numeric prefixes:

- `Inbox` — capture now, triage later
- `Command` — company direction, team roster, operating philosophy, decision records
- `Projects` — one note per project with registry and decomposition
- `Systems` — tooling docs, operational queries, infrastructure notes
- `Roadmap` — timeline views, milestone plans, recovery programs
- `Internals` — SoE megaproject structure (epics, sprints, tooling, workflows, automation, dashboards)
- `Templates` — reusable note templates for mail, tasks, decisions
- `Mail/Wings` — inbound agent mail for Wings
- `Mail/Agents` — agent-to-agent coordination mail

Core properties for operational notes:

| Property | Type | Values |
|----------|------|--------|
| `type` | text | decision, plan, task, mail, project, system |
| `status` | text | new, triaged, in_progress, blocked, resolved, archived |
| `owner` | text | person or agent id |
| `project` | text | project code (e.g. FLAG, CITADEL, FLY) |
| `priority` | text | low, normal, high, critical |
| `due` | date | YYYY-MM-DD |
| `created` | date | YYYY-MM-DD |

## Pattern B: Agent Mail System

Mail filenames use spaces: `2026-03-03 1430 codex to wings Plane decomposition blocker.md`

Template frontmatter:

```yaml
---
status: new
from: codex
to: wings
subject: ""
project: OPS
priority: normal
created: 2026-03-03
due:
action_required:
related_links: []
---
```

The body follows a standard structure: Context, Request, Proposed Next Step, and Acceptance Criteria sections.

## Pattern C: Query Dashboards

Wings' mail inbox query:

````md
```query
path:"Mail/Wings" AND [status:new OR triaged OR blocked]
```
````

Blocked work across the vault:

````md
```query
[status:blocked] OR task-todo:"blocked"
```
````

## Pattern D: Bases for Execution

Build `.base` views for projects, tasks, decisions, and risks. Use property filters to drive sprint and milestone views. These give database-like table and card views without leaving Obsidian.

## Pattern E: URI-Driven Automation

Open a note: `obsidian://open?vault=Riff%20Labs&file=Command%2FOperating%20Philosophy.md`

Create an inbox capture: `obsidian://new?vault=Riff%20Labs&file=Inbox%2Fquick%20capture.md&content=Captured%20from%20automation`

Run a search: `obsidian://search?vault=Riff%20Labs&query=tag:%23blocked`

Spaces in filenames are encoded as `%20` in URIs. This is standard URI encoding and works correctly.
