import { Component, type ReactNode } from "react";

import { Button } from "./Button";
import { EmptyState } from "./EmptyState";

export interface ErrorBoundaryProps {
  children: ReactNode;
  title?: string;
  retryLabel?: string;
}

interface ErrorBoundaryState {
  currentError: Error | null;
}

/**
 * Mirrors the Svelte `<svelte:boundary>` wrapper. React error boundaries
 * require a class component — the only one in the package.
 */
export class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  state: ErrorBoundaryState = { currentError: null };

  static getDerivedStateFromError(error: unknown): ErrorBoundaryState {
    return { currentError: error instanceof Error ? error : new Error(String(error)) };
  }

  reset = (): void => {
    this.setState({ currentError: null });
  };

  render(): ReactNode {
    const { children, title = "Something went wrong", retryLabel = "Try again" } = this.props;

    if (this.state.currentError) {
      return (
        <EmptyState
          title={title}
          message={this.state.currentError.message}
          actions={
            <Button variant="secondary" onClick={this.reset}>
              {retryLabel}
            </Button>
          }
        />
      );
    }

    return children;
  }
}
