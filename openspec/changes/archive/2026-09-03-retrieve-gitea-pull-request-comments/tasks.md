## 1. Structured Gitea Discussion Retrieval

- [x] 1.1 Determine the authenticated structured Gitea endpoint or provider
  mechanism for pull-request comments and reviews; verify it works with the
  configured `tea` credentials without exposing token values
- [x] 1.2 Implement read-only Gitea discussion retrieval using the normalized
  `owner/repository` identifier and PR number; verify provider failures return
  structured redacted errors
- [x] 1.3 Combine Gitea metadata, comments, and reviews into the existing stable
  feedback model; verify comments are not lost when metadata and discussion
  responses are separate
- [x] 1.4 Preserve count/body bounds and truncation reporting; verify oversized
  discussion histories remain bounded

## 2. Provider and CLI Tests

- [x] 2.1 Add fixtures for the Grimoire PR comment and representative Gitea
  review decisions; verify author, body, timestamp, type, and state are returned
- [x] 2.2 Add fake-provider/API tests for successful comments, empty discussions,
  malformed responses, and authentication failures; verify each result is
  structured and actionable
- [ ] 2.3 Add an end-to-end `idea-process feedback` regression test proving the
  Grimoire PR comment appears in output without modifying the PR or worktree
- [x] 2.4 Verify GitHub feedback behavior remains unchanged and its existing
  comments/reviews continue to normalize correctly

## 3. Documentation and Verification

- [x] 3.1 Update Sceptre README/help documentation with the Gitea feedback
  retrieval behavior and authentication requirements; verify examples match the
  implemented command
- [ ] 3.2 Run `cargo fmt --check`, `cargo test --all-targets`, and
  `cargo clippy --all-targets --all-features -- -D warnings` when supported;
  verify all checks pass
