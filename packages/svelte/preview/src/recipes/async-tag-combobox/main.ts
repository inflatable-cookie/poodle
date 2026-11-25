import "@inflatable-cookie/poodle-core/tokens/styles.css";
import "@inflatable-cookie/poodle-core/tokens/theme-eclipse.css";
import { mount } from "svelte";
import TagCombobox from "./TagCombobox.svelte";

const tags = [
  { id: "accessibility", label: "Accessibility" },
  { id: "design", label: "Design" },
  { id: "docs", label: "Documentation" },
  { id: "engineering", label: "Engineering" },
  { id: "research", label: "Research" },
  { id: "archive", label: "Archive", disabled: true },
];

const target = document.querySelector("main");
if (target) {
  mount(TagCombobox, {
    target,
    props: {
      async searchTags(query, signal) {
        await new Promise((resolve) => setTimeout(resolve, 250));
        signal.throwIfAborted();
        const needle = query.trim().toLowerCase();
        if (needle === "error") throw new Error("Example search failure");
        return tags.filter((tag) => tag.label.toLowerCase().includes(needle));
      },
    },
  });
}
