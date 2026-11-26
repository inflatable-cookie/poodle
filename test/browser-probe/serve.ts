/**
 * Dev-server child for the browser probes; `server.ts` spawns it.
 *
 * It binds port 0, so concurrent probes never collide, and reports the port it
 * was given over IPC, so a probe only ever talks to its own server. It exits
 * when the probe goes away, even when the probe was killed.
 *
 *   bun serve.ts <vite entry> <config file> <cache dir> <log level>
 */
import { rmSync } from "node:fs";
import type { AddressInfo } from "node:net";

const [viteEntry, configFile, cacheDir, logLevel] = process.argv.slice(2);
if (!viteEntry || !configFile || !cacheDir || !logLevel || !process.send) {
  throw new Error("serve.ts is spawned by test/browser-probe/server.ts");
}

process.on("exit", () => rmSync(cacheDir, { recursive: true, force: true }));
process.on("disconnect", () => process.exit(0));

const { createServer } = await import(viteEntry);
const server = await createServer({
  configFile,
  // A private optimizer cache: two servers sharing one rewrite each other's deps.
  cacheDir,
  logLevel,
  server: { host: "127.0.0.1", port: 0, strictPort: true },
});
await server.listen();
const { port } = server.httpServer.address() as AddressInfo;
process.send({ url: `http://127.0.0.1:${port}` });
