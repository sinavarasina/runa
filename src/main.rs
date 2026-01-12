use libc::STDERR_FILENO;

mod sys;

fn main() -> std::io::Result<()> {
    sys::proc::close_from(STDERR_FILENO + 1)?;
    let uid = sys::user::get_uid();
    std::print!("{:?}", uid);
    Ok(())
}
