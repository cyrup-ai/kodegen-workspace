use serde_json;

fn main() {
    // Test what gets serialized
    let display = "Hello terminal output\nLine 2\nLine 3";
    let metadata = serde_json::json!({"exit_code": 0, "terminal": 0});

    // Simulate Content::text()
    let raw_content = rmcp::model::RawContent::Text(rmcp::model::RawTextContent {
        text: display.to_string(),
        meta: None,
    });
    let display_content = rmcp::model::Annotated::new(raw_content, None);

    let call_result = rmcp::model::CallToolResult {
        content: vec![display_content],
        structured_content: Some(metadata),
        is_error: None,
        meta: None,
    };

    println!("=== CALL_TOOL_RESULT SERIALIZATION TEST ===\n");
    let serialized = serde_json::to_string_pretty(&call_result).unwrap();
    println!("{}\n", serialized);

    println!("=== CHECKING FIELDS ===");
    println!("content.len() = {}", call_result.content.len());
    if let Some(first) = call_result.content.first() {
        if let Some(text) = first.as_text() {
            println!("content[0] text = {:?}", &text.text[..text.text.len().min(50)]);
        }
    }
    println!("structured_content.is_some() = {}", call_result.structured_content.is_some());
}
