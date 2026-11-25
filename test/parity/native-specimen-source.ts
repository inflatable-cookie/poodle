import { readFileSync } from "node:fs";
import { join } from "node:path";

const PACKAGES = join(import.meta.dirname, "../../packages");
const SHARED_SPECIMEN_SOURCES = new Map([
  ["button", "specimens/src/button.rs"],
  ["select", "specimens/src/select.rs"],
  ["data-table", "specimens/src/data_table.rs"],
  ["drag-number-field", "render/src/audio_specimens.rs"],
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
