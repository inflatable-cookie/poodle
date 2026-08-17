import { fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { VideoPlayer } from "../src/VideoPlayer";

describe("VideoPlayer (react)", () => {
  it("exposes a labelled root with aspect-ratio, size, and density", () => {
    const { container } = render(
      <VideoPlayer src="/video.mp4" ariaLabel="Demo" aspectRatio={4 / 3} size="lg" density="compact" />,
    );
    const root = container.querySelector(".poodle-video-player") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Demo");
    expect(root.style.aspectRatio).toBe("1.3333333333333333 / 1");
    expect(root.dataset.size).toBe("lg");
    expect(root.dataset.density).toBe("compact");
  });

  it("shows the big play button only while paused at the start", () => {
    const { container } = render(<VideoPlayer src="/video.mp4" />);
    const bigPlay = container.querySelector(".poodle-video-player__big-play") as HTMLElement;
    expect(bigPlay).not.toBeNull();
    expect(bigPlay.getAttribute("aria-label")).toBe("Play video");

    fireEvent.click(bigPlay);
    waitFor(() => {
      expect(container.querySelector(".poodle-video-player__big-play")).toBeNull();
    });
  });

  it("toggles the play control label with playback state", () => {
    const { container } = render(<VideoPlayer src="/video.mp4" />);
    expect(container.querySelector('[aria-label="Play"]')).not.toBeNull();

    fireEvent.click(container.querySelector('[aria-label="Play"]') as HTMLElement);
    waitFor(() => {
      expect(container.querySelector('[aria-label="Pause"]')).not.toBeNull();
    });

    fireEvent.click(container.querySelector('[aria-label="Pause"]') as HTMLElement);
    waitFor(() => {
      expect(container.querySelector('[aria-label="Play"]')).not.toBeNull();
    });
  });

  it("toggles mute state through the mute button", () => {
    const { container } = render(<VideoPlayer src="/video.mp4" />);
    expect(container.querySelector('[aria-label="Mute"]')).not.toBeNull();

    fireEvent.click(container.querySelector('[aria-label="Mute"]') as HTMLElement);
    expect(container.querySelector('[aria-label="Unmute"]')).not.toBeNull();

    fireEvent.click(container.querySelector('[aria-label="Unmute"]') as HTMLElement);
    expect(container.querySelector('[aria-label="Mute"]')).not.toBeNull();
  });

  it("labels the seek and volume sliders with contract ranges", () => {
    const { container } = render(<VideoPlayer src="/video.mp4" />);
    const seek = container.querySelector('[aria-label="Seek"]') as HTMLInputElement;
    expect(seek.type).toBe("range");
    expect(seek.step).toBe("0.1");

    const volume = container.querySelector('[aria-label="Volume"]') as HTMLInputElement;
    expect(volume.min).toBe("0");
    expect(volume.max).toBe("1");
    expect(volume.step).toBe("0.01");
  });

  it("renders the captions track only when enabled with a source", () => {
    const withCaptions = render(
      <VideoPlayer src="/video.mp4" showCaptions captionsSrc="/captions.vtt" />,
    );
    const track = withCaptions.container.querySelector("track") as HTMLTrackElement;
    expect(track).not.toBeNull();
    expect(track.kind).toBe("captions");
    expect(track.src).toContain("/captions.vtt");

    const without = render(<VideoPlayer src="/video.mp4" showCaptions={false} />);
    expect(without.container.querySelector("track")).toBeNull();
  });

  it("formats the time display in m:ss / m:ss", () => {
    const { container } = render(<VideoPlayer src="/video.mp4" />);
    expect(container.querySelector(".poodle-video-player__time")?.textContent).toBe("0:00 / 0:00");
  });
});