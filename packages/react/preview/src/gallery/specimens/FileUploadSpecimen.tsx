import { useState, type CSSProperties } from "react";
import { DEFAULT_COMPRESSION, FileUpload } from "@inflatable-cookie/poodle-react";
import type { FileUploadItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const errorStyle: CSSProperties = {
  margin: 0,
  color: "var(--poodle-color-text-danger, #ef4444)",
  fontSize: "0.8125rem",
};

export function FileUploadSpecimen() {
  const [imageFiles, setImageFiles] = useState<FileUploadItem[]>([]);
  const [docFiles, setDocFiles] = useState<FileUploadItem[]>([]);
  const [compressedFiles, setCompressedFiles] = useState<FileUploadItem[]>([]);
  const [errorMsg, setErrorMsg] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <FileUpload size={size} accept="image/*" maxSize={5 * 1024 * 1024} />}
      densities={(density) => <FileUpload density={density} accept="image/*" maxSize={5 * 1024 * 1024} />}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Image upload with preview">
          <FileUpload
            accept="image/*"
            multiple
            maxFiles={5}
            maxSize={5 * 1024 * 1024}
            files={imageFiles}
            onChange={setImageFiles}
            onError={({ message }) => setErrorMsg(message)}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Document upload (single file)">
          <FileUpload
            accept=".pdf,.doc,.docx,.txt"
            maxSize={10 * 1024 * 1024}
            showPreview={false}
            files={docFiles}
            onChange={setDocFiles}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Compressed image upload with custom validation">
          <FileUpload
            accept="image/*"
            multiple
            compress
            compressionOptions={{ ...DEFAULT_COMPRESSION, maxWidth: 1200, maxHeight: 800, quality: 0.8 }}
            files={compressedFiles}
            onChange={setCompressedFiles}
            validate={(file) => (file.name.includes(" ") ? "Filename cannot contain spaces" : null)}
            onError={({ message }) => setErrorMsg(message)}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Disabled">
          <FileUpload disabled />
        </SpecimenGroup>

        {errorMsg ? (
          <SpecimenGroup label="Last error">
            <p style={errorStyle}>{errorMsg}</p>
          </SpecimenGroup>
        ) : null}
      </div>
    </SpecimenLayout>
  );
}
