mod pipeline;
mod settings;
mod svg;

pub use pipeline::{run_pipeline, EngineInput, EngineOutput, EngineStage, ProgressEvent};
pub use settings::{AutoMode, ColorQuantization, EngineSettings, OutputFormat, Preset};

#[cfg(test)]
mod tests;
