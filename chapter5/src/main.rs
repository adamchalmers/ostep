use std::cmp::Ordering;

fn main() {
    q1();
}

fn q1() {
    let x;

    // The manpage for `fork` tells us how to interpret this. I quote:
    let result_code = unsafe { libc::fork() };

    match result_code.cmp(&0) {
        // "Upon successful completion, fork() returns a value of 0 to the child process
        Ordering::Equal => {
            x = 2;
            println!("Child. x = {x}")
        }
        // and returns the process ID of the child process to the parent process.
        Ordering::Greater => {
            x = 1;
            println!("Parent (spawned {result_code}). x = {x}");
        }
        // Otherwise, a value of -1 is returned
        // to the parent process, no child process is created, and the global variable errno is set
        // to indicate the error."
        Ordering::Less => {
            eprintln!("Fork error number {}.", nix::errno::errno());
            return;
        }
    }
}
