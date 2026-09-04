## 1. Provider Feedback Normalization

- [x] 1.1 Inspect the current Gitea `tea pr show --output json` contract and
  define the accepted response shapes; verify the target PR is selected
  unambiguously from a non-empty array
- [x] 1.2 Normalize a one-record Gitea feedback array into the internal feedback
  model and return a structured not-found error for an empty array; verify
  object and array inputs produce equivalent valid results where supported
- [x] 1.3 Preserve comment and review extraction, bounds, truncation indicators,
  and redacted errors after array normalization; verify malformed payloads fail
  explicitly rather than becoming metadata-only results

## 2. Regression Tests

- [ ] 2.1 Add a fixture matching the installed `tea` array-shaped response and
  verify `idea-process feedback` succeeds for Grimoire PR #1-like data
- [x] 2.2 Add fixtures containing general comments and review decisions and
  verify authors, bodies, timestamps, types, and states are returned
- [x] 2.3 Add tests for empty arrays, multiple ambiguous records, malformed
  arrays, and existing object-shaped responses; verify each expected structured
  error or normalized result
- [ ] 2.4 Add an end-to-end fake-provider test proving a newly added Gitea PR
  comment appears in the command's JSON output

## 3. Documentation and Verification

- [x] 3.1 Update Sceptre README/help documentation with the accepted Gitea
  feedback response handling and verify the documented output contract
- [ ] 3.2 Run `cargo fmt --check`, `cargo test --all-targets`, and
  `cargo clippy --all-targets --all-features -- -D warnings` when supported;
  verify all checks pass
