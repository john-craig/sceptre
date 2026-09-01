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
