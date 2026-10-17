pub struct StageManager {
    stages: Vec<std::path::PathBuf>,
}

impl StageManager {
    pub fn new(stages: Vec<std::path::PathBuf>) -> Self {
        Self { stages }
    }

    pub fn stages(&self) -> &[std::path::PathBuf] {
        &self.stages
    }
}
