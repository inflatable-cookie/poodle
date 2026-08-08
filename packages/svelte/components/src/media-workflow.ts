import type { MediaUploadWorkflowStep } from "./types";

export type MediaWorkflowPaginationParams = {
  cursor?: string;
  limit?: number;
};

export type MediaWorkflowPageResponse<TItem> = {
  data: TItem[];
  nextCursor?: string | null;
  hasMore?: boolean;
};

export type MediaBrowseState<TItem> = {
  items: TItem[];
  nextCursor: string | null;
  hasMore: boolean;
};

export type LoadedMediaBrowsePage<TItem> = {
  items: TItem[];
  nextCursor: string | null;
  hasMore: boolean;
};

export type MediaUploadDisplayStep = "select" | MediaUploadWorkflowStep;

export type MediaDuplicateCheckResult<TExisting> = {
  exists: boolean;
  item?: TExisting | null;
};

export type MediaUploadProgress = {
  loaded: number;
  total: number;
  percent: number;
};

export type MediaUploadPlan = {
  uploadUrl: string;
  method: string;
  headers?: Record<string, string> | null;
  expiresAt: string;
  maxBytes: number;
  allowedContentTypes?: string[] | null;
  objectKey?: string;
};

export type MediaUploadInitResult = {
  versionId: string;
  uploadPlan: MediaUploadPlan;
};

export type MediaUploadDuplicateResult<TExisting> = {
  kind: "duplicate";
  fileHash: string;
  existingItem: TExisting;
};

export type MediaUploadCompleteResult<TCreated> = {
  kind: "uploaded";
  fileHash: string;
  createdItem: TCreated;
};

export type MediaUploadWorkflowResult<TExisting, TCreated> =
  | MediaUploadDuplicateResult<TExisting>
  | MediaUploadCompleteResult<TCreated>;

export type LoadMediaBrowsePageInput<TItem> = {
  listPage: (
    params?: MediaWorkflowPaginationParams,
  ) => Promise<MediaWorkflowPageResponse<TItem>>;
  cursor?: string;
  limit?: number;
};

export async function loadMediaBrowsePage<TItem>({
  listPage,
  cursor,
  limit = 12,
}: LoadMediaBrowsePageInput<TItem>): Promise<LoadedMediaBrowsePage<TItem>> {
  const response = await listPage({
    cursor: cursor ?? undefined,
    limit,
  });

  return {
    items: response.data,
    nextCursor: response.nextCursor ?? null,
    hasMore: response.hasMore ?? false,
  };
}

export function mergeMediaBrowseItems<TItem>(
  existingItems: TItem[],
  nextItems: TItem[],
  cursor?: string,
): TItem[] {
  return cursor ? [...existingItems, ...nextItems] : nextItems;
}

export function createResetMediaBrowseState<TItem>(): MediaBrowseState<TItem> {
  return {
    items: [],
    nextCursor: null,
    hasMore: false,
  };
}

