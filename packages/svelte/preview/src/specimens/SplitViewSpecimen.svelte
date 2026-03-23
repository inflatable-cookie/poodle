<script lang="ts">
  import { Eyebrow, Region } from "@poodle/svelte-primitives";
  import { SplitView } from "@poodle/svelte-composites";
</script>

<div class="specimen">
  <!-- 1. Simple layout with Regions -->
  <div class="specimen__group">
    <Eyebrow>Basic horizontal layout</Eyebrow>
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
  </div>

  <div class="specimen__group">
    <Eyebrow>Basic vertical layout</Eyebrow>
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
  </div>

  <!-- 2. Horizontal with collapse toggles -->
  <div class="specimen__group">
    <Eyebrow>Horizontal with collapse toggles (drag to edge to collapse)</Eyebrow>
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
  </div>

  <!-- 3. Vertical with collapse toggles -->
  <div class="specimen__group">
    <Eyebrow>Vertical with collapse toggles</Eyebrow>
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
  </div>

  <!-- 4. Nested splits (IDE-style layout) -->
  <div class="specimen__group">
    <Eyebrow>Nested splits (IDE-style layout)</Eyebrow>
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
  </div>

  <!-- 5. Disabled -->
  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <div class="specimen__frame">
      <SplitView
        orientation="horizontal"
        isDisabled
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
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
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
