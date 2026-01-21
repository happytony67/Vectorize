#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::command;
use vectorize_engine::{run_pipeline, EngineInput, EngineSettings};

#[derive(Debug, Serialize, Deserialize)]
struct VectorizeRequest {
    image_bytes: Vec<u8>,
    settings: EngineSettings,
}

#[derive(Debug, Serialize, Deserialize)]
struct VectorizeResponse {
    svg: String,
    width: u32,
    height: u32,
    layers: usize,
    nodes: usize,
    elapsed_ms: u128,
}

#[command]
fn vectorize(request: VectorizeRequest) -> Result<VectorizeResponse, String> {
    let output = run_pipeline(EngineInput {
        image_bytes: request.image_bytes,
        settings: request.settings,
    })?;
    Ok(VectorizeResponse {
        svg: output.svg,
        width: output.width,
        height: output.height,
        layers: output.layers,
        nodes: output.nodes,
        elapsed_ms: output.elapsed_ms,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![vectorize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
