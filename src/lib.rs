use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, BufRead, Write},
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
    Specset(SpecsetCommand),
    Idea(IdeaCommand),
    Mcp(McpCommand),
}

#[derive(Args, Debug)]
struct McpCommand {
    #[command(subcommand)]
    command: McpAction,
}

#[derive(Subcommand, Debug)]
enum McpAction {
    Serve {
        #[arg(long)]
        repository: PathBuf,
    },
}

#[derive(Args, Debug)]
struct IdeaCommand {
    #[command(subcommand)]
    command: IdeaAction,
}

#[derive(Subcommand, Debug)]
enum IdeaAction {
    Create {
        #[arg(long)]
        repository: PathBuf,
        #[arg(long)]
        title: String,
        #[arg(long)]
        problem: String,
        #[arg(long, value_name = "TEXT")]
        desired_outcome: String,
        #[arg(long)]
        scope: String,
        #[arg(long)]
        non_goals: String,
        #[arg(long)]
        constraints: String,
        #[arg(long)]
        open_questions: String,
        #[arg(long)]
        id: Option<String>,
    },
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

#[derive(Args, Debug)]
struct SpecsetCommand {
    #[command(subcommand)]
    command: SpecsetAction,
}

#[derive(Subcommand, Debug)]
enum SpecsetAction {
    Ready(SpecsetInput),
    Next(SpecsetInput),
    Status {
        #[command(flatten)]
        input: SpecsetInput,
        #[arg(long)]
        repository: Option<String>,
    },
    Materialize {
        #[command(flatten)]
        input: SpecsetInput,
        #[arg(long)]
        worktree: PathBuf,
    },
    Publish {
        #[command(flatten)]
        input: SpecsetInput,
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long)]
        message: String,
        #[arg(long)]
        title: Option<String>,
    },
    Feedback {
        #[arg(long)]
        upstream: String,
        #[arg(long)]
        number: u64,
    },
}

#[derive(Args, Debug, Clone)]
struct SpecsetInput {
    #[arg(long)]
    manifest: PathBuf,
    #[arg(long)]
    catalog: PathBuf,
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
    let result = match cli.command {
        CommandLine::Mcp(mcp) => {
            let McpAction::Serve { repository } = mcp.command;
            return match mcp_serve(&repository) {
                Ok(()) => 0,
                Err(error) => {
                    eprintln!("MCP server error: {error}");
                    1
                }
            };
        }
        CommandLine::Repository(repo) => {
            let RepositoryAction::Create {
                definition,
                templates,
                clone,
                https,
                json: _,
            } = repo.command;
            create_repository(Path::new(&definition), Path::new(&templates), clone, https)
                .map(|body| serde_json::to_value(body).unwrap())
        }
        CommandLine::Specset(specset) => run_specset(specset.command),
        CommandLine::Idea(idea) => match idea.command {
            IdeaAction::Create {
                repository,
                title,
                problem,
                desired_outcome,
                scope,
                non_goals,
                constraints,
                open_questions,
                id,
            } => create_idea(
                &repository,
                IdeaInput {
                    title: &title,
                    problem: &problem,
                    desired_outcome: &desired_outcome,
                    scope: &scope,
                    non_goals: &non_goals,
                    constraints: &constraints,
                    open_questions: &open_questions,
                    id: id.as_deref(),
                },
            )
            .map(|body| serde_json::to_value(body).unwrap()),
        },
    };
    match result {
        Ok(body) => {
            println!("{}", body);
            0
        }
        Err(error) => {
            eprintln!("{}", serde_json::to_string(&error).unwrap());
            1
        }
    }
}

fn mcp_serve(repository: &Path) -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line?;
        if let Some(response) = handle_mcp_request(repository, &line) {
            writeln!(stdout, "{response}")?;
            stdout.flush()?;
        }
    }
    Ok(())
}

/// Handle one line-delimited JSON-RPC request. `None` denotes a notification.
pub fn handle_mcp_request(repository: &Path, input: &str) -> Option<String> {
    let value: serde_json::Value = match serde_json::from_str(input) {
        Ok(value) => value,
        Err(error) => {
            return Some(rpc_error(
                serde_json::Value::Null,
                -32700,
                error.to_string(),
            ))
        }
    };
    let object = match value.as_object() {
        Some(object) => object,
        None => {
            return Some(rpc_error(
                serde_json::Value::Null,
                -32600,
                "request must be an object",
            ))
        }
    };
    let id = object.get("id").cloned();
    let is_notification = !object.contains_key("id");
    if object.get("jsonrpc").and_then(serde_json::Value::as_str) != Some("2.0")
        || object
            .get("method")
            .and_then(serde_json::Value::as_str)
            .is_none()
    {
        return Some(rpc_error(
            id.unwrap_or(serde_json::Value::Null),
            -32600,
            "invalid JSON-RPC 2.0 request",
        ));
    }
    let method = object
        .get("method")
        .and_then(serde_json::Value::as_str)
        .unwrap();
    let response = match method {
        "initialize" => rpc_result(
            id.clone().unwrap_or(serde_json::Value::Null),
            serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "sceptre", "version": env!("CARGO_PKG_VERSION")}
            }),
        ),
        "notifications/initialized" => return None,
        "tools/list" => rpc_result(
            id.clone().unwrap_or(serde_json::Value::Null),
            serde_json::json!({"tools": [create_idea_tool()] }),
        ),
        "tools/call" => match call_tool(repository, object.get("params")) {
            Ok(result) => rpc_result(id.clone().unwrap_or(serde_json::Value::Null), result),
            Err((code, message)) => {
                rpc_error(id.clone().unwrap_or(serde_json::Value::Null), code, message)
            }
        },
        _ => rpc_error(
            id.clone().unwrap_or(serde_json::Value::Null),
            -32601,
            "method not found",
        ),
    };
    if is_notification {
        None
    } else {
        Some(response)
    }
}

