import { useState } from "react";
import { RangeSlider, Rating, SegmentedControl, Slider, ToggleGroup, TriStateSwitch } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function SegmentedControlSpecimen() {
  const [view, setView] = useState<string | null>("list");
  return (
    <SpecimenSection title="SegmentedControl">
      <SegmentedControl
        value={view}
        onValueChange={setView}
        ariaLabel="View"
        options={[
          { value: "list", label: "List" },
          { value: "grid", label: "Grid" },
          { value: "map", label: "Map", disabled: true },
        ]}
      />
      <span data-testid="segmented-value">view: {view}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "segmented-control", title: "SegmentedControl", render: () => <SegmentedControlSpecimen /> });

function ToggleGroupSpecimen() {
  const [formats, setFormats] = useState<string | string[] | null>(["bold"]);
  return (
    <SpecimenSection title="ToggleGroup">
      <ToggleGroup
        selectionMode="multiple"
        value={formats}
        onValueChange={setFormats}
        ariaLabel="Formatting"
        options={[
          { value: "bold", label: "B" },
          { value: "italic", label: "I" },
          { value: "underline", label: "U" },
        ]}
      />
      <span data-testid="toggle-value">on: {Array.isArray(formats) ? formats.join(",") : String(formats)}</span>
      <ToggleGroup
        ariaLabel="Alignment"
        defaultValue="left"
        allowDeactivation
        options={[
          { value: "left", label: "Left" },
          { value: "center", label: "Center" },
          { value: "right", label: "Right" },
        ]}
      />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "toggle-group", title: "ToggleGroup", render: () => <ToggleGroupSpecimen /> });

function TriStateSpecimen() {
  const [state, setState] = useState<"excluded" | "default" | "included">("default");
  return (
    <SpecimenSection title="TriStateSwitch">
      <TriStateSwitch ariaLabel="Filter mode" value={state} onValueChange={setState} />
      <span data-testid="tri-value">state: {state}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "tri-state-switch", title: "TriStateSwitch", render: () => <TriStateSpecimen /> });

function SliderSpecimen() {
  const [v, setV] = useState(40);
  return (
    <SpecimenSection title="Slider">
      <Slider value={v} onValueChange={setV} ariaLabel="Volume" />
      <span data-testid="slider-value">value: {v}</span>
      <Slider defaultValue={25} step={5} disabled ariaLabel="Disabled" />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "slider", title: "Slider", render: () => <SliderSpecimen /> });

function RangeSliderSpecimen() {
  const [range, setRange] = useState<[number, number]>([20, 80]);
  return (
    <SpecimenSection title="RangeSlider">
      <RangeSlider value={range} onValueChange={setRange} ariaLabel="Price" />
      <span data-testid="range-value">
        range: {range[0]}–{range[1]}
      </span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "range-slider", title: "RangeSlider", render: () => <RangeSliderSpecimen /> });

function RatingSpecimen() {
  const [stars, setStars] = useState<number | null>(3.5);
  return (
    <SpecimenSection title="Rating">
      <Rating value={stars} onValueChange={setStars} ariaLabel="Quality" />
      <span data-testid="rating-value">rating: {String(stars)}</span>
      <Rating defaultValue={4} step={1} ariaLabel="Whole stars" />
      <Rating defaultValue={2} disabled ariaLabel="Read-only" />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "rating", title: "Rating", render: () => <RatingSpecimen /> });
