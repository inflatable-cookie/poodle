- **Breaking (`@inflatable-cookie/poodle-core`):** `splitTokenInput` now takes a
  single separator source instead of a pre-compiled regex plus character string:
  - Old: `splitTokenInput(rawValue: string, splitPattern: RegExp | null, separatorChars: string)`
  - New: `splitTokenInput(rawValue: string, separators: readonly string[])`
- Fix focus-trap wrapping when focus is on the modal surface or outside the
  focusable list, include `position: fixed` elements and `a[href]`, `area[href]`,
  `[contenteditable]`, and `summary` while filtering out non-tabbable elements
  (negative `tabindex`, `input[type="hidden"]`, `[inert]`), guard Escape
  dismissal against IME composition and `defaultPrevented`, catch async
  rejections in drag-drop bridge cancellation, clamp `updateDownloadLabel` to
  `0–1`, treat omitted and `null` theme settings consistently as defaults, and
  move the date and colour pickers onto `registerDismissLayer`.
