## 1. Provider Repository Normalization

- [x] 1.1 Implement Gitea repository-identifier normalization for HTTPS,
  `ssh://`, and SCP-style upstreams, including ports and optional `.git`
  suffixes; verify supported forms normalize to `owner/repository`
- [x] 1.2 Reject malformed Gitea upstream paths before invoking provider tools;
  verify structured errors contain no credentials or sensitive URL data
- [x] 1.3 Apply the normalized identifier to every `tea` repository and
  pull-request operation while preserving the SSH upstream for Git operations;
  verify both provider and Git argument paths independently

## 2. Provider Workflow Tests

- [x] 2.1 Add unit tests for URL parsing and normalization across HTTPS,
  `ssh://`, SCP-style, port, suffix, and malformed inputs
- [ ] 2.2 Add a fake `tea` executable fixture that captures arguments and emits
  machine-readable responses; verify `--repo` receives `owner/repository`
- [ ] 2.3 Cover repository lookup, pull-request listing, showing, creation,
  editing, and merging; verify each Gitea operation uses the normalized
  identifier and Git push still uses SSH
- [ ] 2.4 Verify GitHub provider arguments remain unchanged and existing
  authentication failures remain redacted and non-zero

## 3. Documentation and Verification

- [x] 3.1 Document the distinction between Gitea's `owner/repository` provider
  identifier and the SSH Git transport URL; verify examples match the CLI
  behavior
- [ ] 3.2 Run `cargo fmt --check`, `cargo test --all-targets`, and
  `cargo clippy --all-targets --all-features -- -D warnings` when supported;
  verify all checks pass
