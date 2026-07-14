import { EmbedPreview } from "@poodle/react";
import type { ParsedEmbed } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function EmbedPreviewSpecimen() {
  const youtubeParsed: ParsedEmbed = {
    provider: "youtube",
    id: "dQw4w9WgXcQ",
    originalUrl: "https://youtube.com/watch?v=dQw4w9WgXcQ",
  };

  const vimeoParsed: ParsedEmbed = {
    provider: "vimeo",
    id: "76979871",
    originalUrl: "https://vimeo.com/76979871",
  };

  const trustedAudioHtml = '<iframe title="Audio embed" src="about:blank"></iframe>';

  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="YouTube embed">
        <EmbedPreview parsed={youtubeParsed} />
      </SpecimenGroup>

      <SpecimenGroup label="Vimeo embed">
        <EmbedPreview parsed={vimeoParsed} />
      </SpecimenGroup>

      <SpecimenGroup label="Trusted raw embed">
        <EmbedPreview trustedHtml={trustedAudioHtml} aspectRatio="auto" />
      </SpecimenGroup>

      <SpecimenGroup label="Loading state">
        <EmbedPreview loading />
      </SpecimenGroup>

      <SpecimenGroup label="Error state">
        <EmbedPreview error="Failed to load embed. The URL may be invalid or the provider is unavailable." />
      </SpecimenGroup>

      <SpecimenGroup label="Empty state">
        <EmbedPreview emptyMessage="Paste a URL above to see a preview" />
      </SpecimenGroup>
    </div>
  );
}
