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

## Idea creation

Create a Grimoire idea on a remote `idea/<id>` branch with one JSON result:

```text
sceptre idea create --repository PATH --title TITLE --problem TEXT \
  --desired-outcome TEXT --scope TEXT --non-goals TEXT --constraints TEXT \
  --open-questions TEXT [--id ID]
```

`PATH` is a repository mapping JSON containing `id`, `upstream`,
`default_branch`, and `upstream_created`. IDs are derived from the title when
omitted and must be lowercase kebab-case. The command writes
`ideas/<id>/IDEA.md`, starts from the configured default branch, checks both
remote branch and canonical-record duplicates, and removes its temporary clone
after success or failure. GitHub remotes require `gh`; Gitea remotes require
`tea`. Provider and Git diagnostics are redacted and never forwarded.

## MCP server

Run the stdio MCP server against a Grimoire repository mapping with:

```text
sceptre mcp serve --repository PATH
```

The server speaks line-delimited JSON-RPC 2.0 and supports `initialize`,
`notifications/initialized`, `tools/list`, and `tools/call`. It exposes one
`create_idea` tool with the same seven required text fields as the CLI and an
optional `id`. The mapping supplied by `--repository` is used for every tool
call. Successful calls return the CLI result as JSON MCP text content; validation
and remote-write failures return structured JSON MCP content with `isError: true`.
Protocol responses are written only to stdout.

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

## Idea processing and integration

The service-facing command families are:

```text
sceptre idea-process pending --catalog GRIMOIRE
sceptre idea-process status --catalog GRIMOIRE --idea-id ID
sceptre idea-process prepare --catalog GRIMOIRE --idea-id ID
sceptre idea-process publish --catalog GRIMOIRE --idea-id ID --worktree PATH --message MESSAGE [--title TITLE]
sceptre idea-process feedback --upstream URL --number PR_NUMBER

sceptre integration candidates --manifest MANIFEST --catalog GRIMOIRE
sceptre integration status --manifest MANIFEST --catalog GRIMOIRE [--repository ID]
sceptre integration merge --manifest MANIFEST --catalog GRIMOIRE --repository ID --specification SPEC --number PR_NUMBER
sceptre integration sync --manifest MANIFEST --catalog GRIMOIRE --repository ID --specification SPEC --merge-commit COMMIT --number PR_NUMBER
sceptre integration cleanup --manifest MANIFEST --catalog GRIMOIRE --repository ID --specification SPEC --number PR_NUMBER --sync-commit COMMIT
```

These commands emit one JSON object and never start workers, schedulers,
worktrees, or deployment. Idea records are discovered under nested
`ideas/**/IDEA.md` paths. Integration source and destination paths are resolved
from the manifest and the real Grimoire layout; no synchronization destination
argument is accepted. Git uses SSH URLs by default, while `gh` and `tea` use
their configured credentials. `idea-process prepare` validates a pending idea,
creates or reuses `spec/<idea-id>` from the configured default branch, and
verifies the remote branch before returning. Integration cleanup accepts only a
verified synchronization commit from the Grimoire default branch. Provider and
Git diagnostics are redacted.

`idea-process publish` expects the prepared worktree to contain planning changes
under the repository-root `specs/` directory, including nested layouts such as
`specs/<project>/<topic>/...`. It stages only that subtree and returns a
structured input error without pushing or touching a pull request when the
subtree is missing or has no changes.

For Gitea, the configured upstream remains the Git transport URL, while provider
commands receive its normalized `owner/repository` identifier. HTTPS,
`ssh://`, and SCP-style upstreams are accepted, including custom SSH ports and
an optional `.git` suffix.

`idea-process feedback` returns provider-independent `comments` and `reviews`
arrays. Each entry contains `author`, `body`, `timestamp`, `type`, and `state`;
empty arrays are returned when no feedback exists. Content is bounded and the
`feedback_truncated` boolean indicates omitted comments or review content.
Gitea's one-record JSON array response is normalized to this same shape; empty
or ambiguous arrays return a structured provider error.
