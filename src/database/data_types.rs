#[derive(Clone, PartialEq)]
pub struct ToolData {
    pub(crate) name: String,
}

#[derive(Clone, PartialEq)]
pub struct ScriptletData {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) command: String,
    pub(crate) description: String,
}
