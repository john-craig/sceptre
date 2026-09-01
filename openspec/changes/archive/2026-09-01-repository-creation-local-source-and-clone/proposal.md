# Repository Creation Local Sources

## Why

Repository creation must support projects that already have a local Git
repository, while preventing accidental publication of a repository that is
already connected to an unrelated upstream.

## What Changes

- Use an existing `local_path` Git repository as the source instead of the
  cataloged template.
- Reject local sources that already define any Git remote before provider
  operations begin.
- Add optional `--clone` behavior for initializing the configured local path
  from the verified upstream.
- Reuse valid no-remote local sources and assign the verified upstream as
  `origin` without overwriting their contents.
- Reject conflicting non-empty clone destinations.

## Status

Implemented and archived after synchronization with the canonical
repository-creation specification.
