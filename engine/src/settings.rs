use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OutputFormat {
    Svg,
    Pdf,
    Eps,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColorQuantization {
    MedianCut,
    KMeans,
    NeuQuant,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AutoMode {
    Auto,
    Logo,
    Icon,
    Illustration,
    Sketch,
    PhotoEdge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSettings {
    pub auto_mode: AutoMode,
    pub palette_size: u8,
    pub quantization: ColorQuantization,
    pub smoothing: f32,
    pub edge_sensitivity: f32,
    pub min_area: u32,
    pub corner_threshold: f32,
    pub simplify_tolerance: f32,
    pub merge_tolerance: f32,
    pub keep_holes: bool,
    pub remove_background: bool,
    pub background_threshold: f32,
    pub illustrator_friendly: bool,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            auto_mode: AutoMode::Auto,
            palette_size: 8,
            quantization: ColorQuantization::MedianCut,
            smoothing: 0.35,
            edge_sensitivity: 0.65,
            min_area: 16,
            corner_threshold: 0.22,
            simplify_tolerance: 0.8,
            merge_tolerance: 0.15,
            keep_holes: true,
            remove_background: true,
            background_threshold: 0.06,
            illustrator_friendly: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub settings: EngineSettings,
}