fn create_idea_tool() -> serde_json::Value {
    serde_json::json!({
        "name": "create_idea",
        "description": "Create a Grimoire idea on a verified remote branch",
        "inputSchema": {
            "type": "object",
            "properties": {
                "title": {"type": "string"}, "problem": {"type": "string"},
                "desired_outcome": {"type": "string"}, "scope": {"type": "string"},
                "non_goals": {"type": "string"}, "constraints": {"type": "string"},
                "open_questions": {"type": "string"}, "id": {"type": "string"}
            },
            "required": ["title", "problem", "desired_outcome", "scope", "non_goals", "constraints", "open_questions"]
        }
    })
}

fn call_tool(
    repository: &Path,
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, (i32, &'static str)> {
    let params = params
        .and_then(serde_json::Value::as_object)
        .ok_or((-32602, "params.arguments must be an object"))?;
    if params.get("name").and_then(serde_json::Value::as_str) != Some("create_idea") {
        return Err((-32601, "tool not found"));
    }
    let arguments = params
        .get("arguments")
        .and_then(serde_json::Value::as_object)
        .ok_or((-32602, "params.arguments must be an object"))?;
    let get = |name: &'static str| {
        arguments
            .get(name)
            .and_then(serde_json::Value::as_str)
            .ok_or((-32602, "tool arguments must be strings"))
    };
    let title = get("title")?;
    let problem = get("problem")?;
    let desired_outcome = get("desired_outcome")?;
    let scope = get("scope")?;
    let non_goals = get("non_goals")?;
    let constraints = get("constraints")?;
    let open_questions = get("open_questions")?;
    let id = arguments
        .get("id")
        .map(|value| {
            value
                .as_str()
                .ok_or((-32602, "tool arguments must be strings"))
        })
        .transpose()?;
    match create_idea(
        repository,
        IdeaInput {
            title,
            problem,
            desired_outcome,
            scope,
            non_goals,
            constraints,
            open_questions,
            id,
        },
    ) {
        Ok(result) => Ok(
            serde_json::json!({"content": [{"type": "text", "text": serde_json::to_string(&result).unwrap()}]}),
        ),
        Err(error) => Ok(
            serde_json::json!({"isError": true, "content": [{"type": "text", "text": serde_json::to_string(&error).unwrap()}]}),
        ),
    }
}

fn rpc_result(id: serde_json::Value, result: serde_json::Value) -> String {
    serde_json::json!({"jsonrpc":"2.0", "id":id, "result":result}).to_string()
}

fn rpc_error(id: serde_json::Value, code: i32, message: impl Into<String>) -> String {
    serde_json::json!({"jsonrpc":"2.0", "id":id, "error":{"code":code, "message":message.into()}})
        .to_string()
}

#[derive(Debug, Serialize)]
struct SpecsetResult {
    status: String,
    specset_id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    blockers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<SpecTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feedback: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Clone)]
struct SpecTarget {
    repository: String,
    specification: String,
    agent: String,
    dependencies: Vec<String>,
    branch: String,
}

fn run_specset(action: SpecsetAction) -> Result<serde_json::Value, ErrorBody<'static>> {
    if let SpecsetAction::Feedback { upstream, number } = action {
        let provider = provider_for(&upstream)?;
        let value = provider_feedback(&provider, &upstream, number)?;
        return Ok(
            serde_json::json!({"status":"ok", "upstream":upstream, "number":number, "feedback":value}),
        );
    }
    let input = match &action {
        SpecsetAction::Ready(input) | SpecsetAction::Next(input) => input,
        SpecsetAction::Status { input, .. }
        | SpecsetAction::Materialize { input, .. }
        | SpecsetAction::Publish { input, .. } => input,
        SpecsetAction::Feedback { .. } => unreachable!(),
    };
    let manifest: RealManifest = read_catalog_value(&input.manifest)?;
    let state = match load_real_state(&manifest, &input.manifest, &input.catalog) {
        Ok(state) => state,
        Err(error) => {
            return Ok(serde_json::json!({
                "status": "blocked",
                "specset_id": manifest.id,
                "blockers": [error.message],
            }));
        }
    };
    let blockers = validate_real_specset(&state);
    if !blockers.is_empty() {
        return Ok(serde_json::to_value(SpecsetResult {
            status: "blocked".into(),
            specset_id: manifest.id.clone(),
            blockers,
            target: None,
            branch: None,
            pull_request: None,
            feedback: None,
        })
        .unwrap());
    }
    if matches!(action, SpecsetAction::Ready(_)) {
        return Ok(serde_json::to_value(SpecsetResult {
            status: "ready".into(),
            specset_id: manifest.id.clone(),
            blockers: vec![],
            target: None,
            branch: Some(real_branch_name(&manifest)),
            pull_request: None,
            feedback: None,
        })
        .unwrap());
    }
    let target = next_real_target(&state, |repo, spec| {
        merged_for_spec(
            &provider_for(&repo.mapping.upstream)?,
            &repo.upstream,
            &real_branch_name(&manifest),
            &spec.directory,
        )
    })?;
    let branch = real_branch_name(&manifest);
    match action {
        SpecsetAction::Ready(_) => unreachable!(),
        SpecsetAction::Next(_) => Ok(serde_json::to_value(SpecsetResult {
            status: if target.is_some() {
                "available"
            } else {
                "complete"
            }
            .into(),
            specset_id: manifest.id,
            blockers: vec![],
            target,
            branch: Some(branch),
            pull_request: None,
            feedback: None,
        })
        .unwrap()),
        SpecsetAction::Status { repository, .. } => {
            let target = target.filter(|t| repository.as_deref().is_none_or(|r| r == t.repository));
            Ok(serde_json::to_value(SpecsetResult {
                status: if target.is_some() {
                    "incomplete"
                } else {
                    "complete"
                }
                .into(),
                specset_id: manifest.id,
                blockers: vec![],
                target,
                branch: Some(branch),
                pull_request: None,
                feedback: None,
            })
            .unwrap())
        }
        SpecsetAction::Materialize { worktree, .. } => {
            let Some(target) = target else {
                return Ok(serde_json::json!({"status":"complete", "specset_id":manifest.id}));
            };
            materialize_real(&state, &worktree, &target)?;
            Ok(
                serde_json::json!({"status":"materialized", "specset_id":manifest.id, "target":target, "branch":branch}),
            )
        }
        SpecsetAction::Publish {
            worktree,
            message,
            title,
            ..
        } => {
            let Some(target) = target else {
                return Ok(serde_json::json!({"status":"complete", "specset_id":manifest.id}));
            };
            publish_real(&state, &worktree, &target, &message, title.as_deref())
        }
        SpecsetAction::Feedback { .. } => unreachable!(),
    }
}

