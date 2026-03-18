<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { HostedSurface } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ready state</Eyebrow>
    <div class="specimen__frame">
      <HostedSurface
        surfaceId="preview"
        title="Preview"
        state="ready"
        isReloadable
        isDetachable
        isClosable
      >
        <div class="specimen__content">
          <p>Hosted content goes here.</p>
        </div>
      </HostedSurface>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading state</Eyebrow>
    <div class="specimen__frame">
      <HostedSurface
        surfaceId="loading-example"
        title="External Content"
        state="loading"
      >
        <div class="specimen__content">
          <p>Content loading...</p>
        </div>
      </HostedSurface>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Unavailable state</Eyebrow>
    <div class="specimen__frame">
      <HostedSurface
        surfaceId="unavailable-example"
        title="Remote Service"
        state="unavailable"
        stateMessage="Service is temporarily unavailable."
        isReloadable
      >
        <div class="specimen__content">
          <p>Hidden content</p>
        </div>
      </HostedSurface>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Degraded state</Eyebrow>
    <div class="specimen__frame">
      <HostedSurface
        surfaceId="degraded-example"
        title="Analytics"
        state="degraded"
        stateMessage="Some features unavailable"
        isReloadable
      >
        <div class="specimen__content">
          <p>Content with limited functionality.</p>
        </div>
      </HostedSurface>
    </div>
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

  .specimen__frame {
    height: 10rem;
  }

  .specimen__content {
    padding: 1rem;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
  }

  .specimen__content p {
    margin: 0;
  }
</style>
