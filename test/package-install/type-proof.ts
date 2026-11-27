import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { outputIncludes } from "./strip-ansi";

const PROOF_DIR = "packed-types";
const SUPPRESSION_MARKERS = ["@ts-expect-error", "@ts-ignore", "as any", ": any"];

type PositiveCase = { file: string; config: string; outcome: "pass" };
type NegativeCase = {
  importPath: string;
  file: string;
  config: string;
  outcome: "fail";
  diagnostic: string;
};
export type TypeProofCase = PositiveCase | NegativeCase;

export async function runTypeProof(consumerRoot: string, cases: readonly TypeProofCase[]) {
  const compiler = join(consumerRoot, "node_modules", ".bin", "tsc");
  if (!existsSync(compiler)) {
    throw new Error("the packed consumer did not install a TypeScript compiler; the type proof cannot run");
  }
  const positives: { file: string; exitCode: number; diagnostics: never[] }[] = [];
  const negatives: { importPath: string; file: string; exitCode: number; diagnostic: string; suppressed: false }[] = [];
  for (const item of cases) {
    if (item.outcome === "fail") {
      // Inspect comments too: TypeScript suppression directives live in comments.
      const source = readFileSync(join(consumerRoot, PROOF_DIR, item.file), "utf8");
      const found = SUPPRESSION_MARKERS.filter((marker) => source.includes(marker));
      if (found.length) throw new Error(`${item.file} suppresses its expected failure (${found.join(", ")})`);
    }
    const env = { ...globalThis.process.env };
    delete env.FORCE_COLOR;
    const process = Bun.spawn([compiler, "--project", join(PROOF_DIR, item.config)], {
      cwd: consumerRoot, stdout: "pipe", stderr: "pipe", env,
    });
    const [stdout, stderr, exitCode] = await Promise.all([
      new Response(process.stdout).text(), new Response(process.stderr).text(), process.exited,
    ]);
    const output = `${stdout}${stderr}`.trim();
    if (item.outcome === "pass") {
      if (exitCode !== 0 || output.length) throw new Error(`packed positive proof failed (${item.file}):\n${output}`);
      positives.push({ file: item.file, exitCode, diagnostics: [] });
    } else {
      if (exitCode === 0) throw new Error(`packed ${item.importPath} unexpectedly accepted ${item.file}`);
      if (!outputIncludes(output, item.diagnostic)) throw new Error(`packed ${item.importPath} failed with the wrong diagnostic:\n${output}`);
      if (!outputIncludes(output, item.file)) throw new Error(`packed ${item.importPath} reported its diagnostic against another file:\n${output}`);
      negatives.push({ importPath: item.importPath, file: item.file, exitCode, diagnostic: output, suppressed: false });
    }
  }
  return { compiler, positives, negatives };
}
