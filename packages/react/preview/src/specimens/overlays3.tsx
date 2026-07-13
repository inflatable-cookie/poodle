import { useMemo, useState } from "react";
import {
  AlertDialog,
  Button,
  FormActions,
  HoverCard,
  Text,
  ToastHost,
  type ToastHostStoreItem,
} from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function HoverCardSpecimen() {
  return (
    <SpecimenSection title="HoverCard">
      <Row>
        <HoverCard trigger={<span>@poodle</span>} ariaLabel="Profile preview">
          <div style={{ padding: "0.75rem" }}>
            <Text>Poodle — production UI components</Text>
          </div>
        </HoverCard>
      </Row>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "hover-card", title: "HoverCard", render: () => <HoverCardSpecimen /> });

function AlertDialogSpecimen() {
  const [open, setOpen] = useState(false);
  const [confirmed, setConfirmed] = useState(0);
  return (
    <SpecimenSection title="AlertDialog">
      <Row>
        <Button variant="secondary" tone="danger" onClick={() => setOpen(true)}>
          Delete item
        </Button>
        <span data-testid="alert-confirms">confirms: {confirmed}</span>
      </Row>
      <AlertDialog
        open={open}
        title="Delete this item?"
        description="This cannot be undone."
        itemLabel="Item"
        itemValue="Quarterly report"
        confirmLabel="Delete"
        onConfirm={() => setConfirmed((n) => n + 1)}
        onOpenChange={setOpen}
      />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "alert-dialog", title: "AlertDialog", render: () => <AlertDialogSpecimen /> });

function FormActionsSpecimen() {
  const [last, setLast] = useState("");
  return (
    <SpecimenSection title="FormActions">
      <FormActions
        dangerItems={[
          { label: "Archive", onSelect: () => setLast("archive") },
          { label: "Delete", onSelect: () => setLast("delete") },
        ]}
      >
        <Button variant="ghost">Cancel</Button>
        <Button variant="primary">Save</Button>
      </FormActions>
      <span data-testid="form-actions-last">last: {last}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "form-actions", title: "FormActions", render: () => <FormActionsSpecimen /> });

function makeToastStore() {
  let items: ToastHostStoreItem[] = [];
  const subs = new Set<(items: ToastHostStoreItem[]) => void>();
  const emit = () => subs.forEach((s) => s([...items]));
  return {
    toasts: {
      subscribe(run: (items: ToastHostStoreItem[]) => void) {
        subs.add(run);
        run([...items]);
        return () => subs.delete(run);
      },
    },
    dismiss(id: string) {
      items = items.filter((t) => t.id !== id);
      emit();
    },
    push(item: ToastHostStoreItem) {
      items = [...items, item];
      emit();
    },
  };
}

function ToastSpecimen() {
  const store = useMemo(() => makeToastStore(), []);
  const [n, setN] = useState(0);
  return (
    <SpecimenSection title="ToastHost">
      <Row>
        <Button
          variant="secondary"
          onClick={() => {
            setN((v) => v + 1);
            store.push({ id: `t${n}`, title: `Saved ${n}`, message: "All changes stored", tone: "success" });
          }}
        >
          Push toast
        </Button>
        <Button
          variant="ghost"
          onClick={() => {
            setN((v) => v + 1);
            store.push({ id: `d${n}`, title: "Failed", message: "Sticky danger toast", tone: "danger" });
          }}
        >
          Push sticky
        </Button>
      </Row>
      <ToastHost store={store} autoDismissMs={1500} />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "toast-host", title: "ToastHost", render: () => <ToastSpecimen /> });
