// Modules /////////////////////////////////////////////////////////////////////
mod env;
mod error;
mod executor_alg_12;
mod git;
mod log;

// Imports /////////////////////////////////////////////////////////////////////
use env::Env;
use error::Error;
use executor_alg_12::ExecutorAlg12;
use git::Git;
use log::Logger;

// use pushover::{requests::message::SendMessage, API};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    // Load environment variables, initialize logger and open git repo.
    let env = Env::load().unwrap();
    let log = Logger::new(&env);
    let git = Git::open_repo(&env).unwrap();

    // Setup graceful stop
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up the Ctrl+C signal handler
    ctrlc::set_handler(move || {
        println!(": Ctrl+C pressed! Gracefully stopping...");
        r.store(false, Ordering::SeqCst); // Set the flag to false
    })
    .unwrap();

    // Initialize executor and run auto-runner loop.
    let mut exec = ExecutorAlg12::new(env);

    while running.load(Ordering::SeqCst) {
        // Execute algorithm
        let result = exec.run_next();

        // If error -> log and exit
        if let Err(e) = result {
            log.err(&format!("{e}"));
            println!("[ERR] {e}");
            break;
        }

        // Commit changes and push "plots" repo
        let upload = git_upload_data(&git);
        if let Err(e) = upload {
            log.err(&format!("{e}"));
            println!("[ERR] {e}");
        }

        // Log success
        log.success("Execution finished");

        // Wait some time for the computer to cool down
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!();
    }
}

fn git_upload_data(git: &Git) -> Result<(), Error> {
    git.fetch()?;
    git.rebase()?;
    git.add_all()?;
    git.commit()?;
    git.push()?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
