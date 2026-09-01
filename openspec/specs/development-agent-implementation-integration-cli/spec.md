## Purpose

Provide Sceptre CLI operations used by the integration service to identify,
merge, synchronize, and clean up approved implementation pull requests.

## ADDED Requirements

### Requirement: The CLI exposes integration workflow operations

Sceptre SHALL provide commands for integration candidate discovery, pull-request
status, approved merge, implemented-spec synchronization, and branch cleanup.
Each command SHALL support machine-readable JSON output.

#### Scenario: Service discovers integration candidates

- **WHEN** the service invokes integration candidate discovery
- **THEN** Sceptre SHALL return only implementation pull requests matching a
  known specset, repository, branch, and specification identity

#### Scenario: Service checks approval

- **WHEN** the service requests integration status
- **THEN** Sceptre SHALL distinguish pending, changes-requested, approved,
  merged, and synchronization-failed states

### Requirement: Approved merges are explicit and idempotent

The CLI SHALL refuse to merge a pull request without an explicit approved review
state. It SHALL merge only the matching implementation pull request and SHALL
return the verified resulting commit. Repeated merge calls SHALL not create
duplicate merges.

#### Scenario: Approved implementation is merged

- **WHEN** the service invokes merge for an approved candidate
- **THEN** Sceptre SHALL merge it into the configured upstream default branch and
  verify the resulting commit

#### Scenario: Approval is absent

- **WHEN** a candidate is pending review or has changes requested
- **THEN** Sceptre SHALL refuse the merge and return the current review state

### Requirement: The CLI synchronizes merged specifications to Grimoire

Sceptre SHALL read the implemented OpenSpec from the verified upstream default
branch, resolve its canonical Grimoire destination from the specset manifest,
and update that destination without accepting an arbitrary target path.
Synchronization SHALL be idempotent and SHALL verify the remote Grimoire result.

#### Scenario: Merged specification is synchronized

- **WHEN** the service invokes synchronization for a verified merged
  implementation
- **THEN** Sceptre SHALL update the canonical Grimoire specification and return
  the synchronization commit

#### Scenario: Synchronization is retried

- **WHEN** an earlier synchronization failed after the upstream merge
- **THEN** Sceptre SHALL retry the same source and canonical destination without
  rerunning implementation

### Requirement: Cleanup occurs only after complete integration

The CLI SHALL delete the source `feature/<specset-id>` branch only after both
the upstream merge and Grimoire synchronization are verified. It SHALL preserve
the branch when either stage is incomplete or failed.

#### Scenario: Integration is complete

- **WHEN** merge and synchronization have both been verified
- **THEN** Sceptre SHALL report cleanup eligibility and may delete the source
  branch

### Requirement: The CLI does not own services or deployment

Sceptre SHALL not schedule recurring jobs, start OpenCode sessions, define
NixOS services, or invoke the deployment-agent. Those responsibilities SHALL
remain with the Nixpkgs Apocrypha service module.

#### Scenario: Integration service invokes Sceptre

- **WHEN** the Nixpkgs Apocrypha service invokes an integration command
- **THEN** Sceptre SHALL perform the requested remote operation and return a
  structured result without starting another coordinator
