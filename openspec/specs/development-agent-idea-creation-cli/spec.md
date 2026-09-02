## Purpose

Provide a CLI interface for creating Grimoire ideas using the same validation
and remote branch contract as Sceptre's MCP tool.

## ADDED Requirements

### Requirement: The CLI exposes idea creation

The Sceptre CLI SHALL expose an idea-creation command that accepts the required
Grimoire idea content and derives or validates a stable lowercase kebab-case
idea ID. It SHALL use the same idea record and branch contract as the MCP
interface.

#### Scenario: CLI creates an idea

- **WHEN** a user or automation invokes the idea-creation command with valid
  content
- **THEN** the CLI SHALL create the remote `idea/<idea-id>` branch and return
  machine-readable details for the created idea

#### Scenario: CLI rejects invalid input

- **WHEN** required content is missing, the ID is invalid, or the ID already
  exists
- **THEN** the CLI SHALL return a non-zero result with an actionable error and
  SHALL not overwrite an existing record

### Requirement: MCP and CLI behavior remains consistent

The MCP and CLI interfaces SHALL share the same validation, remote-write,
duplicate-protection, and failure semantics rather than implementing divergent
idea creation behavior.

#### Scenario: Interfaces receive equivalent input

- **WHEN** equivalent valid content is submitted through MCP and CLI
- **THEN** both interfaces SHALL produce records conforming to the same
  Grimoire format and branch contract

### Requirement: The CLI creates canonical ideas safely

Sceptre SHALL expose `idea create` with required idea content and an optional
ID. It SHALL derive a stable lowercase kebab-case ID from the title when one is
not supplied, validate explicit IDs, and reject empty required values. It SHALL
write the canonical `ideas/<id>/IDEA.md` format with `processed: false`.

#### Scenario: Valid idea is pushed

- **WHEN** the repository mapping is valid and the requested idea does not
  already exist
- **THEN** Sceptre SHALL create and verify the remote `idea/<id>` branch from
  the configured default branch and return machine-readable details

#### Scenario: Existing idea is protected

- **WHEN** `idea/<id>` or `ideas/<id>/IDEA.md` already exists remotely
- **THEN** Sceptre SHALL return a non-zero duplicate error without modifying the
  existing record

### Requirement: Idea creation is isolated and redacted

Sceptre SHALL select the configured GitHub or Gitea provider tooling, perform
  Git work in an isolated temporary clone, suppress provider and Git output,
  avoid credentials in arguments and results, and remove the temporary clone on
  both success and failure.
