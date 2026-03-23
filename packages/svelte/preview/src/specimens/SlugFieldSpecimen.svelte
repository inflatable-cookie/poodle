<script lang="ts">
  import { SlugField } from "@poodle/svelte-composites";
  import { Eyebrow, TextInput, Field } from "@poodle/svelte-primitives";

  let articleTitle = "My New Blog Post";
  let slug = "";
  let isManual = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Auto-generated from title</Eyebrow>
    <Field id="title-input" label="Title">
      <TextInput
        id="title-input"
        bind:value={articleTitle}
        placeholder="Enter a title"
      />
    </Field>
    <SlugField
      id="slug-input"
      source={articleTitle}
      bind:value={slug}
      bind:isManualOverride={isManual}
    />
    <p class="result">Slug: <code>{slug || "—"}</code></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>With custom prefix</Eyebrow>
    <SlugField
      id="slug-prefix"
      source="Products Page"
      prefix="/products/"
      description="Full URL path for this product page."
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <SlugField
      id="slug-disabled"
      source="Fixed content"
      isDisabled
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .result {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }

  .result code {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: color-mix(in srgb, var(--poodle-color-background-panel) 72%, var(--poodle-color-background-elevated));
  }
</style>
