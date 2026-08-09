# Admin App Shell Recipes

Reusable app-shell composition rules for Poodle-based admin and back-office
apps.

## Purpose

Use this guide when you need the visible shell structure for an admin app:
sidebar, mobile header, content region, toasts, and optional context panel.
Keep auth, route protection, and navigation/runtime orchestration outside this
guide in host code.

## Default Posture

- use Poodle surfaces directly for visible shell regions
- keep the shell app-local even when multiple apps share the same rough shape
- use `ToastHost` directly in the layout
- use `Drawer`, `Card`, `ScrollShell`, `Button`, `Separator`, and `PageHeader`
  as the shell building blocks
- keep nav trees, auth/session state, and route context in host code

## Reference Implementations

Use the ACME app layouts in the separate `underlay-reference` repository as
the concrete shell reference family, especially the admin app layout, admin
account layout, and public project-detail layout.

## Recommended Layout Shape

```svelte
<script lang="ts">
  import { ToastHost } from "@inflatable-cookie/poodle-svelte";
  import { Button, Card, Drawer, ScrollShell, Separator } from "@inflatable-cookie/poodle-svelte";

  let mobileNavOpen = false;
  let contextPanelOpen = false;
</script>

<div class="admin-shell">
  <header class="admin-shell__mobile-header">
    <Button
      type="button"
      variant="ghost"
      size="sm"
      onclick={() => {
        mobileNavOpen = true;
      }}
    >
      Menu
    </Button>

    <Button
      type="button"
      variant="ghost"
      size="sm"
      onclick={() => {
        contextPanelOpen = !contextPanelOpen;
      }}
    >
      Tools
    </Button>
  </header>

  <aside class="admin-shell__sidebar">
    <ScrollShell>
      <Card padding="sm">
        <!-- host-owned brand -->
      </Card>

      <Separator />

      <!-- host-owned nav tree -->
      <!-- host-owned user menu -->
    </ScrollShell>
  </aside>

  <main class="admin-shell__content">
    <ScrollShell>
      <!-- routed page content -->
    </ScrollShell>
  </main>

  {#if contextPanelOpen}
    <aside class="admin-shell__context-panel">
      <Card padding="md">
        <!-- host-owned context tools -->
      </Card>
    </aside>
  {/if}

  <Drawer open={mobileNavOpen} onOpenChange={(nextOpen) => (mobileNavOpen = nextOpen)} title="Navigation">
    <!-- host-owned mobile nav tree -->
  </Drawer>

  <ToastHost />
</div>
```

## Shell Rules

- keep one routed content region
- keep mobile navigation and desktop sidebar as the same host-owned nav content
- keep the context panel optional and app-owned
- keep toast placement at the layout root
- use Poodle surfaces for hierarchy and interaction, not a second shared shell
  wrapper

## What Stays Out

- auth/session refresh
- navigation context and redirect logic
- route protection
- entity-specific context tools
- app branding and nav vocabulary

Those stay in host code or retained runtime/client helpers.

## Related Guides

- [Page Shell And Admin Recipes](./011-page-shell-and-admin-recipes.md)
- [Admin Feature Delivery Recipes](./013-admin-feature-delivery-recipes.md)
- [List And Filter Recipes](./003-list-and-filter-recipes.md)
- [Dialog And Detail Recipes](./004-dialog-and-detail-recipes.md)
