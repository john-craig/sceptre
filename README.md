# Sceptre

Rust CLI for Grimoire development workflows.

## Repository creation

Create a repository from a Grimoire definition and the repository template
catalog with:

```text
sceptre repository create --definition PATH --templates PATH --json
```

Git operations use SSH URLs by default for supported GitHub and Gitea HTTPS
upstreams. Pass `--https` when Git must use the configured HTTPS URL instead.
SSH keys authenticate Git clone, push, and `origin` operations; `gh` or `tea`
credentials separately authenticate provider API operations.

The definition JSON must contain `id`, `template`, `upstream`, and
`default_branch`. `visibility` (`public`, `private`, or `internal`) is optional
and defaults to `public`. If `local_path` exists, it is used as the source
instead of `template`; it must be a Git repository with no configured remotes.
The template catalog
may be a sequence under `templates:` or a mapping keyed by template ID. Each
template entry uses `id` and its `upstream` URL (also accepted: `source`, `url`,
or `repository`):

```yaml
templates:
  - id: rust
    upstream: https://github.com/example/rust-template.git
```

GitHub upstreams use `gh`; Gitea upstreams use `tea`. Provider credentials must
already be configured in those tools or their runtime environment. The command
checks for an existing repository before creation, clones the selected source into
an isolated temporary directory, pushes the requested default branch, and emits one
JSON result or error. Provider and Git output is never forwarded.

Pass `--clone` to clone the verified created or existing upstream into
`local_path` when it does not exist. An existing no-remote local source is
reused without overwriting its contents, and receives the verified upstream as
`origin`. A non-empty unrelated destination is rejected.

## Common commands

- `cargo run`
- `cargo test`
- `nix build`
- `nix run`
- `nix run .#rust-template-tests`
- `nix develop`

## Specification implementation

The machine-readable implementation interface is:

```text
sceptre specset ready --manifest MANIFEST --catalog CATALOG
sceptre specset next --manifest MANIFEST --catalog CATALOG
sceptre specset status --manifest MANIFEST --catalog CATALOG [--repository ID]
sceptre specset materialize --manifest MANIFEST --catalog CATALOG --worktree PATH
sceptre specset publish --manifest MANIFEST --catalog CATALOG --worktree PATH --message MESSAGE [--title TITLE]
sceptre specset feedback --upstream URL --number PR_NUMBER
```

Every command emits one JSON object; provider and Git command output is never
forwarded. A manifest contains `id`, repository ID strings, and
`implementation_order` repository ID strings. Repository mappings are loaded
from `CATALOG/repos/<id>.json` and registered agents from
`CATALOG/agents/<id>.json`; mappings declare `upstream`, `upstream_created`,
and `default_branch`. Repository specification leaves are discovered at
`MANIFEST_DIR/<repo-id>/openspec/specs/<agent-prefixed-dir>/spec.md`. When
present, manifest `spec_dependencies` is a sequence of records such as
`{repository: repo, spec: agent-feature, depends_on: [agent-base]}`; map-form
dependencies are also accepted. Dependency references use exact complete
spec-directory names and must remain in the same repository. Materialization copies the source to
`openspec/specs/<original-spec-directory>/spec.md` in the supplied worktree.

`ready` validates the complete manifest without touching a provider. Invalid
inputs return `{"status":"blocked",...}` and no branch or PR operation is
attempted. `next` and `status` use the deterministic implementation order and
consider a matching merged PR complete. `publish` reuses
`feature/<specset-id>` and updates the one matching PR when it already exists.
Sceptre does not start workers, create worktrees, schedule jobs, or deploy
services.
