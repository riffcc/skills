---
themes:
  - agentic-engineering
  - external-memory
related:
  - [[Palace]]
---
# Obsidian Syntax Reference

## 1. Obsidian Flavored Markdown (OFM)
Core baseline:
- CommonMark + GitHub Flavored Markdown + LaTeX/MathJax.
- OFM extensions include: wikilinks, embeds, block refs, callouts, comments, footnotes, tasks.

## 2. Links
- Wikilink: `[[Note Name]]`
- Markdown link: `[Note Name](Note%20Name.md)`
- Heading link: `[[Note#Heading]]`
- Nested heading: `[[Note#H2#H3]]`
- Block reference: `[[Note#^block-id]]`
- Alias display text: `[[Note|Display Name]]`
- Vault-wide heading search in link suggester: `[[## heading text]]`
- Vault-wide block search in link suggester: `[[^^block text]]`

## 3. Embeds
- Note embed: `![[Note Name]]`
- Heading/block embed: `![[Note#Heading]]`, `![[Note#^block-id]]`
- Image embed: `![[image.png]]`
- Resize embed: `![[image.png|640x480]]` or `![[image.png|320]]`

## 4. Properties (frontmatter)
- YAML frontmatter wrapper:
```yaml
---
key: value
---
```
- Key points:
  - Property types include text, list, number, checkbox, date, datetime, tags.
  - Type is global per property name across the vault.
  - Internal links in property values should be quoted (`"[[Note]]"`).
  - Canonical reserved/common keys: `tags`, `aliases`, `cssclasses`.

## 5. Tasks and checkboxes
- Open task: `- [ ] Task`
- Done task: `- [x] Task`
- Search operators: `task:`, `task-todo:`, `task-done:`

## 6. Callouts
Pattern:
```md
> [!note]
> Body
```
- Supports types such as `note`, `tip`, `warning`, `info`, etc.

## 7. Comments, footnotes, highlights
- Comment: `%% hidden comment %%`
- Footnote ref: `[^1]` with definition `[^1]: text`
- Highlight: `==text==`
- Strikethrough: `~~text~~`

## 8. Code and diagrams
- Fenced code blocks with language.
- Mermaid diagrams using ` ```mermaid `.
- MathJax inline/block expressions using `$...$` and `$$...$$`.

## 9. Query blocks and search syntax
Embed search results:
````md
```query
tag:#ops AND [status:blocked]
```
````
Search operators include:
- `file:`, `path:`, `content:`
- `tag:`, `line:`, `block:`, `section:`
- `task:`, `task-todo:`, `task-done:`
- Property search: `[property]`, `[property:value]`, `[property:null]`
- Boolean: `OR`, `-` negation, grouping with `(...)`
- Regex with `/pattern/`

## 10. Obsidian URI automation
Custom URI format:
- `obsidian://open?...`
- `obsidian://new?...`
- `obsidian://daily?...`
- `obsidian://search?...`
Used for cross-tool automation and scripts.

## 11. Version-sensitive features
- Obsidian CLI is early access and version-gated in official docs.
- Keep CLI instructions version-specific and optional.
