## Purpose

Provide an agent-facing MCP tool that creates Grimoire ideas on remote branches
without leaving local repository copies behind. The tool is part of Sceptre's
shared MCP and CLI workflow-tooling repository.

## ADDED Requirements

### Requirement: The MCP server exposes an idea creation tool

The server SHALL expose a tool that accepts the structured content needed to
create a Grimoire idea: title, problem, desired outcome, scope, non-goals,
constraints, and open questions. The tool SHALL derive or validate a stable
lowercase kebab-case idea ID.

#### Scenario: Agent creates an idea

- **WHEN** an agent calls the idea creation tool with valid content
- **THEN** the tool SHALL create a Grimoire idea record using the canonical
  format and return the idea ID and remote branch name

#### Scenario: Invalid content is rejected

- **WHEN** required content is missing or the requested idea ID is invalid
- **THEN** the tool SHALL return a validation error and SHALL not create a
  branch or commit

### Requirement: Idea creation writes directly to the remote repository

The tool SHALL create the `idea/<idea-id>` branch from the configured default
branch, commit the new `IDEA.md`, and push the commit to `origin`. It SHALL use
the remote repository API or an equivalent remote write mechanism and SHALL not
leave a local clone, checkout, or repository copy after the operation completes.

#### Scenario: Successful creation is remotely visible

- **WHEN** a valid idea creation call completes successfully
- **THEN** the named branch and commit SHALL be visible on the configured
  remote repository

#### Scenario: Local state is not retained

- **WHEN** an idea creation call completes or fails
- **THEN** the tool SHALL not leave a local repository copy or working-tree
  artifact behind

### Requirement: Existing ideas are protected

The tool SHALL fail without modification when the requested idea ID already
exists as a remote branch or canonical idea record. It SHALL return enough
information for the caller to locate the existing branch.

#### Scenario: Duplicate submission is rejected

- **WHEN** an agent submits an idea ID already in use
- **THEN** the tool SHALL report the collision and SHALL not overwrite or
  append to the existing record

### Requirement: Remote write failures are explicit

The tool SHALL report authentication, branch creation, commit, and push failures
as actionable errors. It SHALL not report success unless the remote commit is
confirmed.

#### Scenario: Push fails

- **WHEN** the remote rejects the branch or commit
- **THEN** the tool SHALL report failure and the resulting remote state without
  claiming that the idea was created successfully
