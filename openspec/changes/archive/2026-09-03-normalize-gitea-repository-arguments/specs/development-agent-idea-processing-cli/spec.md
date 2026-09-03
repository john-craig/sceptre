## MODIFIED Requirements

### Requirement: Provider operations use configured remote tooling

The CLI SHALL select Git hosting operations from the configured remote and
shall use the available provider tooling or API for that remote. It SHALL
support the GitHub and Gitea providers used by the workflow without embedding
credentials in arguments, source, or output. For Gitea operations, Sceptre
SHALL pass the provider's canonical `owner/repository` identifier to `tea` while
retaining the configured or derived SSH URL for Git transport operations.

#### Scenario: GitHub Grimoire is configured

- **WHEN** the configured Grimoire remote is GitHub
- **THEN** Sceptre SHALL use the configured GitHub provider integration for
  branch and pull-request operations

#### Scenario: Gitea Grimoire uses a full HTTPS upstream

- **WHEN** the configured Gitea upstream is
  `https://gitea.example/owner/repository`
- **THEN** Sceptre SHALL invoke `tea` with `owner/repository` as its repository
  argument and SHALL continue to use the SSH transport URL for Git operations

#### Scenario: Gitea Grimoire uses an SSH upstream

- **WHEN** the configured Gitea upstream is an SSH URL, including a custom port
  or an optional `.git` suffix
- **THEN** Sceptre SHALL derive the same `owner/repository` provider identifier
  without changing the SSH URL used for Git operations

#### Scenario: Gitea upstream cannot identify an owner and repository

- **WHEN** a Gitea upstream cannot be normalized to exactly an owner and
  repository path
- **THEN** Sceptre SHALL return a structured validation error before invoking
  `tea` or mutating a remote repository

#### Scenario: Provider authentication fails

- **WHEN** the configured provider rejects authentication
- **THEN** Sceptre SHALL return a non-zero result with redacted actionable
  diagnostics
