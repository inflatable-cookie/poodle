<script lang="ts">
  import { Region } from "@poodle/svelte";
  import { SplitView } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <!-- 1. Simple layout with Regions -->
  <SpecimenGroup label="Basic horizontal layout">
    <div class="specimen__frame">
      <SplitView orientation="horizontal">
        <div slot="primary" class="specimen__fill">
          <Region label="Sidebar" color="blue" />
        </div>
        <div slot="secondary" class="specimen__fill">
          <Region label="Main content" color="green" />
        </div>
      </SplitView>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Basic vertical layout">
    <div class="specimen__frame specimen__frame--tall">
      <SplitView orientation="vertical">
        <div slot="primary" class="specimen__fill">
          <Region label="Editor" color="blue" />
        </div>
        <div slot="secondary" class="specimen__fill">
          <Region label="Terminal" color="purple" />
        </div>
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 2. Horizontal with collapse toggles -->
  <SpecimenGroup label="Horizontal with collapse toggles (drag to edge to collapse)">
    <div class="specimen__frame">
      <SplitView
        orientation="horizontal"
        showCollapsePrimary
        showCollapseSecondary
      >
        <div slot="primary" class="specimen__fill">
          <Region label="Primary" color="blue" />
        </div>
        <div slot="secondary" class="specimen__fill">
          <Region label="Secondary" color="green" />
        </div>
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 3. Vertical with collapse toggles -->
  <SpecimenGroup label="Vertical with collapse toggles">
    <div class="specimen__frame specimen__frame--tall">
      <SplitView
        orientation="vertical"
        showCollapsePrimary
        showCollapseSecondary
      >
        <div slot="primary" class="specimen__fill">
          <Region label="Top" color="blue" />
        </div>
        <div slot="secondary" class="specimen__fill">
          <Region label="Bottom" color="purple" />
        </div>
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 4. Nested splits (IDE-style layout) -->
  <SpecimenGroup label="Nested splits (IDE-style layout)">
    <div class="specimen__frame specimen__frame--tall">
      <SplitView
        orientation="horizontal"
        ratio={0.25}
        showCollapsePrimary
      >
        <div slot="primary" class="specimen__fill">
          <Region label="Explorer" color="blue" />
        </div>
        <div slot="secondary" class="specimen__fill">
          <SplitView
            orientation="vertical"
            ratio={0.65}
            showCollapseSecondary
          >
            <div slot="primary" class="specimen__fill">
              <Region label="Editor" color="green" />
            </div>
            <div slot="secondary" class="specimen__fill">
              <Region label="Terminal" color="purple" />
            </div>
          </SplitView>
        </div>
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 5. Disabled -->
  <SpecimenGroup label="Disabled">
    <div class="specimen__frame">
      <SplitView
        orientation="horizontal"
        disabled
        showCollapsePrimary
        showCollapseSecondary
      >
        <div slot="primary" class="specimen__fill">
          <Region label="Left" color="blue" />
        </div>
        <div slot="secondary" class="specimen__fill">
          <Region label="Right" color="green" />
        </div>
      </SplitView>
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__frame {
    height: 10rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: visible;
  }

  .specimen__frame--tall {
    height: 16rem;
  }

  .specimen__fill {
    width: 100%;
    height: 100%;
  }

  .specimen__fill :global(.poodle-region) {
    height: 100%;
    min-height: 0;
  }
</style>
