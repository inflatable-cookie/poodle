export function packedMemberMissing(
  archiveEntries: readonly string[],
  packagePath: string,
): boolean {
  const normalized = packagePath.replace(/^\.\//, "");
  const prefix = `package/${normalized}`;
  return !archiveEntries.some(
    (entry) => entry === prefix || entry.startsWith(`${prefix}/`),
  );
}
