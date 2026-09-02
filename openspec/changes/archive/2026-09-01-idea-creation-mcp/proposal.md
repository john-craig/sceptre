# Idea creation MCP server

Add a standard JSON-RPC 2.0 stdio MCP server to Sceptre. The server is started
with `mcp serve --repository PATH`, exposes the `create_idea` tool, and reuses
the CLI's shared validation and remote-writing implementation.

The server keeps protocol responses on stdout only, reports diagnostics on
stderr, and does not change existing CLI, specset, or repository behavior.

## Status

Implemented and archived after synchronization with the canonical MCP
specification.
