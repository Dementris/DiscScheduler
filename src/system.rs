pub mod system {
    use crate::scheduler::scheduler::{Request, Scheduler};
    use crate::{Disk, LfuCache};
    use std::collections::VecDeque;

    pub struct System<S: Scheduler> {
        pub run_q: VecDeque<Request>,
        pub scheduler: S,
        pub quantum_time: u32,
        pub current_time: u32,
        pub cache: LfuCache,
        pub disk: Disk,
    }

    impl<S: Scheduler> System<S> {
        pub fn new(scheduler: S, cache: LfuCache, disk: Disk, quantum_time: u32) -> Self {
            Self {
                run_q: VecDeque::new(),
                scheduler,
                quantum_time,
                current_time: 0,
                cache,
                disk,
            }
        }

        pub fn add_to_run_q(&mut self, request: Request) {
            println!(
                "[Time {}] SCHEDULER: Adding process for sector {} ({:?}) to run queue.",
                self.current_time, request.sector, request.operation
            );
            self.scheduler.add_request(request);
        }

        pub fn process_requests(&mut self) {
            loop {
                // Відображення стану черги перед отриманням наступного запиту
                println!(
                    "[Time {}] SCHEDULER: Queue status before processing:",
                    self.current_time
                );
                self.scheduler.print_queue_status();

                // Отримання наступного запиту
                if let Some(request) = self.scheduler.get_next_request(0) {
                    println!(
                        "[Time {}] SCHEDULER: Processing request for sector {} ({:?}).",
                        self.current_time, request.sector, request.operation
                    );

                    // Перевірка в кеші
                    if self.cache.left.iter().any(|b| b.sector == request.sector)
                        || self.cache.middle.iter().any(|b| b.sector == request.sector)
                        || self.cache.right.iter().any(|b| b.sector == request.sector)
                    {
                        println!(
                            "[Time {}] CACHE: Sector {} found in cache.",
                            self.current_time, request.sector
                        );
                        self.current_time += 1; // Час доступу до кешу
                    } else {
                        println!(
                            "[Time {}] CACHE: Sector {} not found in cache. Accessing disk.",
                            self.current_time, request.sector
                        );

                        let track = request.sector / self.disk.sectors_per_track;
                        let sector_offset = request.sector % self.disk.sectors_per_track;

                        let disk_time = self.disk.simulate_access(track, sector_offset);
                        println!(
                            "[Time {}] DRIVER: Accessing track {}, sector {}. Time: {}ms.",
                            self.current_time, track, sector_offset, disk_time
                        );
                        self.current_time += disk_time;

                        self.cache.access(request.sector);
                    }

                    self.current_time += self.quantum_time; // Квант часу

                    // Відображення стану черги після обробки
                    println!(
                        "[Time {}] SCHEDULER: Queue status after processing:",
                        self.current_time
                    );
                    self.scheduler.print_queue_status();
                } else {
                    // Завершення обробки, якщо черга порожня
                    println!(
                        "[Time {}] SCHEDULER: No more requests to process.",
                        self.current_time
                    );
                    break;
                }
            }
        }
    }
}
