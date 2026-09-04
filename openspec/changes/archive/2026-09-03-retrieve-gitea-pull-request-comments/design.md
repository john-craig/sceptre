## Context

See `proposal.md` for the motivation. Gitea's `tea pulls <number> --comments`
command displays the desired comment, but the installed client emits human-
readable output even when JSON output is requested. The existing `tea pr show`
path is machine-readable but metadata-only.

## Goals / Non-Goals

**Goals:**

- Retrieve Gitea comments and review decisions as structured data.
- Preserve the existing stable feedback model and bounds.
- Reuse configured Gitea authentication without putting credentials in CLI
  arguments or logs.
- Keep feedback retrieval read-only and preserve GitHub behavior.

**Non-Goals:**

- Parsing terminal-formatted `tea` output as the primary protocol.
- Changing `tea` itself or mutating pull requests.
- Adding a polling scheduler or persistent feedback cache.

## Decisions

- **Use a structured provider/API path for comments.** Prefer a machine-readable
  capability exposed by the configured Gitea integration; if the installed `tea`
  version cannot provide it, use the authenticated Gitea read API through the
  project's supported credential mechanism. Do not parse terminal formatting.
- **Normalize provider data before bounding it.** Convert comments and reviews
  into the existing provider-independent fields, then apply count and body-size
  limits and set the truncation flag.
- **Keep metadata and discussion retrieval separate.** The existing PR detail
  query remains responsible for metadata; the structured discussion query adds
  comments and reviews and is combined before returning the result.
- **Make provider failures explicit.** If the discussion query fails or returns
  malformed data, return a structured error instead of silently returning empty
  collections.
- **Test against the real observed comment.** Add a fixture containing the
  Grimoire PR comment and fake provider/API responses that verify it reaches the
  final CLI JSON output.

## Risks / Trade-offs

- [Gitea and GitHub model reviews differently] → Preserve the common normalized
  fields and discard provider-only fields that cannot be represented safely.
- [Provider API authentication differs from `tea` configuration] → Use an
  already-supported configured credential path and document any new required
  environment or configuration before implementation.
- [Large discussion histories] → Apply existing bounds after normalization and
  report truncation explicitly.

## Migration Plan

No remote migration is required. Upgrade Sceptre and rerun the existing
`idea-process feedback` command. Consumers receive the same top-level operation
result with populated comments and reviews when available. Rollback is a binary
rollback because no remote state is changed.
