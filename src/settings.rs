use anyhow::Result;

use serde::Deserialize;
use std::collections::HashMap;

use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct ClusterContextResource {
    group: String,
    version: String,
    kind: String,
    namespace: Option<String>,
    // add account token?
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ClusterContext {
    resources: Vec<ClusterContextResource>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Policy {
    pub url: String,

    #[serde(default)]
    pub cluster_context: ClusterContext,

    #[serde(skip)]
    pub wasm_module_path: PathBuf,

    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
}

// Reads the policies configuration file, returns a HashMap with String as value
// and Policy as values. The key is the name of the policy as provided by the user
// inside of the configuration file. This name is used to build the API path
// exposing the policy.
pub fn read_policies_file(path: &Path) -> Result<HashMap<String, Policy>> {
    let settings_file = File::open(path)?;
    let ps: HashMap<String, Policy> = serde_yaml::from_reader(&settings_file)?;
    Ok(ps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_settings_when_no_settings_are_provided() {
        let input = r#"
---
example:
  url: file:///tmp/namespace-validate-policy.wasm
"#;

        let policies: HashMap<String, Policy> = serde_yaml::from_str(&input).unwrap();
        assert!(!policies.is_empty());

        let policy = policies.get("example").unwrap();
        assert!(policy.settings.is_empty());
    }
}
