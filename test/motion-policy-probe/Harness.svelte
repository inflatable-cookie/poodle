<script lang="ts">
  import Collapsible from "../../packages/svelte/components/src/Collapsible.svelte";
  import IconButton from "../../packages/svelte/components/src/IconButton.svelte";
  import MotionPolicyProvider from "../../packages/svelte/components/src/MotionPolicyProvider.svelte";
  import Tabs from "../../packages/svelte/components/src/Tabs.svelte";
  import ToastStack from "../../packages/svelte/components/src/ToastStack.svelte";

  let toastItems = $state([
    { id: "keep", title: "Kept" },
    { id: "danger", title: "Danger", tone: "danger" as const },
  ]);
  let tab = $state("a");

  function dismiss(id: string) {
    toastItems = toastItems.filter((item) => item.id !== id);
  }
</script>

<section data-framework="svelte">
  <div data-case="disclosure">
    <Collapsible title="Details" defaultOpen>
      Disclosure body that must keep height while closing.
    </Collapsible>
  </div>

  <div data-case="tabs" style="width: 24rem;">
    <Tabs
      items={[
        { value: "a", label: "Alpha" },
        { value: "b", label: "Beta longer label" },
      ]}
      value={tab}
      activeEdge="underline"
      fullWidth
      onValueChange={(value) => {
        tab = value;
      }}
    />
  </div>

  <div data-case="toast">
    <button type="button" data-entered-from>svelte outside</button>
    <ToastStack items={toastItems} onDismiss={dismiss} />
  </div>

  <div data-case="icon-reduced">
    <MotionPolicyProvider policy="reduced">
      <IconButton icon="star" ariaLabel="Star" />
    </MotionPolicyProvider>
  </div>
</section>
