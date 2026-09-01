## MODIFIED Requirements

### Requirement: An existing local repository is a safe source

When the definition's `local_path` exists, Sceptre SHALL use it as the source
instead of the cataloged template. It SHALL verify that the path is a Git
repository before performing provider operations. The local repository SHALL
have no configured remotes; a configured remote SHALL cause Sceptre to fail
immediately without creating or modifying the upstream repository.

### Requirement: The command can clone the resulting repository

When `--clone` is supplied, Sceptre SHALL clone the created or already-existing
upstream into the definition's `local_path` when that path does not exist. It
SHALL not overwrite a non-empty unrelated path. If the path already contains a
valid no-remote local source, Sceptre SHALL reuse it and assign the verified
upstream as its `origin` after upstream verification.
