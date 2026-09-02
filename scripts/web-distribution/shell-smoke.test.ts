import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { findRepoRoot } from "./core-build";
import { buildReact } from "./react-build";
import { buildSvelte } from "./svelte-build";
import { REACT_PACKAGE_DIR, SVELTE_PACKAGE_DIR } from "./shell-contract";

const repoRoot = findRepoRoot();

function packPackage(packageDir: string, destDir: string): string {
  const result = spawnSync("npm", ["pack", "--pack-destination", destDir], {
    cwd: join(repoRoot, packageDir),
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(`npm pack failed in ${packageDir}:\n${result.stdout}${result.stderr}`);
  }
  const archive = result.stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.endsWith(".tgz"))
    .at(-1);
  if (!archive) throw new Error(`npm pack did not report an archive for ${packageDir}`);
  return join(destDir, archive.split("/").at(-1) ?? archive);
}

function listArchive(archivePath: string): string[] {
  const result = spawnSync("tar", ["tzf", archivePath], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`tar tzf failed: ${result.stderr}`);
  }
  return result.stdout.split("\n").filter(Boolean);
}

function run(command: string, args: string[], cwd: string): string {
  const result = spawnSync(command, args, { cwd, encoding: "utf8", timeout: 60_000 });
  if (result.status !== 0) {
    throw new Error(
      `${command} ${args.join(" ")} failed:\n${result.stdout ?? ""}${result.stderr ?? ""}`,
    );
  }
  return `${result.stdout ?? ""}${result.stderr ?? ""}`;
}

