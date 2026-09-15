## Context

The existing flake packages Sceptre and runs Cargo tests, but has no live
provider check. The reference `weizza/tests/weizza-dry-run.nix` uses
`pkgs.testers.runNixOSTest`, imports `microvm.nixosModules.microvm` and
`osmium.nixosModules.default`, and writes Osmium-generated credentials into
the VM. The same arrangement can isolate all Git and Gitea state for this
test.

## Goals / Non-Goals

**Goals:**

- Exercise the real repository-creation CLI, `tea`, Git HTTP transport, and a
  live Gitea API in one reproducible Nix flake check.
- Ensure the test begins with a target repository that does not exist.
- Verify both remote repository creation and the Grimoire state transition.
- Keep credentials inside the VM and out of derivation output and test logs.

**Non-Goals:**

- Testing repository creation against GitHub.
- Testing the complete downstream implementation workflow.
- Reusing the external repositories or credentials from `weizza`.

## Decisions

### Isolated MicroVM

Add pinned `microvm` and `osmium` inputs following the `weizza` flake. Define
one NixOS test node with Gitea enabled, registration disabled after bootstrap,
an admin, a test user, and a personal access token with repository read/write
scope. Use a local Gitea URL and a `tea` login configured from the generated
token.

### Declarative prerequisites, imperative population

Declare the Grimoire and template repositories with Osmium so they exist
before the test starts. Build deterministic seed trees with `pkgs.runCommand`,
initialize them as Git repositories in the VM, and push their `main` branches
using the generated credential. Do not declare the target repository in
Osmium; its absence is part of the setup assertion.

### Repository fixture

The Grimoire seed contains `repos/created-fixture.json` with a stable
lowercase kebab-case ID, a local Gitea upstream URL for the target, a
`rust-template`-style template ID, `default_branch: "main"`, and
`upstream_created: false`. Its `repos/templates.yaml` points the template ID
at the separately seeded template repository. The template seed contains a
distinct marker file so the resulting content can be verified.

### Reconciliation contract

After creating and verifying the target upstream, Sceptre updates the
matching Grimoire mapping, preserving its other fields, commits the change on
Grimoire's default branch, pushes it, and verifies the remote branch contains
the commit. The test checks the remote Grimoire clone rather than only the
working copy used to invoke Sceptre.

### Flake output

Expose the test as `checks.x86_64-linux.repository-creation-gitea` and keep the
existing package, app, development shell, formatter, and Rust test behavior
unchanged. The check should use the flake's Sceptre package rather than a
system-installed binary.

## Risks / Trade-offs

- MicroVM checks require KVM/QEMU support and are slower than unit tests.
- Gitea startup and token generation require explicit readiness waits to avoid
  race-dependent failures.
- Pushing the Grimoire mutation makes the test more realistic but requires the
  fixture to configure Git identity and credentials carefully.
