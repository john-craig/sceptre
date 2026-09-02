# Proposal

Implement the Sceptre command family required by the idea-processing service.
The commands inspect the real Grimoire catalog, use deterministic
`spec/<idea-id>` branches, and expose redacted provider results as JSON without
starting workers or schedulers.