#[derive(Debug, Deserialize)]
struct RealManifest {
    #[serde(alias = "specset_id", alias = "specsetId")]
    id: String,
    #[serde(default)]
    repositories: Vec<String>,
    #[serde(default, alias = "implementationOrder")]
    implementation_order: Vec<String>,
    #[serde(default)]
    #[serde(alias = "specDependencies")]
    spec_dependencies: serde_yaml::Value,
}

#[derive(Debug, Deserialize)]
struct DependencyRecord {
    repository: String,
    spec: String,
    #[serde(default)]
    depends_on: Vec<DependencyRef>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DependencyRef {
    Name(String),
    Object { repository: String, spec: String },
}

#[derive(Debug, Deserialize)]
struct RealRepositoryMapping {
    #[serde(default)]
    id: String,
    upstream: String,
    #[serde(default, alias = "upstreamCreated")]
    upstream_created: bool,
    #[serde(default, alias = "defaultBranch")]
    default_branch: String,
}

#[derive(Debug, Deserialize)]
struct RealAgent {
    #[serde(default)]
    id: String,
    #[serde(default)]
    prefix: String,
}

#[derive(Debug)]
struct RealSpec {
    directory: String,
    agent: String,
    dependencies: Vec<String>,
    source: PathBuf,
}

#[derive(Debug)]
struct RealRepository {
    id: String,
    mapping: RealRepositoryMapping,
    upstream: String,
    specs: Vec<RealSpec>,
}

#[derive(Debug)]
struct RealState {
    manifest: RealManifest,
    repositories: Vec<RealRepository>,
    agents: std::collections::BTreeSet<String>,
}

fn normalized_dependencies(
    value: &serde_yaml::Value,
) -> std::collections::BTreeMap<String, Vec<String>> {
    if let Ok(records) = serde_yaml::from_value::<Vec<DependencyRecord>>(value.clone()) {
        return records
            .into_iter()
            .map(|record| {
                let key = format!("{}/{}", record.repository, record.spec);
                let dependencies = record
                    .depends_on
                    .into_iter()
                    .map(|dependency| match dependency {
                        DependencyRef::Name(name) => {
                            if name.contains('/') {
                                name
                            } else {
                                format!("{}/{}", record.repository, name)
                            }
                        }
                        DependencyRef::Object { repository, spec } => {
                            format!("{repository}/{spec}")
                        }
                    })
                    .collect();
                (key, dependencies)
            })
            .collect();
    }
    serde_yaml::from_value::<std::collections::BTreeMap<String, Vec<String>>>(value.clone())
        .unwrap_or_default()
        .into_iter()
        .map(|(key, dependencies)| {
            let repository = key.split_once('/').map_or(key.as_str(), |(repo, _)| repo);
            let dependencies = dependencies
                .into_iter()
                .map(|dependency| {
                    if dependency.contains('/') {
                        dependency
                    } else {
                        format!("{repository}/{dependency}")
                    }
                })
                .collect();
            (key, dependencies)
        })
        .collect()
}

fn load_real_state(
    manifest: &RealManifest,
    manifest_path: &Path,
    catalog: &Path,
) -> Result<RealState, ErrorBody<'static>> {
    let agents_dir = catalog.join("agents");
    let mut agents = std::collections::BTreeSet::new();
    if let Ok(entries) = fs::read_dir(&agents_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let value: RealAgent = read_catalog_value(&entry.path())?;
            let fallback = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_owned();
            agents.insert(if value.id.is_empty() {
                fallback
            } else {
                value.id.clone()
            });
            if !value.prefix.is_empty() {
                agents.insert(value.prefix);
            }
        }
    }
    let mut repositories = Vec::new();
    let dependencies = normalized_dependencies(&manifest.spec_dependencies);
    for id in &manifest.repositories {
        let mapping_path = catalog.join("repos").join(format!("{id}.json"));
        let mut mapping: RealRepositoryMapping = read_catalog_value(&mapping_path)?;
        if mapping.id.is_empty() {
            mapping.id = id.clone();
        }
        let specs_root = manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(id)
            .join("openspec/specs");
        let mut specs = Vec::new();
        if let Ok(entries) = fs::read_dir(&specs_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() || !path.join("spec.md").is_file() {
                    continue;
                }
                let directory = entry.file_name().to_string_lossy().into_owned();
                let agent = agents
                    .iter()
                    .filter(|a| directory.starts_with(&format!("{}-", a)))
                    .max_by_key(|a| a.len())
                    .cloned()
                    .unwrap_or_else(|| directory.split('-').next().unwrap_or("").to_owned());
                let key = format!("{id}/{directory}");
                specs.push(RealSpec {
                    directory,
                    agent,
                    dependencies: dependencies.get(&key).cloned().unwrap_or_default(),
                    source: path,
                });
            }
        }
        specs.sort_by(|a, b| a.directory.cmp(&b.directory));
        repositories.push(RealRepository {
            id: id.clone(),
            upstream: mapping.upstream.clone(),
            mapping,
            specs,
        });
    }
    Ok(RealState {
        manifest: RealManifest {
            id: manifest.id.clone(),
            repositories: manifest.repositories.clone(),
            implementation_order: manifest.implementation_order.clone(),
            spec_dependencies: manifest.spec_dependencies.clone(),
        },
        repositories,
        agents,
    })
}

