import { useState } from "react";
import {
  Button,
  EmbedInput,
  EmbedPreview,
  InlineListSection,
  ListContainer,
  PickerShell,
  ResizeHandle,
  TextInput,
  type ParsedEmbed,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

function PickersShellsDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [parsed, setParsed] = useState<ParsedEmbed | null>(null);
  const [listPage, setListPage] = useState(1);
  const [panelWidth, setPanelWidth] = useState(240);

  return (
    <>
      <SpecimenSection title="EmbedInput + EmbedPreview">
        <EmbedInput
          id="embed-demo"
          parseDebounce={100}
          onParse={(nextParsed, error) => {
            setParsed(nextParsed);
            setLastEvent(error ? `embed:error:${error}` : `embed:${nextParsed?.provider ?? "none"}`);
          }}
        />
        <EmbedPreview parsed={parsed} />
      </SpecimenSection>

      <SpecimenSection title="PickerShell states">
        <PickerShell
          title="Choose assets"
          description="Pick from the media library"
          resultCount={12}
          selectionCount={2}
          statusText="12 results available"
          statusId="picker-status"
          toolbar={<TextInput ariaLabel="Filter assets" placeholder="Filter…" />}
        >
          <p data-testid="picker-body">Asset grid goes here.</p>
        </PickerShell>
        <PickerShell title="Loading picker" state="loading" stateTitle="Loading assets" stateMessage="Hold on…" />
        <PickerShell title="Empty picker" state="empty" stateTitle="No assets" stateMessage="Upload something first." />
      </SpecimenSection>

      <SpecimenSection title="InlineListSection">
        <InlineListSection
          title="Attachments"
          count={2}
          items={["design-spec.pdf", "logo.svg"]}
          item={(entry) => <span>{entry}</span>}
          actions={<Button size="sm" variant="ghost" onClick={() => setLastEvent("attachments:add")}>Add</Button>}
        />
        <InlineListSection title="Empty section" framed={false} items={[]} item={() => null} emptyMessage="Nothing attached." />
      </SpecimenSection>

      <SpecimenSection title="ListContainer">
        <ListContainer
          title="Projects"
          subtitle="All workspace projects"
          eyebrow="Workspace"
          state="ready"
          currentPage={listPage}
          totalPages={5}
          totalItems={98}
          pageSize={20}
          onPageChange={(page) => {
            setListPage(page);
            setLastEvent(`list:page:${page}`);
          }}
          actions={<Button size="sm">New project</Button>}
        >
          <p data-testid="list-content">Page {listPage} content</p>
        </ListContainer>
        <ListContainer title="Loading list" state="loading" />
        <ListContainer title="Broken list" state="error" errorMessage="Server unavailable." />
        <ListContainer title="Empty list" state="empty" emptyMessage="Create your first item." />
      </SpecimenSection>

      <SpecimenSection title="ResizeHandle">
        <div style={{ display: "flex", alignItems: "stretch", height: "6rem", border: "1px solid var(--poodle-color-border-subtle)" }}>
          <div style={{ width: `${panelWidth}px`, flexShrink: 0 }} data-testid="resize-panel">
            Panel {Math.round(panelWidth)}px
          </div>
          <ResizeHandle
            orientation="horizontal"
            ariaLabel="Resize panel"
            ariaValueNow={Math.round(panelWidth)}
            onResizeMove={(delta) => setPanelWidth((w) => Math.max(120, w + delta))}
            onResizeStep={(delta) => {
              setPanelWidth((w) => Math.max(120, w + delta));
              setLastEvent(`resize:step:${delta}`);
            }}
          />
          <div style={{ flex: 1 }}>Content</div>
        </div>
      </SpecimenSection>

      {lastEvent ? (
        <SpecimenSection title="Last event">
          <p data-testid="last-event">{lastEvent}</p>
        </SpecimenSection>
      ) : null}
    </>
  );
}

registerSpecimen({
  slug: "pickers-shells",
  title: "EmbedInput / PickerShell / ListContainer / ResizeHandle",
  render: () => <PickersShellsDemo />,
});
