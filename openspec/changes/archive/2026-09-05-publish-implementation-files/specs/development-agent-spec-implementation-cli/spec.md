## MODIFIED Requirements

### Requirement: The CLI manages deterministic implementation branches and PRs

The CLI SHALL prepare or reuse `feature/<specset-id>` in the target repository,
materialize the relevant Grimoire specifications into a caller-provided
worktree, and open or update exactly one matching pull request. Publication
SHALL include the materialized specification and the implementation changes
explicitly selected by the caller. The CLI SHALL reject publication when the
selected paths are missing, unsafe, or outside the target worktree, and SHALL
not stage unrelated worktree content. It SHALL not create duplicate branches or
pull requests on retry.

#### Scenario: New implementation is published

- **WHEN** the service publishes a validated implementation commit and provides
  an explicit set of implementation paths
- **THEN** Sceptre SHALL stage the materialized specification and exactly those
  implementation paths, push the feature branch, and open one pull request
  against the configured upstream default branch

#### Scenario: New implementation files are published

- **WHEN** an explicitly selected implementation path is an untracked file or
  directory within the target worktree
- **THEN** Sceptre SHALL include it in the implementation commit

#### Scenario: Implementation deletions are published

- **WHEN** an explicitly selected implementation path contains a tracked file
  deleted by the implementation
- **THEN** Sceptre SHALL include that deletion in the implementation commit

#### Scenario: Unrelated worktree content is excluded

- **WHEN** the target worktree contains changes outside the materialized
  specification and explicitly selected implementation paths
- **THEN** Sceptre SHALL leave those changes unstaged and SHALL not include them
  in the commit or pull request

#### Scenario: Publication input is unsafe

- **WHEN** a selected path is absolute, escapes the target worktree, names an
  invalid publication input, or cannot be resolved
- **THEN** Sceptre SHALL return a structured input error before pushing or
  opening a pull request

#### Scenario: Existing implementation is revised

- **WHEN** the service publishes a revision for a changes-requested pull request
- **THEN** Sceptre SHALL update the existing branch and pull request using the
  same explicit publication scope
