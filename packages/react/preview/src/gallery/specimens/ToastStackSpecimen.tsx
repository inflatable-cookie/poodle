import { useState } from "react";
import { Button, ToastStack, type ToastItem } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const variantStyle = { width: "min(100%, 24rem)" } as const;

export function ToastStackSpecimen() {
  const [nextId, setNextId] = useState(4);
  const [items, setItems] = useState<ToastItem[]>([
    { id: "1", title: "Changes saved", message: "Your settings have been updated.", tone: "success" },
    { id: "2", title: "New version available", message: "Update to v2.1 for the latest features.", tone: "info", actionLabel: "Update" },
    { id: "3", title: "Rate limit warning", message: "You are approaching your API limit.", tone: "warning" },
  ]);

  const addToast = () => {
    const tones: Array<"info" | "success" | "warning" | "danger"> = ["info", "success", "warning", "danger"];
    const tone = tones[nextId % tones.length];
    setItems((prev) => [
      ...prev,
      { id: String(nextId), title: `Notification #${nextId}`, message: "This is a new toast message.", tone },
    ]);
    setNextId((id) => id + 1);
  };

  return (
    <SpecimenLayout
      showSizes
      showDensities
      bareVariants
      sizes={(size) => (
        <div style={variantStyle}>
          <ToastStack
            items={[
              { id: `${size}-1`, title: `Toast at ${size}`, message: "Chrome scales with size.", tone: "info" },
              { id: `${size}-2`, title: "Action available", message: "Dismiss and action controls follow the same ladder.", tone: "success", actionLabel: "View" },
            ]}
            size={size}
          />
        </div>
      )}
      densities={(density) => (
        <div style={variantStyle}>
          <ToastStack
            items={[
              { id: `${density}-1`, title: "Density example", message: "Spacing changes between compact, default, and comfortable.", tone: "warning" },
              { id: `${density}-2`, title: "Retry failed", message: "Action row and body spacing should ladder correctly.", tone: "danger", actionLabel: "Retry" },
            ]}
            density={density}
          />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Interactive stack" bare>
          <div style={{ display: "flex", gap: "0.5rem", flexWrap: "wrap" }}>
            <Button variant="secondary" sizeRole="chrome" onClick={addToast}>Add toast</Button>
          </div>
          <div style={variantStyle}>
            <ToastStack
              items={items}
              onDismiss={(id) => setItems((prev) => prev.filter((item) => item.id !== id))}
              onAction={(id) => setItems((prev) => prev.filter((item) => item.id !== id))}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