export async function computeFileHash(file: File): Promise<string> {
  const buffer = await file.arrayBuffer();
  const hashBuffer = await crypto.subtle.digest("SHA-256", buffer);
  const bytes = new Uint8Array(hashBuffer);
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

export type UploadMediaWithKnownHashInput<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult,
> = {
  file: File;
  fileHash: string;
  maxFileSize: number;
  createRecord: (request: TCreateRequest) => Promise<TCreatedRecord>;
  buildCreateRequest: (file: File, fileHash: string) => TCreateRequest;
  initiateUpload: (
    createdRecord: TCreatedRecord,
    request: TInitiateRequest,
  ) => Promise<MediaUploadInitResult>;
  buildInitiateRequest: (file: File, fileHash: string) => TInitiateRequest;
  finaliseUpload: (
    createdRecord: TCreatedRecord,
    versionId: string,
    request: TFinaliseRequest,
  ) => Promise<TFinaliseResult>;
  buildFinaliseRequest: (file: File, fileHash: string) => TFinaliseRequest;
  toCreatedItem: (
    finaliseResult: TFinaliseResult,
    createdRecord: TCreatedRecord,
  ) => TCreated;
  onStep?: (step: "uploading" | "finalising") => void;
  onProgress?: (percent: number) => void;
};

export async function uploadMediaWithKnownHash<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult,
>(
  input: UploadMediaWithKnownHashInput<
    TCreated,
    TCreateRequest,
    TCreatedRecord,
    TInitiateRequest,
    TFinaliseRequest,
    TFinaliseResult
  >,
): Promise<TCreated> {
  const createdRecord = await input.createRecord(
    input.buildCreateRequest(input.file, input.fileHash),
  );

  const uploadStart = await input.initiateUpload(
    createdRecord,
    input.buildInitiateRequest(input.file, input.fileHash),
  );

  input.onStep?.("uploading");
  await uploadToBlob(uploadStart.uploadPlan, input.file, input.maxFileSize, input.onProgress);

  input.onStep?.("finalising");
  const finaliseResult = await input.finaliseUpload(
    createdRecord,
    uploadStart.versionId,
    input.buildFinaliseRequest(input.file, input.fileHash),
  );

  return input.toCreatedItem(finaliseResult, createdRecord);
}

export type RunMediaUploadWorkflowInput<
  TExisting,
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult,
> = UploadMediaWithKnownHashInput<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult
> & {
  checkDuplicate: (sha256: string) => Promise<MediaDuplicateCheckResult<TExisting>>;
  onStep?: (step: "checking" | "uploading" | "finalising") => void;
};

export async function runMediaUploadWorkflow<
  TExisting,
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult,
>(
  input: RunMediaUploadWorkflowInput<
    TExisting,
    TCreated,
    TCreateRequest,
    TCreatedRecord,
    TInitiateRequest,
    TFinaliseRequest,
    TFinaliseResult
  >,
): Promise<MediaUploadWorkflowResult<TExisting, TCreated>> {
  input.onStep?.("checking");

  const fileHash = await computeFileHash(input.file);
  const duplicateCheck = await input.checkDuplicate(fileHash);

  if (duplicateCheck.exists && duplicateCheck.item) {
    return {
      kind: "duplicate",
      fileHash,
      existingItem: duplicateCheck.item,
    };
  }

  const createdItem = await uploadMediaWithKnownHash({
    ...input,
    fileHash,
  });

  return {
    kind: "uploaded",
    fileHash,
    createdItem,
  };
}

function uploadToBlob(
  plan: MediaUploadPlan,
  file: File,
  maxFileSize: number,
  onProgress?: (percent: number) => void,
): Promise<void> {
  if (file.size > plan.maxBytes || file.size > maxFileSize) {
    throw new Error("File exceeds maximum upload size");
  }

  const allowedContentTypes = plan.allowedContentTypes ?? [];
  if (allowedContentTypes.length > 0 && !allowedContentTypes.includes(file.type)) {
    throw new Error(`Content type ${file.type} is not allowed`);
  }

  if (new Date(plan.expiresAt) < new Date()) {
    throw new Error("Upload URL has expired");
  }

  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest();
    const headers = {
      "Content-Type": file.type,
      ...(plan.headers ?? {}),
    };

    if (onProgress) {
      xhr.upload.addEventListener("progress", (event) => {
        if (!event.lengthComputable) return;
        onProgress(Math.round((event.loaded / event.total) * 100));
      });
    }

    xhr.addEventListener("load", () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        resolve();
        return;
      }

      reject(new Error(`Upload failed with status ${xhr.status}`));
    });

    xhr.addEventListener("error", () => {
      reject(new Error("Network error during upload"));
    });

    xhr.addEventListener("abort", () => {
      reject(new Error("Upload was aborted"));
    });

    xhr.open(plan.method, plan.uploadUrl);

    for (const [key, value] of Object.entries(headers)) {
      xhr.setRequestHeader(key, value);
    }

    xhr.send(file);
  });
}
