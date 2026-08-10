import { readFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";

const errors: string[] = [];
const list = Bun.spawnSync(["git", "ls-files", "-z"], {
  stdout: "pipe",
  stderr: "pipe",
});

if (list.exitCode !== 0) {
  throw new Error("Unable to enumerate tracked files with git ls-files.");
}

const paths = new TextDecoder()
  .decode(list.stdout)
  .split("\0")
  .filter(Boolean);
const packagePaths = paths.filter((path) => basename(path) === "package.json");
const cargoPaths = paths.filter((path) => basename(path) === "Cargo.toml");
const rootLicense = readFileSync("LICENSE", "utf8");

for (const path of packagePaths) {
  const manifest = JSON.parse(readFileSync(path, "utf8")) as {
    license?: string;
    files?: string[];
    poodleRelease?: { publicIntent?: boolean };
  };

  if (manifest.license !== "MIT") {
    errors.push(`${path}: expected license \"MIT\"`);
  }

  if (manifest.poodleRelease?.publicIntent !== true) continue;

  const packageLicensePath = join(dirname(path), "LICENSE");
  try {
    if (readFileSync(packageLicensePath, "utf8") !== rootLicense) {
      errors.push(`${packageLicensePath}: does not match the repository LICENSE`);
    }
  } catch {
    errors.push(`${packageLicensePath}: public package license is missing`);
  }
}

for (const path of cargoPaths) {
  const source = readFileSync(path, "utf8");
  if (!/^license\s*=\s*"MIT"\s*$/m.test(source)) {
    errors.push(`${path}: expected license = \"MIT\"`);
  }
}

const coreManifest = JSON.parse(
  readFileSync("packages/core/package.json", "utf8"),
) as { files?: string[] };
if (!coreManifest.files?.includes("THIRD_PARTY_NOTICES.md")) {
  errors.push(
    "packages/core/package.json: THIRD_PARTY_NOTICES.md is missing from files",
  );
}

const requiredNotices = [
  {
    path: "packages/core/THIRD_PARTY_NOTICES.md",
    markers: ["Lucide Icons 1.31.0", "ISC License", "Cole Bemis"],
  },
  {
    path: "packages/render/assets/icons/LICENSE.txt",
    markers: ["Lucide Icons and Contributors", "ISC License", "Cole Bemis"],
  },
  {
    path: "packages/gpui/preview/assets/fonts/LICENSE.txt",
    markers: ["The Inter Project Authors", "SIL OPEN FONT LICENSE Version 1.1"],
  },
  {
    path: "THIRD_PARTY_NOTICES.md",
    markers: ["Lucide 1.31.0", "canonical Poodle manifest", "Inter 4.001"],
  },
];

for (const notice of requiredNotices) {
  try {
    const source = readFileSync(notice.path, "utf8");
    for (const marker of notice.markers) {
      if (!source.includes(marker)) {
        errors.push(`${notice.path}: missing required marker ${JSON.stringify(marker)}`);
      }
    }
  } catch {
    errors.push(`${notice.path}: required third-party notice is missing`);
  }
}

if (errors.length > 0) throw new Error(errors.join("\n"));

console.log(
  `License compliance clean: ${packagePaths.length} package manifests, ${cargoPaths.length} Cargo manifests, and ${requiredNotices.length} notice surfaces.`,
);
