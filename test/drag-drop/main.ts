import {
  createDragDropController,
  type DragDropCommitResult,
} from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/drag-drop.css";

const root = document.getElementById("root") as HTMLElement;
const source = document.getElementById("source") as HTMLButtonElement;
const target = document.getElementById("target") as HTMLElement;
const probe = document.getElementById("probe") as HTMLElement;
const overlay = document.getElementById("overlay") as HTMLElement;
const live = document.getElementById("live") as HTMLElement;
const shift = document.getElementById("shift") as HTMLButtonElement;

const originalCapture = source.setPointerCapture.bind(source);
source.setPointerCapture = (id: number) => {
  source.dataset.probeCaptured = "true";
  probe.dataset.captured = "true";
  originalCapture(id);
};

const controller = createDragDropController();
controller.connect(root);

controller.registerSource(source, {
  sourceId: "alpha",
  subject: { kind: "item", id: "alpha" },
  allowedOperations: ["move"],
  label: "Alpha",
});

controller.registerTarget(target, {
  targetId: "list",
  acceptedKinds: ["item"],
  label: "List",
  resolvePosition: () => "inside",
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: (): DragDropCommitResult => ({ status: "committed" }),
});

function paint(): void {
  const snapshot = controller.getSnapshot();
  probe.dataset.phase = snapshot.phase;
  probe.dataset.target = snapshot.targetId ?? "";
  probe.dataset.posture = snapshot.targetPosture ?? "";
  probe.dataset.captured = source.dataset.probeCaptured ?? "false";
  probe.textContent = `${snapshot.phase} ${snapshot.targetId ?? ""} ${snapshot.announcement ?? ""}`;

  overlay.replaceChildren();
  if (snapshot.preview) {
    const preview = document.createElement("div");
    preview.className = "poodle-drag-preview";
    preview.dataset.testid = "preview";
    preview.textContent = snapshot.preview.label;
    preview.style.left = `${snapshot.preview.x}px`;
    preview.style.top = `${snapshot.preview.y}px`;
    overlay.append(preview);
  }
  live.textContent = snapshot.announcement ?? "";
}

controller.subscribe(paint);
paint();

shift.addEventListener("click", () => {
  target.style.transform = "translate(96px, 0)";
  controller.invalidateLayout();
});

(window as unknown as { __poodleDrag: typeof controller }).__poodleDrag = controller;
