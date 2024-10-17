// Modules /////////////////////////////////////////////////////////////////////
mod env;
mod error;
mod git;
mod log;
mod executor;

// Imports /////////////////////////////////////////////////////////////////////
use env::Env;
use error::Error;
use git::Git;
use log::Logger;
use executor::ExecutorAlg12;

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
    }).unwrap();

    // Initialize executor and run auto-runner loop.
    let mut exec = ExecutorAlg12::new(env, &log, git);

    while running.load(Ordering::SeqCst) {
        let result = exec.run_next();

        if let Err(e) = result {
            log.err(&format!("{e}"));
            break;
        }
    }
}

#[allow(unused)]
fn csv() {
    #[derive(serde::Serialize)]
    struct Data {
        a: i32,
        b: f32,
        // c: Vec<i32>,
    }

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_writer(std::io::stdout());

    wtr.serialize(Data {
        a: 1,
        b: 2.4,
        // c: vec![1,2,3]
    })
    .unwrap();

    wtr.flush().unwrap();
}

////////////////////////////////////////////////////////////////////////////////
