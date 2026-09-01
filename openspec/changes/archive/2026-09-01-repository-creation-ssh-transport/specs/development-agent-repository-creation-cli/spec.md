## ADDED Requirements

### Requirement: Git operations use SSH transport by default

Sceptre SHALL use the SSH URL for Git clone, push, and local `origin` operations
when the configured upstream is a supported GitHub or Gitea HTTPS URL. The
command SHALL provide an explicit HTTPS transport override. Provider API
operations SHALL continue to use the configured provider CLI identity.

#### Scenario: GitHub repository uses SSH transport

- **WHEN** a GitHub upstream is configured as an HTTPS URL
- **THEN** Git operations SHALL use the equivalent `git@github.com:` URL

#### Scenario: HTTPS transport is explicitly requested

- **WHEN** the user supplies the HTTPS transport override
- **THEN** Git operations SHALL use the configured HTTPS upstream URL
