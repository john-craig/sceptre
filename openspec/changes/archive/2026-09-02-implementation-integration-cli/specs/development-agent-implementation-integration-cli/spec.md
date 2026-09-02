## MODIFIED Requirements

### Requirement: The CLI exposes integration workflow operations

The CLI SHALL expose `integration candidates`, `status`, `merge`, `sync`, and
`cleanup`, each emitting one JSON result. Candidate and status operations SHALL
resolve repositories and specification identities from the manifest.

### Requirement: The CLI synchronizes merged specifications to Grimoire

`sync` SHALL derive the source OpenSpec and canonical Grimoire destination from
the manifest and require a verified merge commit. It SHALL report an already
matching destination as synchronized without creating a commit.

### Requirement: Cleanup occurs only after complete integration

`cleanup` SHALL require explicit synchronization proof, verify the merged state,
delete `feature/<specset-id>` over SSH Git transport, and verify deletion.
