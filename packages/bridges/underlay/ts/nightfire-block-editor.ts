export type UnderlayNightfireBlock = Record<string, unknown> & {
  id?: string;
  type: string;
  version?: string | number;
  hash?: string | null;
  data?: Record<string, unknown>;
  content?: string;
};

export type UnderlayNightfireTypeOption = {
  type: string;
  label: string;
  icon?: string;
  category?: string | null;
  subcategory?: string;
};

export type UnderlayNightfireGroupedTypeOptions = {
  category: string | null;
  label: string;
  options: UnderlayNightfireTypeOption[];
};

export type UnderlayPoodleBlockTypeDefinition = {
  type: string;
  label: string;
  icon: string;
};

export type UnderlayPoodleBlockTypeGroup = {
  label: string;
  options: UnderlayPoodleBlockTypeDefinition[];
};

export type UnderlayPoodleBlockTypeItems =
  | UnderlayPoodleBlockTypeDefinition[]
  | UnderlayPoodleBlockTypeGroup[];

export type NightfireTypePickerMode = "built-in" | "slot-override";

export type NightfireBlockEditorBridge = {
  blockTypes: UnderlayPoodleBlockTypeDefinition[];
  blockTypeItems: UnderlayPoodleBlockTypeGroup[] | null;
  pickerMode: NightfireTypePickerMode;
};

export type NightfireBlockBridgeOptions = {
  fallbackType?: string;
  createId?: () => string;
};

const DEFAULT_BLOCK_ICON = "file-text";

export function toPoodleEditorBlock(
  block: UnderlayNightfireBlock,
  options: NightfireBlockBridgeOptions = {},
): UnderlayNightfireBlock & { id: string; type: string } {
  const fallbackType = options.fallbackType ?? "markdown";
  const createId = options.createId ?? defaultCreateId;
  const type = typeof block.type === "string" && block.type.length > 0 ? block.type : fallbackType;

  return {
    ...block,
    id: typeof block.id === "string" && block.id.length > 0 ? block.id : createId(),
    type,
  };
}

export function toPoodleEditorBlocks(
  blocks: UnderlayNightfireBlock[],
  options: NightfireBlockBridgeOptions = {},
): Array<UnderlayNightfireBlock & { id: string; type: string }> {
  return blocks.map((block) => toPoodleEditorBlock(block, options));
}

export function toPoodleBlockTypes(
  options: UnderlayNightfireTypeOption[],
): UnderlayPoodleBlockTypeDefinition[] {
  return options.map((option) => ({
    type: option.type,
    label: option.label,
    icon: option.icon ?? DEFAULT_BLOCK_ICON,
  }));
}

export function toPoodleBlockTypeItems(
  optionsOrGroups: UnderlayNightfireTypeOption[] | UnderlayNightfireGroupedTypeOptions[],
): UnderlayPoodleBlockTypeItems {
  if (isGroupedTypeOptions(optionsOrGroups)) {
    return optionsOrGroups.map((group) => ({
      label: group.label,
      options: toPoodleBlockTypes(group.options),
    }));
  }

  if (optionsOrGroups.some((option) => option.category)) {
    return groupNightfireTypeOptions(optionsOrGroups);
  }

  return toPoodleBlockTypes(optionsOrGroups);
}

export function buildNightfireBlockEditorBridge(input: {
  typeOptions: UnderlayNightfireTypeOption[];
  groupedOptions?: UnderlayNightfireGroupedTypeOptions[] | null;
}): NightfireBlockEditorBridge {
  const { typeOptions, groupedOptions = null } = input;
  const blockTypeItems = groupedOptions?.length
    ? toPoodleBlockTypeItems(groupedOptions)
    : typeOptions.some((option) => option.category)
      ? toPoodleBlockTypeItems(typeOptions)
      : null;

  const groupedItems = isPoodleBlockTypeGroupArray(blockTypeItems)
    ? blockTypeItems
    : null;

  return {
    blockTypes: toPoodleBlockTypes(typeOptions),
    blockTypeItems: groupedItems,
    pickerMode: nightfireTypePickerNeedsSlotOverride(groupedOptions ?? typeOptions)
      ? "slot-override"
      : "built-in",
  };
}

export function nightfireTypePickerNeedsSlotOverride(
  optionsOrGroups: UnderlayNightfireTypeOption[] | UnderlayNightfireGroupedTypeOptions[] | null | undefined,
): boolean {
  if (!optionsOrGroups?.length) return false;

  if (isGroupedTypeOptions(optionsOrGroups)) {
    return optionsOrGroups.some((group) =>
      group.options.some((option) => hasSubcategory(option)),
    );
  }

  return optionsOrGroups.some((option) => hasSubcategory(option));
}

function groupNightfireTypeOptions(
  options: UnderlayNightfireTypeOption[],
): UnderlayPoodleBlockTypeGroup[] {
  const groups = new Map<string, UnderlayNightfireTypeOption[]>();

  for (const option of options) {
    const label = option.category?.trim().length ? option.category : "Other";
    groups.set(label, [...(groups.get(label) ?? []), option]);
  }

  return Array.from(groups.entries()).map(([label, groupedOptions]) => ({
    label,
    options: toPoodleBlockTypes(groupedOptions),
  }));
}

function hasSubcategory(option: UnderlayNightfireTypeOption): boolean {
  return typeof option.subcategory === "string" && option.subcategory.trim().length > 0;
}

function isGroupedTypeOptions(
  optionsOrGroups: UnderlayNightfireTypeOption[] | UnderlayNightfireGroupedTypeOptions[],
): optionsOrGroups is UnderlayNightfireGroupedTypeOptions[] {
  return optionsOrGroups.length > 0 && "options" in optionsOrGroups[0];
}

function isPoodleBlockTypeGroupArray(
  items: UnderlayPoodleBlockTypeItems | null,
): items is UnderlayPoodleBlockTypeGroup[] {
  return Array.isArray(items) && items.length > 0 && "options" in items[0];
}

function defaultCreateId(): string {
  const cryptoApi = globalThis.crypto;
  if (cryptoApi?.randomUUID) {
    return cryptoApi.randomUUID();
  }

  return `nightfire-${Math.random().toString(36).slice(2, 10)}`;
}
