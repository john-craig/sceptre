## Context

See `proposal.md` for the publication failure being corrected. The current
`specset publish` operation resets the worktree to
`feature/<specset-id>`, stages `openspec/specs`, commits when the index is
non-empty, pushes the branch, and creates or updates the matching pull request.
The implementation worktree may also contain source changes, new files, and
deletions produced by the worker.

## Goals / Non-Goals

**Goals:**

- Allow callers to declare the implementation paths that belong in the PR.
- Stage existing, new, and deleted files under those paths.
- Keep materialized specifications always included.
- Reject unsafe paths before any commit, push, or provider operation.
- Preserve deterministic branch and pull-request idempotency.

**Non-Goals:**

- Automatically discovering which worker changes are intentional.
- Staging arbitrary worktree content by default.
- Changing provider APIs, review policy, merge behavior, or worktree creation.

## Decisions

### Use repeatable relative path arguments

Add a repeatable `--implementation-path PATH` option to `specset publish`. The
caller supplies the reviewed repository-relative files or directories. A
repeatable option keeps the machine interface explicit and avoids introducing a
temporary manifest format for a short-lived publication request.

Alternatives considered:

- `git add -A`: rejected because it can publish unrelated worker or local files.
- A path-list file: rejected because it adds lifecycle and parsing complexity
  without improving the core safety contract.
- Inferring paths from `git diff`: rejected because intent cannot be reliably
  distinguished from unrelated worktree changes.

### Validate paths before branch mutation or provider access

Normalize each path as a relative lexical path, reject absolute paths and
parent-directory traversal, and reject paths that resolve through symlinks
outside the worktree. Permit a path when it exists in the worktree or is a
tracked path in the current branch, so deleted tracked files remain publishable.
Deduplicate normalized paths before staging.

### Stage with explicit Git pathspecs

After branch preparation, stage `openspec/specs` and each validated
`--implementation-path` using Git pathspecs. This captures additions,
modifications, and deletions under the approved scope while leaving unrelated
files unstaged. Preserve the existing no-op, commit, push, and PR update flow.

### Require an explicit implementation scope

`specset publish` SHALL require at least one `--implementation-path` for an
implementation publication. The materialized specification is still staged
automatically. This makes omission visible to callers rather than silently
creating a specification-only PR.

## Risks / Trade-offs

- [Risk] A broad approved directory can still include unrelated files → callers
  must review the staged diff and prefer narrow paths; Sceptre limits only path
  safety, not semantic intent.
- [Risk] Deleted files cannot be filesystem-canonicalized → validate them using
  Git's tracked-path index while applying traversal and symlink checks to their
  surviving parent path.
- [Risk] Existing callers omit the new option → return a structured input error
  before remote mutation, requiring callers to update explicitly.

## Migration Plan

1. Update Sceptre callers and workflow documentation to pass reviewed
   `--implementation-path` arguments.
2. Deploy the updated CLI without changing existing branches or pull requests.
3. Retry failed specification-only publications with the same feature branch and
   explicit implementation paths; the existing PR will be updated.
4. Roll back by using the previous Sceptre package if necessary. No remote data
   migration is required.
