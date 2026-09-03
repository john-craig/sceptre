## Why

Sceptre's idea-processing publisher assumes that Grimoire planning changes live
under a repository-root `openspec/` directory. Grimoire's coordination catalog
uses a repository-root `specs/` directory with nested project and topic
directories, so publishing those changes currently fails or produces an empty
pull request. The publisher needs to stage the actual `specs/` subtree without
weakening its branch and pull-request safety guarantees.

## What Changes

- Make idea-processing publication discover and stage the Grimoire `specs/`
  subtree instead of hard-coding `openspec/` at the repository root.
- Preserve nested paths when detecting changes, committing, pushing, and
  updating the matching pull request.
- Define behavior for a missing planning subtree and for a clean worktree.
- Add regression coverage for nested project/topic layouts and idempotent retry
  behavior.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `development-agent-idea-processing-cli`: require publication to handle the
  actual nested Grimoire planning path while retaining deterministic branch and
  pull-request behavior.

## Impact

- Sceptre's idea-processing publication path and its Git change-detection logic.
- CLI integration tests and fixture layouts.
- No new provider, credential, or orchestration dependency.
