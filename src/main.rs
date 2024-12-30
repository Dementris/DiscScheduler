mod cache;
mod scheduler;
mod disk;
mod system;

use cache::cache::LfuCache;
use disk::disk::Disk;
use crate::scheduler::scheduler::{IoOperation, Request, FifoScheduler};
use system::system::System;

fn main() {


    let disk = Disk::new(10000, 500, 1, 10, 4, 1);
    let cache = LfuCache::new(10, 4, 3);

    let scheduler = FifoScheduler::new();
    let mut system = System::new(scheduler, cache, disk, 20);

    system.add_to_run_q(Request {
        sector: 100,
        operation: IoOperation::Read,
    });
    system.add_to_run_q(Request {
        sector: 200,
        operation: IoOperation::Write,
    });
    system.add_to_run_q(Request {
        sector: 300,
        operation: IoOperation::Read,
    });

    system.process_requests();
}