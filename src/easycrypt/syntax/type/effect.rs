use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Effect {
    Memory,
    Storage,
    TransientStorage,
    Other,
}

impl Effect {
    pub const ALL_KINDS: &'static [Effect] = &[
        Effect::Memory,
        Effect::Storage,
        Effect::TransientStorage,
        Effect::Other,
    ];
}
impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Effect::Memory => "mem",
            Effect::Storage => "storage",
            Effect::TransientStorage => "transient_storage",
            Effect::Other => "context",
        })
    }
}