fn validate_real_specset(state: &RealState) -> Vec<String> {
    let mut errors = Vec::new();
    if state.manifest.id.is_empty() {
        errors.push("specset id is empty".into());
    }
    if state.manifest.repositories.is_empty() {
        errors.push("no repository mappings".into());
    }
    if state.manifest.implementation_order.is_empty() {
        errors.push("no implementation_order".into());
    }
    if state
        .repositories
        .iter()
        .any(|r| !r.mapping.upstream_created)
    {
        errors.push("repository upstream_created is false".into());
    }
    for repo in &state.repositories {
        if provider_for(&repo.mapping.upstream).is_err() {
            errors.push(format!("repository '{}' has unsupported upstream", repo.id));
        }
        for spec in &repo.specs {
            if spec.agent.is_empty() || !state.agents.contains(&spec.agent) {
                errors.push(format!(
                    "specification '{}' has an unregistered agent prefix",
                    spec.directory
                ));
            }
            for dep in &spec.dependencies {
                let Some((dep_repo, dep_spec)) = dep.split_once('/') else {
                    errors.push(format!("invalid dependency '{}'", dep));
                    continue;
                };
                if dep_repo != repo.id || !repo.specs.iter().any(|s| s.directory == dep_spec) {
                    errors.push(format!("invalid same-repository dependency '{}'", dep));
                }
            }
        }
    }
    for repo_id in &state.manifest.implementation_order {
        if !state.repositories.iter().any(|r| &r.id == repo_id) {
            errors.push(format!(
                "missing implementation_order repository '{}'",
                repo_id
            ));
        }
    }
    let mut graph = std::collections::BTreeMap::new();
    for repo in &state.repositories {
        for spec in &repo.specs {
            graph.insert(
                format!("{}/{}", repo.id, spec.directory),
                spec.dependencies.iter().cloned().collect::<Vec<_>>(),
            );
        }
    }
    fn visit(
        node: &str,
        graph: &std::collections::BTreeMap<String, Vec<String>>,
        active: &mut std::collections::BTreeSet<String>,
        done: &mut std::collections::BTreeSet<String>,
    ) -> bool {
        if active.contains(node) {
            return true;
        }
        if done.contains(node) {
            return false;
        }
        active.insert(node.into());
        let cycle = graph.get(node).map_or(false, |deps| {
            deps.iter().any(|d| visit(d, graph, active, done))
        });
        active.remove(node);
        done.insert(node.into());
        cycle
    }
    let mut active = std::collections::BTreeSet::new();
    let mut done = std::collections::BTreeSet::new();
    if graph
        .keys()
        .any(|n| visit(n, &graph, &mut active, &mut done))
    {
        errors.push("spec dependency cycle".into());
    }
    errors.sort();
    errors.dedup();
    errors
}

fn real_branch_name(manifest: &RealManifest) -> String {
    format!("feature/{}", manifest.id)
}

fn next_real_target<F>(
    state: &RealState,
    mut merged: F,
) -> Result<Option<SpecTarget>, ErrorBody<'static>>
where
    F: FnMut(&RealRepository, &RealSpec) -> Result<bool, ErrorBody<'static>>,
{
    for repo_id in &state.manifest.implementation_order {
        let repo = state
            .repositories
            .iter()
            .find(|r| &r.id == repo_id)
            .unwrap();
        for spec in &repo.specs {
            if merged(repo, spec)? {
                continue;
            }
            let mut dependencies_ready = true;
            for dependency in &spec.dependencies {
                let Some((dependency_repo, dependency_spec)) = dependency.split_once('/') else {
                    dependencies_ready = false;
                    break;
                };
                let Some(dependency_repo) =
                    state.repositories.iter().find(|r| r.id == dependency_repo)
                else {
                    dependencies_ready = false;
                    break;
                };
                let Some(dependency_spec) = dependency_repo
                    .specs
                    .iter()
                    .find(|s| s.directory == dependency_spec)
                else {
                    dependencies_ready = false;
                    break;
                };
                if !merged(dependency_repo, dependency_spec)? {
                    dependencies_ready = false;
                    break;
                }
            }
            if !dependencies_ready {
                continue;
            }
            return Ok(Some(SpecTarget {
                repository: repo.id.clone(),
                specification: spec.directory.clone(),
                agent: spec.agent.clone(),
                dependencies: spec.dependencies.clone(),
                branch: real_branch_name(&state.manifest),
            }));
        }
    }
    Ok(None)
}

fn materialize_real(
    state: &RealState,
    worktree: &Path,
    target: &SpecTarget,
) -> Result<(), ErrorBody<'static>> {
    let repo = state
        .repositories
        .iter()
        .find(|r| r.id == target.repository)
        .ok_or_else(|| err("input_error", "target repository is not in manifest".into()))?;
    let spec = repo
        .specs
        .iter()
        .find(|s| s.directory == target.specification)
        .ok_or_else(|| {
            err(
                "input_error",
                "target specification is not in catalog".into(),
            )
        })?;
    let destination = worktree.join("openspec/specs").join(&spec.directory);
    fs::create_dir_all(&destination).map_err(|_| {
        err(
            "filesystem_error",
            "cannot create materialization directory".into(),
        )
    })?;
    fs::copy(spec.source.join("spec.md"), destination.join("spec.md")).map_err(|_| {
        err(
            "filesystem_error",
            "cannot materialize specification".into(),
        )
    })?;
    Ok(())
}

fn publish_real(
    state: &RealState,
    worktree: &Path,
    target: &SpecTarget,
    message: &str,
    title: Option<&str>,
) -> Result<serde_json::Value, ErrorBody<'static>> {
    let repo = state
        .repositories
        .iter()
        .find(|r| r.id == target.repository)
        .unwrap();
    publish_repository(
        &state.manifest.id,
        &repo.id,
        &repo.mapping.upstream,
        &repo.mapping.default_branch,
        worktree,
        target,
        message,
        title,
    )
}

fn read_catalog_value<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, ErrorBody<'static>> {
    let text = fs::read_to_string(path)
        .map_err(|_| err("input_error", format!("cannot read '{}'", path.display())))?;
    serde_json::from_str(&text)
        .or_else(|_| serde_yaml::from_str(&text))
        .map_err(|_| {
            err(
                "input_error",
                format!("invalid catalog '{}'", path.display()),
            )
        })
}

#[cfg(any())]
fn all_specs(repo: &SpecRepository) -> &[ManifestSpec] {
    if repo.specifications.is_empty() {
        &repo.specs
    } else {
        &repo.specifications
    }
}

