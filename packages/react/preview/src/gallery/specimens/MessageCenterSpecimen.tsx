import { useState } from "react";
import { MessageCenter, type MessageCenterItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const initialItems: MessageCenterItem[] = [
  { id: "job-render", title: "Mix preview", message: "Rendering stems and automation…", meta: "Render queue · 60%", read: true, tone: "pending", progress: { value: 60 }, selectable: false, removable: false, readControl: false },
  { id: "job-upload", title: "Uploading stems", message: "Three of four stems transferred.", meta: "Cloud sync", read: true, tone: "info", progress: { value: null, indeterminate: true }, selectable: false, removable: false, readControl: false },
  { id: "render", title: "Render complete", message: "Mix preview 42 is ready for review.", meta: "Render queue", timestamp: Date.now() - 90_000, read: false, tone: "success", icon: "circle-check" },
  { id: "mention", title: "Ada mentioned you", message: "Can you check the automation pass before we print?", meta: "Mix room", timestamp: Date.now() - 720_000, read: false, tone: "info", icon: "user" },
  { id: "storage", title: "Storage nearing capacity", message: "Workspace media storage is at 86%.", meta: "System", timestamp: Date.now() - 7_200_000, read: true, tone: "warning", icon: "triangle-alert" },
  { id: "sync", title: "Project synced", message: "All workstation changes are available remotely.", meta: "Cloud sync", timestamp: Date.now() - 86_400_000, read: true, tone: "neutral" },
];

export function MessageCenterSpecimen() {
  const [items, setItems] = useState(initialItems);

  return (
    // Same gap HistoryCenter inherited from this specimen: SpecimenLayout
    // advertises the axis tabs regardless, so omitting these left them empty.
    <SpecimenLayout
      bareVariants
      sizes={(size) => <MessageCenter items={items} size={size} />}
      densities={(density) => <MessageCenter items={items} density={density} />}
    >
      <div style={{ display: "grid", gap: "2rem", minHeight: "38rem" }}>
        <SpecimenGroup label="Mixed activity feed">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <MessageCenter
              items={items}
              defaultOpen
              onItemSelect={(id) => console.log(id)}
              onReadChange={(id, read) => setItems((current) => current.map((item) => item.id === id ? { ...item, read } : item))}
              onRemove={(id) => setItems((current) => current.filter((item) => item.id !== id))}
              onMarkAllRead={() => setItems((current) => current.map((item) => ({ ...item, read: true })))}
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
    </SpecimenLayout>
  );
}
