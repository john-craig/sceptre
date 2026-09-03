## MODIFIED Requirements

### Requirement: Branch and pull-request operations are idempotent

The CLI SHALL prepare or reuse the deterministic `spec/<idea-id>` branch and
shall open or update exactly one matching pull request. Repeated calls SHALL
not create duplicate branches, commits, or pull requests solely because the
service was retried. When publishing a prepared Grimoire worktree, the CLI
SHALL stage the repository-root `specs/` subtree, including nested project and
topic directories, without flattening or replacing their relative paths.

#### Scenario: Existing processing pull request is found

- **WHEN** the service requests status or preparation for an idea with an open
  matching pull request
- **THEN** Sceptre SHALL return and reuse that pull request rather than create
  another one

#### Scenario: New proposal is published

- **WHEN** the service publishes a validated commit for an idea without an
  active pull request
- **THEN** Sceptre SHALL push `spec/<idea-id>` and open one pull request against
  the configured default branch

#### Scenario: Nested planning changes are published

- **WHEN** the prepared worktree contains planning changes below a nested
  directory under `specs/`
- **THEN** Sceptre SHALL include those changes in the publication commit while
  preserving their repository-relative paths

#### Scenario: Planning subtree is absent

- **WHEN** the prepared worktree contains no `specs/` directory or no changes
  under it
- **THEN** Sceptre SHALL return a structured error and SHALL NOT push the branch
  or create or update a pull request
