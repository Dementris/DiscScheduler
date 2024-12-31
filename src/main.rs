
mod cache;
mod disk;
mod process;
mod scheduler;
mod system;

use cache::cache::LfuCache;
use disk::disk::Disk;
use process::process::Process;
use scheduler::scheduler::{FifoScheduler, IoOperation, Request};
use system::system::System;
use crate::scheduler::scheduler::{FlookScheduler, LookScheduler};

fn main() {
    let disk = Disk::new(5, 100, 2, 10, 4, 1);
    let cache = LfuCache::new(6, 2, 2);
    let scheduler = FlookScheduler::new();
    let mut system = System::new(scheduler, cache, disk, 20);

    let process1 = Process::new(
        1,
        vec![
            Request {
                sector: 100,
                operation: IoOperation::Read,
            },
            Request {
                sector: 200,
                operation: IoOperation::Write,
            },
        ],
        20,
    );

    let process2 = Process::new(
        2,
        vec![
            Request {
                sector: 100,
                operation: IoOperation::Write, // Overlapping sector 100
            },
            Request {
                sector: 300,
                operation: IoOperation::Read,
            },
        ],
        20,
    );

    let process3 = Process::new(
        3,
        vec![
            Request {
                sector: 150,
                operation: IoOperation::Read,
            },
            Request {
                sector: 250,
                operation: IoOperation::Write,
            },
            Request {
                sector: 350,
                operation: IoOperation::Read,
            },
        ],
        30,
    );

    let process4 = Process::new(
        4,
        vec![
            Request {
                sector: 50,
                operation: IoOperation::Write,
            },
            Request {
                sector: 75,
                operation: IoOperation::Read,
            },
            Request {
                sector: 400,
                operation: IoOperation::Write,
            },
        ],
        25,
    );

    let process5 = Process::new(
        5,
        vec![
            Request {
                sector: 20,
                operation: IoOperation::Read,
            },
            Request {
                sector: 500,
                operation: IoOperation::Write,
            },
        ],
        15,
    );


    system.process_manager.add_process(process1);
    system.process_manager.add_process(process2);
    system.process_manager.add_process(process3);
    system.process_manager.add_process(process4);
    system.process_manager.add_process(process5);

    println!("[LOG] Starting system execution.");
    system.run();
    println!("[LOG] System execution completed.");
}
