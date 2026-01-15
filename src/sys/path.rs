#[cfg(unix)]

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

#[cfg(unix)]
fn is_executable(path: &std::path::Path) {}
