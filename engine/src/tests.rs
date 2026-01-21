use super::*;

#[test]
fn deterministic_svg_output() {
    let bytes = sample_image_bytes();
    let settings = EngineSettings::default();
    let first = run_pipeline(EngineInput { image_bytes: bytes.clone(), settings: settings.clone() })
        .expect("pipeline");
    let second = run_pipeline(EngineInput { image_bytes: bytes, settings })
        .expect("pipeline");
    assert_eq!(first.svg, second.svg);
}

#[test]
fn preserves_holes_setting_roundtrip() {
    let settings = EngineSettings::default();
    let serialized = serde_json::to_string(&settings).expect("serialize");
    let decoded: EngineSettings = serde_json::from_str(&serialized).expect("deserialize");
    assert_eq!(decoded.keep_holes, settings.keep_holes);
}

#[test]
fn svg_contains_viewbox() {
    let bytes = sample_image_bytes();
    let settings = EngineSettings::default();
    let output = run_pipeline(EngineInput { image_bytes: bytes, settings })
        .expect("pipeline");
    assert!(output.svg.contains("viewBox=\"0 0"));
}

fn sample_image_bytes() -> Vec<u8> {
    let mut image = image::RgbaImage::new(4, 4);
    for y in 0..4 {
        for x in 0..4 {
            let color = if (x + y) % 2 == 0 {
                image::Rgba([20, 40, 200, 255])
            } else {
                image::Rgba([220, 40, 60, 255])
            };
            image.put_pixel(x, y, color);
        }
    }
    let mut bytes = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut bytes);
    encoder
        .encode(
            &image,
            image.width(),
            image.height(),
            image::ColorType::Rgba8,
        )
        .expect("encode");
    bytes
}
