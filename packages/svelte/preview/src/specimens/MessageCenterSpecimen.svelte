<script lang="ts">
  import { MessageCenter, type MessageCenterItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let items = $state<MessageCenterItem[]>([
    { id: "job-render", title: "Mix preview", message: "Rendering stems and automation…", meta: "Render queue · 60%", read: true, tone: "pending", progress: { value: 60 }, selectable: false, removable: false, readControl: false },
    { id: "job-upload", title: "Uploading stems", message: "Three of four stems transferred.", meta: "Cloud sync", read: true, tone: "info", progress: { value: null, indeterminate: true }, selectable: false, removable: false, readControl: false },
    { id: "render", title: "Render complete", message: "Mix preview 42 is ready for review.", meta: "Render queue", timestamp: Date.now() - 90_000, read: false, tone: "success", icon: "circle-check" },
    { id: "mention", title: "Ada mentioned you", message: "Can you check the automation pass before we print?", meta: "Mix room", timestamp: Date.now() - 720_000, read: false, tone: "info", icon: "user" },
    { id: "storage", title: "Storage nearing capacity", message: "Workspace media storage is at 86%.", meta: "System", timestamp: Date.now() - 7_200_000, read: true, tone: "warning", icon: "triangle-alert" },
    { id: "sync", title: "Project synced", message: "All workstation changes are available remotely.", meta: "Cloud sync", timestamp: Date.now() - 86_400_000, read: true, tone: "neutral" },
  ]);

  function setRead(id: string, read: boolean) {
    items = items.map((item) => item.id === id ? { ...item, read } : item);
  }
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-message-center-specimen">
      <SpecimenGroup label="Mixed activity feed">
        <div class="poodle-message-center-specimen__anchor">
          <MessageCenter
            {items}
            defaultOpen
            onItemSelect={(id) => console.log(id)}
            onReadChange={setRead}
            onRemove={(id) => items = items.filter((item) => item.id !== id)}
            onMarkAllRead={() => items = items.map((item) => ({ ...item, read: true }))}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Messaging language">
        <MessageCenter items={items.slice(2, 4)} title="Messages" triggerIcon="mail" />
      </SpecimenGroup>

      <SpecimenGroup label="Empty">
        <MessageCenter items={[]} title="Inbox" triggerIcon="inbox" />
      </SpecimenGroup>
    </div>
  {/snippet}

  <!-- Same gap HistoryCenter inherited from this specimen: SpecimenLayout
       advertises the axis tabs regardless, so omitting these left them empty. -->
  {#snippet sizes(size)}
    <MessageCenter {items} {size} />
  {/snippet}

  {#snippet densities(density)}
    <MessageCenter {items} {density} />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-message-center-specimen { display: grid; gap: 2rem; min-height: 38rem; }
  .poodle-message-center-specimen__anchor { display: flex; justify-content: flex-end; width: min(42rem, 100%); }
</style>
