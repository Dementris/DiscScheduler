// main.rs

mod cache;
mod scheduler;
mod disk;
mod system;
mod process;

use cache::cache::LfuCache;
use disk::disk::Disk;
use scheduler::scheduler::{IoOperation, Request, FifoScheduler};
use system::system::System;
use process::process::Process;

fn main() {
    // Initialize disk, cache, and scheduler
    let disk = Disk::new(10000, 500, 1, 10, 4, 1);
    let cache = LfuCache::new(10, 4, 3);
    let scheduler = FifoScheduler::new();

    let mut system = System::new(scheduler, cache, disk, 20);

    // Add processes and their requests
    let process1 = Process::new(1, vec![
        Request {
            sector: 100,
            operation: IoOperation::Read,
        },
        Request {
            sector: 200,
            operation: IoOperation::Write,
        },
    ], 20);

    let process2 = Process::new(2, vec![
        Request {
            sector: 300,
            operation: IoOperation::Read,
        },
        Request {
            sector: 400,
            operation: IoOperation::Write,
        },
    ], 20);

    let process3 = Process::new(3, vec![
        Request {
            sector: 500,
            operation: IoOperation::Read,
        },
        Request {
            sector: 600,
            operation: IoOperation::Write,
        },
    ], 20);

    let process4 = Process::new(4, vec![
        Request {
            sector: 700,
            operation: IoOperation::Read,
        },
        Request {
            sector: 800,
            operation: IoOperation::Write,
        },
    ], 20);

    let process5 = Process::new(5, vec![
        Request {
            sector: 900,
            operation: IoOperation::Read,
        },
        Request {
            sector: 1000,
            operation: IoOperation::Write,
        },
    ], 20);

    // Add processes to the system's process manager
    system.process_manager.add_process(process1);
    system.process_manager.add_process(process2);
    system.process_manager.add_process(process3);
    system.process_manager.add_process(process4);
    system.process_manager.add_process(process5);

    // Run the system
    println!("[LOG] Starting system execution.");
    system.run();
    println!("[LOG] System execution completed.");
}
