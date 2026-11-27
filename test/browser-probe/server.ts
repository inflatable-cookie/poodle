/**
 * Dev-server boot for the browser probes. Every server is private to one probe
 * run, listens on its own free port, and is stopped when the probe body ends,
 * however it ends. URLs are origins, without a trailing slash.
 */
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const REPO_ROOT = fileURLToPath(new URL("../..", import.meta.url));
const SERVE = fileURLToPath(new URL("./serve.ts", import.meta.url));
const CACHE_ROOT = join(REPO_ROOT, "node_modules/.vite-browser-probe");
/** Fixtures have no manifest of their own; they run the Svelte preview's vite. */
const FIXTURE_VITE_HOST = join(REPO_ROOT, "packages/svelte/preview");

const START_TIMEOUT_MS = 60_000;
const REQUEST_TIMEOUT_MS = 5_000;
const RETRY_MS = 150;
const STOP_TIMEOUT_MS = 5_000;

export type PreviewUrls = { svelte: string; react: string };

type ServerSpec = {
  label: string;
  /** Directory whose installed vite serves the app. */
  viteHost: string;
  cwd: string;
  configFile: string;
  logLevel: "info" | "error";
};

type DevServer = { url: string; stop: () => Promise<void> };

/**
 * Serves the vite fixture beside the calling probe (pass `import.meta.url`) for
 * the duration of `body`.
 */
export async function withFixture<T>(probeUrl: string, body: (url: string) => Promise<T>): Promise<T> {
  const fixtureRoot = fileURLToPath(new URL(".", probeUrl));
  const fixture: ServerSpec = {
    label: `fixture ${fixtureRoot}`,
    viteHost: FIXTURE_VITE_HOST,
    cwd: REPO_ROOT,
    configFile: join(fixtureRoot, "vite.config.ts"),
    logLevel: "info",
  };
  return withServers([fixture], ([url]) => body(url!));
}

/** Serves this run's own Svelte and React preview apps for the duration of `body`. */
export async function withPreviews<T>(body: (urls: PreviewUrls) => Promise<T>): Promise<T> {
  const preview = (framework: keyof PreviewUrls): ServerSpec => {
    const app = join(REPO_ROOT, "packages", framework, "preview");
    return {
      label: `${framework} preview`,
      viteHost: app,
      cwd: app,
      configFile: join(app, "vite.config.ts"),
      logLevel: "error",
    };
  };
  return withServers([preview("svelte"), preview("react")], ([svelte, react]) =>
    body({ svelte: svelte!, react: react! }),
  );
}

async function withServers<T>(specs: ServerSpec[], body: (urls: string[]) => Promise<T>): Promise<T> {
  // Settle every start before judging any, so a failed one cannot orphan another.
  const starts = await Promise.allSettled(specs.map(startServer));
  const servers = starts.flatMap((start) => (start.status === "fulfilled" ? [start.value] : []));
  try {
    const failed = starts.find((start) => start.status === "rejected");
    if (failed) throw failed.reason;
    return await body(servers.map((server) => server.url));
  } finally {
    await Promise.all(servers.map((server) => server.stop()));
  }
}

async function startServer(spec: ServerSpec): Promise<DevServer> {
  mkdirSync(CACHE_ROOT, { recursive: true });
  const cacheDir = mkdtempSync(join(CACHE_ROOT, "run-"));
  const reported = Promise.withResolvers<string>();
  const child = Bun.spawn(
    [process.execPath, SERVE, Bun.resolveSync("vite", spec.viteHost), spec.configFile, cacheDir, spec.logLevel],
    {
      cwd: spec.cwd,
      stdout: "inherit",
      stderr: "inherit",
      ipc(message: { url: string }) {
        reported.resolve(message.url);
      },
    },
  );

  const stop = async (): Promise<void> => {
    child.kill();
    const forced = setTimeout(() => child.kill("SIGKILL"), STOP_TIMEOUT_MS);
    await child.exited;
    clearTimeout(forced);
    rmSync(cacheDir, { recursive: true, force: true });
  };

  const died = child.exited.then((code): never => {
    throw new Error(`${spec.label} dev server exited with code ${code} before it served; see its output above`);
  });
  // Every server exits eventually; only a death during startup is a failure.
  died.catch(() => {});

  const deadline = Date.now() + START_TIMEOUT_MS;
  const timedOut = (): Error => new Error(`${spec.label} dev server did not serve within ${START_TIMEOUT_MS}ms`);
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    const url = await Promise.race([
      reported.promise,
      died,
      new Promise<never>((_, reject) => {
        timer = setTimeout(() => reject(timedOut()), START_TIMEOUT_MS);
      }),
    ]);
    while (Date.now() < deadline) {
      const serving = await Promise.race([
        fetch(`${url}/`, { signal: AbortSignal.timeout(REQUEST_TIMEOUT_MS) }).then(
          (response) => response.ok,
          () => false,
        ),
        died,
      ]);
      if (serving) {
        console.log(`  ${spec.label} on ${url}`);
        return { url, stop };
      }
      await Promise.race([Bun.sleep(RETRY_MS), died]);
    }
    throw timedOut();
  } catch (error) {
    await stop();
    throw error;
  } finally {
    clearTimeout(timer);
  }
}
