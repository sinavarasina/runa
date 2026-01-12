use libc::STDERR_FILENO;

mod cli;
mod sys;

fn main() -> std::io::Result<()> {
    sys::proc::close_from(STDERR_FILENO + 1)?;
    let uid = sys::user::get_uid();
    std::println!("{:?}", uid);
    let args = match cli::args::parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Usage: runa [-Lns] [-C config] [-u user] command [args]");
            std::process::exit(1);
        }
    };

    if args.clear_timestamp {
        // TODO: implement the clear_timestamp code later.
        std::println!("clear timestamp triggered");
        std::process::exit(0);
    }

    if args.command.is_empty() && !args.shell {
        eprintln!("Usage: runa [arguments] command");
        std::process::exit(1);
    }
    std::println!("Arguments parsed: {:?}", args);
    std::println!("Target user: {:?}", args.user.unwrap_or("root".to_string()));
    std::println!("Command to run: {:?}", args.command);
    let user = match sys::user::get_user_by_uid(uid) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("runa: no passwd entry: {}", e);
            std::process::exit(1);
        }
    };
    std::println!("User: {:?}", user);

    let groups = match sys::user::get_groups() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("runa: can't get groups: {}", e);
            std::process::exit(1);
        }
    };
    std::println!("groups: {:?}", groups);

    Ok(())
}
