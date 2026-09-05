## Purpose

Provide Sceptre CLI operations used by the implementation service to select
specification work, materialize catalog inputs, and manage implementation pull
requests.

## Requirements

### Requirement: The CLI exposes implementation workflow operations

Sceptre SHALL provide commands for specset eligibility, next-specification
selection, implementation status, specification materialization, pull-request
publication, and review-feedback retrieval. Each command SHALL support
machine-readable JSON output.

#### Scenario: Service selects implementation work

- **WHEN** the service invokes implementation selection for a specset
- **THEN** Sceptre SHALL return the selected repository, specification,
  assigned agent, dependencies, and branch identity

#### Scenario: Service retrieves requested changes

- **WHEN** the service requests feedback for an implementation pull request
- **THEN** Sceptre SHALL return bounded review state and feedback associated with
  that pull request

### Requirement: The CLI validates specset readiness

Sceptre SHALL validate repository mappings, confirmed upstream state, agent
records, agent-prefixed specification directories, implementation order, and
same-repository `spec_dependencies`. It SHALL reject missing references and
dependency cycles before returning an implementation target.

#### Scenario: Specset is not ready

- **WHEN** a repository, agent, specification, or dependency reference is
  invalid
- **THEN** Sceptre SHALL return a structured blocked result and SHALL not prepare
  a feature branch or pull request

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

### Requirement: The CLI does not own implementation workers

Sceptre SHALL not schedule services, start OpenCode sessions, create or remove
worktrees, select prompts, or deploy configurations. Those responsibilities
SHALL remain with the Nixpkgs Apocrypha service and the registered agents.

#### Scenario: Service invokes Sceptre

- **WHEN** the Nixpkgs Apocrypha service invokes Sceptre
- **THEN** Sceptre SHALL perform the requested catalog or remote Git operation
  and return a structured result without starting a second worker
