import type { ReactNode } from "react";
import { Text } from "@inflatable-cookie/poodle-react";

let crashEpoch = 0;
let hasThrownForEpoch = -1;

export function armErrorBoundaryCrash(): void {
  crashEpoch += 1;
}

export function getErrorBoundaryCrashEpoch(): number {
  return crashEpoch;
}

export function ErrorBoundaryCrashOnce(): ReactNode {
  if (hasThrownForEpoch !== crashEpoch) {
    hasThrownForEpoch = crashEpoch;
    throw new Error("Preview child failed during render.");
  }

  return <Text>Recovered child content.</Text>;
}
