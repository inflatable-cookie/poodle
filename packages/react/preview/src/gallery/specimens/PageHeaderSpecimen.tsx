import { useState } from "react";
import { PageHeader, IconButton, MetaBar, Pill, TimeAgo } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const metaTextStyle = {
  color: "var(--poodle-color-text-secondary)",
  fontSize: "0.8125rem",
} as const;

const breadcrumbsStyle = {
  display: "inline-flex",
  flexWrap: "wrap",
  alignItems: "center",
  gap: "0.375rem",
  color: "var(--poodle-color-text-secondary)",
  fontSize: "0.8125rem",
} as const;

const breadcrumbLinkStyle = { color: "inherit", textDecoration: "none" } as const;
const chevronStyle = { opacity: 0.7 } as const;
const hintStyle = {
  margin: "0.5rem 0 0",
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
} as const;

function DemoBreadcrumbs() {
  return (
    <nav style={breadcrumbsStyle} aria-label="Breadcrumb">
      <a href="/learning/pathways" style={breadcrumbLinkStyle}>Pathways</a>
      <span style={chevronStyle} aria-hidden="true">›</span>
      <a href="/learning/pathways/foundation" style={breadcrumbLinkStyle}>Foundation</a>
      <span style={chevronStyle} aria-hidden="true">›</span>
      <span>Module</span>
    </nav>
  );
}

export function PageHeaderSpecimen() {
  const [navAction, setNavAction] = useState("");
  const [hierarchyAction, setHierarchyAction] = useState("");
  const [statusAction, setStatusAction] = useState("");
  const [metaAction, setMetaAction] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <SpecimenGroup label={size.toUpperCase()}>
          <PageHeader
            title="Media Library"
            subtitle="Browse, review, and manage uploaded files."
            backHref="/dashboard"
            backLabel="Dashboard"
            size={size}
            actions={
              <>
                <IconButton icon="upload" ariaLabel="Upload" variant="secondary" />
                <IconButton icon="settings" ariaLabel="Settings" variant="secondary" />
              </>
            }
          />
        </SpecimenGroup>
      )}
      densities={(density) => (
        <SpecimenGroup label={density.toUpperCase()}>
          <PageHeader
            title="Media Library"
            subtitle="Browse, review, and manage uploaded files."
            backHref="/dashboard"
            backLabel="Dashboard"
            density={density}
            actions={
              <>
                <IconButton icon="upload" ariaLabel="Upload" variant="secondary" />
                <IconButton icon="settings" ariaLabel="Settings" variant="secondary" />
              </>
            }
          />
        </SpecimenGroup>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Page title and summary">
          <PageHeader title="Components" subtitle="Browse and manage your component library." />
          <PageHeader title="Settings" />
        </SpecimenGroup>

        <SpecimenGroup label="Navigation and actions">
          <PageHeader
            title="Media Library"
            subtitle="Browse, review, and manage uploaded files."
            backHref="/dashboard"
            backLabel="Dashboard"
            actions={
              <>
                <IconButton icon="upload" ariaLabel="Upload" variant="secondary" onClick={() => setNavAction("Upload")} />
                <IconButton icon="settings" ariaLabel="Settings" variant="secondary" onClick={() => setNavAction("Settings")} />
              </>
            }
          />
          <PageHeader
            title="Cash flow forecasts"
            section="Module"
            subtitle="Manage content and ordering for this module."
            backHref="/learning/pathways"
            backLabel="Pathways"
            breadcrumbs={<DemoBreadcrumbs />}
            actions={
              <>
                <IconButton icon="upload" ariaLabel="Upload" variant="secondary" onClick={() => setNavAction("Upload module")} />
                <IconButton icon="settings" ariaLabel="Settings" variant="secondary" onClick={() => setNavAction("Settings module")} />
              </>
            }
          />
          {navAction ? (
            <p style={hintStyle}>Last action: <strong>{navAction}</strong></p>
          ) : null}
        </SpecimenGroup>

        <SpecimenGroup label="Hierarchy and count">
          <PageHeader
            title="Button"
            eyebrow="Primitive"
            subtitle="Primary interactive control for triggering actions."
            actions={
              <>
                <IconButton icon="code" ariaLabel="View source" variant="secondary" onClick={() => setHierarchyAction("View source")} />
                <IconButton icon="pencil" ariaLabel="Edit" variant="secondary" onClick={() => setHierarchyAction("Edit")} />
              </>
            }
          />
          <PageHeader title="Users" count={128} backHref="/dashboard" backLabel="Dashboard" />
          {hierarchyAction ? (
            <p style={hintStyle}>Last action: <strong>{hierarchyAction}</strong></p>
          ) : null}
        </SpecimenGroup>

        <SpecimenGroup label="Contextual status">
          <PageHeader
            section="Scheduled Task"
            title="Nightly Sync"
            backHref="/system/tasks"
            backLabel="Tasks"
            backIsContextual={true}
            bannerMessage="This task is currently paused."
            bannerTone="warning"
            actions={
              <>
                <IconButton icon="play" ariaLabel="Run now" variant="secondary" onClick={() => setStatusAction("Run now")} />
                <IconButton icon="pencil" ariaLabel="Edit" variant="secondary" onClick={() => setStatusAction("Edit task")} />
              </>
            }
          />
          {statusAction ? (
            <p style={hintStyle}>Last action: <strong>{statusAction}</strong></p>
          ) : null}
        </SpecimenGroup>

        <SpecimenGroup label="Operational metadata">
          <PageHeader
            title="Nightly Sync"
            section="Scheduled Task"
            backHref="/system/tasks"
            backLabel="Tasks"
            meta={
              <MetaBar>
                <Pill tone="success" appearance="badge">Active</Pill>
                <span style={metaTextStyle}>Every 6 hours</span>
                <span style={metaTextStyle}>Last run <TimeAgo datetime="2026-03-30T08:15:00Z" /></span>
              </MetaBar>
            }
            actions={
              <>
                <IconButton icon="play" ariaLabel="Run now" variant="secondary" onClick={() => setMetaAction("Run now")} />
                <IconButton icon="calendar" ariaLabel="Edit schedule" variant="secondary" onClick={() => setMetaAction("Edit schedule")} />
              </>
            }
          />
          {metaAction ? (
            <p style={hintStyle}>Last action: <strong>{metaAction}</strong></p>
          ) : null}
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
