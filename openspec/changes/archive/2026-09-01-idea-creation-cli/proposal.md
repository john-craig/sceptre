# Idea creation CLI

Add a machine-readable `sceptre idea create` command for creating canonical
Grimoire ideas from repository mappings. It uses the configured GitHub or Gitea
tooling, creates an isolated temporary clone from the default branch, and
pushes a verified `idea/<id>` branch without retaining local state.

## Status

Implemented and archived after synchronization with the canonical
idea-creation specification.
