import {
  createDragDropController,
  type DragDropCommitResult,
} from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/drag-drop.css";

const root = document.getElementById("root") as HTMLElement;
const source = document.getElementById("source") as HTMLButtonElement;
const target = document.getElementById("target") as HTMLElement;
const linkSource = document.getElementById("link-source") as HTMLAnchorElement;
const probe = document.getElementById("probe") as HTMLElement;
const overlay = document.getElementById("overlay") as HTMLElement;
const live = document.getElementById("live") as HTMLElement;
const shift = document.getElementById("shift") as HTMLButtonElement;
const outerScroll = document.getElementById("outer-scroll") as HTMLElement;
const innerScroll = document.getElementById("inner-scroll") as HTMLElement;
const nestedSource = document.getElementById("nested-source") as HTMLButtonElement;
const nestedTarget = document.getElementById("nested-target") as HTMLElement;

source.addEventListener("gotpointercapture", (event) => {
  const pointer = event as PointerEvent;
  probe.dataset.captured = "true";
  probe.dataset.captureId = String(pointer.pointerId);
});
source.addEventListener("lostpointercapture", () => {
  probe.dataset.captured = "false";
});

const controller = createDragDropController();
controller.connect(root);

controller.registerSource(source, {
  sourceId: "alpha",
  subject: { kind: "item", id: "alpha" },
  allowedOperations: ["move"],
  label: "Alpha",
  keyboardOrder: 0,
});

controller.registerSource(linkSource, {
  sourceId: "link-alpha",
  subject: { kind: "item", id: "link-alpha" },
  allowedOperations: ["move"],
  label: "Link source",
});

controller.registerTarget(target, {
  targetId: "list",
  acceptedKinds: ["item"],
  label: "List",
  resolvePosition: () => "inside",
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: (intent): DragDropCommitResult => {
    probe.dataset.drop = `${intent.targetId}:${intent.position}:${intent.operation}`;
    return { status: "committed" };
  },
});

function paint(): void {
  const snapshot = controller.getSnapshot();
  probe.dataset.phase = snapshot.phase;
  probe.dataset.target = snapshot.targetId ?? "";
  probe.dataset.posture = snapshot.targetPosture ?? "";
  probe.dataset.outerScroll = String(outerScroll.scrollTop);
  probe.dataset.innerScroll = String(innerScroll.scrollTop);
  const captured = [0, 1, 2, 3, 4, 5, 6, 7].some((id) => source.hasPointerCapture(id));
  probe.dataset.captured = captured ? "true" : "false";
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

controller.registerSource(nestedSource, {
  sourceId: "nested-alpha",
  subject: { kind: "item", id: "nested-alpha" },
  allowedOperations: ["move"],
  label: "Nested Alpha",
  keyboardOrder: 0,
});

controller.registerTarget(nestedTarget, {
  targetId: "nested-list",
  acceptedKinds: ["item"],
  label: "Nested list",
  autoScroll: true,
  resolvePosition: () => "inside",
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: (intent): DragDropCommitResult => {
    probe.dataset.drop = `${intent.targetId}:${intent.position}:${intent.operation}`;
    return { status: "committed" };
  },
});

shift.addEventListener("click", () => {
  target.style.minHeight = "160px";
  target.style.width = "280px";
});

(window as unknown as { __poodleDrag: typeof controller }).__poodleDrag = controller;
