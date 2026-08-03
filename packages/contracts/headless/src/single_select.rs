//! Single-select machine (RadioGroup, SegmentedControl, TriStateSwitch).
//! Mirror of core `single-select.ts`.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    pub value: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleSelectContext {
    pub value: Option<String>,
    pub options: Vec<SelectOption>,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleSelectEvent {
    Select { value: String },
    SetValue { value: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleSelectEffect {
    EmitValueChange { value: String },
}

pub fn single_select_transition(
    context: SingleSelectContext,
    event: SingleSelectEvent,
) -> (SingleSelectContext, Vec<SingleSelectEffect>) {
    match event {
        SingleSelectEvent::Select { value } => {
            let option = context
                .options
                .iter()
                .find(|candidate| candidate.value == value);
            let same = context.value.as_deref() == Some(value.as_str());

            match option {
                Some(option) if !context.disabled && !option.disabled && !same => {
                    let effects = vec![SingleSelectEffect::EmitValueChange {
                        value: value.clone(),
                    }];

                    (
                        SingleSelectContext {
                            value: Some(value),
                            ..context
                        },
                        effects,
                    )
                }
                _ => (context, vec![]),
            }
        }
        SingleSelectEvent::SetValue { value } => (SingleSelectContext { value, ..context }, vec![]),
    }
}
