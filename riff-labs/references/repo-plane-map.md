# Repo And Plane Map

This note maps the public `riffcc` GitHub surface to the current Plane project model in the `riffcc` workspace.

It is intentionally public-safe:

- it focuses on active public repos and public-facing project names
- it may mention a private-backed system by product name when that system is part of the real public story
- it does not enumerate clearly private internal repos just to prove they exist

Audit basis for this note:

- GitHub org audit via `gh repo list riffcc`
- Plane project audit via Palace `plane list project`
- snapshot date: `2026-03-14`

## Direct Matches

These are the clearest one-repo to one-Plane-project links in the current portfolio.

| GitHub repo | Plane project | Notes |
|---|---|---|
| `riffcc/flagship` | `FLAG` / Flagship | User-facing application layer for `riff.cc`. |
| `riffcc/librarian` | `LIBRA` / Librarian | Content ingest and mirroring engine. |
| `riffcc/citadel` | `CITADEL` / Citadel P2P | Core peer-to-peer substrate. |
| `riffcc/dragonfly` | `FLY` / Dragonfly | Metal management and infrastructure product. |
| `riffcc/jetpack` | `JETPACK` / Jetpack | Automation subsystem inside the Dragonfly family. |
| `riffcc/palace` | `PAL` / Palace | Transitional callable capability layer under Codex. |
| `riffcc/codex` | `ROLO` / Rolodex | Public repo name remains `codex`, but the Riff project identity is `Rolodex`. |
| `riffcc/neverust` | `NEVER` / Neverust | Archivist storage node in Rust. |
| `riffcc/lagoon` | `LAGOON` / Lagoon | Communication/community product on the Citadel stack. |
| `riffcc/grand` | `GRAND` / Grand | Experimental Grand project track. |
| `riffcc/eth2077` | `ETH` / ETH2077 | Experimental Ethereum-adjacent project track. |
| `riffcc/llm-code-sdk` | `LCS` / LLM Code SDK | Shared code intelligence and tool substrate. |
| `riffcc/moosefs-patches` | `MFS` / Extended MooseFS | MooseFS extension and storage systems work. |

## Public Supporting Repos

These repos matter, but they are better understood as support layers around larger Plane projects rather than as standalone Plane projects of their own.

### Codex And SoE Layer

- `riffcc/codex`
- `riffcc/codex-mcp`
- `riffcc/codex-desktop-linux`
- `riffcc/Handy`
- `riffcc/better-agentic-extended`

These mostly support:

- `ROLO` / Rolodex
- `PAL` / Palace
- `LCS` / LLM Code SDK
- `ENV` / Riff Environment

The public story here is not “many unrelated repos.” It is a shared agentic development stack.

### Parser And Search Support

- `riffcc/tree-sitter-lean`
- `riffcc/tree-sitter-nim`
- `riffcc/minirust-search`

These are best treated as supporting repos for `LCS` and related code-intelligence work.

### Product And Docs Surface

- `riffcc/riffcc.github.io`
- `riffcc/riff-docs`
- `riffcc/dragonflycomputer.github.io`
- `riffcc/flagship.github.io`
- `riffcc/rifflabs`

These mostly support:

- `DOCS` / Riff Docs
- `FLAG` / Flagship
- `FLY` / Dragonfly

They are public-facing docs or site repos around the core product families rather than primary project engines.

### Product Family Support

- `riffcc/lens-sdk`
- `riffcc/lens-node`
- `riffcc/orbiter-core`
- `riffcc/river`
- `riffcc/rosa`
- `riffcc/torrent2ipfs`
- `riffcc/two-generals`
- `riffcc/rifflabs-infrastructure`

These sit around the main product families:

- `FLAG` / Flagship
- `CITADEL` / Citadel P2P
- `FLY` / Dragonfly
- `JETPACK` / Jetpack

They are important, but they are usually not the first repo a new builder should start from unless the task clearly points there.

The important historical correction here is:

- the old Lens line is not a separate active project family anymore
- `lens-sdk` and `lens-node` should be understood as older public repo surface around work that conceptually folded into the Citadel direction

## Plane Projects Without A Clear Public Repo

These are real project tracks in Plane, but they are not currently represented by one obvious public repo in the `riffcc` org.

### Public-Facing Systems With Private Or Mixed Backing

- `IPFS` / IPFS HA
- `FAB` / Fabrica

These can still be discussed publicly by project name. The important thing is to describe the system, not to leak its private repo layout.

### Project Tracks Whose Repo Surface Is Still Emerging

- `ARCH` / Archivist
- `KEEPER` / Keeper
- `MOOSE` / pve-moosefs
- `ENV` / Riff Environment

For these, the real work is visible in Plane even though the public repo story is still catching up or is spread across several repos.

### Editorial Or Operational Projects

- `CONTENT` / Content Curation

This is intentionally a project, not a software repo. It belongs in the operating model even though it is not a codebase.

## Practical Routing Rules

When using `riff-labs` to orient someone quickly:

1. start with the Plane project when the user already knows the project name
2. route to the direct-match repo first when one exists
3. route to a supporting repo only when the task is clearly about the support layer
4. if the Plane project is real but the repo surface is private or mixed, explain the project by product name and avoid speculating about private repo internals
5. if the Plane project and the repo name differ, prefer the Plane project identity while naming the repo explicitly when needed, as with `ROLO` / Rolodex and `riffcc/codex`

## Current Gaps To Remember

- not every real Riff project has a clean one-repo public home yet
- not every public repo deserves to become a first-class project in the skill
- the skill should preserve the project model first, then use repos as the grounding substrate
