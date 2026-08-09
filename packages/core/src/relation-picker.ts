/** Framework-neutral item and drill-down contracts shared by relation pickers. */
export type PickerItem = {
  id: string;
  label: string;
  description?: string | null;
  meta?: string | null;
  disabled?: boolean;
};

export type PickerFilterOption = {
  id: string;
  label: string;
};

export type PickerFilterConfig = {
  key: string;
  label: string;
  options: PickerFilterOption[];
  includeAll?: boolean;
  allLabel?: string;
};

export type DrillDownItem = PickerItem & {
  count?: number;
  expandable?: boolean;
};

export type DrillDownContext = Record<string, string>;

export type DrillDownSearchFn = (
  query: string,
  context: DrillDownContext,
) => DrillDownItem[] | Promise<DrillDownItem[]>;

export type DrillDownLevel = {
  key: string;
  label: string;
  items: DrillDownItem[] | DrillDownSearchFn;
  searchPlaceholder?: string;
};

export type DrillDownItemsFn = (
  query: string,
  context: DrillDownContext,
) => PickerItem[] | Promise<PickerItem[]>;

export type DrillDownConfig = {
  levels: DrillDownLevel[];
  finalItems?: DrillDownItemsFn;
};
