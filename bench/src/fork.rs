#[cfg(unix)]
pub(crate) fn fork<T: serde::Serialize + serde::de::DeserializeOwned>(
    f: impl FnOnce() -> T,
) -> nix::Result<T> {
    let (read_fd, write_fd) = nix::unistd::pipe()?;

    match unsafe { nix::unistd::fork() }? {
        nix::unistd::ForkResult::Parent { child } => {
            let (received_bytes_sender, received_bytes_receiver) =
                std::sync::mpsc::channel::<bool>();
            let handle = std::thread::spawn(move || {
                nix::sys::wait::waitpid(child, None).unwrap();
                let received_bytes = received_bytes_receiver
                    .recv_timeout(std::time::Duration::from_secs(1))
                    .unwrap_or(false);

                if !received_bytes {
                    nix::unistd::write(write_fd, b"exited").unwrap();
                    return;
                }
            });

            let mut buff = [0u8; 8092];
            let len = nix::unistd::read(read_fd, &mut buff)?;

            let _ = received_bytes_sender.send(true);
            handle.join().unwrap();

            if &buff[..len] == b"panic" {
                panic!("Benchmark function panicked");
            } else if &buff[..len] == b"exited" {
                panic!("Benchmark function exited without sending benchmark data. The function may have ran out of memory and was killed by the OS.");
            }

            let t = serde_json::from_slice(&buff[..len]).unwrap();
            nix::unistd::close(read_fd)?;
            nix::unistd::close(write_fd)?;

            Ok(t)
        }
        nix::unistd::ForkResult::Child => {
            std::panic::set_hook({
                let default_hook = std::panic::take_hook();
                Box::new(move |panic_info| {
                    nix::unistd::write(write_fd, b"panic").unwrap();
                    default_hook(panic_info);
                })
            });

            let t = f();
            let t_json = serde_json::to_string(&t).unwrap();
            nix::unistd::write(write_fd, t_json.as_bytes())?;

            std::process::exit(0);
        }
    }
}

#[cfg(not(unix))]
pub(crate) fn fork<T: serde::Serialize + serde::de::DeserializeOwned>(
    f: impl FnOnce() -> T,
) -> nix::Result<T> {
    Ok(f())
}
