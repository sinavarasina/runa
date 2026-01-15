pub fn close_from(min_fd: libc::c_int) -> std::io::Result<()> {
    // if the os is linux they will run close_range, but if the close_range was not available they
    // will return ENOSYS and run the fallback method, if the return was not ENOSYS the programm will error,
    #[cfg(target_os = "linux")]
    {
        let max_fd = !0 as libc::c_uint;
        let ret = unsafe { libc::close_range(min_fd as libc::c_uint, max_fd, 0) };
        if ret == 0 {
            return Ok(());
        }
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() != Some(libc::ENOSYS) {
            return Err(err);
        }
    }

    // fallback (in case not linux/ENOSYS)
    let max_open = unsafe { libc::sysconf(libc::_SC_OPEN_MAX) as libc::c_int };
    if max_open < 0 {
        return Err(std::io::Error::last_os_error());
    }

    for fd in min_fd..max_open {
        unsafe {
            libc::close(fd);
        }
    }
    Ok(())
}
