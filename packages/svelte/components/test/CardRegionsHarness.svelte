<script lang="ts">
  import Card from "../src/Card.svelte";
  import { createRawSnippet } from "svelte";

  interface Props {
    headerText?: string;
    bodyText?: string;
    footerText?: string;
    showMedia?: boolean;
    showHeader?: boolean;
    showFooter?: boolean;
  }

  let {
    headerText = "",
    bodyText = "",
    footerText = "",
    showMedia = false,
    showHeader = false,
    showFooter = false,
  }: Props = $props();

  // Raw snippets keep the branded Snippet type through the conditional prop
  // ternary (plain `{#snippet}` declarations widen it away), and they
  // materialize real text under the test DOM where a plain thunk renders as a
  // comment node.
  const mediaContentSnippet = createRawSnippet(() => ({
    render: () => `<img src="/hero.png" alt="hero" />`,
  }));
  const headerSnippet = createRawSnippet(() => ({
    render: () => `<span class="harness-header">${headerText}</span>`,
  }));
  const bodySnippet = createRawSnippet(() => ({
    render: () => `<span class="harness-body">${bodyText}</span>`,
  }));
  const footerSnippet = createRawSnippet(() => ({
    render: () => `<span class="harness-footer">${footerText}</span>`,
  }));
</script>

<Card
  media={showMedia}
  mediaContent={showMedia ? mediaContentSnippet : undefined}
  header={showHeader ? headerSnippet : undefined}
  footer={showFooter ? footerSnippet : undefined}
>
  {@render bodySnippet()}
</Card>
