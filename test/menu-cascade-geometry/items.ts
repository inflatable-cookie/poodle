export const items = [
  { value: "action", label: "Action" },
  { value: "disabled", label: "Unavailable", disabled: true },
  { value: "frames", label: "Go to frame", children: [
    { value: "depth", label: "More frames", children: [{ value: "deep", label: "Deep frame" }] },
    ...Array.from({ length: new URLSearchParams(location.search).has("tall") ? 40 : 8 }, (_, i) => ({
      value: `frame-${i}`, label: `Frame ${i + 1}`, kind: "radio" as const, checked: i === 0,
    })),
  ] },
];
export function geometry(change: unknown) {
  const win = window as typeof window & { geometry?: unknown[] };
  (win.geometry ??= []).push(change);
}
export function action(value: string) { document.body.dataset.action = value; }
