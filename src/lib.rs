use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn greeting(name: &str) -> String {
    format!("Hello, {name}. This is the Rust template.")
}
pub fn run(name: Option<&str>) -> String {
    greeting(name.unwrap_or("world"))
}

#[derive(Debug, Deserialize)]
pub struct RepositoryDefinition {
    pub id: String,
    pub template: String,
    pub upstream: String,
    #[serde(default)]
    pub local_path: Option<PathBuf>,
    #[serde(default)]
    pub visibility: Visibility,
    #[serde(alias = "defaultBranch")]
    pub default_branch: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Public
    }
}

#[derive(Debug, Deserialize)]
struct Template {
    #[serde(default)]
    id: String,
    upstream: Option<String>,
    #[serde(alias = "url", alias = "source")]
    source: Option<String>,
    repository: Option<String>,
}

impl Template {
    fn source(&self) -> Option<&str> {
        self.upstream
            .as_deref()
            .or(self.source.as_deref())
            .or(self.repository.as_deref())
    }
}

#[derive(Debug, Serialize)]
struct ResultBody {
    id: String,
    upstream: String,
    visibility: Visibility,
    default_branch: String,
    provider: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    message: String,
}

#[derive(Parser, Debug)]
#[command(name = "sceptre", version)]
struct Cli {
    #[command(subcommand)]
    command: CommandLine,
}

#[derive(Subcommand, Debug)]
enum CommandLine {
    Repository(RepositoryCommand),
}

#[derive(Args, Debug)]
struct RepositoryCommand {
    #[command(subcommand)]
    command: RepositoryAction,
}

#[derive(Subcommand, Debug)]
enum RepositoryAction {
    Create {
        #[arg(long)]
        definition: String,
        #[arg(long)]
        templates: String,
        #[arg(long)]
        clone: bool,
        #[arg(long)]
        https: bool,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum Provider {
    Github,
    Gitea,
}

impl Provider {
    fn tool(&self) -> &'static str {
        match self {
            Self::Github => "gh",
            Self::Gitea => "tea",
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Github => "github",
            Self::Gitea => "gitea",
        }
    }
}

pub fn cli() -> i32 {
    let cli = Cli::parse();
    let CommandLine::Repository(repo) = cli.command;
    let RepositoryAction::Create {
        definition,
        templates,
        clone,
        https,
        json: _,
    } = repo.command;
    match create_repository(Path::new(&definition), Path::new(&templates), clone, https) {
        Ok(body) => {
            println!("{}", serde_json::to_string(&body).unwrap());
            0
        }
        Err(error) => {
            eprintln!("{}", serde_json::to_string(&error).unwrap());
            1
        }
    }
}

fn create_repository(
    definition_path: &Path,
    templates_path: &Path,
    clone: bool,
    use_https: bool,
) -> Result<ResultBody, ErrorBody<'static>> {
    let definition: RepositoryDefinition = read_json(definition_path)?;
    // A local source takes precedence, and must be safe before any provider or
    // template work is attempted.
    let local_source = match definition.local_path.as_deref() {
        Some(path) if path.exists() => {
            validate_local_source(path)?;
            Some(path.to_owned())
        }
        _ => None,
    };
    let template = if local_source.is_none() {
        Some(resolve_template(templates_path, &definition.template)?)
    } else {
        None
    };
    let provider = provider_for(&definition.upstream)?;
    if !tool_available(provider.tool()) {
        return Err(err(
            "provider_unavailable",
            format!(
                "required executable '{}' was not found in PATH",
                provider.tool()
            ),
        ));
    }
    let exists = provider_exists(&provider, &definition.upstream);
    if exists {
        verify_provider(&provider, &definition)?;
        if clone {
            let status = clone_or_reuse(
                definition.local_path.as_deref(),
                &definition.upstream,
                use_https,
            )?;
            return Ok(result(&definition, &provider, status));
        }
        return Ok(result(&definition, &provider, "already_exists"));
    }

