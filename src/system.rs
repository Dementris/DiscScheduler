pub mod system {
    use crate::process::process::{ProcessManager};
    use crate::scheduler::scheduler::{Scheduler, Request};
    use crate::{Disk, LfuCache};

    pub struct System<S: Scheduler> {
        pub process_manager: ProcessManager,
        pub scheduler: S,
        pub quantum_time: u32,
        pub current_time: u32,
        pub cache: LfuCache,
        pub disk: Disk,
    }

    impl<S: Scheduler> System<S> {
        pub fn new(scheduler: S, cache: LfuCache, disk: Disk, quantum_time: u32) -> Self {
            Self {
                process_manager: ProcessManager::new(),
                scheduler,
                quantum_time,
                current_time: 0,
                cache,
                disk,
            }
        }

        pub fn run(&mut self) {
            while !self.process_manager.run_q.is_empty() || !self.process_manager.sleep_q.is_empty() {

                if let Some(mut process) = self.process_manager.run_q.pop_front() {
                    println!("[Time {}] Running process {}.", self.current_time, process.id);

                    // Add all process requests to the scheduler queue
                    for request in &process.requests {
                        self.scheduler.add_request(request.clone());
                    }

                    while process.time_remaining > 0 && process.has_requests() {
                        if let Some(request) = process.next_request() {
                            println!(
                                "[Time {}] Process {} processing request for sector {} ({:?}).",
                                self.current_time, process.id, request.sector, request.operation
                            );

                            if self.cache.left.iter().any(|b| b.sector == request.sector)
                                || self.cache.middle.iter().any(|b| b.sector == request.sector)
                                || self.cache.right.iter().any(|b| b.sector == request.sector)
                            {
                                println!(
                                    "[Time {}] CACHE: Sector {} found in cache.",
                                    self.current_time, request.sector
                                );
                                self.current_time += 1;
                            } else {
                                println!(
                                    "[Time {}] CACHE: Sector {} not found in cache. Accessing disk.",
                                    self.current_time, request.sector
                                );
                                let (track, sector_offset) = self.disk.get_track_sector(request.sector);
                                let disk_time = self.disk.simulate_access(track, sector_offset);
                                println!(
                                    "[Time {}] DRIVER: Accessing track {}, sector {}. Time: {}ms.",
                                    self.current_time, track, sector_offset, disk_time
                                );
                                self.current_time += disk_time;
                                self.cache.access(request.sector);
                            }

                            process.time_remaining -= 1;
                        } else {
                            break;
                        }
                        if let Some(next_request) = self.scheduler.get_next_request(self.current_time) {
                            println!(
                                "[Time {}] SCHEDULER: Retrieved request for sector {} ({:?})",
                                self.current_time, next_request.sector, next_request.operation
                            );
                        }
                        self.scheduler.print_queue_status();
                    }

                    if process.has_requests() {
                        println!("[Time {}] Process {} moved to end of run queue.", self.current_time, process.id);
                        process.time_remaining = self.quantum_time;
                        self.process_manager.move_to_sleep(process.id);
                    } else {
                        println!("[Time {}] Process {} completed.", self.current_time, process.id);
                    }
                } else if !self.process_manager.sleep_q.is_empty() {
                    if let Some(sleeping_process) = self.process_manager.sleep_q.pop_front() {
                        println!("[Time {}] Waking up process {} from sleep queue.", self.current_time, sleeping_process.id);
                        self.process_manager.wake_up_process(sleeping_process.id);
                    }
                } else {
                    println!("\n[Time {}] SCHEDULER: No more requests to process.", self.current_time);
                    break;
                }
            }

            println!("\n[Time {}] All processes completed.", self.current_time);
        }
    }
}