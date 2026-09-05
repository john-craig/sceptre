## Why

`specset publish` is intended to publish implementation pull requests, but it
currently stages only `openspec/specs/`. As a result, an agent can complete an
implementation while Sceptre creates a pull request containing only the
materialized specification and silently omits the code and tests. The publish
operation needs an explicit, reviewable way to include the implementation
changes before it is used by the automated workflow.

## What Changes

- Make `specset publish` publish the reviewed implementation diff as well as the
  materialized specification.
- Define how the caller identifies the files or paths that are allowed to be
  staged for publication.
- Reject ambiguous or unsafe publication inputs instead of staging unrelated
  worktree content.
- Preserve the existing branch, commit, push, and idempotent pull-request
  update behavior.
- Add tests covering implementation files, new files, deletions, unrelated
  worktree content, and repeated publication.

## Capabilities

### New Capabilities

### Modified Capabilities

- `development-agent-spec-implementation-cli`: `specset publish` must include
  explicitly approved implementation changes in the generated pull request
  without publishing unrelated worktree files.

## Impact

- Affects Sceptre's `specset publish` CLI contract and its Git staging logic.
- Affects callers that invoke `specset publish`; they may need to provide an
  explicit implementation path or manifest of approved paths.
- Adds no new provider or service dependency. Existing Git and `tea`/`gh`
  integrations remain responsible for branch and pull-request operations.
