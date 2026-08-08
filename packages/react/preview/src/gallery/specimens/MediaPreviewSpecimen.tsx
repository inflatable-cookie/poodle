import type { CSSProperties } from "react";
import { MediaPreview } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const variantStyle: CSSProperties = { width: "min(100%, 24rem)" };

const placeholderStyle: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  width: "100%",
  height: "100%",
  background: "var(--poodle-color-bg-subtle)",
  color: "var(--poodle-color-text-secondary)",
  fontSize: "0.875rem",
};

export function MediaPreviewSpecimen() {
  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <div style={variantStyle}>
          <MediaPreview
            title="Hero banner"
            description="Main landing page banner image for the product launch."
            eyebrow="Image"
            meta={["1920 × 1080", "245 KB", "PNG"]}
            kind="image"
            aspectRatio="landscape"
            size={size}
            mediaContent={<div style={placeholderStyle}>Image placeholder</div>}
          />
        </div>
      )}
      densities={(density) => (
        <div style={variantStyle}>
          <MediaPreview
            title="Hero banner"
            description="Main landing page banner image for the product launch."
            eyebrow="Image"
            meta={["1920 × 1080", "245 KB", "PNG"]}
            kind="image"
            aspectRatio="landscape"
            density={density}
            mediaContent={<div style={placeholderStyle}>Image placeholder</div>}
          />
        </div>
      )}
    >
      <div className="poodle-specimen" style={{ maxWidth: "24rem" }}>
        <SpecimenGroup label="Image preview" bare>
          <div style={variantStyle}>
            <MediaPreview
              title="Hero banner"
              description="Main landing page banner image for the product launch."
              eyebrow="Image"
              meta={["1920 × 1080", "245 KB", "PNG"]}
              kind="image"
              aspectRatio="landscape"
              mediaContent={<div style={placeholderStyle}>Image placeholder</div>}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Video preview" bare>
          <div style={variantStyle}>
            <MediaPreview
              title="Onboarding walkthrough"
              eyebrow="Video"
              meta={["3:42", "48 MB"]}
              kind="video"
              aspectRatio="video"
              mediaContent={<div style={placeholderStyle}>Video placeholder</div>}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Error state" bare>
          <div style={variantStyle}>
            <MediaPreview
              title="Corrupted file"
              kind="document"
              state="error"
              stateTitle="Preview unavailable"
              stateMessage="This file cannot be previewed."
              aspectRatio="landscape"
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
