use libc::STDERR_FILENO;

mod cli;
mod config;
mod sys;

fn main() -> std::io::Result<()> {
    sys::proc::close_from(STDERR_FILENO + 1)?;
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

    Ok(())
}