#[cfg(any())]
fn validate_specset(manifest: &SpecsetManifest, catalog: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    if manifest.id.is_empty() {
        errors.push("specset id is empty".into());
    }
    if manifest.repositories.is_empty() {
        errors.push("no repository mappings".into());
    }
    if manifest.agents.is_empty() {
        errors.push("no registered agents".into());
    }
    let mut repos = std::collections::BTreeMap::new();
    for repo in &manifest.repositories {
        if repos.insert(&repo.id, repo).is_some() {
            errors.push(format!("duplicate repository '{}'", repo.id));
        }
        if !repo.upstream_created {
            errors.push(format!(
                "repository '{}' upstream_created is false",
                repo.id
            ));
        }
        if provider_for(&repo.upstream).is_err() {
            errors.push(format!("repository '{}' has unsupported upstream", repo.id));
        }
        if repo.agent_prefix.is_empty() {
            errors.push(format!("repository '{}' has no agent prefix", repo.id));
        }
        let registered = manifest
            .agents
            .iter()
            .any(|a| a.id == repo.agent_prefix || a.prefix == repo.agent_prefix);
        if !registered {
            errors.push(format!(
                "repository '{}' agent prefix '{}' is not registered",
                repo.id, repo.agent_prefix
            ));
        }
        for spec in all_specs(repo) {
            if spec.agent.is_empty() || !spec.agent.starts_with(&repo.agent_prefix) {
                errors.push(format!("spec '{}' is not agent-prefixed", spec.id));
            }
            let path = catalog.join(&repo.agent_prefix).join(&spec.id);
            if !path.is_dir() {
                errors.push(format!(
                    "missing specification directory '{}'",
                    path.display()
                ));
            }
            for dep in &spec.spec_dependencies {
                let Some((dep_repo, dep_spec)) = dep.split_once('/') else {
                    errors.push(format!("invalid dependency '{}'", dep));
                    continue;
                };
                if dep_repo != repo.id {
                    errors.push(format!("dependency '{}' crosses repositories", dep));
                }
                if !all_specs(repo).iter().any(|s| s.id == dep_spec) {
                    errors.push(format!("missing dependency '{}'", dep));
                }
            }
        }
    }
    for item in &manifest.implementation_order {
        if !item.contains('/')
            || !manifest.repositories.iter().any(|r| {
                all_specs(r)
                    .iter()
                    .any(|s| format!("{}/{}", r.id, s.id) == *item)
            })
        {
            errors.push(format!("missing implementation_order reference '{}'", item));
        }
    }
    let mut edges = std::collections::BTreeMap::<String, Vec<String>>::new();
    for repo in &manifest.repositories {
        for spec in all_specs(repo) {
            edges.insert(
                format!("{}/{}", repo.id, spec.id),
                spec.spec_dependencies
                    .iter()
                    .map(|d| {
                        format!(
                            "{}/{}",
                            repo.id,
                            d.split_once('/').map_or(d.as_str(), |(_, s)| s)
                        )
                    })
                    .collect(),
            );
        }
    }
    fn visit(
        node: &str,
        edges: &std::collections::BTreeMap<String, Vec<String>>,
        visiting: &mut std::collections::BTreeSet<String>,
        done: &mut std::collections::BTreeSet<String>,
    ) -> bool {
        if visiting.contains(node) {
            return true;
        }
        if done.contains(node) {
            return false;
        }
        visiting.insert(node.into());
        let cycle = edges.get(node).map_or(false, |ds| {
            ds.iter().any(|d| visit(d, edges, visiting, done))
        });
        visiting.remove(node);
        done.insert(node.into());
        cycle
    }
    let mut visiting = std::collections::BTreeSet::new();
    let mut done = std::collections::BTreeSet::new();
    if edges
        .keys()
        .any(|n| visit(n, &edges, &mut visiting, &mut done))
    {
        errors.push("spec dependency cycle".into());
    }
    errors.sort();
    errors.dedup();
    errors
}

#[cfg(any())]
fn branch_name(manifest: &SpecsetManifest) -> String {
    format!("feature/{}", manifest.id)
}

#[cfg(any())]
fn next_target<F>(
    manifest: &SpecsetManifest,
    _catalog: &Path,
    mut merged: F,
) -> Result<Option<SpecTarget>, ErrorBody<'static>>
where
    F: FnMut(&SpecRepository) -> Result<bool, ErrorBody<'static>>,
{
    let ordered: Vec<String> = if manifest.implementation_order.is_empty() {
        manifest
            .repositories
            .iter()
            .flat_map(|r| all_specs(r).iter().map(|s| format!("{}/{}", r.id, s.id)))
            .collect()
    } else {
        manifest.implementation_order.clone()
    };
    for key in ordered {
        let Some((rid, sid)) = key.split_once('/') else {
            continue;
        };
        let repo = manifest.repositories.iter().find(|r| r.id == rid).unwrap();
        let spec = all_specs(repo).iter().find(|s| s.id == sid).unwrap();
        if !merged(repo)? {
            return Ok(Some(SpecTarget {
                repository: rid.into(),
                specification: sid.into(),
                agent: if spec.agent.is_empty() {
                    repo.agent_prefix.clone()
                } else {
                    spec.agent.clone()
                },
                dependencies: spec.spec_dependencies.clone(),
                branch: branch_name(manifest),
            }));
        }
    }
    Ok(None)
}

#[cfg(any())]
fn merged_for(
    provider: &Provider,
    upstream: &str,
    branch: &str,
) -> Result<bool, ErrorBody<'static>> {
    let value = provider_prs(provider, upstream, branch)?;
    Ok(value.as_array().map_or(false, |prs| {
        prs.iter().any(|pr| {
            pr.get("state")
                .and_then(serde_json::Value::as_str)
                .map_or(false, |s| s.eq_ignore_ascii_case("merged"))
        })
    }))
}

