import { expect, test } from "bun:test";
import { join, resolve } from "node:path";

import { resolvePackArchivePath } from "./pack-archives";

const destination = resolve("/tmp/poodle-pack-dest");

test("resolves a relative pack filename into the temp destination", () => {
  expect(
    resolvePackArchivePath(
      "@inflatable-cookie/poodle-core",
      "poodle-core-0.3.0.tgz\n",
      destination,
    ),
  ).toBe(join(destination, "poodle-core-0.3.0.tgz"));
});

test("keeps an absolute archive that already sits in the destination", () => {
  const archive = join(destination, "poodle-svelte-0.3.0.tgz");
  expect(
    resolvePackArchivePath("@inflatable-cookie/poodle-svelte", `${archive}\n`, destination),
  ).toBe(archive);
});

test("rejects an archive path that would land in the package checkout", () => {
  expect(() =>
    resolvePackArchivePath(
      "@inflatable-cookie/poodle-react",
      "/Users/tom/Dev/projects/poodle/packages/react/components/poodle-react-0.3.0.tgz\n",
      destination,
    ),
  ).toThrow(/outside the temp destination/);
});
