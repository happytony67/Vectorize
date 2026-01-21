import { useMemo, useState } from "react";
import { vectorizeWeb } from "./vectorizeWeb";

const presets = [
  "Auto",
  "Logo / Flat colors",
  "Icon / UI",
  "Illustration",
  "Sketch/Scan",
  "Photo (edge-focused)"
];

const defaultSettings = {
  auto_mode: "Auto",
  palette_size: 8,
  quantization: "MedianCut",
  smoothing: 0.35,
  edge_sensitivity: 0.65,
  min_area: 16,
  corner_threshold: 0.22,
  simplify_tolerance: 0.8,
  merge_tolerance: 0.15,
  keep_holes: true,
  remove_background: true,
  background_threshold: 0.06,
  illustrator_friendly: true
};

export default function App() {
  const [preset, setPreset] = useState(presets[0]);
  const [status, setStatus] = useState("Ready");
  const [svg, setSvg] = useState<string | null>(null);
  const [info, setInfo] = useState({ layers: 0, nodes: 0, elapsed: 0 });
  const [originalUrl, setOriginalUrl] = useState<string | null>(null);

  const summary = useMemo(() => {
    return `Layers ${info.layers} · Nodes ${info.nodes} · ${info.elapsed}ms`;
  }, [info]);

  const onFile = async (file: File) => {
    setOriginalUrl(URL.createObjectURL(file));
    const buffer = await file.arrayBuffer();
    setStatus("Vectorizing");
    try {
      let response: { svg: string; layers: number; nodes: number; elapsed_ms: number };
      if (isTauriEnvironment()) {
        const { invoke } = await import("@tauri-apps/api/core");
        response = (await invoke("vectorize", {
          request: {
            image_bytes: Array.from(new Uint8Array(buffer)),
            settings: defaultSettings
          }
        })) as {
          svg: string;
          layers: number;
          nodes: number;
          elapsed_ms: number;
        };
      } else {
        response = await vectorizeWeb(file, defaultSettings);
      }
      setSvg(response.svg);
      setInfo({
        layers: response.layers,
        nodes: response.nodes,
        elapsed: response.elapsed_ms
      });
      setStatus("Ready");
    } catch (error) {
      setStatus("Failed to vectorize");
      console.error(error);
    }
  };

  return (
    <div className="app">
      <header className="top-bar">
        <div className="brand">Vectorize</div>
        <div className="actions">
          <label className="button">
            Import
            <input
              type="file"
              accept="image/*,.pdf"
              onChange={(event) => {
                const file = event.target.files?.[0];
                if (file) {
                  void onFile(file);
                }
              }}
              hidden
            />
          </label>
          <select value={preset} onChange={(event) => setPreset(event.target.value)}>
            {presets.map((item) => (
              <option key={item} value={item}>
                {item}
              </option>
            ))}
          </select>
          <button className="button">Export</button>
        </div>
      </header>
      <main className="content">
        <aside className="panel">
          <h2>Controls</h2>
          <section>
            <h3>Colors</h3>
            <div className="row">
              <label>Palette size</label>
              <input type="range" min={2} max={64} defaultValue={8} />
            </div>
          </section>
          <section>
            <h3>Detail</h3>
            <div className="row">
              <label>Edge sensitivity</label>
              <input type="range" min={0} max={1} step={0.01} defaultValue={0.65} />
            </div>
          </section>
          <section>
            <h3>Layers</h3>
            <div className="row">
              <label>Merge adjacent regions</label>
              <input type="checkbox" defaultChecked />
            </div>
          </section>
          <section>
            <h3>Background</h3>
            <div className="row">
              <label>Remove background</label>
              <input type="checkbox" defaultChecked />
            </div>
          </section>
          <section>
            <h3>Output</h3>
            <div className="row">
              <label>Illustrator friendly</label>
              <input type="checkbox" defaultChecked />
            </div>
          </section>
        </aside>
        <section className="canvas">
          <div className="preview">
            <div className="preview-pane">
              <div className="preview-title">Original</div>
              <div className="preview-body">
                {originalUrl ? (
                  <img className="preview-image" src={originalUrl} alt="Original upload" />
                ) : (
                  "Drop an image to start"
                )}
              </div>
            </div>
            <div className="preview-pane">
              <div className="preview-title">Vector</div>
              <div
                className="preview-body vector-output"
                dangerouslySetInnerHTML={{ __html: svg ?? "" }}
              />
            </div>
          </div>
        </section>
      </main>
      <footer className="status">
        <span>{status}</span>
        <span>{summary}</span>
      </footer>
    </div>
  );
}

function isTauriEnvironment() {
  const windowAny = window as typeof window & {
    __TAURI__?: unknown;
    __TAURI_INTERNALS__?: unknown;
  };
  return Boolean(windowAny.__TAURI__ || windowAny.__TAURI_INTERNALS__);
}
