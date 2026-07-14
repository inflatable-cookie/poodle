import "@poodle/styles/file-upload.css";

import {
  forwardRef,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
  type ChangeEvent,
  type DragEvent as ReactDragEvent,
  type KeyboardEvent as ReactKeyboardEvent,
} from "react";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, FileUploadItem, SemanticControlSizeRole } from "./types";

import {
  DEFAULT_COMPRESSION,
  compressImage,
  formatFileSize,
  generateFileUploadId,
  validateUploadFile,
  type FileUploadValidationError,
  type ImageCompressionOptions,
} from "./file-upload";

export interface FileUploadProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  accept?: string | null;
  maxSize?: number;
  multiple?: boolean;
  maxFiles?: number;
  showPreview?: boolean;
  disabled?: boolean;
  files?: FileUploadItem[];
  defaultFiles?: FileUploadItem[];
  validate?: ((file: File) => string | null) | undefined;
  compress?: boolean;
  compressionOptions?: ImageCompressionOptions;
  onChange?: ((files: FileUploadItem[]) => void) | undefined;
  onUpload?: ((files: File[]) => void) | undefined;
  onError?: ((event: FileUploadValidationError) => void) | undefined;
  onRemove?: ((item: FileUploadItem) => void) | undefined;
}

export interface FileUploadHandle {
  updateProgress: (id: string, progress: number) => void;
  setError: (id: string, message: string) => void;
  clear: () => void;
}

