import { AudioPlayer } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const src = "https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3";

export function AudioPlayerSpecimen() {
  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <div style={{ width: "min(100%, 36rem)" }}>
          <AudioPlayer src={src} ariaLabel={`Audio player at ${size}`} showSpeedControl size={size} />
        </div>
      )}
      densities={(density) => (
        <div style={{ width: "min(100%, 36rem)" }}>
          <AudioPlayer src={src} ariaLabel={`Audio player at ${density} density`} showSpeedControl density={density} />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Basic audio player" bare>
          <AudioPlayer src={src} ariaLabel="T-Rex roar audio" />
        </SpecimenGroup>

        <SpecimenGroup label="With speed control" bare>
          <AudioPlayer src={src} ariaLabel="Audio with speed control" showSpeedControl />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
