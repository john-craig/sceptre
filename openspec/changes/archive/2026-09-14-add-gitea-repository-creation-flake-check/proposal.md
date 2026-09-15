# Add Gitea Repository Creation Flake Check

## Why

Sceptre's repository-creation tests currently exercise provider behavior with
unit-level doubles, but do not prove that the command can create a repository
against a live Git provider or reconcile the corresponding Grimoire mapping.
The `weizza-dry-run` test provides an established pattern for running a
deterministic Gitea instance in a MicroVM with Osmium-managed users and
credentials.

## What Changes

- Add a NixOS integration check to Sceptre's flake, based on the
  `weizza-dry-run` MicroVM structure.
- Add `microvm.nix` and the Osmium upstream flake inputs needed by the check.
- Declaratively provision a Gitea admin, a Sceptre test user, and credentials
  for the test user.
- Seed and push a Grimoire repository containing a repository mapping with
  `upstream_created: false` and a template catalog.
- Seed and push a separate template repository used as the creation source.
- Run Sceptre's repository-creation command against the live Gitea instance.
- Verify that the target repository exists, contains the template content on
  its default branch, and that the updated Grimoire mapping was committed and
  pushed with `upstream_created: true`.

## Scope

This is a test and flake-integration change. It does not add a production
Gitea dependency or make the test depend on an external Gitea deployment.
