import { useRef, useState } from "react";
import { PageLoading, Button } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const inlineShellStyle = {
  minHeight: "18rem",
  border: "1px dashed var(--poodle-color-border-default)",
  borderRadius: "var(--poodle-radius-surface)",
  background: "color-mix(in srgb, var(--poodle-color-background-surface) 94%, transparent)",
} as const;

const dismissStyle = {
  position: "fixed",
  bottom: "1rem",
  right: "1rem",
  zIndex: 10000,
  padding: "0.5rem 1rem",
  border: "1px solid var(--poodle-color-border-default)",
  borderRadius: "var(--poodle-radius-control)",
  background: "var(--poodle-color-background-elevated)",
  color: "var(--poodle-color-text-primary)",
  font: "inherit",
  fontSize: "0.75rem",
  cursor: "pointer",
} as const;

export function PageLoadingSpecimen() {
  const [showIndeterminate, setShowIndeterminate] = useState(false);
  const [showDeterminate, setShowDeterminate] = useState(false);
  const [showWithCancel, setShowWithCancel] = useState(false);
  const [showInline, setShowInline] = useState(true);
  const [demoProgress, setDemoProgress] = useState(0);
  const progressTimer = useRef<ReturnType<typeof setInterval> | null>(null);

  function startDeterminate() {
    setDemoProgress(0);
    setShowDeterminate(true);
    progressTimer.current = setInterval(() => {
      setDemoProgress((prev) => {
        const next = prev + 8;
        if (next >= 100) {
          if (progressTimer.current) clearInterval(progressTimer.current);
          setTimeout(() => {
            setShowDeterminate(false);
            setDemoProgress(0);
          }, 600);
          return 100;
        }
        return next;
      });
    }, 300);
  }

  function closeAll() {
    setShowIndeterminate(false);
    setShowDeterminate(false);
    setShowWithCancel(false);
    if (progressTimer.current) clearInterval(progressTimer.current);
    setDemoProgress(0);
  }

  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Inline">
        <Button variant="secondary" onClick={() => setShowInline((prev) => !prev)}>
          Toggle inline loading
        </Button>
        {showInline ? (
          <div style={inlineShellStyle}>
            <PageLoading visible presentation="inline" message="Loading section content..." />
          </div>
        ) : null}
      </SpecimenGroup>

      <SpecimenGroup label="Indeterminate (spinner only)">
        <Button variant="secondary" onClick={() => setShowIndeterminate(true)}>
          Show loading overlay
        </Button>
        <PageLoading visible={showIndeterminate} message="Loading data..." />
      </SpecimenGroup>

      <SpecimenGroup label="Determinate (with progress bar)">
        <Button variant="secondary" onClick={startDeterminate}>
          Show progress overlay
        </Button>
        <PageLoading visible={showDeterminate} value={demoProgress} message={`Uploading files... ${demoProgress}%`} />
      </SpecimenGroup>

      <SpecimenGroup label="With cancel button">
        <Button variant="secondary" onClick={() => setShowWithCancel(true)}>
          Show cancellable loading
        </Button>
        <PageLoading visible={showWithCancel} message="Processing request..." canCancel onCancel={closeAll} />
      </SpecimenGroup>

      {showIndeterminate || showDeterminate || showWithCancel ? (
        <button style={dismissStyle} onClick={closeAll}>
          Dismiss overlay (click here if stuck)
        </button>
      ) : null}
    </div>
  );
}
