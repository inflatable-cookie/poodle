<script lang="ts">
  import { Code, Eyebrow, Field, Surface, TokenInput } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let tags = $state(["ifrs", "tax", "audit"]);
  let workflowTags = $state(["draft", "review"]);
  let longTags = $state([
    "legacy-migration",
    "a-very-long-tag-that-should-wrap-inside-the-token-without-breaking-the-field",
  ]);
</script>

<SpecimenLayout>
  <Surface tone="panel" border="subtle" padding="md">
    <div class="poodle-specimen">
      <div class="poodle-specimen__item">
        <Eyebrow>Default</Eyebrow>
        <div class="poodle-specimen__control">
          <Field
            id="blog-tags"
            label="Tags"
            description="Type a tag, then use comma, enter, tab, or blur to commit it."
          >
            <TokenInput
              id="blog-tags"
              name="tags"
              bind:values={tags}
              placeholder="Type a tag…"
            />
          </Field>
        </div>
        <Code source={JSON.stringify(tags)} inline={false} inlineVariant="plain" />
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Multiple separators</Eyebrow>
        <div class="poodle-specimen__control">
          <Field
            id="workflow-tags"
            label="Workflow tags"
            description="Supports comma and semicolon separators."
          >
            <TokenInput
              id="workflow-tags"
              bind:values={workflowTags}
              separators={[",", ";"]}
              placeholder="draft; review; live"
            />
          </Field>
        </div>
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Narrow and long values</Eyebrow>
        <div class="poodle-specimen__control poodle-specimen__control--narrow">
          <Field
            id="long-tags"
            label="Migration labels"
            description="Long tokens wrap inside the field instead of overflowing."
          >
            <TokenInput
              id="long-tags"
              bind:values={longTags}
              placeholder="Type a label..."
            />
          </Field>
        </div>
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Read only</Eyebrow>
        <div class="poodle-specimen__control">
          <Field id="readonly-tags" label="Locked tags">
            <TokenInput
              id="readonly-tags"
              values={["premium", "acca", "2027"]}
              readOnly
            />
          </Field>
        </div>
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Disabled</Eyebrow>
        <div class="poodle-specimen__control">
          <Field id="disabled-tags" label="Disabled tags">
            <TokenInput
              id="disabled-tags"
              values={["finance", "ethics"]}
              disabled
            />
          </Field>
        </div>
      </div>
    </div>
  </Surface>

  {#snippet sizes(size)}
    <div class="poodle-specimen__control">
      <TokenInput
        id={"size-" + size}
        values={["alpha", "beta"]}
        {size}
        placeholder={size.toUpperCase()}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__control">
      <TokenInput
        id={"density-" + density}
        values={["audit", "reporting"]}
        {density}
        placeholder="Type a tag…"
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .poodle-specimen__item {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .poodle-specimen__control {
    width: 100%;
    max-width: 40rem;
  }

  .poodle-specimen__control--narrow {
    max-width: 18rem;
  }
</style>
