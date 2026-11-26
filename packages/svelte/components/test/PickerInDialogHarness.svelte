<script lang="ts">
  import ColorPicker from "../src/ColorPicker.svelte";
  import DatePicker from "../src/DatePicker.svelte";
  import DateRangePicker from "../src/DateRangePicker.svelte";
  import DateTimePicker from "../src/DateTimePicker.svelte";
  import DateTimeRangePicker from "../src/DateTimeRangePicker.svelte";
  import Dialog from "../src/Dialog.svelte";

  interface Props {
    picker: "date" | "date-range" | "date-time" | "date-time-range" | "color";
  }

  let { picker }: Props = $props();
  let dialogOpen = $state(true);
</script>

<div data-poodle-theme-root style="z-index: 0;">
  <Dialog bind:open={dialogOpen} title="Schedule" dismissOnOutsideInteract>
    {#if picker === "date"}
      <DatePicker defaultValue="2026-03-14" />
    {:else if picker === "date-range"}
      <DateRangePicker defaultValue={{ start: "2026-03-14", end: "2026-03-18" }} />
    {:else if picker === "date-time"}
      <DateTimePicker defaultValue={{ date: "2026-03-14", time: "10:00" }} />
    {:else if picker === "date-time-range"}
      <DateTimeRangePicker
        defaultValue={{
          start: { date: "2026-03-14", time: "10:00" },
          end: { date: "2026-03-18", time: "14:00" },
        }}
      />
    {:else}
      <ColorPicker value="#6366f1" />
    {/if}
  </Dialog>
</div>
