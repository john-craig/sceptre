## Purpose

Provide Sceptre CLI operations that a host-managed idea-processing service can
use to inspect Grimoire, prepare specification branches, and manage pull
requests.

## ADDED Requirements

### Requirement: The CLI exposes machine-readable idea-processing operations

Sceptre SHALL provide commands for pending-idea discovery, processing status,
specification branch preparation, pull-request publication, and review
feedback retrieval. Each command SHALL support machine-readable JSON output. The
feedback operation SHALL return pull-request metadata together with bounded
general comments and review decisions, using a stable provider-independent
structure.

#### Scenario: Service discovers pending ideas

- **WHEN** the service invokes the pending-idea command against Grimoire
- **THEN** Sceptre SHALL return valid unprocessed ideas and their relevant pull
  request state

#### Scenario: Service requests review feedback

- **WHEN** the service invokes the feedback command for an open changes-requested
  pull request
- **THEN** Sceptre SHALL return bounded review state and feedback associated with
  that pull request

#### Scenario: Pull request has a general comment

- **WHEN** the feedback operation reads a pull request containing a general
  comment
- **THEN** the result SHALL include that comment's author, body, timestamp, and
  provider-independent comment type

#### Scenario: Pull request has a review decision

- **WHEN** the feedback operation reads a pull request containing a review
  decision, including a request for changes
- **THEN** the result SHALL include the reviewer's identity, decision state, body
  when available, and timestamp

#### Scenario: Pull request has no comments or reviews

- **WHEN** the feedback operation reads a pull request with no comments or review
  decisions
- **THEN** Sceptre SHALL return successful pull-request metadata with empty
  comments and reviews collections

#### Scenario: Feedback exceeds the configured bound

- **WHEN** a provider returns more comments or review content than the feedback
  limit
- **THEN** Sceptre SHALL return only the bounded result and SHALL indicate that
  feedback was truncated without exposing provider diagnostics

### Requirement: Branch and pull-request operations are idempotent

The CLI SHALL prepare or reuse the deterministic `spec/<idea-id>` branch and
shall open or update exactly one matching pull request. Repeated calls SHALL
not create duplicate branches, commits, or pull requests solely because the
service was retried.

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

### Requirement: Provider operations use configured remote tooling

The CLI SHALL select Git hosting operations from the configured remote and
shall use the available provider tooling or API for that remote. It SHALL
support the GitHub and Gitea providers used by the workflow without embedding
credentials in arguments, source, or output. For Gitea operations, Sceptre SHALL
pass the provider's canonical `owner/repository` identifier to `tea` while
retaining the configured or derived SSH URL for Git transport operations.

#### Scenario: GitHub Grimoire is configured

- **WHEN** the configured Grimoire remote is GitHub
- **THEN** Sceptre SHALL use the configured GitHub provider integration for
  branch and pull-request operations

#### Scenario: Provider authentication fails

- **WHEN** the configured provider rejects authentication
- **THEN** Sceptre SHALL return a non-zero result with redacted actionable
  diagnostics

#### Scenario: Gitea upstream uses a full repository URL

- **WHEN** the configured Gitea upstream is
  `https://gitea.example/owner/repository`
- **THEN** Sceptre SHALL invoke `tea` with `owner/repository` as its repository
  argument and SHALL continue to use the SSH transport URL for Git operations

#### Scenario: Gitea upstream cannot identify an owner and repository

- **WHEN** a Gitea upstream cannot be normalized to exactly an owner and
  repository path
- **THEN** Sceptre SHALL return a structured validation error before invoking
  `tea` or mutating a remote repository

### Requirement: The CLI does not own worker orchestration

Sceptre SHALL return explicit operation results and SHALL not start OpenCode
sessions, choose agent prompts, schedule recurring runs, or own host worktree
cleanup. Those responsibilities SHALL remain with the Nixpkgs Apocrypha service.

#### Scenario: Service invokes Sceptre

- **WHEN** the Nixpkgs Apocrypha coordinator invokes a Sceptre command
- **THEN** the command SHALL complete the requested remote workflow operation
  and return control without starting a second orchestration process
