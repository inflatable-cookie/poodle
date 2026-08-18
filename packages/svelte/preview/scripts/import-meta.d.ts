interface ImportMeta {
  readonly dir: string;
}

declare const Bun: {
  spawnSync(
    command: string[],
    options?: { cwd?: string; stdout?: string; stderr?: string },
  ): { success: boolean; stdout: Uint8Array; stderr: Uint8Array };
  Glob: new (pattern: string) => {
    scanSync(options?: { cwd?: string; absolute?: boolean }): string[];
  };
};
