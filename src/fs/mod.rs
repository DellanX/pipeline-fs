pub struct PipelineFs {
    stage_manager: crate::stage::StageManager,
}

impl PipelineFs {
    pub fn new(stage_manager: crate::stage::StageManager) -> Self {
        Self { stage_manager }
    }
}