export const FileUpload = forwardRef<FileUploadHandle, FileUploadProps>(function FileUpload(
  {
    size = null,
    sizeRole = "control",
    density = null,
    accept = null,
    maxSize = 10 * 1024 * 1024,
    multiple = false,
    maxFiles = 10,
    showPreview = true,
    disabled = false,
    files: controlledFiles,
    defaultFiles = [],
    validate = undefined,
    compress = false,
    compressionOptions = DEFAULT_COMPRESSION,
    onChange = undefined,
    onUpload = undefined,
    onError = undefined,
    onRemove = undefined,
  },
  ref,
) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  const inputRef = useRef<HTMLInputElement | null>(null);
  const [dragActive, setDragActive] = useState(false);
  const [uncontrolledFiles, setUncontrolledFiles] = useState<FileUploadItem[]>(defaultFiles);

  const isControlled = controlledFiles !== undefined;
  const files = isControlled ? controlledFiles : uncontrolledFiles;
  const filesRef = useRef(files);
  filesRef.current = files;

  function commitFiles(next: FileUploadItem[]): void {
    if (!isControlled) {
      setUncontrolledFiles(next);
    }
    filesRef.current = next;
    onChange?.(next);
  }

  function createPreviewUrl(file: File): string | null {
    if (!showPreview || !file.type.startsWith("image/")) {
      return null;
    }

    return URL.createObjectURL(file);
  }

  async function addFiles(newFiles: FileList | File[]): Promise<void> {
    const fileArray = Array.from(newFiles);
    const filesToUpload: File[] = [];
    let next = filesRef.current;

    for (const file of fileArray) {
      if (!multiple && next.length >= 1) {
        break;
      }

      if (multiple && next.length >= maxFiles) {
        const message = `Maximum of ${maxFiles} files allowed`;
        onError?.({ file, message });
        break;
      }

      const error = validateUploadFile({
        file,
        maxSize,
        accept: accept ?? "*",
        validate,
      });

      if (error) {
        onError?.({ file, message: error });
        continue;
      }

      let processedFile = file;
      let originalFile: File | undefined;

      if (compress && file.type.startsWith("image/")) {
        const compressed = await compressImage(file, compressionOptions);
        if (compressed !== file) {
          originalFile = file;
          processedFile = compressed;
        }
      }

      const item: FileUploadItem = {
        file: processedFile,
        id: generateFileUploadId(),
        progress: 0,
        status: "pending",
        previewUrl: createPreviewUrl(processedFile),
        originalFile,
      };

      next = [...next, item];
      filesToUpload.push(processedFile);
    }

    commitFiles(next);

    if (filesToUpload.length > 0) {
      onUpload?.(filesToUpload);
    }
  }

  function removeFile(id: string): void {
    const item = filesRef.current.find((f) => f.id === id);

    if (!item) {
      return;
    }

    if (item.previewUrl) {
      URL.revokeObjectURL(item.previewUrl);
    }

    commitFiles(filesRef.current.filter((f) => f.id !== id));
    onRemove?.(item);
  }

  useImperativeHandle(ref, () => ({
    updateProgress(id: string, progress: number): void {
      commitFiles(
        filesRef.current.map((f) =>
          f.id === id
            ? {
                ...f,
                progress: Math.min(100, Math.max(0, progress)),
                status: progress >= 100 ? "complete" : ("uploading" as const),
              }
            : f,
        ),
      );
    },
    setError(id: string, message: string): void {
      commitFiles(
        filesRef.current.map((f) => (f.id === id ? { ...f, status: "error" as const, error: message } : f)),
      );
    },
    clear(): void {
      for (const f of filesRef.current) {
        if (f.previewUrl) {
          URL.revokeObjectURL(f.previewUrl);
        }
      }

      commitFiles([]);
    },
  }));

  function handleDrop(event: ReactDragEvent): void {
    event.preventDefault();
    setDragActive(false);

    if (disabled || !event.dataTransfer?.files.length) {
      return;
    }

    void addFiles(event.dataTransfer.files);
  }

  function handleDragOver(event: ReactDragEvent): void {
    event.preventDefault();

    if (!disabled) {
      setDragActive(true);
    }
  }

  function handleDragLeave(): void {
    setDragActive(false);
  }

  function handleInputChange(event: ChangeEvent<HTMLInputElement>): void {
    const target = event.target;

    if (target.files?.length) {
      void addFiles(target.files);
      target.value = "";
    }
  }

  function handleClick(): void {
    if (!disabled) {
      inputRef.current?.click();
    }
  }

  function handleKeydown(event: ReactKeyboardEvent): void {
    if ((event.key === "Enter" || event.key === " ") && !disabled) {
      event.preventDefault();
      inputRef.current?.click();
    }
  }

  useEffect(() => {
    return () => {
      for (const f of filesRef.current) {
        if (f.previewUrl) {
          URL.revokeObjectURL(f.previewUrl);
        }
      }
    };
  }, []);

  return (
    <div
      className={disabled ? "poodle-file-upload poodle-file-upload--disabled" : "poodle-file-upload"}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div
        className={
          dragActive
            ? "poodle-file-upload__dropzone poodle-file-upload__dropzone--active"
            : "poodle-file-upload__dropzone"
        }
        role="button"
        tabIndex={disabled ? -1 : 0}
        aria-label={multiple ? "Drop files here or click to browse" : "Drop a file here or click to browse"}
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onClick={handleClick}
        onKeyDown={handleKeydown}
      >
        <input
          ref={inputRef}
          type="file"
          accept={accept ?? undefined}
          multiple={multiple}
          disabled={disabled}
          className="poodle-file-upload__input"
          onChange={handleInputChange}
          tabIndex={-1}
        />
        <div className="poodle-file-upload__dropzone-content">
          <svg className="poodle-file-upload__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
            <path d="M12 16V4m0 0L8 8m4-4l4 4" strokeLinecap="round" strokeLinejoin="round" />
            <path d="M20 16v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2" strokeLinecap="round" strokeLinejoin="round" />
          </svg>
          <p className="poodle-file-upload__label">
            {dragActive ? (
              "Drop to upload"
            ) : (
              <>
                Drop files here or <span className="poodle-file-upload__browse">browse</span>
              </>
            )}
          </p>
          {accept || maxSize ? (
            <p className="poodle-file-upload__hint">
              {accept}
              {accept && maxSize ? " · " : null}
              {maxSize ? `Max ${formatFileSize(maxSize)}` : null}
            </p>
          ) : null}
        </div>
      </div>

      {files.length > 0 ? (
        <ul className="poodle-file-upload__list" role="list">
          {files.map((item) => (
            <li
              key={item.id}
              className={
                item.status === "error"
                  ? "poodle-file-upload__item poodle-file-upload__item--error"
                  : "poodle-file-upload__item"
              }
            >
              {item.previewUrl ? (
                <img className="poodle-file-upload__preview" src={item.previewUrl} alt="" />
              ) : (
                <div className="poodle-file-upload__file-icon">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
                    <path
                      d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                    <path d="M14 2v6h6" strokeLinecap="round" strokeLinejoin="round" />
                  </svg>
                </div>
              )}

              <div className="poodle-file-upload__meta">
                <span className="poodle-file-upload__name">{item.file.name}</span>
                <span className="poodle-file-upload__size">
                  {formatFileSize(item.file.size)}
                  {item.status === "error" && item.error ? (
                    <>
                      {" · "}
                      <span className="poodle-file-upload__error-text">{item.error}</span>
                    </>
                  ) : item.status === "uploading" ? (
                    <> · {item.progress}%</>
                  ) : item.status === "complete" ? (
                    <> · Complete</>
                  ) : null}
                </span>
              </div>

              {item.status === "uploading" ? (
                <div className="poodle-file-upload__progress">
                  <div className="poodle-file-upload__progress-bar" style={{ width: `${item.progress}%` }} />
                </div>
              ) : null}

              <button
                type="button"
                className="poodle-file-upload__remove"
                aria-label={`Remove ${item.file.name}`}
                onClick={(event) => {
                  event.stopPropagation();
                  removeFile(item.id);
                }}
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <path d="M18 6L6 18M6 6l12 12" strokeLinecap="round" />
                </svg>
              </button>
            </li>
          ))}
        </ul>
      ) : null}
    </div>
  );
});
