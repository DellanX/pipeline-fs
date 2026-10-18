use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub name: String,
    pub value: i32,
}

fn main() {
    let original = TestConfig {
        name: "test".to_string(),
        value: 42,
    };
    
    // Serialize to YAML
    let yaml_string = serde_yaml::to_string(&original).unwrap();
    println!("Serialized YAML:\n{}", yaml_string);
    
    // Deserialize back
    let deserialized: TestConfig = serde_yaml::from_str(&yaml_string).unwrap();
    
    // Verify they match
    assert_eq!(original.name, deserialized.name);
    assert_eq!(original.value, deserialized.value);
    
    println!("YAML serialization test passed!");
}
