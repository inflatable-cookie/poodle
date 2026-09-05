import { join, resolve } from "node:path";

/** Resolve `bun pm pack` output against a temp destination, never the package cwd. */
export function resolvePackArchivePath(
  packageName: string,
  packOutput: string,
  destination: string,
): string {
  const archivePath = packOutput
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.endsWith(".tgz"))
    .at(-1);
  if (!archivePath) {
    throw new Error(
      `${packageName} pack did not report a .tgz archive path:\n${packOutput.trim()}`,
    );
  }
  const resolved = resolve(
    archivePath.startsWith("/") ? archivePath : join(destination, archivePath),
  );
  const destinationRoot = resolve(destination);
  const inside =
    resolved === destinationRoot || resolved.startsWith(`${destinationRoot}/`);
  if (!inside) {
    throw new Error(
      `${packageName} pack reported an archive outside the temp destination:\n${resolved}`,
    );
  }
  return resolved;
}
