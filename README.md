# Vectorize

A professional, lightweight, offline-first Image → Vector desktop app built with Tauri + React and a Rust vectorization engine.

## Architecture Overview

### Components
- **Desktop UI (Tauri + React)**: minimal, fast controls and preview canvas.
- **Vectorization Engine (Rust crate)**: deterministic pipeline for preprocessing, segmentation, and SVG generation.
- **Shared Settings**: JSON schema + presets used by UI and engine.

### Data Flow
1. UI imports raster/PDF, decodes to bytes.
2. Tauri command calls the engine pipeline with settings.
3. Engine returns SVG + metadata (layers, nodes, duration).
4. UI renders side-by-side preview and export controls.

### Module Map
- `engine/src/pipeline.rs`: orchestrates preprocessing, quantization, tracing, and SVG export.
- `engine/src/svg.rs`: Illustrator-friendly SVG structure.
- `engine/src/settings.rs`: settings model aligned with `config/settings.schema.json`.
- `src/App.tsx`: minimal UI layout and preview.

## Implementation Plan

### Step A: MVP (current)
- Import raster, run monochrome/color placeholder tracing, export SVG.
- Preview SVG in app.

### Step B: Color tracing (current)
- Palette quantization and multi-layer tracing with clean grouping.

### Step C: Auto mode classifier
- Heuristic detection of logo/icon/illustration/photo.
- Auto applies presets.

### Step D: Batch + PDF import
- PDF page selection and resolution.
- Batch export to folder.

### Step E: Optimization
- Multithreading for heavy stages.
- Caching and progress callbacks.

## Build Instructions

### Web Preview
```
npm install
npm run dev
```

The web build runs the lightweight TypeScript tracing path for offline use when the Tauri runtime is not available.

### Desktop App
```
npm install
npm run dev
# in another terminal
npm run tauri dev
```

### Engine Tests
```
cargo test -p vectorize_engine
```

## Output Quality Goals
- Minimal nodes with preserved corners.
- Clean Illustrator-friendly layers and paths.
- Deterministic output per input + settings.

## Third Party
See `THIRD_PARTY_NOTICES.md`.
