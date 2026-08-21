import type { ReactNode } from "react";
import { Text } from "@inflatable-cookie/poodle-react";

const thrownTokens = new WeakSet<object>();

export function ErrorBoundaryCrashOnce({ token }: { token: object }): ReactNode {
  if (!thrownTokens.has(token)) {
    thrownTokens.add(token);
    throw new Error("Preview child failed during render.");
  }

  return <Text>Recovered child content.</Text>;
}
