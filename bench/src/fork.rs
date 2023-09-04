#[cfg(unix)]
pub(crate) fn fork<T: serde::Serialize + serde::de::DeserializeOwned>(
    f: impl FnOnce() -> T,
) -> nix::Result<T> {
    let (read_fd, write_fd) = nix::unistd::pipe()?;

    match unsafe { nix::unistd::fork() }? {
        nix::unistd::ForkResult::Parent { .. } => {
            let mut buff = [0u8; 8092];
            let len = nix::unistd::read(read_fd, &mut buff)?;
            let t = serde_json::from_slice(&buff[..len]).unwrap();
            nix::unistd::close(read_fd)?;
            nix::unistd::close(write_fd)?;

            Ok(t)
        }
        nix::unistd::ForkResult::Child => {
            let t = f();
            let t_json = serde_json::to_string(&t).unwrap();
            nix::unistd::write(write_fd, t_json.as_bytes())?;

            std::process::exit(0);
        }
    }
}

#[cfg(not(unix))]
pub(crate) fn fork<T: serde::Serialize>(f: impl FnOnce() -> T) -> nix::Result<T> {
    Ok(f())
}
