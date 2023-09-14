use memory_stats::memory_stats;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

/// Fork is only enabled on cfg(unix).
/// If we ran this without fork, we would get incorrect results.
///
/// When used with fork, the memory is immediately freed after the fork,
/// so it doesn't affect the next benchmark run.
#[cfg(unix)]
pub(crate) fn monitor() -> impl FnOnce() -> Option<usize> {
    let stop_signal = Arc::new(AtomicBool::new(false));
    let handle = std::thread::spawn({
        let stop_signal = Arc::clone(&stop_signal);

        move || {
            let start_memory = memory_stats()?.physical_mem;
            let mut max_memory = start_memory;
            while !stop_signal.load(Ordering::Relaxed) {
                let memory = memory_stats()?.physical_mem;

                if memory > max_memory {
                    max_memory = memory;
                }

                std::thread::sleep(Duration::from_millis(1));
            }

            Some((start_memory, max_memory))
        }
    });

    move || {
        stop_signal.store(true, Ordering::Relaxed);
        let (start_memory, max_memory) = handle.join().unwrap()?;
        Some(max_memory - start_memory)
    }
}

#[cfg(not(unix))]
pub(crate) fn monitor() -> impl FnOnce() -> Option<usize> {
    move || None
}
