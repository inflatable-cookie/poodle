<script lang="ts">
  import { Banner, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Tones</Eyebrow>
    <div class="specimen__stack">
      <Banner tone="info" title="Information" message="This is an informational banner with helpful context." />
      <Banner tone="success" title="Success" message="The operation completed successfully." />
      <Banner tone="warning" title="Warning" message="This action may have unintended consequences." />
      <Banner tone="danger" title="Error" message="Something went wrong. Please try again." />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Dismissible</Eyebrow>
    <Banner
      tone="info"
      title="Dismissible banner"
      message="This banner can be dismissed by the user."
      isDismissible
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

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
