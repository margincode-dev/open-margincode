//! `.av-task`: the definition is `margincode_core::taskfile`, shared with the daemon (F14).

pub use margincode_core::taskfile::*;

use serde_json::Value;

use crate::client::Client;

/// The owner a file the CLI writes names (`task`'s `owner_agent`, the daemon's name for it: a
/// profile's name an alias resolved to, or an alias the task was started or claimed under), with
/// the other names its profile goes by (`other_names`), as the daemon's `owner_of` writes them
/// (T1a): a process acting under any of the owner's names holds the claim, and one acting under
/// another name does not. Names nobody when the profiles cannot be read: an owner named without
/// its other names could be fenced.
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
