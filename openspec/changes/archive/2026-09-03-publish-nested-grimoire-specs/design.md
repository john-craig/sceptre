## Context

The existing idea-processing publisher uses a fixed repository-root `openspec`
path when staging a Grimoire worktree. The change is described in
`proposal.md`; the relevant behavioral contract is in the delta spec.

## Goals / Non-Goals

**Goals:**

- Publish nested Grimoire planning trees without losing their relative paths.
- Keep publication deterministic and fail closed when there is nothing to
  publish.
- Preserve the existing provider, branch, and pull-request workflow.

**Non-Goals:**

- Redesigning Grimoire's catalog layout.
- Publishing implementation specifications to downstream repositories.
- Adding worker orchestration, worktree creation, or a new provider.

## Decisions

- **Treat the repository-root `specs/` directory as the source of truth.** The
  publisher should resolve planning files from that worktree subtree rather than
  assuming `openspec/` at the root. This preserves arbitrary nested project and
  topic directories and matches the Grimoire catalog layout.
- **Stage the resolved subtree with Git pathspec behavior.** Git should receive
  the exact resolved relative path, allowing nested directories while avoiding
  unrelated repository changes. A clean index after staging is an error before
  push or PR operations.
- **Keep branch and PR identity unchanged.** The deterministic
  `spec/<idea-id>` branch and existing provider upsert logic remain the public
  workflow contract; only source-path discovery and staging change.
- **Use fixture repositories for regression coverage.** Tests should create the
  same nested layout as Grimoire and assert the committed paths and no-op error
  behavior without contacting a provider.

## Risks / Trade-offs

- [Missing planning root] → Validate `specs/` and return a structured input
  error rather than publishing an empty branch or PR.
- [Unrelated files accidentally published] → Stage only the resolved planning
  subtree and assert the staged file list in tests.
- [Existing consumers rely on root `openspec/`] → Retain root-level support when
  it is the configured planning path; nested support is additive.

## Migration Plan

No data migration is required. Deploy the Sceptre binary, then retry the normal
idea-processing publication command. Existing branches and matching pull
requests remain reusable. Rollback consists of reverting to the previous
binary; no remote data format changes are introduced.
