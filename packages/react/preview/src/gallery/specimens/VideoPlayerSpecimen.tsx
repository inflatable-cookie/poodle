import { VideoPlayer } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function VideoPlayerSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <VideoPlayer
          src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
          ariaLabel="Sample video"
          size={size}
        />
      )}
      densities={(density) => (
        <VideoPlayer
          src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
          ariaLabel="Sample video"
          density={density}
        />
      )}
    >
      <SpecimenGroup label="Video player">
        <VideoPlayer
          src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
          ariaLabel="Sample video"
        />
      </SpecimenGroup>

      <SpecimenGroup label="Custom aspect ratio (4:3)">
        <VideoPlayer
          src="https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4"
          aspectRatio={4 / 3}
          ariaLabel="4:3 aspect video"
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
