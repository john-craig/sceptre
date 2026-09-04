## Why

The `idea-process feedback` command currently reports pull-request metadata but
omits the comments and review decisions that determine whether a specification
needs changes. A coordinator polling the command therefore cannot distinguish
an untouched PR from one that contains actionable reviewer feedback.

## What Changes

- Return pull-request comments and review decisions from the feedback operation.
- Preserve the existing bounded, machine-readable feedback response.
- Normalize GitHub and Gitea provider response shapes into a stable result for
  consumers.
- Bound comment and review content and redact sensitive provider diagnostics.
- Add regression tests proving newly added PR comments are returned.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `development-agent-idea-processing-cli`: require feedback retrieval to include
  actionable pull-request comments and review decisions, not only PR metadata.

## Impact

- `idea-process feedback` provider parsing and JSON output.
- GitHub and Gitea command integrations.
- CLI/provider fixture tests and documentation.
- No change to pull-request mutation, authentication, or orchestration behavior.
