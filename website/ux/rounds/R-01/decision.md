# R-01 decision D1: add a local search to the docs site's navbar?

Finding: UXF-002. Target metric: `always_visible.hidden` on `site:all`, now 32 of the after-fix
count (16 pages x 2 themes, `top` and `end` of each page are one each), expected 0 for search.

Yes: add `@easyops-cn/docusaurus-search-local` 0.55.3 (the plugin the slot docs site uses) to
`website/package.json` and `website/docusaurus.config.ts`, remove the UXF-002 entry from the
adapter's `KNOWN_PROBLEMS`; the gate then holds search on every page. Shot: none taken (the
dependency was not installed this round).
No: the repository build stays without search; UXF-002 stays open with its gate exception, and
search is left to the published site (Website and docs-system own search there).

Default if unanswered by the next round: yes, because the skill's docs-site adapter names search as
always visible, the change is one dependency and one config line, and it is reverted by removing
both.

Why it is a question: this session's permission check refused `npm install` of the new
dependency, so it was not added in R-01.

Answer: open
