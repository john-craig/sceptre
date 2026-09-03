## 1. Publication Path Resolution

- [x] 1.1 Inspect the Grimoire catalog and publication inputs, then implement
  repository-root `specs/` path resolution in the idea-processing publisher;
  verify the resolved path is rejected with a structured error when absent
- [x] 1.2 Replace root-only staging with nested-safe staging of the `specs/`
  subtree while preserving repository-relative paths; verify unrelated files
  are not staged
- [x] 1.3 Preserve deterministic branch, commit, push, and pull-request
  idempotency after the staging-path change; verify retries reuse the matching
  branch and pull request

## 2. Regression Coverage

- [x] 2.1 Add a fixture containing nested `specs/<project>/<topic>/` planning
  files and verify publication stages and commits the exact nested paths
- [x] 2.2 Add tests for a missing or empty `specs/` subtree and verify no push
  or pull-request operation is attempted
- [x] 2.3 Add a publication retry test and verify no duplicate commit or pull
  request is created solely because the operation is repeated

## 3. Documentation and Verification

- [x] 3.1 Update CLI help or README examples to document the Grimoire `specs/`
  layout and verify the documented command matches the CLI interface
- [x] 3.2 Run `cargo fmt --check` and `cargo test --all-targets`; verify all
  existing and new tests pass
