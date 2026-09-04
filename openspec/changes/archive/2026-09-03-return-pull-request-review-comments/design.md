## Context

See `proposal.md` for the motivation. The current feedback path parses the
provider's pull-request summary and applies a size bound, but the Gitea summary
returned by `tea pr show --output json` does not contain general comments or
review decisions.

## Goals / Non-Goals

**Goals:**

- Return comments and reviews through the existing `idea-process feedback`
  command.
- Expose one stable JSON shape for GitHub and Gitea consumers.
- Preserve bounded output, redaction, and successful empty collections.
- Keep provider authentication and remote selection unchanged.

**Non-Goals:**

- Mutating comments, reviews, or pull requests.
- Implementing a persistent feedback cache or polling scheduler.
- Returning arbitrary provider-specific payloads to callers.

## Decisions

- **Use a normalized feedback model.** Represent pull-request metadata separately
  from `comments` and `reviews`, with common fields for author, body, timestamp,
  and type/state. Provider-specific fields may be discarded rather than leaking
  unstable schemas.
- **Fetch comments through the provider-supported read operation.** Continue
  using the existing PR-detail query for metadata, and add the appropriate
  read-only comment/review query for providers whose PR detail output omits
  discussion data. Do not infer comments from the PR description.
- **Apply bounds after normalization.** Limit the number and size of returned
  comments and reviews using the existing feedback bound, and include an
  explicit truncation indicator when data is omitted.
- **Fail explicitly on malformed feedback payloads.** A provider response that
  cannot be normalized should produce a redacted structured provider error rather
  than silently returning metadata-only feedback.
- **Test provider adapters independently.** Use captured GitHub and Gitea JSON
  fixtures plus fake provider executables to verify command arguments, parsing,
  normalization, bounds, and empty results without live mutations.

## Risks / Trade-offs

- [Provider APIs expose different comment and review concepts] → Normalize only
  shared fields and retain an explicit provider-independent type/state.
- [Large or sensitive review content] → Reuse bounded feedback handling and
  redact provider command diagnostics.
- [Additional provider calls can fail independently] → Return an actionable
  structured provider error rather than claiming there is no feedback.

## Migration Plan

No remote migration is required. Consumers continue invoking the same command;
the feedback result gains populated `comments` and `reviews` collections. Update
consumers to tolerate and use the additional fields, then deploy the new Sceptre
binary. Rollback is a binary rollback because no remote state is changed.