    let work = temporary_directory(&definition.id)?;
    let source = local_source
        .as_deref()
        .and_then(Path::to_str)
        .map(str::to_owned)
        .or_else(|| {
            template
                .as_ref()
                .and_then(Template::source)
                .map(str::to_owned)
        })
        .ok_or_else(|| {
            err(
                "input_error",
                format!(
                    "template '{}' has no repository source",
                    definition.template
                ),
            )
        })?;
    let git_source = git_url(&source, use_https);
    run_git(&["clone", "--depth", "1", &git_source, work.to_str().unwrap()])?;
    let branch = &definition.default_branch;
    run_git_in(&work, &["checkout", "-B", branch])?;
    create_remote(&provider, &definition, &work)?;
    let git_upstream = git_url(&definition.upstream, use_https);
    run_git_in(&work, &["remote", "set-url", "origin", &git_upstream])?;
    run_git_in(&work, &["push", "-u", "origin", branch])?;
    verify_remote(&work, &git_upstream, branch)?;
    verify_provider(&provider, &definition)?;
    let _ = fs::remove_dir_all(&work);
    if clone {
        let status = clone_or_reuse(
            definition.local_path.as_deref(),
            &definition.upstream,
            use_https,
        )?;
        return Ok(result(&definition, &provider, status));
    }
    Ok(result(&definition, &provider, "created"))
}

fn validate_local_source(path: &Path) -> Result<(), ErrorBody<'static>> {
    let valid = Command::new("git")
        .current_dir(path)
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|output| output.status.success() && output.stdout == b"true\n")
        .unwrap_or(false);
    if !valid {
        return Err(err(
            "invalid_local_source",
            format!("local source '{}' is not a Git repository", path.display()),
        ));
    }
    let remotes = git_output(path, &["remote"])?;
    if !remotes.trim().is_empty() {
        return Err(err(
            "local_source_has_remote",
            format!("local source '{}' already has a remote", path.display()),
        ));
    }
    Ok(())
}

fn clone_or_reuse(
    local_path: Option<&Path>,
    upstream: &str,
    use_https: bool,
) -> Result<&'static str, ErrorBody<'static>> {
    let Some(path) = local_path else {
        return Err(err(
            "clone_destination_required",
            "--clone requires local_path in the repository definition".into(),
        ));
    };
    let git_upstream = git_url(upstream, use_https);
    if path.exists() {
        // Existing paths were validated before provider operations. Assigning
        // origin happens only after the upstream has been verified.
        run_git_in(path, &["remote", "add", "origin", &git_upstream])?;
        Ok("reused")
    } else {
        run_git(&[
            "clone",
            &git_upstream,
            path.to_str().ok_or_else(|| {
                err(
                    "filesystem_error",
                    "clone destination is not valid UTF-8".into(),
                )
            })?,
        ])?;
        Ok("cloned")
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, ErrorBody<'static>> {
    let text = fs::read_to_string(path).map_err(|_| {
        err(
            "input_error",
            format!(
                "cannot read definition '{}', or templates file",
                path.display()
            ),
        )
    })?;
    serde_json::from_str(&text).map_err(|_| {
        err(
            "input_error",
            format!("invalid JSON in '{}'", path.display()),
        )
    })
}

fn resolve_template(path: &Path, id: &str) -> Result<Template, ErrorBody<'static>> {
    let text = fs::read_to_string(path).map_err(|_| {
        err(
            "input_error",
            format!("cannot read templates file '{}'", path.display()),
        )
    })?;
    let value: serde_yaml::Value = serde_yaml::from_str(&text)
        .map_err(|_| err("input_error", "invalid templates YAML".into()))?;
    let candidates = value.get("templates").unwrap_or(&value);
    let template = if let Some(items) = candidates.as_sequence() {
        items.iter().find_map(|item| {
            serde_yaml::from_value::<Template>(item.clone())
                .ok()
                .filter(|t| t.id == id)
        })
    } else {
        candidates.get(id).and_then(|item| {
            let mut template = serde_yaml::from_value::<Template>(item.clone()).ok()?;
            if template.id.is_empty() {
                template.id = id.to_owned();
            }
            Some(template)
        })
    };
    template.ok_or_else(|| {
        err(
            "template_not_found",
            format!("template '{}' is not defined in '{}'", id, path.display()),
        )
    })
}

fn provider_for(url: &str) -> Result<Provider, ErrorBody<'static>> {
    let host = url
        .split("//")
        .nth(1)
        .and_then(|s| s.split('/').next())
        .unwrap_or("")
        .to_ascii_lowercase();
    if host == "github.com" || host.ends_with(".github.com") {
        Ok(Provider::Github)
    } else if host == "gitea.com" || host.contains("gitea") {
        Ok(Provider::Gitea)
    } else {
        Err(err(
            "unsupported_provider",
            format!(
                "upstream host '{}' is not a supported GitHub or Gitea host",
                host
            ),
        ))
    }
}

