## Why

The latest feedback implementation expects Gitea's pull-request detail response
to be a JSON object, but the installed `tea` client returns a one-element JSON
array. As a result, `idea-process feedback` fails before returning the comments
and review data it was changed to expose.

## What Changes

- Accept and normalize the array-shaped Gitea pull-request feedback response.
- Preserve support for object-shaped provider responses where applicable.
- Return the stable feedback structure, including comments and reviews, after
  provider-shape normalization.
- Add regression coverage using the actual Gitea response shape.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `development-agent-idea-processing-cli`: require feedback retrieval to accept
  the provider's array-shaped Gitea response and return normalized feedback.

## Impact

- Gitea feedback parsing in `idea-process feedback`.
- Provider fixture and CLI integration tests.
- No change to command-line arguments, Git transport, authentication, or
  pull-request mutation behavior.
