mod cli;
mod config;
mod sys;

fn main() -> std::io::Result<()> {
    sys::proc::close_from(libc::STDERR_FILENO + 1)?;
    let uid = sys::user::get_uid();
    std::println!("{:?}", uid);

    let mut args = match cli::args::parse() {
        Ok(a) => a,
        Err(e) => {
            std::eprintln!("Error: {}", e);
            std::eprintln!("Usage: runa [-Lns] [-C config] [-u user] command [args]");
            std::process::exit(1);
        }
    };

    if args.clear_timestamp {
        // TODO: implement the clear_timestamp code later.
        std::println!("clear timestamp triggered");
        std::process::exit(0);
    }

    if args.command.is_empty() && !args.shell {
        std::eprintln!("Usage: runa [arguments] command");
        std::process::exit(1);
    }

    std::println!("Arguments parsed: {:?}", args);
    std::println!("Target user: {:?}", args.user.as_deref().unwrap_or("root"));
    std::println!("Command to run: {:?}", args.command);

    let user = match sys::user::get_user_by_uid(uid) {
        Ok(u) => u,
        Err(e) => {
            std::eprintln!("runa: no passwd entry: {}", e);
            std::process::exit(1);
        }
    };
    std::println!("User: {:?}", user);

    let groups = match sys::user::get_groups() {
        Ok(g) => g,
        Err(e) => {
            std::eprintln!("runa: can't get groups: {}", e);
            std::process::exit(1);
        }
    };
    std::println!("groups: {:?}", groups);

    let target_name = args.user.clone().unwrap_or_else(|| "root".to_string());

    let target_user = match sys::user::get_user_by_name(&target_name) {
        Ok(u) => u,
        Err(_) => {
            std::eprintln!("runa: unknown user {}", target_name);
            std::process::exit(1);
        }
    };

    if cfg!(debug_assertions) {
        std::println!("Target UID resolved: {:?}", target_user.uid);
    }

    if args.shell {
        let shell_env = std::env::var("SHELL");

        let target_shell = match shell_env {
            Ok(s) if !s.is_empty() => s,
            _ => user.shell.clone(),
        };

        args.command = vec![target_shell];
    }

    if args.command.is_empty() {
        std::eprintln!("Usage: runa [options] command [args]");
        std::process::exit(1);
    }

    let euid = sys::user::get_effective_uid();
    if !euid.is_root() {
        if cfg!(debug_assertions) {
            std::println!("runa is not running as root (EUID != 0). Config parsing might fail.");
        } else {
            std::eprintln!("runa: not installed setuid");
            // std::process::exit(1);
        }
    }

    let config_path = args
        .conf_path
        .as_deref()
        .unwrap_or("/home/alifatihfh/runa.conf");
    std::println!("Reading config from {:?}", config_path);

    let rules = match config::parser::parse_config_file(config_path) {
        Ok(r) => {
            std::println!("Parsing success, found {} rules", r.len());
            if cfg!(debug_assertions) {
                for (i, rule) in r.iter().enumerate() {
                    std::println!("Rule #{}: {:#?}", i + 1, rule);
                }
            }
            r
        }
        Err(e) => {
            std::eprintln!("Config Error: {}", e);
            std::process::exit(1);
        }
    };

    let cmd_prog = &args.command[0];
    let cmd_args = &args.command[1..];

    let cmd_resolved = match sys::path::resolve_command(cmd_prog) {
        Some(p) => p,
        None => {
            std::eprintln!("runa: command not found: {}", cmd_prog);
            std::process::exit(1);
        }
    };

    if cfg!(debug_assertions) {
        std::println!("Path Resolution: '{}' -> '{}'", cmd_prog, cmd_resolved);
    }

    let match_result = config::matcher::permit(
        &rules,
        &user,
        &groups,
        &target_name,
        &cmd_resolved,
        cmd_args,
    );

    match match_result {
        Some(rule) => {
            match rule.action {
                config::ast::Action::Permit => {
                    std::println!("Permit, Matched rule: {:#?}", rule);
                    // TODO: authentication
                }
                config::ast::Action::Deny => {
                    std::eprintln!("Denied, explicit denied rule found");
                    std::process::exit(1);
                }
            }
        }
        None => {
            std::eprintln!("Denied, no matching rule found");
            std::process::exit(1);
        }
    }

    Ok(())
}
