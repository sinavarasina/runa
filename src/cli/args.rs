use std::env;

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

pub fn parse() -> Result<RunaArgs, String> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut out = RunaArgs::default();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];

        if !arg.starts_with("-") || arg == "-" {
            out.command = args.split_off(i);
            break;
        }

        if arg == "--" {
            i += 1;
            if i < args.len() {
                let _ = out.command.split_off(i);
                break;
            }
        }

        let chars: Vec<char> = arg.chars().skip(1).collect();

        for (idx, &c) in chars.iter().enumerate() {
            match c {
                'L' => out.clear_timestamp = true,
                'n' => out.non_interactive = true,
                's' => out.shell = true,
                'C' | 'u' => {
                    let target_field = if c == 'C' {
                        &mut out.conf_path
                    } else {
                        &mut out.user
                    };
                    let err_msg = if c == 'C' {
                        "-C arguments required path"
                    } else {
                        "-C arguments required user"
                    };

                    let value = if idx + 1 < chars.len() {
                        let val: String = chars[idx + 1..].iter().collect();
                        Some(val)
                    } else {
                        i += 1;
                        if i >= args.len() {
                            return Err(err_msg.to_string());
                        }
                        Some(args[i].clone())
                    };

                    *target_field = value;
                    break;
                }
                _ => return Err(format!("undefined arguments: -{}", c)),
            }
        }
        i += 1;
    }

    Ok(out)
}
