use crate::types::WorkspaceLayoutSnapshot;

pub fn serialize_workspace_layout_snapshot(
    snapshot: &WorkspaceLayoutSnapshot,
) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(snapshot)
}

pub fn parse_workspace_layout_snapshot(
    serialized: &str,
) -> Result<WorkspaceLayoutSnapshot, serde_json::Error> {
    serde_json::from_str(serialized)
}
