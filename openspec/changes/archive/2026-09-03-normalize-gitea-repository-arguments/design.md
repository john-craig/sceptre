## Context

See `proposal.md` for the motivation. Provider operations currently receive the
same upstream string used to identify a repository and to configure Git
transport, although `tea` and Git require different representations.

## Goals / Non-Goals

**Goals:**

- Separate provider repository identifiers from Git transport URLs.
- Normalize HTTPS, `ssh://`, and SCP-style Gitea inputs safely.
- Apply the normalized identifier consistently to all `tea` operations.
- Keep GitHub behavior and SSH Git transport unchanged.

**Non-Goals:**

- Changing repository configuration formats.
- Replacing `tea` or adding a Gitea API client.
- Changing authentication, credential storage, or remote URL selection.

## Decisions

- **Introduce a Gitea-only repository identifier conversion.** Parse the
  configured upstream into its host and path, remove an optional `.git` suffix,
  and require exactly two non-empty path components: owner and repository. This
  matches `tea --repo` semantics and makes malformed input fail before a remote
  command runs.
- **Keep transport conversion independent.** Continue using the existing Git
  URL conversion for clone and push. The provider identifier is an API-tool
  argument, not a replacement for the Git remote URL.
- **Normalize at every Gitea call boundary.** Repository existence, pull-request
  listing, showing, creation, editing, and merging must all use the same helper;
  this prevents one workflow from regressing after another is fixed.
- **Test command arguments through a fake provider executable.** A test fixture
  should capture `tea` arguments and return representative JSON, proving both
  normalization and that the SSH URL is not passed as `--repo`.

## Risks / Trade-offs

- [Unexpected Gitea URL forms] → Reject malformed forms explicitly and cover
  supported HTTPS, `ssh://`, SCP-style, port, and `.git` variants with tests.
- [Provider identifier differs from Git path] → Treat them as separate values;
  never derive the Git remote from the `tea` argument.
- [Different `tea` versions vary in output] → Keep machine-readable output
  flags explicit and retain JSON validation around provider responses.

## Migration Plan

No repository or configuration migration is required. Upgrade Sceptre and retry
the existing workflow. If normalization rejects an existing upstream, correct
the repository mapping before retrying. Rollback is a binary rollback because
the change does not alter remote data.
