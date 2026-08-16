import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import AudioPlayer from "../src/AudioPlayer.svelte";

const src = "https://example.com/track.mp3";

describe("AudioPlayer (svelte)", () => {
  it("toggles play and pause through the transport button and the audio element", async () => {
    const { container } = render(AudioPlayer, { props: { src } });
    const play = container.querySelector(".poodle-audio-player__play") as HTMLButtonElement;
    const audio = container.querySelector("audio") as HTMLMediaElement;

    expect(play.getAttribute("aria-label")).toBe("Play");
    expect(audio.paused).toBe(true);

    await fireEvent.click(play);
    expect(play.getAttribute("aria-label")).toBe("Pause");
    expect(audio.paused).toBe(false);

    await fireEvent.click(play);
    expect(play.getAttribute("aria-label")).toBe("Play");
    expect(audio.paused).toBe(true);
  });

  it("seeks the audio element and the time readout from the seek slider", async () => {
    const { container } = render(AudioPlayer, { props: { src } });
    const audio = container.querySelector("audio") as HTMLMediaElement;
    const seek = container.querySelector(".poodle-audio-player__seek") as HTMLInputElement;
    const times = container.querySelectorAll(".poodle-audio-player__time");

    await fireEvent.input(seek, { target: { value: "65" } });

    expect(audio.currentTime).toBe(65);
    expect(times[0].textContent).toBe("1:05");
  });

  it("toggles mute from the mute button and reflects it in label and volume slider", async () => {
    const { container } = render(AudioPlayer, { props: { src } });
    const audio = container.querySelector("audio") as HTMLMediaElement;
    const mute = container.querySelector(".poodle-audio-player__mute") as HTMLButtonElement;
    const volume = container.querySelector(".poodle-audio-player__volume") as HTMLInputElement;

    await fireEvent.click(mute);
    expect(mute.getAttribute("aria-label")).toBe("Unmute");
    expect(audio.muted).toBe(true);
    expect(volume.value).toBe("0");

    await fireEvent.click(mute);
    expect(mute.getAttribute("aria-label")).toBe("Mute");
    expect(audio.muted).toBe(false);
    expect(volume.value).toBe("1");
  });

  it("drives the audio volume from the volume slider and flags muted at zero", async () => {
    const { container } = render(AudioPlayer, { props: { src } });
    const audio = container.querySelector("audio") as HTMLMediaElement;
    const mute = container.querySelector(".poodle-audio-player__mute") as HTMLButtonElement;
    const volume = container.querySelector(".poodle-audio-player__volume") as HTMLInputElement;

    await fireEvent.input(volume, { target: { value: "0.25" } });
    expect(audio.volume).toBe(0.25);
    expect(mute.getAttribute("aria-label")).toBe("Mute");

    await fireEvent.input(volume, { target: { value: "0" } });
    expect(mute.getAttribute("aria-label")).toBe("Unmute");
  });

  it("renders the speed selector only on request and applies the chosen rate", async () => {
    const base = render(AudioPlayer, { props: { src } });
    expect(base.container.querySelector(".poodle-audio-player__speed")).toBeNull();

    const { container } = render(AudioPlayer, { props: { src, showSpeedControl: true } });
    const audio = container.querySelector("audio") as HTMLMediaElement;
    const select = container.querySelector(".poodle-audio-player__speed") as HTMLSelectElement;

    expect([...select.options].map((option) => option.value)).toEqual([
      "0.5",
      "0.75",
      "1",
      "1.25",
      "1.5",
      "2",
    ]);

    await fireEvent.change(select, { target: { value: "1.5" } });
    expect(audio.playbackRate).toBe(1.5);
  });
});
