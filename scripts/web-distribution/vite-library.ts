import { build, type PluginOption } from "vite";

export type ViteLibraryBuild = {
  root: string;
  outDir: string;
  entries: Record<string, string>;
  fileName: (entryName: string) => string;
  externals?: string[];
  plugins?: PluginOption[];
  ssr?: boolean;
  chunkFileNames?: string;
};

export function isExternalId(id: string, modules: readonly string[]): boolean {
  if (id.startsWith("\0") || id.startsWith(".") || id.startsWith("/")) return false;
  if (/^[A-Za-z]:[\\/]/.test(id)) return false;
  return modules.some((name) => id === name || id.startsWith(`${name}/`));
}

export type ViteLibraryGraph = {
  moduleIds: string[];
  specifiers: string[];
};

type RollupChunk = {
  type?: string;
  moduleIds?: string[];
  imports?: string[];
  dynamicImports?: string[];
};

type RollupBuildResult = {
  output?: RollupChunk[];
};

function collectGraph(result: unknown): ViteLibraryGraph {
  const outputs = Array.isArray(result) ? result : [result];
  const moduleIds: string[] = [];
  const specifiers: string[] = [];
  for (const output of outputs) {
    const chunks = (output as RollupBuildResult | undefined)?.output ?? [];
    for (const chunk of chunks) {
      if (chunk?.type !== "chunk") continue;
      moduleIds.push(...(chunk.moduleIds ?? []));
      specifiers.push(...(chunk.imports ?? []), ...(chunk.dynamicImports ?? []));
    }
  }
  return { moduleIds, specifiers };
}

export async function buildViteLibrary(options: ViteLibraryBuild): Promise<ViteLibraryGraph> {
  const names = Object.keys(options.entries);
  if (names.length === 0) throw new Error("vite library build needs at least one entry");
  const sorted = [...names].sort();
  if (names.join("\0") !== sorted.join("\0")) {
    throw new Error("vite library entries must be supplied in sorted name order");
  }

  const externals = options.externals ?? [];
  const result = await build({
    configFile: false,
    root: options.root,
    logLevel: "warn",
    plugins: options.plugins,
    esbuild: { jsx: "automatic", jsxDev: false },
    build: {
      target: "es2022",
      minify: false,
      sourcemap: false,
      emptyOutDir: false,
      cssCodeSplit: false,
      ssr: options.ssr === true,
      outDir: options.outDir,
      lib: {
        entry: options.entries,
        formats: ["es"],
        fileName: (_format, entryName) => options.fileName(entryName),
      },
      rollupOptions: {
        preserveEntrySignatures: "exports-only",
        external: (id) => isExternalId(id, externals),
        output: {
          entryFileNames: (chunk) => options.fileName(chunk.name),
          chunkFileNames: options.chunkFileNames ?? "chunks/[name].js",
          assetFileNames: "[name][extname]",
        },
      },
    },
  });

  return collectGraph(result);
}
