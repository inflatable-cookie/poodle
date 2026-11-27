import { readFileSync } from "node:fs";
import { join } from "node:path";

const PACKAGES = join(import.meta.dirname, "../../packages");
const SHARED_SPECIMEN_SOURCES = new Map([
  ["button", "specimens/src/button.rs"],
  ["checkbox", "specimens/src/checkbox.rs"],
  ["code-input", "specimens/src/code_input.rs"],
  ["data-table", "specimens/src/data_table.rs"],
  ["drag-number-field", "render/src/audio_specimens.rs"],
  ["icon-button", "specimens/src/icon_button.rs"],
  ["number-input", "specimens/src/number_input.rs"],
  ["radio", "specimens/src/radio.rs"],
  ["radio-group", "specimens/src/radio_group.rs"],
  ["rating", "specimens/src/rating.rs"],
  ["segmented-control", "specimens/src/segmented_control.rs"],
  ["select", "specimens/src/select.rs"],
  ["split-button", "specimens/src/split_button.rs"],
  ["switch", "specimens/src/switch.rs"],
  ["text-input", "specimens/src/text_input.rs"],
  ["toggle-group", "specimens/src/toggle_group.rs"],
  ["token-input", "specimens/src/token_input.rs"],
  ["tri-state-switch", "specimens/src/tri_state_switch.rs"],
]);

/** Read the source that authors a native route's content. */
export function readNativeSpecimenSource({
  slug,
  gpui = slug.replaceAll("-", "_"),
}: {
  slug: string;
  /** File stem for a GPUI-local specimen with a nonstandard filename. */
  gpui?: string;
}): string {
  const path =
    SHARED_SPECIMEN_SOURCES.get(slug) ?? `gpui/preview/src/specimens/${gpui}.rs`;
  return readFileSync(join(PACKAGES, path), "utf8");
}
