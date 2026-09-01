# Repository Creation SSH Transport

## Why

Git operations currently use the configured HTTPS upstream URL, which can cause
credential prompts even when the developer has configured SSH credentials.

## What Changes

- Use equivalent SSH URLs for Git clone, push, and local `origin` operations by
  default.
- Preserve the configured HTTPS URL for provider API operations.
- Add an explicit `--https` override for environments that require HTTPS Git
  transport.
- Document the distinction between SSH Git authentication and provider CLI API
  authentication.

## Status

Implemented and archived after synchronization with the canonical
repository-creation specification.
