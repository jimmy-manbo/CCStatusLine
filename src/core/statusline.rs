use crate::config::{Config, InputData};
use crate::core::segments::{
    DirectorySegment, GitSegment, ModelSegment, Segment, UpdateSegment, UsageSegment,
};

pub struct StatusLineGenerator {
    config: Config,
}

impl StatusLineGenerator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate(&self, input: &InputData) -> String {
        let mut segments = Vec::new();

        // Assemble segments with proper colors
        if self.config.segments.model {
            let model_segment = ModelSegment::new(true);
            let content = model_segment.render(input);
            segments.push(format!("🧠 {}", content));
        }

        if self.config.segments.directory {
            let dir_segment = DirectorySegment::new(true);
            let content = dir_segment.render(input);
            // Extract directory name without icon
            let dir_name = content.trim_start_matches('📂').trim_start();
            segments.push(format!("📁 {}", dir_name));
        }

        if self.config.segments.git {
            let git_segment = GitSegment::new(true);
            let git_output = git_segment.render(input);
            if !git_output.is_empty() {
                segments.push(format!("{}", git_output));
            }
        }

        if self.config.segments.usage {
            let usage_segment = UsageSegment::new(true);
            let content = usage_segment.render(input);
            segments.push(format!("{}", content));
        }

        // Add update segment (always enabled when there's an update)
        let update_segment = UpdateSegment::new();
        if update_segment.enabled() {
            let content = update_segment.render(input);
            segments.push(format!("🔃 {}", content));
        }

        // Join segments with white separator
        segments.join(" | ")
    }
}