fn git_url(upstream: &str, use_https: bool) -> String {
    if use_https || provider_for(upstream).is_err() {
        return upstream.to_owned();
    }

    let Some(authority_and_path) = upstream.strip_prefix("https://") else {
        return upstream.to_owned();
    };
    let Some((authority, path)) = authority_and_path.split_once('/') else {
        return upstream.to_owned();
    };
    format!("git@{authority}:{path}")
}

fn tool_available(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
fn provider_exists(provider: &Provider, upstream: &str) -> bool {
    let args = match provider {
        Provider::Github => vec!["repo", "view", upstream],
        Provider::Gitea => vec!["repo", "show", upstream],
    };
    Command::new(provider.tool())
        .args(args)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
fn create_remote(
    provider: &Provider,
    definition: &RepositoryDefinition,
    work: &Path,
) -> Result<(), ErrorBody<'static>> {
    let visibility = match definition.visibility {
        Visibility::Public => "--public",
        Visibility::Private => "--private",
        Visibility::Internal => "--internal",
    };
    let args = match provider {
        Provider::Github => vec!["repo", "create", &definition.upstream, visibility],
        Provider::Gitea => vec!["repo", "create", &definition.upstream, visibility],
    };
    let _ = work;
    command(provider.tool(), &args, "create upstream repository")
}
fn verify_provider(
    provider: &Provider,
    definition: &RepositoryDefinition,
) -> Result<(), ErrorBody<'static>> {
    if provider_exists(provider, &definition.upstream) {
        Ok(())
    } else {
        Err(err(
            "verification_failed",
            "provider could not verify the upstream repository".into(),
        ))
    }
}
fn verify_remote(work: &Path, upstream: &str, branch: &str) -> Result<(), ErrorBody<'static>> {
    let remote = git_output(work, &["remote", "get-url", "origin"])?;
    if remote.trim_end() != upstream {
        return Err(err(
            "verification_failed",
            "configured remote does not match upstream".into(),
        ));
    }
    let current = git_output(work, &["branch", "--show-current"])?;
    if current.trim() != branch {
        return Err(err(
            "verification_failed",
            "working tree is not on the requested default branch".into(),
        ));
    }
    Ok(())
}
fn run_git(args: &[&str]) -> Result<(), ErrorBody<'static>> {
    command("git", args, "clone template")
}
fn run_git_in(dir: &Path, args: &[&str]) -> Result<(), ErrorBody<'static>> {
    Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(|_| err("git_error", "could not execute git".into()))
        .and_then(|o| {
            if o.status.success() {
                Ok(())
            } else {
                Err(err("git_error", "git operation failed".into()))
            }
        })
}
fn git_output(dir: &Path, args: &[&str]) -> Result<String, ErrorBody<'static>> {
    Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(|_| err("git_error", "could not execute git".into()))
        .and_then(|o| {
            if o.status.success() {
                Ok(String::from_utf8_lossy(&o.stdout).into_owned())
            } else {
                Err(err("git_error", "git verification failed".into()))
            }
        })
}
fn command(tool: &str, args: &[&str], operation: &str) -> Result<(), ErrorBody<'static>> {
    Command::new(tool)
        .args(args)
        .output()
        .map_err(|_| {
            err(
                "provider_error",
                format!("could not execute {} to {}", tool, operation),
            )
        })
        .and_then(|o| {
            if o.status.success() {
                Ok(())
            } else {
                Err(err(
                    "provider_error",
                    format!("{} failed while attempting to {}", tool, operation),
                ))
            }
        })
}
fn temporary_directory(id: &str) -> Result<std::path::PathBuf, ErrorBody<'static>> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "sceptre-{}-{}",
        id.replace(|c: char| !c.is_ascii_alphanumeric(), "_"),
        stamp
    ));
    fs::create_dir_all(&path).map_err(|_| {
        err(
            "filesystem_error",
            "could not create isolated template directory".into(),
        )
    })?;
    Ok(path)
}
fn result(d: &RepositoryDefinition, p: &Provider, status: &str) -> ResultBody {
    ResultBody {
        id: d.id.clone(),
        upstream: d.upstream.clone(),
        visibility: d.visibility,
        default_branch: d.default_branch.clone(),
        provider: p.name().into(),
        status: status.into(),
    }
}
fn err<'a>(error: &'a str, message: String) -> ErrorBody<'a> {
    ErrorBody { error, message }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selects_provider_from_upstream() {
        assert_eq!(
            provider_for("https://github.com/a/b").unwrap(),
            Provider::Github
        );
        assert_eq!(
            provider_for("https://gitea.example/a/b").unwrap(),
            Provider::Gitea
        );
    }
    #[test]
    fn rejects_unknown_provider() {
        assert!(provider_for("https://gitlab.com/a/b").is_err());
    }
    #[test]
    fn missing_template_is_reported() {
        let path = std::env::temp_dir().join("sceptre-test-templates.yaml");
        fs::write(&path, "templates: []").unwrap();
        let error = resolve_template(&path, "missing").unwrap_err();
        assert_eq!(error.error, "template_not_found");
        let _ = fs::remove_file(path);
    }
    #[test]
    fn resolves_template_from_mapping() {
        let path = std::env::temp_dir().join("sceptre-test-template-map.yaml");
        fs::write(&path, "rust:\n  url: https://example.test/rust.git\n").unwrap();
        let template = resolve_template(&path, "rust").unwrap();
        assert_eq!(template.source(), Some("https://example.test/rust.git"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn local_source_with_remote_is_rejected() {
        let path = test_directory("remote");
        git_in(&path, &["init"]);
        git_in(
            &path,
            &["remote", "add", "origin", "https://example.test/source.git"],
        );

        let error = validate_local_source(&path).unwrap_err();
        assert_eq!(error.error, "local_source_has_remote");
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn local_source_validation_precedes_template_and_provider_checks() {
        let path = test_directory("precedence");
        git_in(&path, &["init"]);
        git_in(
            &path,
            &["remote", "add", "origin", "https://example.test/source.git"],
        );
        let definition = path.join("definition.json");
        fs::write(
            &definition,
            format!(
                r#"{{"id":"test","template":"missing","upstream":"https://gitlab.com/a/b","local_path":"{}","default_branch":"main"}}"#,
                path.display()
            ),
        )
        .unwrap();

        let error =
            create_repository(&definition, &path.join("missing.yaml"), false, false).unwrap_err();
        assert_eq!(error.error, "local_source_has_remote");
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn local_source_without_remote_is_valid() {
        let path = test_directory("no-remote");
        git_in(&path, &["init"]);
        assert!(validate_local_source(&path).is_ok());
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn clone_reuses_local_source_and_assigns_origin() {
        let path = test_directory("clone-reuse");
        git_in(&path, &["init"]);

        assert_eq!(
            clone_or_reuse(Some(&path), "https://github.com/a/b", false).unwrap(),
            "reused"
        );
        assert_eq!(
            git_output(&path, &["remote", "get-url", "origin"])
                .unwrap()
                .trim(),
            "git@github.com:a/b"
        );
        let _ = fs::remove_dir_all(path);

        let path = test_directory("clone-reuse-https");
        git_in(&path, &["init"]);
        assert_eq!(
            clone_or_reuse(Some(&path), "https://github.com/a/b", true).unwrap(),
            "reused"
        );
        assert_eq!(
            git_output(&path, &["remote", "get-url", "origin"])
                .unwrap()
                .trim(),
            "https://github.com/a/b"
        );
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn converts_supported_https_urls_to_ssh() {
        assert_eq!(
            git_url("https://github.com/a/b.git", false),
            "git@github.com:a/b.git"
        );
        assert_eq!(
            git_url("https://gitea.example/a/b", false),
            "git@gitea.example:a/b"
        );
    }

    #[test]
    fn preserves_https_for_override_and_unsupported_hosts() {
        assert_eq!(
            git_url("https://github.com/a/b.git", true),
            "https://github.com/a/b.git"
        );
        assert_eq!(
            git_url("https://gitlab.com/a/b.git", false),
            "https://gitlab.com/a/b.git"
        );
    }

    fn test_directory(label: &str) -> std::path::PathBuf {
        let path =
            std::env::temp_dir().join(format!("sceptre-test-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn git_in(path: &Path, args: &[&str]) {
        assert!(Command::new("git")
            .current_dir(path)
            .args(args)
            .output()
            .unwrap()
            .status
            .success());
    }
}
