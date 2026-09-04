## Why

The `tea` client can display Gitea pull-request comments with
`pulls <number> --comments`, but its machine-readable PR detail output omits
those comments. Sceptre therefore cannot return the review feedback required by
the idea-processing workflow even though the provider exposes it interactively.

## What Changes

- Add a structured, read-only retrieval path for Gitea pull-request comments and
  review decisions.
- Merge the retrieved data into Sceptre's stable `idea-process feedback` result.
- Preserve provider-independent fields, output bounds, truncation reporting, and
  redacted errors.
- Keep SSH Git transport and `tea` authentication behavior unchanged.
- Add regression coverage proving the Grimoire PR comment is returned.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `development-agent-idea-processing-cli`: require Gitea feedback retrieval to
  return comments and reviews through a structured provider query rather than
  relying on metadata-only PR output.

## Impact

- Gitea feedback provider integration and `idea-process feedback` output.
- Possible use of a structured Gitea API/client request if `tea` cannot provide
  machine-readable comments.
- Provider fixtures, CLI tests, and feedback documentation.
- No changes to pull-request mutation, Git transport, or orchestration.
