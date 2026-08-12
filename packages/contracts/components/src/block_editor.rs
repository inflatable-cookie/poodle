use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BlockEditorMode {
    Single,
    #[default]
    Multi,
}

/// A single block type definition (provided by the consumer).
///
/// Defines the schema for one kind of block: its type identifier, display label,
/// and icon name. The consumer owns the full list of block types — BlockEditor
/// is a pure shell with no built-in types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockTypeDefinition {
    /// Unique type identifier (e.g. "heading", "paragraph", "code").
    pub block_type: String,
    /// Display label (e.g. "Heading").
    pub label: String,
    /// Icon name (e.g. "heading-1", "text", "code").
    pub icon: String,
}

impl BlockTypeDefinition {
    pub fn new(
        block_type: impl Into<String>,
        label: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        Self {
            block_type: block_type.into(),
            label: label.into(),
            icon: icon.into(),
        }
    }
}

/// A single block in the editor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditorBlock {
    /// Unique block identifier (e.g. UUID).
    pub id: String,
    /// Block type matching a BlockTypeDefinition.
    pub block_type: String,
    /// Optional block schema version.
    pub version: Option<String>,
    /// Optional content hash or revision marker.
    pub hash: Option<String>,
    /// Optional legacy fallback content string.
    pub content: Option<String>,
}

impl EditorBlock {
    pub fn new(id: impl Into<String>, block_type: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            block_type: block_type.into(),
            version: None,
            hash: None,
            content: None,
        }
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }
}

/// BlockEditor — pure shell for a block-based editor.
///
/// The shell provides the chrome (toolbar, drag-grip, type select, move/remove
/// controls) but delegates all block content rendering and type definitions to
/// the consumer via slots and block types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockEditorSpec {
    /// Block data (bound for two-way editing in Svelte; passed explicitly in GPUI).
    pub blocks: Vec<EditorBlock>,
    /// Block type definitions — consumer provides all available types.
    pub block_types: Vec<BlockTypeDefinition>,
    /// Disables all editing controls.
    pub is_disabled: bool,
    /// Accessible label for the root container.
    pub aria_label: String,
    pub mode: BlockEditorMode,
    pub allow_reorder: Option<bool>,
    pub allow_add: Option<bool>,
    pub allow_remove: Option<bool>,
    pub allow_type_change: Option<bool>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for BlockEditorSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockEditorSpec {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            block_types: Vec::new(),
            is_disabled: false,
            aria_label: String::from("Block editor"),
            mode: BlockEditorMode::default(),
            allow_reorder: None,
            allow_add: None,
            allow_remove: None,
            allow_type_change: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_blocks(mut self, blocks: Vec<EditorBlock>) -> Self {
        self.blocks = blocks;
        self
    }

    pub fn with_block_types(mut self, block_types: Vec<BlockTypeDefinition>) -> Self {
        self.block_types = block_types;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn with_mode(mut self, mode: BlockEditorMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_allow_reorder(mut self, allow_reorder: bool) -> Self {
        self.allow_reorder = Some(allow_reorder);
        self
    }

    pub fn with_allow_add(mut self, allow_add: bool) -> Self {
        self.allow_add = Some(allow_add);
        self
    }

    pub fn with_allow_remove(mut self, allow_remove: bool) -> Self {
        self.allow_remove = Some(allow_remove);
        self
    }

    pub fn with_allow_type_change(mut self, allow_type_change: bool) -> Self {
        self.allow_type_change = Some(allow_type_change);
        self
    }

    /// Legacy: returns the number of blocks (kept for back-compat with existing call sites).
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Legacy: set block count by creating placeholder blocks. Prefer with_blocks().
    pub fn with_block_count(mut self, block_count: usize) -> Self {
        self.blocks = (0..block_count)
            .map(|i| EditorBlock::new(format!("block-{}", i), "paragraph"))
            .collect();
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn block_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
