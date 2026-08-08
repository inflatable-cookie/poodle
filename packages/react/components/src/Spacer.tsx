import "@inflatable-cookie/poodle-styles/spacer.css";

export interface SpacerProps {
  grow?: number;
  minSize?: string | null;
}

export function Spacer({ grow = 1, minSize = null }: SpacerProps) {
  return (
    <div
      className="poodle-spacer"
      aria-hidden="true"
      style={{
        flex: `${grow} 1 0%`,
        ...(minSize ? { minWidth: minSize, minHeight: minSize } : null),
      }}
    />
  );
}
