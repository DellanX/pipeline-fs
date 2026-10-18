use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfig {
    pub source: String,
    pub stage_0: String,
    pub stages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub stages: StageConfig,
    pub policies: Policies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policies {
    pub auto_migrate: bool,
    pub ttl_hours: u32,
}

impl Default for Policies {
    fn default() -> Self {
        Self {
            auto_migrate: true,
            ttl_hours: 168, // 1 week
        }
    }
}

impl PipelineConfig {
    pub fn load_or_create(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if Path::new(config_path).exists() {
            let contents = fs::read_to_string(config_path)?;
            let config: PipelineConfig = serde_yaml::from_str(&contents)?;
            Ok(config)
        } else {
            // Create default config
            let default_config = Self::default();
            let serialized = serde_yaml::to_string(&default_config)?;
            fs::write(config_path, serialized)?;
            Ok(default_config)
        }
    }

    pub fn save(&self, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_yaml::to_string(self)?;
        fs::write(config_path, serialized)?;
        Ok(())
    }

    pub fn default() -> Self {
        Self {
            stages: StageConfig {
                source: "/tmp/pipeline-fs/source".to_string(),
                stage_0: "/tmp/pipeline-fs/0".to_string(),
                stages: vec![
                    "/tmp/pipeline-fs/1".to_string(),
                    "/tmp/pipeline-fs/2".to_string(),
                ],
            },
            policies: Policies::default(),
        }
    }

    pub fn get_stage_paths(&self) -> Vec<String> {
        let mut paths = vec![self.stages.source.clone(), self.stages.stage_0.clone()];
        paths.extend(self.stages.stages.clone());
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_config_serialization() {
        let original_config = PipelineConfig::default();
        
        // Serialize to YAML string
        let yaml_string = serde_yaml::to_string(&original_config).unwrap();
        
        // Deserialize back from YAML string
        let deserialized_config: PipelineConfig = serde_yaml::from_str(&yaml_string).unwrap();
        
        // Verify they match
        assert_eq!(original_config.stages.source, deserialized_config.stages.source);
        assert_eq!(original_config.stages.stage_0, deserialized_config.stages.stage_0);
        assert_eq!(original_config.stages.stages, deserialized_config.stages.stages);
        assert_eq!(original_config.policies.auto_migrate, deserialized_config.policies.auto_migrate);
        assert_eq!(original_config.policies.ttl_hours, deserialized_config.policies.ttl_hours);
    }

    #[test]
    fn test_config_file_load_save() {
        // Create a temporary config file
        let temp_config = "temp_test_config.yaml";
        
        // Load or create default config
        let config = PipelineConfig::load_or_create(temp_config).unwrap();
        
        // Save it back to disk
        config.save(temp_config).unwrap();
        
        // Load it again
        let loaded_config = PipelineConfig::load_or_create(temp_config).unwrap();
        
        // Verify they match
        assert_eq!(config.stages.source, loaded_config.stages.source);
        assert_eq!(config.stages.stage_0, loaded_config.stages.stage_0);
        assert_eq!(config.stages.stages, loaded_config.stages.stages);
        assert_eq!(config.policies.auto_migrate, loaded_config.policies.auto_migrate);
        assert_eq!(config.policies.ttl_hours, loaded_config.policies.ttl_hours);
        
        // Clean up
        std::fs::remove_file(temp_config).unwrap();
    }
}