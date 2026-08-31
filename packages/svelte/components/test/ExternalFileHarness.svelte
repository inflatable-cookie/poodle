<script lang="ts">
  import type {
    DragExportBridge,
    InboundFileConstraints,
    InboundFileHostBridge,
  } from "@inflatable-cookie/poodle-core";

  import DragDropProvider from "../src/DragDropProvider.svelte";
  import ExternalFileSurface from "./ExternalFileSurface.svelte";

  interface Props {
    exportBridge?: DragExportBridge;
    inboundFileBridge?: InboundFileHostBridge;
    constraints?: InboundFileConstraints;
  }

  let { exportBridge, inboundFileBridge, constraints }: Props = $props();
</script>

<DragDropProvider {inboundFileBridge}>
  <ExternalFileSurface {exportBridge} {constraints} />
</DragDropProvider>
