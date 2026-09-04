## 1. Feedback Model

- [x] 1.1 Define the stable machine-readable feedback shape for pull-request
  metadata, comments, reviews, author identity, body, timestamps, type, and
  state; verify the shape is documented and provider-independent
- [x] 1.2 Extend provider feedback retrieval to collect general comments and
  review decisions for GitHub and Gitea; verify metadata-only responses no
  longer discard available feedback
- [x] 1.3 Normalize provider-specific comment and review payloads and preserve
  successful empty `comments` and `reviews` collections; verify malformed
  provider payloads return structured errors

## 2. Bounds and Safety

- [x] 2.1 Apply existing feedback size and count bounds after normalization and
  return an explicit truncation indicator; verify oversized bodies and large
  collections are bounded
- [x] 2.2 Preserve credential isolation and redacted provider diagnostics;
  verify comments, review bodies, and command failures do not expose secrets
- [x] 2.3 Keep feedback retrieval read-only and verify the command does not
  modify pull requests, branches, or local worktrees

## 3. Regression Coverage and Documentation

- [ ] 3.1 Add GitHub and Gitea fixtures containing general comments and review
  decisions; verify author, body, timestamp, type, and state are returned
- [ ] 3.2 Add a regression test for a pull request with no comments or reviews
  and verify successful empty collections are returned
- [ ] 3.3 Add an end-to-end CLI test proving a newly added PR comment appears in
  `idea-process feedback` output
- [x] 3.4 Update README/help documentation with the feedback response shape and
  run `cargo fmt --check`, `cargo test --all-targets`, and clippy when supported
