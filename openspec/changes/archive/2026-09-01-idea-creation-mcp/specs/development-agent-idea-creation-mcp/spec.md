## ADDED Requirements

### Requirement: The MCP server provides a standard stdio protocol

Sceptre SHALL provide `mcp serve --repository PATH` as a line-delimited JSON-RPC
2.0 stdio server. It SHALL support `initialize`,
`notifications/initialized`, `tools/list`, and `tools/call`; stdout SHALL contain
only JSON-RPC responses.

#### Scenario: Client discovers the server

- **WHEN** a client sends `initialize` followed by `tools/list`
- **THEN** the server SHALL return MCP capabilities and exactly one `create_idea`
  tool with the required idea fields and optional `id`

#### Scenario: Invalid protocol request

- **WHEN** a client sends malformed JSON or an unknown request method
- **THEN** the server SHALL return the corresponding JSON-RPC error without
  writing diagnostic or provider output to stdout

### Requirement: MCP idea creation matches the CLI

The `create_idea` tool SHALL use the configured repository mapping path and the
same validation and remote-writing path as `idea create`. Successful calls SHALL
return JSON result content; validation or remote failures SHALL return JSON
content with `isError: true` and the shared structured error.

#### Scenario: Shared validation rejects invalid content

- **WHEN** a tool call supplies empty required content or an invalid explicit ID
- **THEN** it SHALL return the same validation error category as the CLI and SHALL
  not create a branch or commit
