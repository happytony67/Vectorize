# Vectorize Pipeline

```
Raster Input
   │
   ▼
Decode → Preprocess → Quantize → Trace → Post-process → SVG/PDF/EPS
   │         │            │        │           │
   │         │            │        │           └─ Simplify paths, merge shapes, fix holes
   │         │            │        └─ Contour extraction, curve fitting
   │         │            └─ Palette reduction, color merging
   │         └─ Denoise, deblur, background removal
   └─ Format detection, PDF rasterization
```

Key goals
- Deterministic outputs
- Illustrator-safe SVG structure
- Minimal nodes with preserved corners