fn merged_for_spec(
    provider: &Provider,
    upstream: &str,
    branch: &str,
    specification: &str,
) -> Result<bool, ErrorBody<'static>> {
    let value = provider_prs(provider, upstream, branch)?;
    let Some(prs) = value.as_array() else {
        return Ok(false);
    };
    let merged: Vec<&serde_json::Value> = prs
        .iter()
        .filter(|pr| {
            pr.get("state")
                .and_then(serde_json::Value::as_str)
                .map_or(false, |state| state.eq_ignore_ascii_case("merged"))
        })
        .collect();
    let identified = merged.iter().any(|pr| pr_spec_matches(pr, ""));
    if !identified {
        return Ok(!merged.is_empty());
    }
    Ok(merged.iter().any(|pr| pr_spec_matches(pr, specification)))
}

fn pr_spec_matches(pr: &serde_json::Value, specification: &str) -> bool {
    ["specification", "spec", "specification_dir"]
        .iter()
        .any(|key| {
            pr.get(*key)
                .and_then(serde_json::Value::as_str)
                .map_or(false, |value| {
                    specification.is_empty() || value == specification
                })
        })
        || (!specification.is_empty()
            && ["title", "body"].iter().any(|key| {
                pr.get(*key)
                    .and_then(serde_json::Value::as_str)
                    .map_or(false, |value| value.contains(specification))
            }))
}

fn provider_prs(
    provider: &Provider,
    upstream: &str,
    branch: &str,
) -> Result<serde_json::Value, ErrorBody<'static>> {
    let output = match provider {
        Provider::Github => command_output(
            "gh",
            &[
                "pr",
                "list",
                "--repo",
                upstream,
                "--head",
                branch,
                "--state",
                "all",
                "--json",
                "number,title,body,state,url,headRefName",
            ],
            "list pull requests",
        )?,
        Provider::Gitea => command_output(
            "tea",
            &["pr", "list", "--repo", upstream, "--state", "all"],
            "list pull requests",
        )?,
    };
    serde_json::from_str(&output).map_err(|_| {
        err(
            "provider_error",
            "provider returned invalid pull-request JSON".into(),
        )
    })
}

fn provider_feedback(
    provider: &Provider,
    upstream: &str,
    number: u64,
) -> Result<serde_json::Value, ErrorBody<'static>> {
    let number = number.to_string();
    let output = match provider {
        Provider::Github => command_output(
            "gh",
            &[
                "pr",
                "view",
                &number,
                "--repo",
                upstream,
                "--json",
                "number,state,reviews,comments",
            ],
            "read pull-request feedback",
        )?,
        Provider::Gitea => command_output(
            "tea",
            &["pr", "show", &number, "--repo", upstream],
            "read pull-request feedback",
        )?,
    };
    let value: serde_json::Value = serde_json::from_str(&output).map_err(|_| {
        err(
            "provider_error",
            "provider returned invalid feedback JSON".into(),
        )
    })?;
    Ok(bound_feedback(value))
}

fn bound_feedback(mut value: serde_json::Value) -> serde_json::Value {
    if let Some(object) = value.as_object_mut() {
        for key in ["reviews", "comments"] {
            if let Some(items) = object
                .get_mut(key)
                .and_then(serde_json::Value::as_array_mut)
            {
                items.truncate(50);
            }
        }
    }
    value
}

#[cfg(any())]
fn materialize(
    manifest: &SpecsetManifest,
    catalog: &Path,
    worktree: &Path,
    target: &SpecTarget,
) -> Result<(), ErrorBody<'static>> {
    let repo = manifest
        .repositories
        .iter()
        .find(|repo| repo.id == target.repository)
        .ok_or_else(|| err("input_error", "target repository is not in manifest".into()))?;
    copy_tree(
        &catalog.join(&repo.agent_prefix).join(&target.specification),
        &worktree
            .join("openspec/specs")
            .join(&target.agent)
            .join(&target.specification),
    )
}

#[cfg(any())]
fn copy_tree(source: &Path, destination: &Path) -> Result<(), ErrorBody<'static>> {
    fs::create_dir_all(destination).map_err(|_| {
        err(
            "filesystem_error",
            format!("cannot create '{}'", destination.display()),
        )
    })?;
    for entry in fs::read_dir(source).map_err(|_| {
        err(
            "filesystem_error",
            format!("cannot read '{}'", source.display()),
        )
    })? {
        let entry =
            entry.map_err(|_| err("filesystem_error", "cannot read catalog entry".into()))?;
        let from = entry.path();
        let to = destination.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to)?;
        } else {
            fs::copy(&from, &to).map_err(|_| {
                err(
                    "filesystem_error",
                    format!("cannot materialize '{}'", from.display()),
                )
            })?;
        }
    }
    Ok(())
}

fn publish_repository(
    specset_id: &str,
    repository: &str,
    upstream: &str,
    default_branch: &str,
    worktree: &Path,
    target: &SpecTarget,
    message: &str,
    title: Option<&str>,
) -> Result<serde_json::Value, ErrorBody<'static>> {
    let branch = format!("feature/{specset_id}");
    run_git_in(worktree, &["checkout", "-B", &branch])?;
    run_git_in(worktree, &["add", "openspec/specs"])?;
    let _ = run_git_in(worktree, &["commit", "-m", message]);
    run_git_in(worktree, &["push", "-u", "origin", &branch])?;
    let provider = provider_for(upstream)?;
    let prs = provider_prs(&provider, upstream, &branch)?;
    if prs.as_array().map_or(0, Vec::len) > 1 {
        return Err(err(
            "provider_error",
            format!("multiple pull requests match branch '{}'", branch),
        ));
    }
    let existing = prs
        .as_array()
        .and_then(|a| a.first())
        .and_then(|p| p.get("number"))
        .and_then(serde_json::Value::as_u64);
    let pr = if let Some(number) = existing {
        let n = number.to_string();
        if let Some(title) = title {
            command(
                provider.tool(),
                &["pr", "edit", &n, "--title", title],
                "update pull request",
            )?;
        }
        serde_json::json!({"number": number, "status": "updated"})
    } else {
        let title = title.unwrap_or(&target.specification);
        let base = if default_branch.is_empty() {
            "main"
        } else {
            default_branch
        };
        let output = match provider {
            Provider::Github => command_output(
                "gh",
                &[
                    "pr", "create", "--repo", upstream, "--head", &branch, "--base", base,
                    "--title", title, "--body", message,
                ],
                "create pull request",
            )?,
            Provider::Gitea => command_output(
                "tea",
                &[
                    "pr",
                    "create",
                    "--repo",
                    upstream,
                    "--head",
                    &branch,
                    "--base",
                    base,
                    "--title",
                    title,
                    "--description",
                    message,
                ],
                "create pull request",
            )?,
        };
        serde_json::json!({"status": "created", "output": output.trim()})
    };
    Ok(
        serde_json::json!({"status":"published", "specset_id":specset_id, "repository":repository, "target":target, "branch":branch, "pull_request":pr}),
    )
}

