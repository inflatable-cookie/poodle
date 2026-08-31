/**
 * A curated external-file surface: one drop zone for files arriving from
 * outside the application, one row that can be dragged out to it.
 *
 * The React half of the same specimen. It shows the states worth showing — a
 * zone being offered files, one refusing them with a reason, and an export
 * moving through preparing, armed, dragging, and its ending — and nothing here
 * has any idea what a path, a `File`, or a shell is. Exhaustive transport and
 * validation cases live in the controller tests.
 */
import { useState } from "react";

import {
  INBOUND_FILE_SUBJECT_KIND,
  type DragExportBridge,
  type DropCommitContext,
  type InboundFileConstraints,
} from "@inflatable-cookie/poodle-core";

import { useDragDrop, useDragSource, useDropTarget } from "../src/drag-drop";

interface Props {
  exportBridge?: DragExportBridge;
  constraints?: InboundFileConstraints;
}

function displayNames(files: readonly { name: string | null }[] | undefined): string[] {
  return (files ?? []).map((file) => file.name).filter((name): name is string => name !== null);
}

function ExportableClip({ exportBridge }: { exportBridge?: DragExportBridge }) {
  const { snapshot } = useDragDrop();
  const { getSourceProps } = useDragSource({
    sourceId: "clip-1",
    subject: { kind: "clip", id: "clip-1" },
    allowedOperations: ["copy"],
    label: "Intro clip",
    fileExportBridge: exportBridge,
  });

  return (
    <div className="poodle-drag-drop-item" data-testid="clip" {...getSourceProps()}>
      Intro clip
      <span data-testid="export-state">{snapshot.fileExport?.state ?? "idle"}</span>
      {snapshot.fileExport?.displayName ? (
        <span data-testid="export-name">{snapshot.fileExport.displayName}</span>
      ) : null}
    </div>
  );
}

export function ExternalFileSurface({
  exportBridge,
  constraints = { accept: "audio/*", maxFiles: 4 },
}: Props) {
  const { snapshot } = useDragDrop();
  const [accepted, setAccepted] = useState<string[]>([]);

  // Display names only. The receipts the host issued are opaque, and the
  // surface never asks for anything more than something to render.
  const offered = displayNames(snapshot.inboundFiles?.files);

  const { getTargetProps } = useDropTarget({
    targetId: "library",
    acceptedKinds: [INBOUND_FILE_SUBJECT_KIND],
    label: "Sample library",
    inboundFiles: constraints,
    resolvePosition: () => "inside",
    canDrop: (intent) => ({ accepted: true, intent }),
    onDrop: (_intent, context: DropCommitContext) => {
      setAccepted(displayNames(context.inboundFiles?.files));
      return { status: "committed" };
    },
  });

  return (
    <>
      <section className="poodle-drag-drop-list" data-testid="library" {...getTargetProps()}>
        <h3>Sample library</h3>
        {snapshot.targetPosture === "accepted" ? (
          <p data-testid="library-state">Drop {offered.length || "files"} here</p>
        ) : snapshot.targetPosture === "rejected" ? (
          <p data-testid="library-state">Cannot take these files: {snapshot.rejectedReason}</p>
        ) : (
          <p data-testid="library-state">Drop audio files here</p>
        )}
        <ul data-testid="library-files">
          {accepted.map((name) => (
            <li key={name}>{name}</li>
          ))}
        </ul>
      </section>
      <ExportableClip exportBridge={exportBridge} />
      <div data-testid="announcement">{snapshot.announcement ?? ""}</div>
    </>
  );
}
