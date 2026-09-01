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
