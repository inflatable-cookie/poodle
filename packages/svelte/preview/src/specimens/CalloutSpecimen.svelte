<script lang="ts">
  import { Callout, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Tones</Eyebrow>
    <Callout tone="neutral" title="Neutral callout">
      This is a general informational message with no specific severity.
    </Callout>
    <Callout tone="info" title="Info">
      Your changes have been saved and will take effect on next deploy.
    </Callout>
    <Callout tone="success" title="Success">
      All tests passed. The build is ready for production.
    </Callout>
    <Callout tone="warning" title="Warning">
      This API key expires in 7 days. Rotate it to avoid service interruption.
    </Callout>
    <Callout tone="danger" title="Error">
      Unable to connect to the database. Check your credentials and try again.
    </Callout>
  </div>

  <div class="specimen__group">
    <Eyebrow>Message prop</Eyebrow>
    <Callout tone="info" title="Information" message="This is an informational callout using the message prop instead of slot content." />
  </div>

  <div class="specimen__group">
    <Eyebrow>Dismissible</Eyebrow>
    <Callout
      tone="info"
      title="Dismissible callout"
      message="This callout can be dismissed by the user."
      isDismissible
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Without title</Eyebrow>
    <Callout tone="info">
      A simple inline callout without a title for brief contextual notes.
    </Callout>
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
    gap: 0.75rem;
  }
</style>
