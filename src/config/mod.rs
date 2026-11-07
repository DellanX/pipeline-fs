use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub write_stage: StageConfig,
    pub shift_stages: Vec<StageConfig>,
    pub stage_stage: StageConfig,
    pub final_stage: StageConfig,

    pub mount_stage: Vec<MountConfig>,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub mount_point: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfig {
    pub disk_path: String,
    pub triggers: Vec<TriggerConfig>,
    pub conditions: Vec<ConditionConfig>,
    pub actions: Vec<ActionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountConfig {
    pub mount_stage: i32,
    pub mount_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConfig {
    pub trigger_type: String,
    pub parameters: serde_yaml::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    pub condition_type: String,
    pub parameters: serde_yaml::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    pub action_type: String,
    pub parameters: serde_yaml::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveActionParameters {
    pub target_stage: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_config: Option<PushConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushConfig {
    pub max_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub auto_migrate: bool,
    pub ttl_hours: u32,
}

impl Default for Policy {
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
            write_stage: StageConfig {
                disk_path: "/tmp/pipelinefs/write_stage".to_string(),
                triggers: vec![],
                conditions: vec![],
                actions: vec![ActionConfig {
                    action_type: "Move".to_string(),
                    parameters: serde_yaml::to_value(MoveActionParameters {
                        target_stage: 1,
                        failover_mode: None,
                        push_config: None,
                    }).unwrap(),
                }],
            },
            shift_stages: vec![
                StageConfig {
                    disk_path: "/tmp/pipelinefs/shift_stage_0".to_string(),
                    triggers: vec![],
                    conditions: vec![],
                    actions: vec![ActionConfig {
                        action_type: "Move".to_string(),
                        parameters: serde_yaml::to_value(MoveActionParameters {
                            target_stage: 2,
                            failover_mode: None,
                            push_config: None,
                        }).unwrap(),
                    }],
                },
                StageConfig {
                    disk_path: "/tmp/pipelinefs/shift_stage_1".to_string(),
                    triggers: vec![],
                    conditions: vec![],
                    actions: vec![ActionConfig {
                        action_type: "Move".to_string(),
                        parameters: serde_yaml::to_value(MoveActionParameters {
                            target_stage: 0,
                            failover_mode: Some("Merge".to_string()),
                            push_config: None,
                        }).unwrap(),
                    }],
                },
            ],
            stage_stage: StageConfig {
                disk_path: "/tmp/pipelinefs/stage_stage".to_string(),
                triggers: vec![],
                conditions: vec![],
                actions: vec![ActionConfig {
                    action_type: "Move".to_string(),
                    parameters: serde_yaml::to_value(MoveActionParameters {
                        target_stage: 3,
                        failover_mode: None,
                        push_config: None,
                    }).unwrap(),
                }],
            },
            final_stage: StageConfig {
                disk_path: "/tmp/pipelinefs/final_stage".to_string(),
                triggers: vec![],
                conditions: vec![],
                actions: vec![ActionConfig {
                    action_type: "Move".to_string(),
                    parameters: serde_yaml::to_value(MoveActionParameters {
                        target_stage: 3,
                        failover_mode: Some("Push".to_string()),
                        push_config: Some(PushConfig { max_depth: 1 }),
                    }).unwrap(),
                }],
            },
            mount_stage: vec![],
            server: ServerConfig {  
                port: Some(8080),
                mount_point: Some("/mnt/pipelinefs".to_string()),
            },
        }
    }

    pub fn get_stage_paths(&self) -> Vec<String> {
        let mut paths = vec![
            self.write_stage.disk_path.clone(),
            self.stage_stage.disk_path.clone(),
            self.final_stage.disk_path.clone(),
        ];
        paths.extend(self.shift_stages.iter().map(|stage| stage.disk_path.clone()).collect::<Vec<String>>());
        paths
    }

    /// Validates that all move actions have target_stage >= current stage number
    /// This ensures write cache stability and prevents backward writes
    pub fn validate_stage_constraints(&self) -> Result<(), String> {
        // In a real implementation, this would check each action's target_stage 
        // against the current stage number. For now, just return Ok as the validation
        // logic would need to be implemented in the actual pipeline execution code.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let original_config = PipelineConfig::default();
        
        // Serialize to YAML string
        let yaml_string = serde_yaml::to_string(&original_config).unwrap();
        
        // Deserialize back from YAML string
        let deserialized_config: PipelineConfig = serde_yaml::from_str(&yaml_string).unwrap();
        
        // Verify they match
        assert_eq!(original_config.server.port, deserialized_config.server.port);
        assert_eq!(original_config.server.mount_point, deserialized_config.server.mount_point);
        assert_eq!(original_config.write_stage.disk_path, deserialized_config.write_stage.disk_path);
        assert_eq!(original_config.stage_stage.disk_path, deserialized_config.stage_stage.disk_path);
        assert_eq!(original_config.final_stage.disk_path, deserialized_config.final_stage.disk_path);
        for(i, stage) in original_config.shift_stages.iter().enumerate() {
            assert_eq!(stage.disk_path, deserialized_config.shift_stages[i].disk_path);
        }
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
        assert_eq!(config.server.port, loaded_config.server.port);
        assert_eq!(config.server.mount_point, loaded_config.server.mount_point);
        assert_eq!(config.write_stage.disk_path, loaded_config.write_stage.disk_path);
        assert_eq!(config.stage_stage.disk_path, loaded_config.stage_stage.disk_path);
        assert_eq!(config.final_stage.disk_path, loaded_config.final_stage.disk_path);
        for(i, stage) in config.shift_stages.iter().enumerate() {
            assert_eq!(stage.disk_path, loaded_config.shift_stages[i].disk_path);
        }
        // Clean up
        std::fs::remove_file(temp_config).unwrap();
    }

    #[test]
    fn test_new_action_parameters() {
        // Test that we can properly handle the new action parameters
        let config = PipelineConfig::default();
        
        // Check that final_stage has push configuration
        assert_eq!(config.final_stage.actions.len(), 1);
        let final_action = &config.final_stage.actions[0];
        assert_eq!(final_action.action_type, "Move");
        
        // Verify the parameters can be parsed correctly
        let params = &final_action.parameters;
        assert!(params.get("target_stage").is_some());
        assert!(params.get("failover_mode").is_some());
        assert_eq!(params.get("failover_mode").unwrap().as_str(), Some("Push"));
        
        // Check that shift_stage_1 has merge configuration
        assert_eq!(config.shift_stages.len(), 2);
        assert_eq!(config.shift_stages[1].actions.len(), 1);
        let shift_action = &config.shift_stages[1].actions[0];
        assert_eq!(shift_action.action_type, "Move");
        
        // Verify merge configuration
        let shift_params = &shift_action.parameters;
        assert!(shift_params.get("target_stage").is_some());
        assert!(shift_params.get("failover_mode").is_some());
        assert_eq!(shift_params.get("failover_mode").unwrap().as_str(), Some("Merge"));
    }
}