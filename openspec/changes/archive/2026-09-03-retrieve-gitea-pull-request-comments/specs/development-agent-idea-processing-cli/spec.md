## MODIFIED Requirements

### Requirement: The CLI exposes machine-readable idea-processing operations

Sceptre SHALL provide commands for pending-idea discovery, processing status,
specification branch preparation, pull-request publication, and review feedback
retrieval. Each command SHALL support machine-readable JSON output. The feedback
operation SHALL return pull-request metadata together with bounded general
comments and review decisions, using a stable provider-independent structure.
For Gitea, the feedback operation SHALL obtain comments and review decisions
through a structured provider response rather than relying on metadata-only
pull-request output or parsing unbounded human-readable output.

#### Scenario: Service discovers pending ideas

- **WHEN** the service invokes the pending-idea command against Grimoire
- **THEN** Sceptre SHALL return valid unprocessed ideas and their relevant pull
  request state

#### Scenario: Service requests review feedback

- **WHEN** the service invokes the feedback command for an open changes-requested
  pull request
- **THEN** Sceptre SHALL return bounded review state and feedback associated with
  that pull request

#### Scenario: Gitea contains a general pull-request comment

- **WHEN** the feedback operation reads a Gitea pull request containing a general
  comment
- **THEN** the result SHALL include that comment's author, body, timestamp, and
  provider-independent comment type

#### Scenario: Gitea contains a review decision

- **WHEN** the feedback operation reads a Gitea pull request containing a review
  decision, including a request for changes
- **THEN** the result SHALL include the reviewer's identity, decision state, body
  when available, and timestamp

#### Scenario: Gitea has no comments or reviews

- **WHEN** the feedback operation reads a Gitea pull request with no comments or
  review decisions
- **THEN** Sceptre SHALL return successful pull-request metadata with empty
  comments and reviews collections

#### Scenario: Structured Gitea feedback retrieval fails

- **WHEN** the configured Gitea provider cannot return structured comments or
  review decisions
- **THEN** Sceptre SHALL return a non-zero result with a redacted actionable
  provider error and SHALL NOT claim that feedback is absent
