import { useState } from "react";
import {
  AudioPlayer,
  Button,
  MediaBrowsePanel,
  MediaPicker,
  MediaPreview,
  MediaThumbnail,
  VideoPlayer,
  type MediaPickerItem,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

// 1x1 transparent PNG
const tinyPng =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";

const browseItems: MediaPickerItem[] = [
  { id: "m1", label: "hero.png", thumbnailUrl: tinyPng, kind: "image", meta: "1.2 MB" },
  { id: "m2", label: "intro.mp4", kind: "video", meta: "48 MB" },
  { id: "m3", label: "podcast.mp3", kind: "audio", meta: "12 MB" },
  { id: "m4", label: "spec.pdf", kind: "pdf", meta: "300 KB" },
];

function MediaDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [pickerOpen, setPickerOpen] = useState(false);
  const [loadCount, setLoadCount] = useState(0);

  return (
    <>
      <SpecimenSection title="MediaThumbnail states">
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(10rem, 1fr))", gap: "0.75rem" }}>
          <MediaThumbnail kind="image" title="Placeholder" meta="No content" />
          <MediaThumbnail kind="video" title="Video kind" badge="4K" />
          <MediaThumbnail kind="image" state="loading" title="Loading" />
          <MediaThumbnail kind="image" state="error" title="Broken" stateMessage="Could not load." />
        </div>
      </SpecimenSection>

      <SpecimenSection title="MediaPreview">
        <div style={{ maxWidth: "24rem" }}>
          <MediaPreview
            title="Launch teaser"
            description="30-second cut for social"
            eyebrow="Campaign"
            caption="Final color pass pending."
            meta={["48 MB", "16:9"]}
            badge="Draft"
            kind="video"
            mediaContent={<img src={tinyPng} alt="" style={{ width: "100%", height: "100%", objectFit: "cover" }} />}
          />
        </div>
      </SpecimenSection>

      <SpecimenSection title="AudioPlayer">
        <AudioPlayer src="data:audio/wav;base64,UklGRiQAAABXQVZFZm10IBAAAAABAAEAQB8AAIA+AAACABAAZGF0YQAAAAA=" showSpeedControl />
      </SpecimenSection>

      <SpecimenSection title="VideoPlayer">
        <div style={{ maxWidth: "28rem" }}>
          <VideoPlayer src="data:video/mp4;base64,AAAA" poster={tinyPng} ariaLabel="Demo video" />
        </div>
      </SpecimenSection>

      <SpecimenSection title="MediaBrowsePanel">
        <MediaBrowsePanel
          items={browseItems}
          hasMore
          onSelect={(item) => setLastEvent(`browse:${item.id}`)}
          onLoadMore={() => {
            setLoadCount((count) => count + 1);
            setLastEvent(`browse:load-more:${loadCount + 1}`);
          }}
        />
        <MediaBrowsePanel items={[]} emptyMessage="Library is empty." />
        <MediaBrowsePanel items={[]} error="Media service unreachable." />
      </SpecimenSection>

      <SpecimenSection title="MediaPicker">
        <Button onClick={() => setPickerOpen(true)}>Open media picker</Button>
        <MediaPicker
          open={pickerOpen}
          items={browseItems}
          onSelect={(item) => setLastEvent(`picker:${item.id}`)}
          onOpenChange={setPickerOpen}
        />
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
  slug: "media",
  title: "Media players + pickers",
  render: () => <MediaDemo />,
});
