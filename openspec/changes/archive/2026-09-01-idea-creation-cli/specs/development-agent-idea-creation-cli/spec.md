## ADDED Requirements

### Requirement: The CLI creates canonical ideas

Sceptre SHALL expose `idea create` with required idea content and an optional
ID. It SHALL derive a stable lowercase kebab-case ID from the title when one is
not supplied, validate all IDs, and reject empty required values.

#### Scenario: Valid idea is pushed

- **WHEN** the repository mapping is valid and the requested idea does not
  already exist
- **THEN** Sceptre SHALL write `ideas/<id>/IDEA.md` with `processed: false`,
  commit it from the configured default branch, push `idea/<id>`, verify the
  remote commit, and return one JSON result.

#### Scenario: Existing idea is protected

- **WHEN** `idea/<id>` or `ideas/<id>/IDEA.md` already exists remotely
- **THEN** Sceptre SHALL return a non-zero structured duplicate error without
  modifying the remote.

### Requirement: Idea creation is isolated and redacted

Sceptre SHALL select `gh` for GitHub and `tea` for Gitea, require the selected
tool, suppress provider/Git output, avoid credentials in arguments or results,
and remove its temporary clone on both success and failure.
