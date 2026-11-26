/** Pass/fail reporting for the browser probes: one failure count per probe run. */

let failures = 0;

export function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? `  — ${detail}` : ""}`);
}

export function failureCount(): number {
  return failures;
}

/** Forgets the failures of an attempt that is about to be retried. */
export function rewindFailures(count: number): void {
  failures = count;
}

/** Prints the verdict and exits non-zero when any check failed. */
export function finish(probe: string): never {
  if (failures > 0) {
    console.error(`\n${failures} ${probe} check(s) failed`);
    process.exit(1);
  }
  console.log(`\nall ${probe} checks passed`);
  process.exit(0);
}
