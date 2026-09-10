<script lang="ts">
  import CodeEditor from "../../src/CodeEditor.svelte";

  interface Props {
    value?: string;
  }

  let { value = "const x = 1;" }: Props = $props();
</script>

<CodeEditor {value} language="typescript" />
