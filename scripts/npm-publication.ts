// Spec 071: `packages/release-manifest.json` is the one machine-readable npm
// publication authority. Scripts, the archive certificate and the release
// workflow checker derive the publication set and the candidate identity
// manifest name from here instead of repeating package paths.

import { readFileSync } from "node:fs";
import { join } from "node:path";

export type NpmPublicationPackage = {
  name: string;
  path: string;
};

export type NpmPublicationAuthority = {
  candidateManifestName: string;
  packages: NpmPublicationPackage[];
};

export function readNpmPublicationAuthority(root: string): NpmPublicationAuthority {
  const manifestPath = join(root, "packages/release-manifest.json");
  const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as Record<string, unknown>;
  const authority = manifest.npmPublication;
  if (typeof authority !== "object" || authority === null) {
    throw new Error("release manifest must declare npmPublication authority");
  }
  const entry = authority as Record<string, unknown>;
  if (typeof entry.candidateManifestName !== "string" || entry.candidateManifestName === "") {
    throw new Error("release manifest npmPublication must name its candidate identity manifest");
  }
  if (!Array.isArray(entry.packages) || entry.packages.length === 0) {
    throw new Error("release manifest npmPublication must list at least one package");
  }
  const packages = entry.packages.map((candidate) => {
    if (typeof candidate !== "object" || candidate === null) {
      throw new Error("release manifest npmPublication package entries must be objects");
    }
    const { name, path } = candidate as Record<string, unknown>;
    if (typeof name !== "string" || typeof path !== "string") {
      throw new Error("release manifest npmPublication packages need a name and a path");
    }
    return { name, path };
  });
  return { candidateManifestName: entry.candidateManifestName, packages };
}
