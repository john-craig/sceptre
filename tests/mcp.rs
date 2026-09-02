use rust_template::handle_mcp_request;
use serde_json::Value;
use std::path::Path;

fn request(input: &str) -> Value {
    serde_json::from_str(&handle_mcp_request(Path::new("/does/not/need/to/exist"), input).unwrap())
        .unwrap()
}

#[test]
fn initialize_returns_mcp_capabilities() {
    let response = request(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#);
    assert_eq!(
        response["result"]["capabilities"]["tools"],
        serde_json::json!({})
    );
    assert_eq!(response["result"]["protocolVersion"], "2024-11-05");
}

#[test]
fn tools_list_exposes_create_idea_schema() {
    let response = request(r#"{"jsonrpc":"2.0","id":"list","method":"tools/list"}"#);
    let tool = &response["result"]["tools"][0];
    assert_eq!(tool["name"], "create_idea");
    for field in [
        "title",
        "problem",
        "desired_outcome",
        "scope",
        "non_goals",
        "constraints",
        "open_questions",
    ] {
        assert!(tool["inputSchema"]["required"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == field));
    }
}

#[test]
fn tool_uses_shared_validation_for_empty_content() {
    let response = request(
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"create_idea","arguments":{"title":" ","problem":"problem","desired_outcome":"outcome","scope":"scope","non_goals":"none","constraints":"none","open_questions":"none"}}}"#,
    );
    assert_eq!(response["result"]["isError"], true);
    let error: Value =
        serde_json::from_str(response["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(error["error"], "missing_content");
}

#[test]
fn malformed_and_unknown_requests_are_json_rpc_errors() {
    let malformed = request("not json");
    assert_eq!(malformed["error"]["code"], -32700);
    let unknown = request(r#"{"jsonrpc":"2.0","id":3,"method":"unknown"}"#);
    assert_eq!(unknown["error"]["code"], -32601);
}

#[test]
fn initialized_notification_has_no_response() {
    assert!(handle_mcp_request(
        Path::new("/does/not/need/to/exist"),
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    )
    .is_none());
}
