#[derive(Debug)]
pub struct RunaArgs {
    pub conf_path: Option<String>,
    pub user: Option<String>,
    pub clear_timestamp: bool,
    pub non_interactive: bool,
    pub shell: bool,
    pub command: Vec<String>,
}

impl Default for RunaArgs {
    fn default() -> Self {
        Self {
            conf_path: None,
            user: None,
            clear_timestamp: false,
            non_interactive: false,
            shell: false,
            command: Vec::new(),
        }
    }
}
