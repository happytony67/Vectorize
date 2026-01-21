Benchmarks (placeholder)

- Target hardware: M2 Pro / 16GB RAM
- Sample: checker.ppm 4x4
- Expected: < 5ms vectorization, negligible memory

Run plan (future):
1. Use sample logos at 1024x1024, 4096x4096, 8192x8192.
2. Capture wall time and RSS.
3. Record outputs in benchmarks/results.json.
