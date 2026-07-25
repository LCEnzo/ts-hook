use chrono::{Local, SecondsFormat};
use serde_json::json;

/// UserPromptSubmit hook: inject the wall-clock time into context.
/// Portable replacement for `jq -n --arg ts "$(date -Iseconds)" ...`.
fn main() {
    let ts = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
    let out = json!({
        "hookSpecificOutput": {
            "hookEventName": "UserPromptSubmit",
            "additionalContext": format!("[ts: {ts}]")
        }
    });
    println!("{out}");
}
