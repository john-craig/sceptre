## Purpose

Provide a Rust Sceptre CLI command that initializes a repository from a
cataloged template or an existing local repository and creates its configured
upstream repository.

## ADDED Requirements

### Requirement: The CLI creates repositories from Grimoire definitions

Sceptre SHALL provide a Rust repository-creation command that accepts or loads
a Grimoire repository definition and resolves its `template` value through
`repos/templates.yaml`. The command SHALL preserve the repository ID, upstream
location, visibility, and default branch requested by the definition.

#### Scenario: Valid repository definition is supplied

- **WHEN** the command receives a valid definition and a resolvable template
- **THEN** it SHALL prepare the upstream repository using the selected template
  source and report the repository identity and provider

#### Scenario: Template cannot be resolved

- **WHEN** the definition names a template missing from `repos/templates.yaml`
- **THEN** the command SHALL fail before creating an upstream repository and
  SHALL report the missing template ID

### Requirement: An existing local repository is a safe source

When the definition's `local_path` exists, Sceptre SHALL use it as the source
instead of the cataloged template. It SHALL verify that the path is a Git
repository before performing provider operations. The local repository SHALL
have no configured remotes; a configured remote SHALL cause Sceptre to fail
immediately without creating or modifying the upstream repository.

#### Scenario: Local repository has no remote

- **WHEN** `local_path` points to a valid Git repository with no configured
  remotes
- **THEN** Sceptre SHALL use its contents as the upstream source

#### Scenario: Local repository has a remote

- **WHEN** `local_path` points to a Git repository with one or more configured
  remotes
- **THEN** Sceptre SHALL fail before provider operations and SHALL report that
  the local source already has a remote

#### Scenario: Local source is not a Git repository

- **WHEN** `local_path` exists but is not a Git repository
- **THEN** Sceptre SHALL fail with an actionable local-source error

### Requirement: Provider tooling is selected from the upstream location

The command SHALL select the provider tooling appropriate for the configured
upstream location. It SHALL use `gh` for GitHub locations and `tea` for Gitea
locations when the required executable is available. It MAY support either
tool when both are available, but SHALL not silently use a mismatched provider.

#### Scenario: GitHub provider is available

- **WHEN** the upstream is a GitHub repository and `gh` is available
- **THEN** the command SHALL use `gh` or an equivalent explicitly configured
  GitHub operation to initialize and create the repository

#### Scenario: Gitea provider is available

- **WHEN** the upstream is a Gitea repository and `tea` is available
- **THEN** the command SHALL use `tea` or an equivalent explicitly configured
  Gitea operation to initialize and create the repository

#### Scenario: Required provider tooling is unavailable

- **WHEN** the matching provider executable is unavailable
- **THEN** the command SHALL fail with an actionable installation or
  configuration error and SHALL not claim that creation succeeded

### Requirement: Template initialization is reproducible

The command SHALL initialize the new upstream repository from the resolved
template repository without copying unrelated local repository state. It SHALL
create the requested default branch and preserve the template's intended
project structure unless the definition explicitly overrides it.

#### Scenario: Repository is initialized from a template

- **WHEN** a valid template and provider are available
- **THEN** the created upstream repository SHALL contain the template's
  initialized content on its configured default branch

#### Scenario: Template initialization fails

- **WHEN** the provider cannot initialize the repository from the template
- **THEN** the command SHALL report the failure and SHALL not return a completed
  repository result

### Requirement: Creation is idempotent and verifies the remote result

The command SHALL check whether the configured upstream already exists before
creating it. It SHALL treat an existing matching repository as already
provisioned, verify its remote identity, and SHALL not create a duplicate.

#### Scenario: Upstream already exists

- **WHEN** the configured upstream repository exists and matches the definition
- **THEN** the command SHALL return an already-existing result without modifying
  the repository solely because the command was rerun

#### Scenario: Creation succeeds

- **WHEN** the provider reports successful creation
- **THEN** the command SHALL verify the upstream repository and default branch
  before returning success

### Requirement: The command can clone the resulting repository

When `--clone` is supplied, Sceptre SHALL clone the created or already-existing
upstream into the definition's `local_path` when that path does not exist. It
SHALL not overwrite a non-empty unrelated path. If the path already contains a
valid no-remote local source, Sceptre SHALL reuse it and assign the verified
upstream as its `origin` after upstream verification.

#### Scenario: A new local clone is requested

- **WHEN** `--clone` is supplied and `local_path` does not exist
- **THEN** Sceptre SHALL clone the verified upstream into `local_path` and
  report that the local repository was cloned

#### Scenario: An existing local source is reused

- **WHEN** `--clone` is supplied and `local_path` is a no-remote Git repository
- **THEN** Sceptre SHALL preserve its contents, configure `origin` to the
  verified upstream, and report that the local repository was reused

#### Scenario: Clone destination conflicts

- **WHEN** `--clone` targets a non-empty path that is not a valid no-remote Git
  source
- **THEN** Sceptre SHALL fail without overwriting that path

### Requirement: Credentials and provider output are handled safely

The command SHALL use credentials supplied by the runtime environment or
configured provider tools. It SHALL not accept credentials as ordinary
repository content or write tokens to logs, generated files, or pull request
messages. Provider failures SHALL be returned with actionable, redacted
diagnostics.

#### Scenario: Provider authentication fails

- **WHEN** `tea` or `gh` cannot authenticate
- **THEN** the command SHALL return a non-zero result without exposing the
  credential or claiming that the upstream repository exists

### Requirement: Git operations use SSH transport by default

Sceptre SHALL use the SSH URL for Git clone, push, and local `origin` operations
when the configured upstream is a supported GitHub or Gitea HTTPS URL. The
provider CLI SHALL continue to receive the configured provider URL or repository
identity for API operations. The command SHALL provide an explicit `--https`
override.

#### Scenario: GitHub repository uses SSH transport

- **WHEN** a GitHub upstream is configured as an HTTPS URL
- **THEN** Git operations SHALL use the equivalent `git@github.com:` URL without
  prompting for HTTPS credentials

#### Scenario: HTTPS transport is explicitly requested

- **WHEN** the user supplies the HTTPS transport override
- **THEN** Git operations SHALL use the configured HTTPS upstream URL

#### Scenario: Provider API authentication is required

- **WHEN** Sceptre invokes `gh` or `tea` for repository discovery or creation
- **THEN** it SHALL use the provider CLI's configured API authentication and
  SHALL not treat SSH credentials as provider API authentication