fn command_output(
    tool: &str,
    args: &[&str],
    operation: &str,
) -> Result<String, ErrorBody<'static>> {
    Command::new(tool)
        .args(args)
        .output()
        .map_err(|_| {
            err(
                "provider_error",
                format!("could not execute {} to {}", tool, operation),
            )
        })
        .and_then(|output| {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).into_owned())
            } else {
                Err(err(
                    "provider_error",
                    format!("{} failed while attempting to {}", tool, operation),
                ))
            }
        })
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

struct IdeaInput<'a> {
    title: &'a str,
    problem: &'a str,
    desired_outcome: &'a str,
    scope: &'a str,
    non_goals: &'a str,
    constraints: &'a str,
    open_questions: &'a str,
    id: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct IdeaResult {
    status: &'static str,
    id: String,
    branch: String,
    commit: String,
    repository: String,
    path: String,
    provider: &'static str,
}

const IDEA_PATH: &str = "ideas/{id}/IDEA.md";

fn derive_idea_id(title: &str) -> String {
    let mut id = String::new();
    for character in title.chars() {
        if character.is_ascii_alphanumeric() {
            id.push(character.to_ascii_lowercase());
        } else if !id.is_empty() && !id.ends_with('-') {
            id.push('-');
        }
    }
    id.trim_end_matches('-').to_owned()
}

fn validate_idea_id(id: &str) -> Result<(), ErrorBody<'static>> {
    let valid = !id.is_empty()
        && id.len() <= 80
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !id.starts_with('-')
        && !id.ends_with('-')
        && !id.contains("--");
    if valid {
        Ok(())
    } else {
        Err(err(
            "invalid_idea_id",
            "idea ID must be 1-80 characters of lowercase letters, digits, and single hyphens"
                .into(),
        ))
    }
}

fn required_idea_text(name: &str, value: &str) -> Result<(), ErrorBody<'static>> {
    if value.trim().is_empty() {
        Err(err(
            "missing_content",
            format!("--{name} must not be empty"),
        ))
    } else {
        Ok(())
    }
}

fn render_idea(input: &IdeaInput<'_>) -> String {
    format!(
        "---\nprocessed: false\n---\n\n# {}\n\n## Problem\n{}\n\n## Desired Outcome\n{}\n\n## Scope\n{}\n\n## Non-Goals\n{}\n\n## Constraints\n{}\n\n## Open Questions\n{}\n",
        input.title.trim(),
        input.problem.trim(),
        input.desired_outcome.trim(),
        input.scope.trim(),
        input.non_goals.trim(),
        input.constraints.trim(),
        input.open_questions.trim()
    )
}

fn create_idea(
    repository_path: &Path,
    input: IdeaInput<'_>,
) -> Result<IdeaResult, ErrorBody<'static>> {
    for (name, value) in [
        ("title", input.title),
        ("problem", input.problem),
        ("desired-outcome", input.desired_outcome),
        ("scope", input.scope),
        ("non-goals", input.non_goals),
        ("constraints", input.constraints),
        ("open-questions", input.open_questions),
    ] {
        required_idea_text(name, value)?;
    }
    let id = input
        .id
        .map_or_else(|| derive_idea_id(input.title), str::to_owned);
    validate_idea_id(&id)?;
    let mapping: RealRepositoryMapping = read_catalog_value(repository_path)?;
    if mapping.upstream.trim().is_empty() || mapping.default_branch.trim().is_empty() {
        return Err(err(
            "invalid_repository_mapping",
            "repository mapping must contain non-empty upstream and default_branch".into(),
        ));
    }
    if !mapping.upstream_created {
        return Err(err(
            "repository_not_created",
            "repository mapping has upstream_created=false; create the upstream first".into(),
        ));
    }
    let provider = provider_for(&mapping.upstream)?;
    if !tool_available(provider.tool()) {
        return Err(err(
            "provider_unavailable",
            format!(
                "required executable '{}' was not found in PATH",
                provider.tool()
            ),
        ));
    }
    let branch = format!("idea/{id}");
    let git_upstream = git_url(&mapping.upstream, false);
    if remote_ref_exists(&git_upstream, &branch)? {
        return Err(err(
            "duplicate_idea",
            format!("remote branch '{branch}' already exists; choose another ID"),
        ));
    }

    let work = temporary_directory(&id)?;
    let cleanup = TempCleanup::new(work.clone());
    let clone_path = work.to_str().ok_or_else(|| {
        err(
            "filesystem_error",
            "isolated clone path is not valid UTF-8".into(),
        )
    })?;
    run_git(&[
        "clone",
        "--branch",
        &mapping.default_branch,
        "--single-branch",
        &git_upstream,
        clone_path,
    ])?;
    let record_path = IDEA_PATH.replace("{id}", &id);
    if git_ref_path_exists(&work, &mapping.default_branch, &record_path)? {
        return Err(err(
            "duplicate_idea",
            format!("canonical idea record '{record_path}' already exists; choose another ID"),
        ));
    }
    let destination = work.join(&record_path);
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .map_err(|_| err("filesystem_error", "could not create idea directory".into()))?;
    }
    fs::write(&destination, render_idea(&input))
        .map_err(|_| err("filesystem_error", "could not write IDEA.md".into()))?;
    run_git_in(&work, &["add", &record_path])?;
    run_git_in(&work, &["commit", "-m", &format!("Create idea {id}")])?;
    let commit = git_output(&work, &["rev-parse", "HEAD"])?;
    let commit = commit.trim().to_owned();
    run_git_in(&work, &["push", "-u", "origin", &branch])?;
    let remote_commit = remote_commit(&git_upstream, &branch)?;
    if remote_commit != commit {
        return Err(err(
            "verification_failed",
            format!("remote branch '{branch}' did not resolve to the committed IDEA.md"),
        ));
    }
    drop(cleanup);
    Ok(IdeaResult {
        status: "created",
        id,
        branch,
        commit,
        repository: redact_remote(&mapping.upstream),
        path: record_path,
        provider: provider.name(),
    })
}

