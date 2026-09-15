# Tasks

- [x] Add pinned `microvm.nix` and `osmium` inputs to `flake.nix`.
- [x] Add the `repository-creation-gitea` NixOS check output for
  `x86_64-linux`.
- [x] Build deterministic Grimoire and template seed trees in the test.
- [x] Configure Osmium Gitea users, repositories, and scoped credentials.
- [x] Add readiness, seeding, and target-absence assertions to the test script.
- [x] Update repository creation to reconcile and push the verified Grimoire
  mapping state.
- [x] Invoke the packaged Sceptre binary against the live fixture and assert
  target content and remote Grimoire state.
- [x] Run OpenSpec validation and the new flake check with KVM/QEMU available.
