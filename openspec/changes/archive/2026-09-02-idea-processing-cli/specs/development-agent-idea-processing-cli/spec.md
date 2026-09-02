## MODIFIED Requirements

### Requirement: The CLI exposes machine-readable idea-processing operations

The CLI SHALL expose `idea-process pending`, `status`, `prepare`, `publish`, and
`feedback`. Every operation SHALL emit one JSON object on success or failure.
Pending discovery SHALL scan nested `ideas/**/IDEA.md` records on the real
Grimoire catalog and include matching provider pull-request state.

### Requirement: Branch and pull-request operations are idempotent

`prepare` and `publish` SHALL use `spec/<idea-id>`. Publication SHALL reuse one
matching pull request and SHALL not create a commit when the staged tree is
unchanged.
