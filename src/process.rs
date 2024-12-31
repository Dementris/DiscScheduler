pub mod process {
    use crate::scheduler::scheduler::Request;
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct Process {
        pub id: u32,
        pub requests: Vec<Request>,
        pub time_remaining: u32, // Time remaining in current quantum
    }

    impl Process {
        pub fn new(id: u32, requests: Vec<Request>, quantum_time: u32) -> Self {
            Self {
                id,
                requests,
                time_remaining: quantum_time,
            }
        }

        pub fn has_requests(&self) -> bool {
            !self.requests.is_empty()
        }

        pub fn next_request(&mut self) -> Option<Request> {
            self.requests.pop()
        }
    }

    pub struct ProcessManager {
        pub run_q: VecDeque<Process>,
        pub sleep_q: VecDeque<Process>,
    }

    impl ProcessManager {
        pub fn new() -> Self {
            Self {
                run_q: VecDeque::new(),
                sleep_q: VecDeque::new(),
            }
        }

        pub fn add_process(&mut self, process: Process) {
            self.run_q.push_back(process);
        }

        pub fn move_to_sleep(&mut self, process_id: u32) {
            if let Some(pos) = self.run_q.iter().position(|p| p.id == process_id) {
                let process = self.run_q.remove(pos).unwrap();
                println!(
                    "[ProcessManager] Moving process {} to sleep queue.",
                    process.id
                );
                self.sleep_q.push_back(process);
            } else {
                println!(
                    "[ProcessManager] Process {} not found in run queue.",
                    process_id
                );
            }
        }

        pub fn wake_up_process(&mut self, process_id: u32) {
            if let Some(pos) = self.sleep_q.iter().position(|p| p.id == process_id) {
                let process = self.sleep_q.remove(pos).unwrap();
                println!(
                    "[ProcessManager] Waking up process {} and moving to run queue.",
                    process.id
                );
                self.run_q.push_back(process);
            } else {
                println!(
                    "[ProcessManager] Process {} not found in sleep queue.",
                    process_id
                );
            }
        }
    }
}
