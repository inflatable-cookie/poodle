<script module lang="ts">
  let crashEpoch = 0;
  let hasThrownForEpoch = -1;

  export function armErrorBoundaryCrash(): void {
    crashEpoch += 1;
  }

  export function getErrorBoundaryCrashEpoch(): number {
    return crashEpoch;
  }
</script>

<script lang="ts">
  import { Text } from "@inflatable-cookie/poodle-svelte";

  if (hasThrownForEpoch !== crashEpoch) {
    hasThrownForEpoch = crashEpoch;
    throw new Error("Preview child failed during render.");
  }
</script>

<Text>Recovered child content.</Text>
