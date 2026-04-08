export interface ImageCompressionOptions {
  maxWidth?: number;
  maxHeight?: number;
  quality?: number;
  format?: "image/jpeg" | "image/png" | "image/webp";
}

export interface FileUploadValidationError {
  file: File;
  message: string;
}

export const DEFAULT_COMPRESSION: ImageCompressionOptions = {
  maxWidth: 1920,
  maxHeight: 1080,
  quality: 0.85,
};

export function generateFileUploadId(): string {
  return `file-${Date.now()}-${Math.random().toString(36).slice(2, 11)}`;
}

export function formatFileSize(bytes: number | null | undefined): string {
  if (bytes == null || Number.isNaN(bytes)) return "";
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

function isAcceptedFileType(file: File, accept: string): boolean {
  const acceptedTypes = accept.split(",").map((type) => type.trim());
  const fileType = file.type;
  const fileExtension = `.${file.name.split(".").pop()?.toLowerCase()}`;

  return acceptedTypes.some((acceptedType) => {
    if (acceptedType.startsWith(".")) {
      return fileExtension === acceptedType.toLowerCase();
    }
    if (acceptedType.endsWith("/*")) {
      return fileType.startsWith(acceptedType.slice(0, -1));
    }
    return fileType === acceptedType;
  });
}

export function validateUploadFile({
  file,
  maxSize,
  accept,
  validate,
}: {
  file: File;
  maxSize: number;
  accept: string;
  validate?: (file: File) => string | null;
}): string | null {
  if (file.size > maxSize) {
    return `File too large. Maximum size is ${formatFileSize(maxSize)}`;
  }

  if (accept !== "*" && !isAcceptedFileType(file, accept)) {
    return `File type not accepted. Accepted types: ${accept}`;
  }

  return validate ? validate(file) : null;
}

export async function compressImage(
  file: File,
  options: ImageCompressionOptions = DEFAULT_COMPRESSION,
): Promise<File> {
  if (!file.type.startsWith("image/")) {
    return file;
  }

  if (file.type === "image/svg+xml" || file.type === "image/gif") {
    return file;
  }

  const {
    maxWidth = 1920,
    maxHeight = 1080,
    quality = 0.85,
    format,
  } = options;

  return new Promise((resolve) => {
    const img = new Image();
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");

    img.onload = () => {
      let { width, height } = img;

      if (width > maxWidth || height > maxHeight) {
        const ratio = Math.min(maxWidth / width, maxHeight / height);
        width = Math.round(width * ratio);
        height = Math.round(height * ratio);
      }

      canvas.width = width;
      canvas.height = height;

      if (ctx) {
        ctx.drawImage(img, 0, 0, width, height);

        const outputFormat =
          format || (file.type === "image/png" ? "image/png" : "image/jpeg");

        canvas.toBlob(
          (blob) => {
            if (blob && blob.size < file.size) {
              resolve(
                new File([blob], file.name, {
                  type: outputFormat,
                  lastModified: Date.now(),
                }),
              );
            } else {
              resolve(file);
            }
          },
          outputFormat,
          quality,
        );
      } else {
        resolve(file);
      }

      URL.revokeObjectURL(img.src);
    };

    img.onerror = () => {
      URL.revokeObjectURL(img.src);
      resolve(file);
    };

    img.src = URL.createObjectURL(file);
  });
}
