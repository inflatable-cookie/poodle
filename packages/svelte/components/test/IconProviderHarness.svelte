<script lang="ts">
  import Icon from "../src/Icon.svelte";
  import IconProvider from "../src/IconProvider.svelte";

  const icons = {
    rocket: [["path", { d: "M1 2 3 4" }]],
  };
</script>

<IconProvider {icons}>
  <Icon icon="rocket" ariaLabel="Rocket" />
</IconProvider>
