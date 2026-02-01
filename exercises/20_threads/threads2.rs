// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    os::macos::raw::stat,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        // Arc (Atomic Reference Counter) get reference, increment reference count.
        let status_shared_mutex = Arc::clone(&status);

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Mutex manage conncurent updates to an object, to lock object to update.
            // The book states, "If another thread holding a lock paniced then lock would also
            // panic for other threads" to justify using unwrap here
            let mut status_shared = status_shared_mutex.lock().unwrap();

            // TODO: You must take an action before you update a shared value.
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.try_lock().unwrap().jobs_done);
}
