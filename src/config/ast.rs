#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Permit,
    Deny,
}

#[derive(Debug, Clone, Default)]
pub struct RuleOptions {
    pub nopass: bool,
    pub persist: bool,
    pub keepenv: bool,
    pub nolog: bool,
    pub setenv: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub action: Action,
    pub options: RuleOptions,
    pub identity: String,
    pub target: String,
    pub cmd: Option<String>,
    pub args: Option<Vec<String>>,
}