describe("disposable installed shell smoke", () => {
  test("packed archives install, SSR/browser, and reject client SSR plus missing marked", async () => {
    await buildSvelte(repoRoot);
    await buildReact(repoRoot);
    const root = mkdtempSync(join(tmpdir(), "poodle-shell-smoke-"));
    try {
      const packDir = join(root, "packs");
      mkdirSync(packDir);
      const coreArchive = packPackage("packages/core", packDir);
      const svelteArchive = packPackage(SVELTE_PACKAGE_DIR, packDir);
      const reactArchive = packPackage(REACT_PACKAGE_DIR, packDir);

      for (const archive of [coreArchive, svelteArchive, reactArchive]) {
        const members = listArchive(archive);
        expect(members.some((name) => name === "package/dist/.poodle-build.json")).toBe(true);
        expect(members.some((name) => name.includes("/src/"))).toBe(false);
        expect(members.some((name) => name.endsWith(".map"))).toBe(false);
        expect(
          members.some((name) => name.endsWith(".svelte") && !name.endsWith(".svelte.d.ts")),
        ).toBe(false);
      }
      const svelteMembers = listArchive(svelteArchive);
      expect(svelteMembers).toContain("package/dist/types.js");
      expect(svelteMembers).toContain("package/dist/types.d.ts");
      expect(svelteMembers).toContain("package/dist/Button.svelte.d.ts");
      expect(svelteMembers).not.toContain("package/dist/MenuSurface.client.js");
      expect(svelteMembers).not.toContain("package/dist/MenuSurface.server.js");
      expect(svelteMembers).not.toContain("package/dist/MenuSurface.svelte.d.ts");
      expect(svelteMembers).not.toContain("package/dist/DragDropProvider.client.js");
      expect(svelteMembers).not.toContain("package/dist/DragDropProvider.svelte.d.ts");

      const consumer = join(root, "consumer");
      mkdirSync(consumer);
      const consumerManifest = {
        name: "poodle-shell-smoke",
        type: "module",
        private: true,
        dependencies: {
          "@inflatable-cookie/poodle-core": `file:${coreArchive}`,
          "@inflatable-cookie/poodle-svelte": `file:${svelteArchive}`,
          "@inflatable-cookie/poodle-react": `file:${reactArchive}`,
          svelte: "5.56.8",
          react: "19.2.8",
          "react-dom": "19.2.8",
          "happy-dom": "20.11.2",
        },
        overrides: {
          "@inflatable-cookie/poodle-core": `file:${coreArchive}`,
          "@inflatable-cookie/poodle-svelte": `file:${svelteArchive}`,
          "@inflatable-cookie/poodle-react": `file:${reactArchive}`,
        },
      };
      writeFileSync(join(consumer, "package.json"), `${JSON.stringify(consumerManifest)}\n`);
      writeFileSync(
        join(consumer, "css-load.mjs"),
        `export async function load(url, context, nextLoad) {
  if (url.endsWith(".css") || url.includes(".css?")) {
    return { format: "module", shortCircuit: true, source: "export default {};\\n" };
  }
  return nextLoad(url, context);
}
`,
      );
      writeFileSync(
        join(consumer, "css-register.mjs"),
        `import { register } from "node:module";
import { pathToFileURL } from "node:url";
register("./css-load.mjs", pathToFileURL("./"));
`,
      );
      writeFileSync(
        join(consumer, "ssr.mjs"),
        `import DirectButton from "@inflatable-cookie/poodle-svelte/Button.svelte";
import DirectSelect from "@inflatable-cookie/poodle-svelte/Select.svelte";
import { render } from "svelte/server";
const button = render(DirectButton, { props: { type: "button" } }).body;
const select = render(DirectSelect, { props: {} }).body;
if (!button.includes("poodle-button")) throw new Error("direct Button SSR missed anatomy");
if (!select.includes("poodle-select")) throw new Error("direct Select SSR missed anatomy");
process.stdout.write("ssr-ok\\n");
`,
      );
      writeFileSync(
        join(consumer, "worker.mjs"),
        `import { Worker } from "node:worker_threads";
const worker = new Worker(new URL("./ssr.mjs", import.meta.url), {
  execArgv: ["--import", "./css-register.mjs"],
  stdout: true,
  stderr: true,
});
worker.stdout.on("data", (chunk) => process.stdout.write(chunk));
worker.stderr.on("data", (chunk) => process.stderr.write(chunk));
worker.on("error", (error) => {
  console.error(error);
  process.exit(1);
});
worker.on("exit", (code) => process.exit(code ?? 1));
`,
      );
      writeFileSync(
        join(consumer, "resolve-export.mjs"),
        `const url = import.meta.resolve("@inflatable-cookie/poodle-svelte/Button.svelte");
process.stdout.write(url);
`,
      );
      writeFileSync(
        join(consumer, "mount.mjs"),
        `import { Window } from "happy-dom";
import { mount, unmount } from "svelte";
const window = new Window({ url: "https://poodle.test/" });
const assigned = [];
for (const key of Object.getOwnPropertyNames(window)) {
  if (key in globalThis) continue;
  try {
    globalThis[key] = window[key];
    assigned.push(key);
  } catch {}
}
const previous = {
  window: globalThis.window,
  document: globalThis.document,
};
globalThis.window = window;
globalThis.document = window.document;
try {
  Object.defineProperty(globalThis, "navigator", {
    configurable: true,
    get: () => window.navigator,
  });
} catch {}
try {
  const { default: Button } = await import("@inflatable-cookie/poodle-svelte/Button.svelte");
  const target = window.document.createElement("div");
  window.document.body.appendChild(target);
  const app = mount(Button, { target, props: { type: "button" } });
  if (!target.innerHTML.includes("poodle-button")) {
    throw new Error(\`browser mount missed anatomy: \${target.innerHTML}\`);
  }
  unmount(app);
  process.stdout.write("mount-ok\\n");
} finally {
  Object.assign(globalThis, previous);
  for (const key of assigned) delete globalThis[key];
  window.close();
}
`,
      );
      writeFileSync(
        join(consumer, "client-ssr-negative.mjs"),
        `import { pathToFileURL } from "node:url";
import { render } from "svelte/server";
const client = await import(pathToFileURL("./node_modules/@inflatable-cookie/poodle-svelte/dist/Button.client.js").href);
try {
  render(client.default, { props: { type: "button" } }).body;
  throw new Error("client artifact served SSR");
} catch (error) {
  if (error instanceof Error && error.message === "client artifact served SSR") throw error;
  process.stdout.write("client-ssr-rejected\\n");
}
`,
      );

      run("bun", ["install"], consumer);

      const installedSvelte = join(consumer, "node_modules/@inflatable-cookie/poodle-svelte");
      expect(existsSync(join(installedSvelte, "src"))).toBe(false);
      expect(existsSync(join(installedSvelte, "dist/Button.client.js"))).toBe(true);
      expect(existsSync(join(installedSvelte, "dist/types.js"))).toBe(true);

      expect(
        readFileSync(join(installedSvelte, "dist/Button.server.js"), "utf8"),
      ).not.toMatch(/from ["']marked["']/);
      expect(
        readFileSync(join(installedSvelte, "dist/index.server.js"), "utf8"),
      ).not.toMatch(/from ["']marked["']/);
      const nodeSsr = run("node", ["--import", "./css-register.mjs", "./ssr.mjs"], consumer);
      expect(nodeSsr).toContain("ssr-ok");
      const workerSsr = run("node", ["--import", "./css-register.mjs", "./worker.mjs"], consumer);
      expect(workerSsr).toContain("ssr-ok");
      const defaultResolved = run(
        "node",
        ["--import", "./css-register.mjs", "./resolve-export.mjs"],
        consumer,
      );
      expect(defaultResolved).toContain("Button.server.js");
      expect(defaultResolved).not.toContain("Button.client.js");
      const browserResolved = run(
        "node",
        ["--conditions=browser", "--import", "./css-register.mjs", "./resolve-export.mjs"],
        consumer,
      );
      expect(browserResolved).toContain("Button.client.js");
      const internalResolve = spawnSync(
        "node",
        [
          "--import",
          "./css-register.mjs",
          "-e",
          `import "@inflatable-cookie/poodle-svelte/MenuSurface.svelte"`,
        ],
        { cwd: consumer, encoding: "utf8" },
      );
      expect(internalResolve.status).not.toBe(0);
      expect(`${internalResolve.stdout}${internalResolve.stderr}`).toMatch(
        /ERR_MODULE_NOT_FOUND|Cannot find module/i,
      );

      const mountOut = run(
        "node",
        ["--conditions=browser", "--import", "./css-register.mjs", "./mount.mjs"],
        consumer,
      );
      expect(mountOut).toContain("mount-ok");
      const negative = run(
        "node",
        ["--import", "./css-register.mjs", "./client-ssr-negative.mjs"],
        consumer,
      );
      expect(negative).toContain("client-ssr-rejected");

      const missingMarked = spawnSync(
        "node",
        ["--import", "./css-register.mjs", "-e", `import "@inflatable-cookie/poodle-svelte/markdown"`],
        { cwd: consumer, encoding: "utf8" },
      );
      expect(missingMarked.status).not.toBe(0);
      expect(`${missingMarked.stdout}${missingMarked.stderr}`).toMatch(/marked/i);

      writeFileSync(
        join(consumer, "root-import.mjs"),
        `import * as SvelteRoot from "@inflatable-cookie/poodle-svelte";
import * as ReactRoot from "@inflatable-cookie/poodle-react";
const leaked = ["AgentMessage", "AgentPlan", "AgentPlanRecord", "AgentTranscript", "MarkdownEditor"];
for (const name of leaked) {
  if (name in SvelteRoot) throw new Error(\`svelte root leaked \${name}\`);
  if (name in ReactRoot) throw new Error(\`react root leaked \${name}\`);
}
if (typeof SvelteRoot.Button !== "function") throw new Error("svelte root Button missing");
if (typeof ReactRoot.Button !== "function") throw new Error("react root Button missing");
process.stdout.write("root-parser-free\\n");
`,
      );
      expect(
        run("node", ["--import", "./css-register.mjs", "./root-import.mjs"], consumer),
      ).toContain("root-parser-free");

      writeFileSync(
        join(consumer, "package.json"),
        `${JSON.stringify({
          ...consumerManifest,
          dependencies: { ...consumerManifest.dependencies, marked: "18.0.9" },
        })}\n`,
      );
      run("bun", ["install"], consumer);
      writeFileSync(
        join(consumer, "markdown-ssr.mjs"),
        `import { Button as RootButton } from "@inflatable-cookie/poodle-svelte";
import { AgentMessage, AgentPlan, AgentPlanRecord } from "@inflatable-cookie/poodle-svelte/markdown";
import DirectPlan from "@inflatable-cookie/poodle-svelte/AgentPlan.svelte";
import { render } from "svelte/server";
const markdown = "## Proposed plan\\n\\n1. Add the surface";
const root = render(RootButton, { props: { type: "button" } }).body;
if (!root.includes("poodle-button")) throw new Error("root Button SSR missed anatomy");
const message = render(AgentMessage, { props: { markdown: "hi" } }).body;
if (!message.includes("poodle-agent-message")) throw new Error("markdown SSR missed anatomy");
const plan = render(AgentPlan, { props: { plan: markdown } }).body;
if (!plan.includes("poodle-agent-plan__body")) throw new Error("AgentPlan SSR missed body");
if (!plan.includes("poodle-agent-message")) throw new Error("AgentPlan SSR missed markdown");
if (!plan.includes("Proposed plan")) throw new Error("AgentPlan SSR did not render markdown");
if (plan.includes("<!--$!-->") || /suspended/i.test(plan)) throw new Error("AgentPlan SSR used Suspense");
const directPlan = render(DirectPlan, { props: { plan: markdown } }).body;
if (!directPlan.includes("Proposed plan")) throw new Error("direct AgentPlan SSR missed markdown");
const record = render(AgentPlanRecord, {
  props: { plan: markdown, status: "accepted", expanded: true },
}).body;
if (!record.includes("poodle-agent-plan-record__body")) throw new Error("AgentPlanRecord SSR missed body");
if (!record.includes("poodle-agent-message")) throw new Error("AgentPlanRecord SSR missed markdown");
if (!record.includes("Proposed plan")) throw new Error("AgentPlanRecord SSR did not render markdown");
if (record.includes("<!--$!-->") || /suspended/i.test(record)) {
  throw new Error("AgentPlanRecord SSR used Suspense");
}
process.stdout.write("markdown-ok\\n");
`,
      );
      expect(run("node", ["--import", "./css-register.mjs", "./markdown-ssr.mjs"], consumer)).toContain(
        "markdown-ok",
      );

      writeFileSync(
        join(consumer, "plan-mount.mjs"),
        `import { Window } from "happy-dom";
import { mount, unmount } from "svelte";
const window = new Window({ url: "https://poodle.test/" });
const assigned = [];
for (const key of Object.getOwnPropertyNames(window)) {
  if (key in globalThis) continue;
  try {
    globalThis[key] = window[key];
    assigned.push(key);
  } catch {}
}
const previous = { window: globalThis.window, document: globalThis.document };
globalThis.window = window;
globalThis.document = window.document;
try {
  Object.defineProperty(globalThis, "navigator", {
    configurable: true,
    get: () => window.navigator,
  });
} catch {}
try {
  const { AgentPlan, AgentPlanRecord } = await import("@inflatable-cookie/poodle-svelte/markdown");
  const target = window.document.createElement("div");
  window.document.body.appendChild(target);
  const plan = mount(AgentPlan, {
    target,
    props: { plan: "## Proposed plan\\n\\n1. Add the surface" },
  });
  if (!target.innerHTML.includes("poodle-agent-plan__body")) {
    throw new Error(\`plan mount missed body: \${target.innerHTML}\`);
  }
  if (!target.innerHTML.includes("Proposed plan")) {
    throw new Error(\`plan mount missed markdown: \${target.innerHTML}\`);
  }
  unmount(plan);
  target.replaceChildren();
  const record = mount(AgentPlanRecord, {
    target,
    props: { plan: "## Proposed plan\\n\\n1. Add the surface", status: "accepted", expanded: true },
  });
  if (!target.innerHTML.includes("poodle-agent-plan-record__body")) {
    throw new Error(\`record mount missed body: \${target.innerHTML}\`);
  }
  if (!target.innerHTML.includes("Proposed plan")) {
    throw new Error(\`record mount missed markdown: \${target.innerHTML}\`);
  }
  unmount(record);
  process.stdout.write("plan-mount-ok\\n");
} finally {
  Object.assign(globalThis, previous);
  for (const key of assigned) delete globalThis[key];
  window.close();
}
`,
      );
      expect(
        run("node", ["--conditions=browser", "--import", "./css-register.mjs", "./plan-mount.mjs"], consumer),
      ).toContain("plan-mount-ok");

      writeFileSync(
        join(consumer, "react-plan-ssr.mjs"),
        `import { createElement } from "react";
import { renderToString } from "react-dom/server";
import { AgentPlan, AgentPlanRecord } from "@inflatable-cookie/poodle-react/markdown";
const markdown = "## Proposed plan\\n\\n1. Add the surface";
const plan = renderToString(createElement(AgentPlan, { plan: markdown }));
if (!plan.includes("poodle-agent-plan__body")) throw new Error("react AgentPlan SSR missed body");
if (!plan.includes("poodle-agent-message")) throw new Error("react AgentPlan SSR missed markdown");
if (!plan.includes("Proposed plan")) throw new Error("react AgentPlan SSR did not render markdown");
if (/suspended|abort/i.test(plan)) throw new Error("react AgentPlan SSR used Suspense");
const record = renderToString(
  createElement(AgentPlanRecord, { plan: markdown, status: "accepted", expanded: true }),
);
if (!record.includes("poodle-agent-plan-record__body")) {
  throw new Error("react AgentPlanRecord SSR missed body");
}
if (!record.includes("Proposed plan")) throw new Error("react AgentPlanRecord SSR did not render markdown");
if (/suspended|abort/i.test(record)) throw new Error("react AgentPlanRecord SSR used Suspense");
process.stdout.write("react-plan-ok\\n");
`,
      );
      expect(
        run("node", ["--import", "./css-register.mjs", "./react-plan-ssr.mjs"], consumer),
      ).toContain("react-plan-ok");

      const reactOut = run(
        "node",
        [
          "--import",
          "./css-register.mjs",
          "-e",
          `import { Button } from "@inflatable-cookie/poodle-react/Button";
if (typeof Button !== "function") throw new Error("react Button is not a function");
process.stdout.write("react-ok\\n");`,
        ],
        consumer,
      );
      expect(reactOut).toContain("react-ok");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 180_000);
});
