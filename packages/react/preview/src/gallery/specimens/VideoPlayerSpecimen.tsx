import { VideoPlayer } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function VideoPlayerSpecimen() {
  return (
    <div className="poodle-specimen">
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
    </div>
  );
}
