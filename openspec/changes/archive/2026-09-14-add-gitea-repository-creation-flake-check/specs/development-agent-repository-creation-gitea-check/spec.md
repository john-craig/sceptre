## ADDED Requirements

### Requirement: The flake exposes a live Gitea repository-creation check

The Sceptre flake SHALL expose an `x86_64-linux` NixOS check that runs a
deterministic Gitea MicroVM using the `microvm.nix` and Osmium upstream flakes.
The check SHALL provision its own admin, test user, and repository credentials
and SHALL not contact an external Gitea service.

#### Scenario: Gitea fixture becomes ready

- **WHEN** the NixOS check starts
- **THEN** it SHALL wait for Gitea, Osmium repository bootstrap, and credential
  generation before invoking Sceptre

### Requirement: The test seeds the Grimoire and template repositories

The check SHALL create deterministic local Git trees for a Grimoire repository
and a template repository, initialize each with its `main` branch, and push
both repositories to the fixture Gitea instance. The Grimoire tree SHALL
contain a valid target mapping with `upstream_created: false` and a template
catalog entry pointing to the seeded template repository.

#### Scenario: Target repository is initially absent

- **WHEN** the seeded repositories have been pushed
- **THEN** the check SHALL verify that the target upstream does not exist before
  running Sceptre

### Requirement: Sceptre creates the target from the live template

The check SHALL invoke the packaged Sceptre repository-creation command with
the seeded Grimoire mapping and template catalog, using the fixture Gitea
provider. Sceptre SHALL create the target upstream, publish the configured
default branch, and initialize it from the template repository.

#### Scenario: Repository creation succeeds

- **WHEN** Sceptre runs against the absent target mapping and available template
- **THEN** the command SHALL succeed, the target repository SHALL be discoverable
  through Gitea, and its `main` branch SHALL contain the template marker

#### Scenario: Repository creation fails

- **WHEN** the provider, template, or target verification operation fails
- **THEN** the NixOS check SHALL fail and SHALL not report a passing result

### Requirement: Successful creation reconciles Grimoire

After verifying the target upstream, Sceptre SHALL update the matching
Grimoire repository mapping to set `upstream_created: true`, preserve the
mapping's identity and upstream fields, commit the mutation on the configured
default branch, and push it to the remote Grimoire repository.

#### Scenario: Grimoire records the created upstream

- **WHEN** the target repository has been created and verified
- **THEN** a fresh clone of the remote Grimoire default branch SHALL contain a
  commit whose mapping has `upstream_created: true`

#### Scenario: Creation is not verified

- **WHEN** target creation or remote verification fails
- **THEN** Sceptre SHALL leave `upstream_created` false and SHALL not push a
  success mutation to Grimoire
