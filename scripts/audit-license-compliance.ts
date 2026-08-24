import { readFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { missingNoticeMarkers } from "./license-compliance-policy.ts";

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
    // `bzip2`/`libbz2-rs-sys` left every graph with the GPUI fork (g16.005), so
    // their notice and the separate node-backend notice surface were removed
    // in g16.006. A notice surface describes the CURRENT graph: keeping the
    // text would have been a false claim about what Poodle distributes. The
    // sweep below is what stops it drifting back in either direction.
    path: "THIRD_PARTY_NOTICES.md",
    markers: ["Lucide 1.31.0", "canonical Poodle manifest", "Inter 4.001"],
  },
];

// Notice truth is bidirectional. A marker list only catches a notice that went
// missing; it cannot catch a notice for a crate that left the graph, which is
// exactly the drift g16.006 had to repair. So derive the claim from the locks:
// if no lockfile resolves the crate, no tracked source may still name it.
const retiredNoticeCrates = [
  { crate: "bzip2", lockNames: ["bzip2", "libbz2-rs-sys"], claim: /bzip2|libbz2/i },
];
const lockPaths = paths.filter((path) => basename(path) === "Cargo.lock");
const noticeSweepPaths = paths.filter(
  (path) =>
    basename(path) === "THIRD_PARTY_NOTICES.md" ||
    path === "deny.toml" ||
    path === "docs/specs/022-packaging-versioning-and-release-channel-rules.md",
);

for (const retired of retiredNoticeCrates) {
  const resolvedIn = lockPaths.filter((lockPath) => {
    const lock = readFileSync(lockPath, "utf8");
    return retired.lockNames.some((name) => lock.includes(`name = "${name}"`));
  });

  if (resolvedIn.length > 0) {
    // The crate came back. Re-adding the notice is the correct response, but it
    // is a deliberate licence decision, so fail rather than pass silently.
    errors.push(
      `${retired.crate} is resolved again in ${resolvedIn.join(", ")}: restore its notice, licence allow entry, and spec text, then remove it from retiredNoticeCrates.`,
    );
    continue;
  }

  for (const path of noticeSweepPaths) {
    // A tracked path can be absent from the working tree mid-change. A file
    // that does not exist cannot claim anything, and `requiredNotices` below
    // is what reports a notice that was supposed to be there.
    let source: string;
    try {
      source = readFileSync(path, "utf8");
    } catch {
      continue;
    }
    if (retired.claim.test(source)) {
      errors.push(
        `${path}: still claims ${retired.crate}, which no lockfile resolves.`,
      );
    }
  }
}

for (const notice of requiredNotices) {
  try {
    const source = readFileSync(notice.path, "utf8");
    for (const marker of missingNoticeMarkers(source, notice.markers)) {
      errors.push(`${notice.path}: missing required marker ${JSON.stringify(marker)}`);
    }
  } catch {
    errors.push(`${notice.path}: required third-party notice is missing`);
  }
}

if (errors.length > 0) throw new Error(errors.join("\n"));

console.log(
  `License compliance clean: ${packagePaths.length} package manifests, ${cargoPaths.length} Cargo manifests, and ${requiredNotices.length} notice surfaces.`,
);
