import { build } from "vite";

export type ViteLibraryBuild = {
  root: string;
  outDir: string;
  entries: Record<string, string>;
  fileName: (entryName: string) => string;
  externals?: string[];
};

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

  const result = await build({
    configFile: false,
    root: options.root,
    logLevel: "warn",
    build: {
      target: "es2022",
      minify: false,
      sourcemap: false,
      emptyOutDir: false,
      cssCodeSplit: false,
      outDir: options.outDir,
      lib: {
        entry: options.entries,
        formats: ["es"],
        fileName: (_format, entryName) => options.fileName(entryName),
      },
      rollupOptions: {
        external: options.externals ?? [],
        output: {
          entryFileNames: (chunk) => options.fileName(chunk.name),
          chunkFileNames: "chunks/[name].js",
          assetFileNames: "[name][extname]",
        },
      },
    },
  });

  return collectGraph(result);
}
