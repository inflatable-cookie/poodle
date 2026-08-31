<script lang="ts">
  import type { DropIntent } from "@inflatable-cookie/poodle-core";

  import DragDropProvider from "../src/DragDropProvider.svelte";
  import Tabs from "../src/Tabs.svelte";
  import TabsSubjectCompositionTarget from "./TabsSubjectCompositionTarget.svelte";
  import type { TabItem } from "../src/types";

  /**
   * Two Tabs strips under one provider, with an accepting composite target
   * around the second one.
   *
   * `kind` decides whether the strips share a semantic family. With the
   * default (`null`) they are instance-scoped and mutually ineligible; with an
   * explicit shared kind, strip A's subject reaches strip B — and must fall
   * through B's own reorder targets to the composite that wraps it.
   */
  interface Props {
    kind?: string | null;
    left: TabItem[];
    right: TabItem[];
    onCompositeDrop?: (intent: DropIntent) => void;
    onLeftReorder?: (order: string[]) => void;
    onRightReorder?: (order: string[]) => void;
  }

  let {
    kind = null,
    left,
    right,
    onCompositeDrop = () => {},
    onLeftReorder,
    onRightReorder,
  }: Props = $props();
</script>

<DragDropProvider>
  <div data-testid="left-host">
    <Tabs
      items={left}
      value={left[0]?.value ?? null}
      reorderable
      dragSubjectKind={kind}
      ariaLabel="Left strip"
      onReorder={onLeftReorder}
    />
  </div>

  {#if kind === null}
    <div data-testid="right-host">
      <Tabs
        items={right}
        value={right[0]?.value ?? null}
        reorderable
        dragSubjectKind={kind}
        ariaLabel="Right strip"
        onReorder={onRightReorder}
      />
    </div>
  {:else}
    <TabsSubjectCompositionTarget {kind} onDropped={onCompositeDrop}>
      <div data-testid="right-host">
        <Tabs
          items={right}
          value={right[0]?.value ?? null}
          reorderable
          dragSubjectKind={kind}
          ariaLabel="Right strip"
          onReorder={onRightReorder}
        />
      </div>
    </TabsSubjectCompositionTarget>
  {/if}
</DragDropProvider>
