fn expand_tilde(path_str: &str) -> Option<std::path::PathBuf> {
    if path_str.starts_with("~") {
        if let Ok(home) = std::env::var("HOME") {
            let path_after_tilde = &path_str[1..];
            let path_buf = std::path::PathBuf::from(home);

            if path_after_tilde.is_empty() {
                return Some(path_buf);
            } else {
                let slash_trimmed = path_after_tilde.trim_start_matches('/');
                return Some(path_buf.join(slash_trimmed));
            }
        }
    }
    Some(std::path::PathBuf::from(path_str))
}

#[cfg(not(unix))]
fn is_executable(path: &std::path::Path) -> bool {
    path.exists() && path.is_file()
}

#[cfg(unix)]
fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(metadata) = path.metadata() {
        return metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0);
    }
    false
}

pub fn resolve_command(cmd: &str) -> Option<String> {
    let expanded_path = expand_tilde(cmd)?;
    let expanded_cmd = expanded_path.to_string_lossy();

    if expanded_cmd.contains('/') {
        let path = &expanded_path;
        if path.exists() && is_executable(path) {
            return std::fs::canonicalize(path)
                .ok()
                .map(|p| p.to_string_lossy().into_owned())
                .or_else(|| Some(expanded_cmd.into_owned()));
        }
        return None;
    }
    let path_env = std::env::var_os("PATH")?;
    for path in std::env::split_paths(&path_env) {
        let full_path = path.join(cmd);
        if full_path.exists() && is_executable(&full_path) {
            return Some(full_path.to_string_lossy().into_owned());
        }
    }
    None
}
