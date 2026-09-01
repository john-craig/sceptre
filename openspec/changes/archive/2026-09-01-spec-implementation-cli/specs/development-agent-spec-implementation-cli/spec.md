## ADDED Requirements

### Requirement: The CLI exposes real Grimoire implementation operations

Sceptre SHALL provide machine-readable `specset` commands for readiness,
selection, status, specification materialization, pull-request publication, and
review-feedback retrieval. The commands SHALL consume repository mappings,
registered agents, and repository OpenSpec leaves from the Grimoire catalog.

### Requirement: The CLI validates implementation readiness

Sceptre SHALL validate confirmed repository upstreams, registered agent
prefixes, agent-prefixed specification directories, implementation order,
same-repository dependency references, and dependency cycles. Invalid inputs
SHALL return a structured blocked result without preparing a branch or pull
request.

### Requirement: Implementation targets and branches are deterministic

Sceptre SHALL select the first incomplete specification in repository
`implementation_order`, defer specifications whose same-repository dependencies
are not merged, and use `feature/<specset-id>` for implementation work. It SHALL
materialize specifications under `openspec/specs/<spec-directory>` and reuse or
update the single matching provider pull request on retry.
