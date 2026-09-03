## Why

Sceptre correctly uses an SSH upstream URL for Git transport, but it currently
passes the configured full repository URL directly to `tea` for Gitea API
operations. The installed `tea` client expects an `owner/repository` identifier,
so pull-request discovery fails before Sceptre can create or update a PR.

## What Changes

- Normalize supported Gitea upstream forms into the repository identifier
  expected by `tea`.
- Use the normalized identifier for every Gitea provider operation while
  preserving the SSH URL for Git clone, fetch, and push operations.
- Return a structured validation error for malformed Gitea upstream paths.
- Add provider-command regression coverage for URL normalization and argument
  separation.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `development-agent-idea-processing-cli`: require Gitea pull-request operations
  to use a valid provider repository identifier independently of Git transport.

## Impact

- Gitea provider argument construction for repository and pull-request commands.
- Idea-processing and other provider-backed CLI workflows.
- Unit and integration tests for provider command invocation.
- No change to Git remote URLs, credentials, GitHub behavior, or orchestration.
