## Context

See `proposal.md` for the motivation. The current feedback implementation expects
the Gitea pull-request detail payload to be a JSON object, while the supported
`tea` client returns a JSON array even when it contains one pull request. This
causes feedback retrieval to fail before comments or reviews can be normalized.

## Goals / Non-Goals

**Goals:**

- Accept the actual array-shaped Gitea feedback payload.
- Preserve the stable feedback model introduced by the preceding change.
- Return a clear not-found error for an empty array.
- Keep GitHub and existing object-shaped provider handling intact.

**Non-Goals:**

- Changing the `tea` client or its output format.
- Changing pull-request comments, reviews, or provider state.
- Expanding the feedback model beyond the prior comments and reviews contract.

## Decisions

- **Normalize at the provider boundary.** Convert a Gitea array response into
  the internal pull-request detail representation before common feedback
  extraction. This keeps provider-specific response handling out of the public
  result model.
- **Require one target record.** Accept the relevant object from a non-empty
  array, reject an empty array as not found, and reject ambiguous or malformed
  payloads with structured provider errors rather than silently selecting data.
- **Keep provider commands explicit.** Continue passing Gitea's normalized
  `owner/repository` identifier to `tea` and keep machine-readable output flags
  on every query.
- **Test the installed provider shape.** Add a fixture reproducing the actual
  array response and a fake `tea` executable test that verifies the feedback
  command returns comments and reviews.

## Risks / Trade-offs

- [Future `tea` versions change the shape again] → Validate both supported array
  and object forms, and return a structured error for unknown shapes.
- [An array contains multiple records] → Require the target PR to be selected
  unambiguously or reject the response rather than returning the wrong feedback.
- [Provider detail omits comments] → Keep comment retrieval and normalization
  explicit; do not claim feedback is absent solely because metadata is present.

## Migration Plan

No remote migration is required. Upgrade Sceptre and rerun the existing
`idea-process feedback` command. Consumers retain the stable feedback response;
the provider-specific array is handled internally. Rollback is a binary rollback
because no remote state is changed.
