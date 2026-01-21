type Settings = {
  edge_sensitivity: number;
  background_threshold: number;
  illustrator_friendly: boolean;
};

type VectorizeResult = {
  svg: string;
  layers: number;
  nodes: number;
  elapsed_ms: number;
};

type Point = { x: number; y: number };

type Segment = { start: Point; end: Point };

export async function vectorizeWeb(file: File, settings: Settings): Promise<VectorizeResult> {
  const start = performance.now();
  const bitmap = await createImageBitmap(file);
  const canvas = document.createElement("canvas");
  canvas.width = bitmap.width;
  canvas.height = bitmap.height;
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("Canvas not supported");
  }
  ctx.drawImage(bitmap, 0, 0);
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const threshold = 0.6 - settings.edge_sensitivity * 0.3;
  const mask = buildMask(imageData, threshold, settings.background_threshold);
  const segments = marchingSquares(mask, canvas.width, canvas.height);
  const paths = segmentsToPaths(segments);
  const d = paths.length ? paths.map(pathToSvg).join(" ") : "";
  const svg = renderSvg(canvas.width, canvas.height, d, settings.illustrator_friendly);
  const nodes = d ? d.split(/\s+/).length : 0;
  return {
    svg,
    layers: 1,
    nodes,
    elapsed_ms: Math.round(performance.now() - start)
  };
}

function buildMask(
  imageData: ImageData,
  threshold: number,
  backgroundThreshold: number
): Uint8Array {
  const { data, width, height } = imageData;
  const mask = new Uint8Array(width * height);
  const bg = [data[0], data[1], data[2]];
  for (let i = 0; i < width * height; i += 1) {
    const idx = i * 4;
    const alpha = data[idx + 3] / 255;
    if (alpha < 0.1) {
      mask[i] = 0;
      continue;
    }
    const r = data[idx];
    const g = data[idx + 1];
    const b = data[idx + 2];
    const lum = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255;
    const bgDelta = Math.abs(r - bg[0]) + Math.abs(g - bg[1]) + Math.abs(b - bg[2]);
    if (bgDelta / 765 < backgroundThreshold) {
      mask[i] = 0;
      continue;
    }
    mask[i] = lum < threshold ? 1 : 0;
  }
  return mask;
}

function marchingSquares(mask: Uint8Array, width: number, height: number): Segment[] {
  const segments: Segment[] = [];
  const lookup: Record<number, [Point, Point][]> = {
    1: [[{ x: 0, y: 0.5 }, { x: 0.5, y: 0 }]],
    2: [[{ x: 0.5, y: 0 }, { x: 1, y: 0.5 }]],
    3: [[{ x: 0, y: 0.5 }, { x: 1, y: 0.5 }]],
    4: [[{ x: 1, y: 0.5 }, { x: 0.5, y: 1 }]],
    5: [
      [{ x: 0, y: 0.5 }, { x: 0.5, y: 0 }],
      [{ x: 1, y: 0.5 }, { x: 0.5, y: 1 }]
    ],
    6: [[{ x: 0.5, y: 0 }, { x: 0.5, y: 1 }]],
    7: [[{ x: 0, y: 0.5 }, { x: 0.5, y: 1 }]],
    8: [[{ x: 0.5, y: 1 }, { x: 0, y: 0.5 }]],
    9: [[{ x: 0.5, y: 0 }, { x: 0.5, y: 1 }]],
    10: [
      [{ x: 0.5, y: 0 }, { x: 1, y: 0.5 }],
      [{ x: 0.5, y: 1 }, { x: 0, y: 0.5 }]
    ],
    11: [[{ x: 1, y: 0.5 }, { x: 0.5, y: 1 }]],
    12: [[{ x: 1, y: 0.5 }, { x: 0, y: 0.5 }]],
    13: [[{ x: 0.5, y: 0 }, { x: 1, y: 0.5 }]],
    14: [[{ x: 0, y: 0.5 }, { x: 0.5, y: 0 }]]
  };

  for (let y = 0; y < height - 1; y += 1) {
    for (let x = 0; x < width - 1; x += 1) {
      const idx = y * width + x;
      const a = mask[idx];
      const b = mask[idx + 1];
      const c = mask[idx + width + 1];
      const d = mask[idx + width];
      const code = (a << 3) | (b << 2) | (c << 1) | d;
      const edges = lookup[code];
      if (!edges) {
        continue;
      }
      for (const edge of edges) {
        segments.push({
          start: { x: x + edge[0].x, y: y + edge[0].y },
          end: { x: x + edge[1].x, y: y + edge[1].y }
        });
      }
    }
  }

  return segments;
}

function segmentsToPaths(segments: Segment[]): Point[][] {
  const adjacency = new Map<string, Point[]>();
  const points = new Map<string, Point>();

  const addEdge = (start: Point, end: Point) => {
    const startKey = pointKey(start);
    const endKey = pointKey(end);
    points.set(startKey, start);
    points.set(endKey, end);
    const startList = adjacency.get(startKey) ?? [];
    startList.push(end);
    adjacency.set(startKey, startList);
    const endList = adjacency.get(endKey) ?? [];
    endList.push(start);
    adjacency.set(endKey, endList);
  };

  for (const segment of segments) {
    addEdge(segment.start, segment.end);
  }

  const visited = new Set<string>();
  const paths: Point[][] = [];

  for (const key of adjacency.keys()) {
    if (visited.has(key)) {
      continue;
    }
    const path: Point[] = [];
    let currentKey = key;
    let prevKey: string | null = null;

    while (currentKey) {
      visited.add(currentKey);
      const currentPoint = points.get(currentKey);
      if (!currentPoint) {
        break;
      }
      path.push(currentPoint);
      const neighbors = adjacency.get(currentKey) ?? [];
      const next = neighbors.find((neighbor) => pointKey(neighbor) !== prevKey);
      if (!next) {
        break;
      }
      prevKey = currentKey;
      currentKey = pointKey(next);
      if (currentKey === key) {
        path.push(next);
        break;
      }
    }

    if (path.length > 2) {
      paths.push(path);
    }
  }

  return paths;
}

function pathToSvg(path: Point[]): string {
  const [first, ...rest] = path;
  const commands = [`M ${first.x.toFixed(2)} ${first.y.toFixed(2)}`];
  for (const point of rest) {
    commands.push(`L ${point.x.toFixed(2)} ${point.y.toFixed(2)}`);
  }
  commands.push("Z");
  return commands.join(" ");
}

function renderSvg(width: number, height: number, d: string, illustratorFriendly: boolean): string {
  const groupStart = illustratorFriendly ? "<g id=\"vector-layers\">" : "";
  const groupEnd = illustratorFriendly ? "</g>" : "";
  const path = d ? `<path d=\"${d}\" fill=\"#111111\" />` : "";
  return `<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"${width}\" height=\"${height}\" viewBox=\"0 0 ${width} ${height}\">${groupStart}${path}${groupEnd}</svg>`;
}

function pointKey(point: Point) {
  return `${point.x.toFixed(2)},${point.y.toFixed(2)}`;
}
