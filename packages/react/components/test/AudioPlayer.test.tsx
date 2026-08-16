import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { AudioPlayer } from "../src/AudioPlayer";

const src = "https://example.com/track.mp3";

describe("AudioPlayer (react)", () => {
  it("toggles play and pause through the transport button and the audio element", () => {
    const { container } = render(<AudioPlayer src={src} />);
    const play = container.querySelector<HTMLButtonElement>(".poodle-audio-player__play")!;
    const audio = container.querySelector("audio") as HTMLMediaElement;

    expect(play.getAttribute("aria-label")).toBe("Play");
    expect(audio.paused).toBe(true);

    fireEvent.click(play);
    expect(play.getAttribute("aria-label")).toBe("Pause");
    expect(audio.paused).toBe(false);

    fireEvent.click(play);
    expect(play.getAttribute("aria-label")).toBe("Play");
    expect(audio.paused).toBe(true);
  });

  it("seeks the audio element and the time readout from the seek slider", () => {
    const { container } = render(<AudioPlayer src={src} />);
    const audio = container.querySelector("audio") as HTMLMediaElement;
    const seek = container.querySelector<HTMLInputElement>(".poodle-audio-player__seek")!;
    const times = container.querySelectorAll(".poodle-audio-player__time");

    fireEvent.change(seek, { target: { value: "65" } });

    expect(audio.currentTime).toBe(65);
    expect(times[0].textContent).toBe("1:05");
  });

  it("toggles mute from the mute button and reflects it in label and volume slider", () => {
    const { container } = render(<AudioPlayer src={src} />);
    const audio = container.querySelector("audio") as HTMLMediaElement;
    const mute = container.querySelector<HTMLButtonElement>(".poodle-audio-player__mute")!;
    const volume = container.querySelector<HTMLInputElement>(".poodle-audio-player__volume")!;

    fireEvent.click(mute);
    expect(mute.getAttribute("aria-label")).toBe("Unmute");
    expect(audio.muted).toBe(true);
    expect(volume.value).toBe("0");

    fireEvent.click(mute);
    expect(mute.getAttribute("aria-label")).toBe("Mute");
    expect(audio.muted).toBe(false);
    expect(volume.value).toBe("1");
  });

  it("drives the audio volume from the volume slider and flags muted at zero", () => {
    const { container } = render(<AudioPlayer src={src} />);
    const audio = container.querySelector("audio") as HTMLMediaElement;
    const mute = container.querySelector<HTMLButtonElement>(".poodle-audio-player__mute")!;
    const volume = container.querySelector<HTMLInputElement>(".poodle-audio-player__volume")!;

    fireEvent.change(volume, { target: { value: "0.25" } });
    expect(audio.volume).toBe(0.25);
    expect(mute.getAttribute("aria-label")).toBe("Mute");

    fireEvent.change(volume, { target: { value: "0" } });
    expect(mute.getAttribute("aria-label")).toBe("Unmute");
  });

  it("renders the speed selector only on request and applies the chosen rate", () => {
    const base = render(<AudioPlayer src={src} />);
    expect(base.container.querySelector(".poodle-audio-player__speed")).toBeNull();

    const { container } = render(<AudioPlayer src={src} showSpeedControl />);
    const audio = container.querySelector("audio") as HTMLMediaElement;
    const select = container.querySelector<HTMLSelectElement>(".poodle-audio-player__speed")!;

    expect([...select.options].map((option) => option.value)).toEqual([
      "0.5",
      "0.75",
      "1",
      "1.25",
      "1.5",
      "2",
    ]);

    fireEvent.change(select, { target: { value: "1.5" } });
    expect(audio.playbackRate).toBe(1.5);
  });
});
