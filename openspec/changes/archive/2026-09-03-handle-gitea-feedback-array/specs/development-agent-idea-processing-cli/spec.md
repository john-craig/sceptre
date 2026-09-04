## MODIFIED Requirements

### Requirement: The CLI exposes machine-readable idea-processing operations

Sceptre SHALL provide commands for pending-idea discovery, processing status,
specification branch preparation, pull-request publication, and review feedback
retrieval. Each command SHALL support machine-readable JSON output. The feedback
operation SHALL return pull-request metadata together with bounded general
comments and review decisions, using a stable provider-independent structure.
The operation SHALL accept the array-shaped pull-request response emitted by
the supported Gitea `tea` client and normalize it before extracting feedback.

#### Scenario: Service discovers pending ideas

- **WHEN** the service invokes the pending-idea command against Grimoire
- **THEN** Sceptre SHALL return valid unprocessed ideas and their relevant pull
  request state

#### Scenario: Service requests review feedback

- **WHEN** the service invokes the feedback command for an open changes-requested
  pull request
- **THEN** Sceptre SHALL return bounded review state and feedback associated with
  that pull request

#### Scenario: Gitea returns an array-shaped pull-request response

- **WHEN** `tea pr show --output json` returns a JSON array containing the target
  pull request
- **THEN** Sceptre SHALL normalize the response and return the stable feedback
  structure instead of reporting that the provider response is not an object

#### Scenario: Gitea returns an empty array

- **WHEN** the Gitea provider returns an empty JSON array for pull-request
  feedback
- **THEN** Sceptre SHALL return a structured provider error indicating that the
  target pull request could not be found

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
