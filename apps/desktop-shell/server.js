import http from "node:http";
import path from "node:path";
import { readFile, readdir, stat } from "node:fs/promises";
import { fileURLToPath } from "node:url";

import { MediaCoreAdapterClient } from "../../packages/desktop-shell/src/index.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, "..", "..");
const defaultProjectsRoot = path.join(repoRoot, "target", "demo-ui");

const MIME_TYPES = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".md": "text/markdown; charset=utf-8",
};

function json(response, statusCode, payload) {
  response.writeHead(statusCode, { "content-type": "application/json; charset=utf-8" });
  response.end(JSON.stringify(payload, null, 2));
}

function toPosixPath(filePath) {
  return filePath.split(path.sep).join("/");
}

function toRepoRelativePath(filePath) {
  return toPosixPath(path.relative(repoRoot, filePath));
}

function getProjectPairKey(fileName) {
  if (fileName.endsWith("snapshot.json")) {
    return fileName.replace(/snapshot\.json$/, "");
  }
  if (fileName.endsWith("event-log.json")) {
    return fileName.replace(/event-log\.json$/, "");
  }
  return null;
}

async function walkFiles(root) {
  const files = [];
  let entries = [];

  try {
    entries = await readdir(root, { withFileTypes: true });
  } catch (error) {
    if (error && typeof error === "object" && error.code === "ENOENT") {
      return files;
    }
    throw error;
  }

  for (const entry of entries) {
    const entryPath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      files.push(...(await walkFiles(entryPath)));
      continue;
    }
    if (entry.isFile()) {
      files.push(entryPath);
    }
  }

  return files;
}

async function readSnapshotMetadata(snapshotPath) {
  try {
    const content = await readFile(snapshotPath, "utf8");
    const parsed = JSON.parse(content);
    const state = parsed?.state;
    if (!state || typeof state !== "object") {
      return {};
    }

    return {
      projectId: state.id ?? null,
      name: state.name ?? null,
      aspectRatio: state.aspect_ratio ?? null,
    };
  } catch {
    return {};
  }
}

async function discoverProjectDocuments(projectsRoot) {
  const files = await walkFiles(projectsRoot);
  const pairs = new Map();

  for (const filePath of files) {
    const fileName = path.basename(filePath);
    const pairKey = getProjectPairKey(fileName);
    if (pairKey === null) {
      continue;
    }

    const directory = path.dirname(filePath);
    const mapKey = `${directory}::${pairKey}`;
    const existing = pairs.get(mapKey) ?? {};

    if (fileName.endsWith("snapshot.json")) {
      existing.snapshotPath = filePath;
    } else if (fileName.endsWith("event-log.json")) {
      existing.eventLogPath = filePath;
    }

    pairs.set(mapKey, existing);
  }

  const projects = await Promise.all(
    [...pairs.values()]
      .filter((entry) => entry.snapshotPath && entry.eventLogPath)
      .map(async (entry) => {
        const [snapshotStats, eventLogStats, metadata] = await Promise.all([
          stat(entry.snapshotPath),
          stat(entry.eventLogPath),
          readSnapshotMetadata(entry.snapshotPath),
        ]);
        const snapshotPath = toRepoRelativePath(entry.snapshotPath);
        const eventLogPath = toRepoRelativePath(entry.eventLogPath);
        const prefix = getProjectPairKey(path.basename(entry.snapshotPath)) || "snapshot";
        const updatedAt = new Date(
          Math.max(snapshotStats.mtimeMs, eventLogStats.mtimeMs),
        ).toISOString();
        const fallbackLabel = prefix.replace(/[-_]$/, "") || "Project Document";

        return {
          id: `${snapshotPath}::${eventLogPath}`,
          label: metadata.name ?? fallbackLabel,
          projectId: metadata.projectId ?? null,
          name: metadata.name ?? null,
          aspectRatio: metadata.aspectRatio ?? null,
          snapshotPath,
          eventLogPath,
          updatedAt,
        };
      }),
  );

  projects.sort((left, right) => right.updatedAt.localeCompare(left.updatedAt));
  return projects;
}

async function readJsonBody(request) {
  const chunks = [];
  for await (const chunk of request) {
    chunks.push(chunk);
  }
  const body = Buffer.concat(chunks).toString("utf8");
  return body.trim() ? JSON.parse(body) : {};
}

function resolveStaticPath(urlPathname) {
  if (urlPathname === "/") {
    return path.join(repoRoot, "apps", "desktop-shell", "public", "index.html");
  }

  const decoded = decodeURIComponent(urlPathname);
  const candidate = path.resolve(repoRoot, `.${decoded}`);
  if (!candidate.startsWith(repoRoot)) {
    return null;
  }
  return candidate;
}

async function serveStatic(request, response) {
  const url = new URL(request.url, "http://localhost");
  const filePath = resolveStaticPath(url.pathname);
  if (!filePath) {
    json(response, 403, { ok: false, message: "forbidden path" });
    return;
  }

  try {
    const buffer = await readFile(filePath);
    const contentType = MIME_TYPES[path.extname(filePath)] ?? "application/octet-stream";
    response.writeHead(200, { "content-type": contentType });
    response.end(buffer);
  } catch {
    json(response, 404, { ok: false, message: `not found: ${url.pathname}` });
  }
}

export function createDesktopShellServer(options = {}) {
  const adapter =
    options.adapter ??
    new MediaCoreAdapterClient({
      cwd: options.cwd ?? repoRoot,
    });
  const projectsRoot = path.resolve(options.projectsRoot ?? defaultProjectsRoot);

  return http.createServer(async (request, response) => {
    const url = new URL(request.url, "http://localhost");

    if (request.method === "GET" && url.pathname === "/api/health") {
      json(response, 200, { ok: true, message: "desktop shell ready" });
      return;
    }

    if (request.method === "POST" && url.pathname === "/api/adapter") {
      try {
        const adapterRequest = await readJsonBody(request);
        const adapterResponse = await adapter.send(adapterRequest);
        json(response, 200, adapterResponse);
      } catch (error) {
        json(response, 500, {
          ok: false,
          message: error instanceof Error ? error.message : String(error),
        });
      }
      return;
    }

    if (request.method === "GET" && url.pathname === "/api/projects") {
      try {
        const projects = await discoverProjectDocuments(projectsRoot);
        json(response, 200, {
          ok: true,
          root: toRepoRelativePath(projectsRoot),
          projects,
        });
      } catch (error) {
        json(response, 500, {
          ok: false,
          message: error instanceof Error ? error.message : String(error),
        });
      }
      return;
    }

    if (request.method === "GET") {
      await serveStatic(request, response);
      return;
    }

    json(response, 405, { ok: false, message: "method not allowed" });
  });
}

if (process.argv[1] === __filename) {
  const port = Number(process.env.PORT ?? 4173);
  const server = createDesktopShellServer();
  server.listen(port, () => {
    console.log(`CineWeave desktop shell running at http://127.0.0.1:${port}`);
  });
}