struct TempCleanup(PathBuf);
impl TempCleanup {
    fn new(path: PathBuf) -> Self {
        Self(path)
    }
}
impl Drop for TempCleanup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn remote_ref_exists(remote: &str, branch: &str) -> Result<bool, ErrorBody<'static>> {
    let output = Command::new("git")
        .args([
            "ls-remote",
            "--exit-code",
            remote,
            &format!("refs/heads/{branch}"),
        ])
        .output()
        .map_err(|_| err("git_error", "could not check the remote idea branch".into()))?;
    if output.status.success() {
        Ok(!output.stdout.is_empty())
    } else if output.status.code() == Some(2) {
        // `--exit-code` uses status 2 when the remote is reachable but the ref
        // does not exist. Other failures must not be mistaken for a free ID.
        Ok(false)
    } else {
        Err(err(
            "git_error",
            "could not verify whether the remote idea branch exists".into(),
        ))
    }
}

fn remote_commit(remote: &str, branch: &str) -> Result<String, ErrorBody<'static>> {
    let output = command_output(
        "git",
        &["ls-remote", remote, &format!("refs/heads/{branch}")],
        "verify pushed idea",
    )?;
    output
        .split_whitespace()
        .next()
        .map(str::to_owned)
        .ok_or_else(|| {
            err(
                "verification_failed",
                format!("remote branch '{branch}' was not found after push"),
            )
        })
}

fn git_ref_path_exists(work: &Path, branch: &str, path: &str) -> Result<bool, ErrorBody<'static>> {
    let reference = format!("origin/{branch}:{path}");
    let output = Command::new("git")
        .current_dir(work)
        .args(["cat-file", "-e", &reference])
        .output()
        .map_err(|_| {
            err(
                "git_error",
                "could not inspect the canonical idea record".into(),
            )
        })?;
    Ok(output.status.success())
}

fn redact_remote(remote: &str) -> String {
    if let Some((scheme, rest)) = remote.split_once("://") {
        if let Some((_, host)) = rest.split_once('@') {
            return format!("{scheme}://[redacted]@{host}");
        }
    }
    remote.to_owned()
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

    #[test]
    fn derives_stable_kebab_case_idea_ids() {
        assert_eq!(
            derive_idea_id("Plan OAuth 2.0 / Login"),
            "plan-oauth-2-0-login"
        );
        assert_eq!(derive_idea_id("  Already--Stable  "), "already-stable");
    }

    #[test]
    fn validates_explicit_idea_ids() {
        assert!(validate_idea_id("valid-idea-2").is_ok());
        assert!(validate_idea_id("Not-valid").is_err());
        assert!(validate_idea_id("two--hyphens").is_err());
    }

    #[test]
    fn renders_canonical_idea_sections() {
        let input = IdeaInput {
            title: "A title",
            problem: "A problem",
            desired_outcome: "An outcome",
            scope: "The scope",
            non_goals: "Not this",
            constraints: "A constraint",
            open_questions: "A question",
            id: None,
        };
        let rendered = render_idea(&input);
        assert!(rendered.starts_with("---\nprocessed: false\n---\n"));
        for section in [
            "# A title",
            "## Problem",
            "## Desired Outcome",
            "## Scope",
            "## Non-Goals",
            "## Constraints",
            "## Open Questions",
        ] {
            assert!(rendered.contains(section), "missing {section}");
        }
    }

    #[test]
    fn redacts_credentials_in_remote_results() {
        assert_eq!(
            redact_remote("https://token:secret@example.test/a/b"),
            "https://[redacted]@example.test/a/b"
        );
    }

    #[test]
    fn discovers_and_materializes_real_grimoire_layout() {
        let root = test_directory("real-layout");
        let catalog = root.join("catalog");
        fs::create_dir_all(catalog.join("repos")).unwrap();
        fs::create_dir_all(catalog.join("agents")).unwrap();
        fs::write(catalog.join("agents/agent.json"), r#"{"id":"agent"}"#).unwrap();
        fs::write(catalog.join("repos/repo.json"), r#"{"upstream":"https://github.com/a/b","upstream_created":true,"default_branch":"main"}"#).unwrap();
        let leaf = root.join("repo/openspec/specs/agent-a-dependent");
        fs::create_dir_all(&leaf).unwrap();
        fs::write(leaf.join("spec.md"), "# dependent spec\n").unwrap();
        let dependent = root.join("repo/openspec/specs/agent-z-base");
        fs::create_dir_all(&dependent).unwrap();
        fs::write(dependent.join("spec.md"), "# base spec\n").unwrap();
        let manifest_path = root.join("manifest.yaml");
        fs::write(
            &manifest_path,
            "id: demo\nrepositories: [repo]\nimplementation_order: [repo]\nspec_dependencies:\n  - repository: repo\n    spec: agent-a-dependent\n    depends_on:\n      - agent-z-base\n",
        )
        .unwrap();

        let manifest: RealManifest = read_catalog_value(&manifest_path).unwrap();
        let state = load_real_state(&manifest, &manifest_path, &catalog).unwrap();
        assert!(validate_real_specset(&state).is_empty());
        assert_eq!(
            state.repositories[0].specs[0].dependencies,
            vec!["repo/agent-z-base"]
        );
        let target = next_real_target(&state, |_, _| Ok(false)).unwrap().unwrap();
        assert_eq!(target.specification, "agent-z-base");
        let worktree = root.join("worktree");
        materialize_real(&state, &worktree, &target).unwrap();
        assert_eq!(
            fs::read_to_string(worktree.join("openspec/specs/agent-z-base/spec.md")).unwrap(),
            "# base spec\n"
        );
        let _ = fs::remove_dir_all(root);
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
