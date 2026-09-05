## 1. Extend the publish input contract

- [x] 1.1 Add repeatable `--implementation-path` arguments to `specset publish`.
- [x] 1.2 Require at least one implementation path and return a structured input
      error when it is absent.
- [x] 1.3 Document the option in the Sceptre CLI README and workflow examples.

## 2. Implement safe scoped staging

- [x] 2.1 Normalize and validate relative implementation paths before branch
      mutation or provider calls.
- [x] 2.2 Support tracked deletions, untracked files, directories, and nested
      paths without allowing traversal or external symlink targets.
- [x] 2.3 Stage only `openspec/specs` and the validated implementation paths.
- [x] 2.4 Preserve no-op handling, commit messages, branch push behavior, and
      existing pull-request create/update idempotency.

## 3. Verify behavior

- [x] 3.1 Add CLI tests for missing, absolute, traversal, and symlinked paths.
- [x] 3.2 Add publication tests proving modifications, additions, and deletions
      are included while unrelated files remain unstaged.
- [x] 3.3 Add retry coverage for an existing implementation pull request.
- [x] 3.4 Run focused Sceptre tests, `cargo fmt --check`, `cargo clippy`, and
      `cargo test`.
