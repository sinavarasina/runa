use crate::{
    config::ast::Rule,
    sys::user::{Gid, User, get_gid_by_name},
};

fn match_rule(
    rule: &Rule,
    user: &User,
    user_groups: &[Gid],
    target_user: &str,
    cmd: &str,
    cmd_args: &[String],
) -> bool {
    if rule.target != target_user {
        return false;
    }

    match (&rule.cmd, &rule.args) {
        (Some(rc), _) if rc != cmd => return false,
        (Some(_), Some(ra)) if ra != cmd_args => return false,
        _ => {}
    }

    match rule.identity.strip_prefix(':') {
        Some(group_name) => {
            let Ok(rule_gid) = get_gid_by_name(group_name) else {
                return false;
            };

            user_groups.contains(&rule_gid)
        }

        None => rule.identity == user.name,
    }
}

pub fn permit<'a>(
    rules: &'a [Rule],
    user: &User,
    user_groups: &[Gid],
    target_user: &str,
    cmd: &str,
    cmd_args: &[String],
) -> Option<&'a Rule> {
    rules
        .iter()
        .filter(|rule| match_rule(rule, user, user_groups, target_user, cmd, cmd_args))
        .last()
}
