# Tasks

- [x] Select an existing `local_path` repository before resolving templates or
  invoking provider tooling.
- [x] Validate that an existing local source is a Git repository with no
  configured remotes.
- [x] Use the validated local repository as the upstream creation source.
- [x] Add the `--clone` CLI option.
- [x] Clone a verified upstream into a missing `local_path`.
- [x] Reuse a valid no-remote local source and assign its `origin`.
- [x] Reject unrelated existing clone destinations without overwriting them.
- [x] Add regression tests and update the CLI documentation.
