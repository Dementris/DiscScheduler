pub mod scheduler {
    use std::collections::VecDeque;

    #[derive(Debug, Clone)]
    pub enum IoOperation {
        Read,
        Write,
    }

    #[derive(Debug, Clone)]
    pub struct Request {
        pub sector: u32,
        pub operation: IoOperation,
    }

    pub trait Scheduler {
        fn add_request(&mut self, request: Request);
        fn get_next_request(&mut self, current_sector: u32) -> Option<Request>;
        fn print_queue_status(&self);
    }

    pub struct LookScheduler {
        pub queue: Vec<Request>,
        pub direction: bool,
    }

    impl LookScheduler {
        pub fn new() -> Self {
            Self {
                queue: Vec::new(),
                direction: true,
            }
        }
    }

    impl Scheduler for LookScheduler {
        fn add_request(&mut self, request: Request) {
            println!(
                "[LOOK] Adding request for sector {} ({:?})",
                request.sector, request.operation
            );
            self.queue.push(request);
            self.queue.sort_by_key(|req| req.sector);
        }

        fn get_next_request(&mut self, current_sector: u32) -> Option<Request> {
            if self.direction {
                if let Some(pos) = self
                    .queue
                    .iter()
                    .position(|req| req.sector >= current_sector)
                {
                    let request = self.queue.remove(pos);
                    println!(
                        "[LOOK] Serving request at sector {} moving OUT",
                        request.sector
                    );
                    return Some(request);
                }
                self.direction = false;
                println!("[LOOK] Changing direction to IN");
            }

            if let Some(pos) = self
                .queue
                .iter()
                .rposition(|req| req.sector <= current_sector)
            {
                let request = self.queue.remove(pos);
                println!(
                    "[LOOK] Serving request at sector {} moving IN",
                    request.sector
                );
                return Some(request);
            }
            self.direction = true;
            println!("[LOOK] Changing direction to OUT");
            None
        }

        fn print_queue_status(&self) {
            if self.queue.is_empty() {
                println!("[LOOK] Queue is empty.");
            } else {
                println!("[LOOK] Current queue status:");
                for (i, request) in self.queue.iter().enumerate() {
                    println!(
                        "  [{}] Sector {} ({:?})",
                        i + 1,
                        request.sector,
                        request.operation
                    );
                }
            }
        }
    }

    pub struct FlookScheduler {
        pub active_queue: Vec<Request>,
        pub waiting_queue: Vec<Request>,
        pub direction: bool, // true for OUT (increasing), false for IN (decreasing)
    }
    impl FlookScheduler {
        pub fn new() -> Self {
            Self {
                active_queue: Vec::new(),
                waiting_queue: Vec::new(),
                direction: true,
            }
        }
    }
    impl Scheduler for FlookScheduler {
        fn add_request(&mut self, request: Request) {
            println!(
                "[FLOOK] Adding request for sector {} ({:?})",
                request.sector, request.operation
            );
            self.waiting_queue.push(request);
        }

        fn get_next_request(&mut self, current_sector: u32) -> Option<Request> {
            if self.active_queue.is_empty() {
                println!("[FLOOK] Switching active and waiting queues.");
                std::mem::swap(&mut self.active_queue, &mut self.waiting_queue);
                self.active_queue.sort_by_key(|req| req.sector);
            }

            if self.direction {
                if let Some(pos) = self
                    .active_queue
                    .iter()
                    .position(|req| req.sector >= current_sector)
                {
                    let request = self.active_queue.remove(pos);
                    println!(
                        "[FLOOK] Serving request at sector {} moving OUT",
                        request.sector
                    );
                    return Some(request);
                }
                self.direction = false;
                println!("[FLOOK] Changing direction to IN");
            }

            if let Some(pos) = self
                .active_queue
                .iter()
                .rposition(|req| req.sector <= current_sector)
            {
                let request = self.active_queue.remove(pos);
                println!(
                    "[FLOOK] Serving request at sector {} moving IN",
                    request.sector
                );
                return Some(request);
            }
            self.direction = true;
            println!("[FLOOK] Changing direction to OUT");
            None
        }

        fn print_queue_status(&self) {
            println!("[FLOOK] Active queue:");
            if self.active_queue.is_empty() {
                println!("  Empty");
            } else {
                for (i, request) in self.active_queue.iter().enumerate() {
                    println!(
                        "  [{}] Sector {} ({:?})",
                        i + 1,
                        request.sector,
                        request.operation
                    );
                }
            }

            println!("[FLOOK] Waiting queue:");
            if self.waiting_queue.is_empty() {
                println!("  Empty");
            } else {
                for (i, request) in self.waiting_queue.iter().enumerate() {
                    println!(
                        "  [{}] Sector {} ({:?})",
                        i + 1,
                        request.sector,
                        request.operation
                    );
                }
            }
        }
    }

    pub struct FifoScheduler {
        queue: VecDeque<Request>,
    }

    impl FifoScheduler {
        pub fn new() -> Self {
            Self {
                queue: VecDeque::new(),
            }
        }
    }

    impl Scheduler for FifoScheduler {
        fn add_request(&mut self, request: Request) {
            println!(
                "[FIFO] Adding request for sector {} ({:?})",
                request.sector, request.operation
            );
            self.queue.push_back(request);
        }

        fn get_next_request(&mut self, _current_sector: u32) -> Option<Request> {
            if let Some(request) = self.queue.pop_front() {
                println!(
                    "[FIFO] Serving request at sector {} ({:?})",
                    request.sector, request.operation
                );
                Some(request)
            } else {
                println!("[FIFO] No requests to serve.");
                None
            }
        }
        fn print_queue_status(&self) {
            if self.queue.is_empty() {
                println!("[FIFO] Queue is empty.");
            } else {
                println!("[FIFO] Current queue status:");
                for (i, request) in self.queue.iter().enumerate() {
                    println!(
                        "  [{}] Sector {} ({:?})",
                        i + 1,
                        request.sector,
                        request.operation
                    );
                }
            }
        }
    }
}
