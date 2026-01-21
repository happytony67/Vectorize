use crate::settings::EngineSettings;
use crate::svg::{SvgDocument, SvgPath};
use image::{DynamicImage, GenericImageView, Rgba};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineInput {
    pub image_bytes: Vec<u8>,
    pub settings: EngineSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineOutput {
    pub svg: String,
    pub width: u32,
    pub height: u32,
    pub layers: usize,
    pub nodes: usize,
    pub elapsed_ms: u128,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EngineStage {
    Decode,
    Preprocess,
    Quantize,
    Trace,
    PostProcess,
    Export,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub stage: EngineStage,
    pub progress: f32,
}

pub fn run_pipeline(input: EngineInput) -> Result<EngineOutput, String> {
    let start = Instant::now();
    let image = image::load_from_memory(&input.image_bytes)
        .map_err(|err| format!("Decode error: {err}"))?;
    let (width, height) = image.dimensions();
    let simplified = simplify_image(&image, &input.settings);
    let palette = build_palette(&simplified, &input.settings);
    let layers = trace_layers(&simplified, &palette, &input.settings);

    let mut doc = SvgDocument::new(width, height, input.settings.illustrator_friendly);
    for layer in &layers {
        doc.add_path(layer.clone());
    }

    let svg = doc.render();
    let nodes = layers.iter().map(|layer| layer.node_count()).sum();

    Ok(EngineOutput {
        svg,
        width,
        height,
        layers: layers.len(),
        nodes,
        elapsed_ms: start.elapsed().as_millis(),
    })
}

fn simplify_image(image: &DynamicImage, settings: &EngineSettings) -> DynamicImage {
    let mut rgba = image.to_rgba8();
    if settings.remove_background {
        let bg = rgba.get_pixel(0, 0).0;
        for pixel in rgba.pixels_mut() {
            let delta = color_distance(pixel.0, bg);
            if delta < settings.background_threshold * 255.0 {
                *pixel = Rgba([pixel[0], pixel[1], pixel[2], 0]);
            }
        }
    }
    DynamicImage::ImageRgba8(rgba)
}

fn build_palette(image: &DynamicImage, settings: &EngineSettings) -> Vec<[u8; 4]> {
    let mut buckets = vec![[0u8; 4]; settings.palette_size as usize];
    let mut counts = vec![0u32; settings.palette_size as usize];
    for (index, pixel) in image.to_rgba8().pixels().enumerate() {
        let bucket = index % buckets.len().max(1);
        let entry = &mut buckets[bucket];
        entry[0] = entry[0].saturating_add(pixel[0] / 2);
        entry[1] = entry[1].saturating_add(pixel[1] / 2);
        entry[2] = entry[2].saturating_add(pixel[2] / 2);
        entry[3] = 255;
        counts[bucket] = counts[bucket].saturating_add(1);
    }
    for (idx, entry) in buckets.iter_mut().enumerate() {
        let count = counts[idx].max(1) as u8;
        entry[0] = entry[0].saturating_mul(2) / count;
        entry[1] = entry[1].saturating_mul(2) / count;
        entry[2] = entry[2].saturating_mul(2) / count;
    }
    buckets
}

fn trace_layers(
    image: &DynamicImage,
    palette: &[[u8; 4]],
    settings: &EngineSettings,
) -> Vec<SvgPath> {
    let mut layers = Vec::new();
    for (index, color) in palette.iter().enumerate() {
        let bounds = layer_bounds(image, *color, index as u32);
        if bounds.is_none() {
            continue;
        }
        let (x, y, w, h) = bounds.unwrap();
        if w * h < settings.min_area {
            continue;
        }
        layers.push(SvgPath::rect(x, y, w, h, *color));
    }
    if layers.is_empty() {
        layers.push(SvgPath::rect(0, 0, image.width(), image.height(), [0, 0, 0, 255]));
    }
    layers
}

fn layer_bounds(image: &DynamicImage, color: [u8; 4], seed: u32) -> Option<(u32, u32, u32, u32)> {
    let rgba = image.to_rgba8();
    let mut min_x = image.width();
    let mut min_y = image.height();
    let mut max_x = 0u32;
    let mut max_y = 0u32;
    let mut found = false;
    for (x, y, pixel) in rgba.enumerate_pixels() {
        if matches_color(pixel.0, color, seed) {
            found = true;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }
    if !found {
        return None;
    }
    Some((min_x, min_y, (max_x - min_x).max(1), (max_y - min_y).max(1)))
}

fn matches_color(pixel: [u8; 4], color: [u8; 4], seed: u32) -> bool {
    let dist = color_distance(pixel, color);
    dist < 42.0 + (seed % 5) as f32
}

fn color_distance(a: [u8; 4], b: [u8; 4]) -> f32 {
    let dr = a[0] as f32 - b[0] as f32;
    let dg = a[1] as f32 - b[1] as f32;
    let db = a[2] as f32 - b[2] as f32;
    (dr * dr + dg * dg + db * db).sqrt()
}
