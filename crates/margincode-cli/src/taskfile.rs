pub use margincode_core::taskfile::*;

use serde_json::Value;

use crate::client::Client;

/// The owner a file the CLI writes names, with its profile's other names, as the daemon's
/// `owner_of` writes them. Names nobody when the profiles cannot be read: an owner named
/// without its other names could be fenced.
pub fn owned_by(client: &Client, task: &Value) -> (Option<String>, Vec<String>) {
    let Some(owner) = task["owner_agent"]
        .as_str()
        .filter(|o| !o.trim().is_empty())
    else {
        return (None, Vec::new());
    };
    let Some(listed) = client.get("/v1/agents", &[]).ok() else {
        return (None, Vec::new());
    };
    let Some(profiles) = listed["agents"].as_array() else {
        return (None, Vec::new());
    };
    let profiles: Vec<(&str, Vec<String>)> = profiles
        .iter()
        .filter_map(|p| {
            let aliases = p["aliases"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect();
            Some((p["name"].as_str()?, aliases))
        })
        .collect();
    let aliases = other_names(
        owner,
        profiles.iter().map(|(name, a)| (*name, a.as_slice())),
    );
    (Some(owner.to_owned()), aliases)
}
