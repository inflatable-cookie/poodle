const fullRevision = /^[0-9a-f]{40}$/;

export const approvedGitRevisions = new Map([
  [
    "https://github.com/inflatable-cookie/zed",
    "87d9afbe71ef06ea0634499dc35d104bb29dc020",
  ],
  [
    "https://github.com/zed-industries/font-kit",
    "94b0f28166665e8fd2f53ff6d268a14955c82269",
  ],
  [
    "https://github.com/zed-industries/scap",
    "4afea48c3b002197176fb19cd0f9b180dd36eaac",
  ],
  [
    "https://github.com/zed-industries/wasm_thread",
    "0cf96c7708dfb97ccf3da50347e25edcf75d6937",
  ],
  [
    "https://github.com/proptest-rs/proptest",
    "3dca198a8fef1b32e3a66f1e1897c955b4dc5b5b",
  ],
]);

function activeManifestLines(source: string): Array<{ line: string; number: number }> {
  return source.split(/\r?\n/).flatMap((line, index) => {
    if (line.trimStart().startsWith("#")) return [];
    return [{ line, number: index + 1 }];
  });
}

export function validateCargoManifestSources(
  filePath: string,
  source: string,
): string[] {
  const errors: string[] = [];

  for (const { line, number } of activeManifestLines(source)) {
    for (const match of line.matchAll(/\bgit\s*=\s*"([^"]+)"/g)) {
      const url = match[1]!;
      const revision = line.match(/\brev\s*=\s*"([^"]+)"/)?.[1];
      const expected = approvedGitRevisions.get(url);

      if (!expected) {
        errors.push(
          `${filePath}:${number}: Git repository ${JSON.stringify(url)} is not approved`,
        );
      }

      if (!revision) {
        errors.push(
          `${filePath}:${number}: Git dependency ${JSON.stringify(url)} must use an immutable full rev`,
        );
        continue;
      }

      if (!fullRevision.test(revision)) {
        errors.push(
          `${filePath}:${number}: Git dependency ${JSON.stringify(url)} uses non-full revision ${JSON.stringify(revision)}`,
        );
      }

      if (expected && revision !== expected) {
        errors.push(
          `${filePath}:${number}: Git dependency ${JSON.stringify(url)} uses ${revision}; expected reviewed revision ${expected}`,
        );
      }
    }
  }

  return errors;
}

export function validateCargoLockSources(
  filePath: string,
  source: string,
): string[] {
  const errors: string[] = [];

  for (const [index, line] of source.split(/\r?\n/).entries()) {
    const sourceMatch = line.match(/^source\s*=\s*"git\+([^"]+)"$/);
    if (!sourceMatch) continue;

    const rawSource = sourceMatch[1]!;
    const parsed = rawSource.match(/^(.+?)\?rev=([^#]+)#(.+)$/);
    if (!parsed) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile source must use URL?rev=FULL_REV#FULL_REV, got ${JSON.stringify(rawSource)}`,
      );
      continue;
    }

    const [, url, revision, resolvedRevision] = parsed;
    const expected = approvedGitRevisions.get(url);
    if (!expected) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile repository ${JSON.stringify(url)} is not approved`,
      );
    }

    if (!fullRevision.test(revision) || !fullRevision.test(resolvedRevision)) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile source for ${JSON.stringify(url)} must contain full immutable revisions`,
      );
    }

    if (revision !== resolvedRevision) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile source for ${JSON.stringify(url)} has mismatched requested and resolved revisions`,
      );
    }

    if (expected && (revision !== expected || resolvedRevision !== expected)) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile source for ${JSON.stringify(url)} is not the reviewed revision ${expected}`,
      );
    }
  }

  return errors;
}
