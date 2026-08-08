import "@inflatable-cookie/poodle-styles/media-picker.css";

import { useState } from "react";

import { Dialog } from "./Dialog";
import { FileUpload } from "./FileUpload";
import { Tabs, type TabsItem } from "./Tabs";
import { TextInput } from "./TextInput";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  FileUploadItem,
  MediaPickerItem,
  SemanticControlSizeRole,
} from "./types";

export interface MediaPickerProps {
  open?: boolean | null | undefined;
  items?: MediaPickerItem[];
  accept?: string;
  maxFileSize?: number;
  title?: string;
  emptyMessage?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onSelect?: ((item: MediaPickerItem) => void) | undefined;
  onUpload?: ((files: FileUploadItem[]) => void) | undefined;
  onOpenChange?: ((open: boolean) => void) | undefined;
}

const tabItems: TabsItem[] = [
  { value: "browse", label: "Browse" },
  { value: "upload", label: "Upload" },
];

export function MediaPicker({
  open = undefined,
  items = [],
  accept = "image/*",
  maxFileSize = 25 * 1024 * 1024,
  title = "Select media",
  emptyMessage = "No media items found.",
  size = null,
  sizeRole = "control",
  density = null,
  onSelect = undefined,
  onUpload = undefined,
  onOpenChange = undefined,
}: MediaPickerProps) {
  const uiPresentation = useUiPresentation();

  const [activeTab, setActiveTab] = useState("browse");
  const [searchQuery, setSearchQuery] = useState("");
  const [uploadFiles, setUploadFiles] = useState<FileUploadItem[]>([]);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(false);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = open !== undefined && open !== null;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  const filteredItems = searchQuery
    ? items.filter((item) => item.label.toLowerCase().includes(searchQuery.toLowerCase()))
    : items;

  function requestOpenChange(nextOpen: boolean): void {
    if (!isControlled) {
      setUncontrolledOpen(nextOpen);
    }

    onOpenChange?.(nextOpen);
  }

  function handleSelect(item: MediaPickerItem): void {
    onSelect?.(item);
    requestOpenChange(false);
  }

  function handleUploadChange(nextFiles: FileUploadItem[]): void {
    setUploadFiles(nextFiles);
    onUpload?.(nextFiles);
  }

  return (
    <Dialog open={isOpen} title={title} kind="dialog" onOpenChange={requestOpenChange}>
      <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
        <div className="poodle-media-picker" data-size={resolvedSize} data-density={resolvedDensity}>
          <Tabs items={tabItems} value={activeTab} onValueChange={(value) => setActiveTab(value)} />

          {activeTab === "browse" ? (
            <>
              <div className="poodle-media-picker__search">
                <TextInput
                  id="media-picker-search"
                  value={searchQuery}
                  placeholder="Search media..."
                  onValueChange={setSearchQuery}
                />
              </div>

              {filteredItems.length === 0 ? (
                <div className="poodle-media-picker__empty">
                  <p>{emptyMessage}</p>
                </div>
              ) : (
                <div className="poodle-media-picker__grid" role="listbox" aria-label="Media items">
                  {filteredItems.map((item) => (
                    <button
                      key={item.id}
                      type="button"
                      className="poodle-media-picker__item"
                      role="option"
                      aria-selected="false"
                      onClick={() => handleSelect(item)}
                    >
                      {item.thumbnailUrl ? (
                        <img className="poodle-media-picker__thumb" src={item.thumbnailUrl} alt="" />
                      ) : (
                        <div className="poodle-media-picker__thumb poodle-media-picker__thumb--placeholder">
                          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
                            <rect x="3" y="3" width="18" height="18" rx="2" />
                            <circle cx="8.5" cy="8.5" r="1.5" />
                            <path d="M21 15l-5-5L5 21" />
                          </svg>
                        </div>
                      )}
                      <span className="poodle-media-picker__label">{item.label}</span>
                    </button>
                  ))}
                </div>
              )}
            </>
          ) : (
            <div className="poodle-media-picker__upload">
              <FileUpload accept={accept} maxSize={maxFileSize} multiple files={uploadFiles} onChange={handleUploadChange} />
            </div>
          )}
        </div>
      </UiPresentationProvider>
    </Dialog>
  );
}
