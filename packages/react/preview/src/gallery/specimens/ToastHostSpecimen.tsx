import { useMemo, type ReactNode } from "react";
import { ToastHost, Button, type ToastHostStoreItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

type ToneSeed = "info" | "success" | "warning" | "error";
const seedTones: ToneSeed[] = ["info", "success", "warning", "error"];

function makeToastStore() {
  let nextId = 4;
  let items: ToastHostStoreItem[] = [
    { id: "1", variant: "success", title: "Saved", message: "Your changes have been stored." },
    { id: "2", variant: "warning", title: "Retry later", message: "Background sync is delayed." },
    { id: "3", variant: "error", message: "Publishing failed. Check your connection." },
  ];
  const subs = new Set<(items: ToastHostStoreItem[]) => void>();
  const emit = () => subs.forEach((run) => run([...items]));

  return {
    toasts: {
      subscribe(run: (items: ToastHostStoreItem[]) => void) {
        subs.add(run);
        run([...items]);
        return () => subs.delete(run);
      },
    },
    dismiss(id: string) {
      items = items.filter((item) => item.id !== id);
      emit();
    },
    push() {
      const variant = seedTones[nextId % seedTones.length];
      const id = String(nextId++);
      items = [
        ...items,
        {
          id,
          variant,
          title: variant === "error" ? undefined : `Toast #${id}`,
          message: variant === "error" ? "This one stays until you dismiss it." : "A new runtime-host toast was added.",
        },
      ];
      emit();
    },
  };
}

export function ToastHostSpecimen() {
  const store = useMemo(() => makeToastStore(), []);

  const surface = (node: ReactNode) => (
    <div
      data-toast-surface
      style={{
        position: "relative",
        minHeight: "16rem",
        border: "1px dashed color-mix(in srgb, var(--poodle-color-border-default) 82%, transparent)",
        borderRadius: "var(--poodle-radius-surface)",
        background: "color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent)",
      }}
    >
      <style>{`[data-toast-surface] .poodle-toast-host { position: absolute; }`}</style>
      {node}
    </div>
  );

  return (
    <SpecimenLayout
      sizes={(size) => surface(<ToastHost store={store} size={size} />)}
      densities={(density) => surface(<ToastHost store={store} density={density} />)}
    >
      <SpecimenGroup label="Runtime host">
        <p style={{ margin: 0, color: "var(--poodle-color-text-secondary)" }}>
          The host owns timer policy and fixed positioning while `ToastStack` stays presentational.
        </p>
        <Button variant="secondary" onClick={() => store.push()}>Add toast</Button>
      </SpecimenGroup>

      {surface(<ToastHost store={store} />)}
    </SpecimenLayout>
  );
}
