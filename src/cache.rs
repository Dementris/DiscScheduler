pub mod cache {
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct Buffer {
        pub sector: u32,
        pub counter: u32,
    }

    pub struct LfuCache {
        pub left: VecDeque<Buffer>,
        pub middle: VecDeque<Buffer>,
        pub right: VecDeque<Buffer>,
        pub left_max: usize,
        pub middle_max: usize,
        pub total_buffers: usize,
    }

    impl LfuCache {
        pub fn new(total_buffers: usize, left_max: usize, middle_max: usize) -> Self {
            Self {
                left: VecDeque::new(),
                middle: VecDeque::new(),
                right: VecDeque::new(),
                left_max,
                middle_max,
                total_buffers,
            }
        }

        pub fn access(&mut self, sector: u32) {
            // Left segment
            if let Some(buffer) = self.left.iter_mut().find(|b| b.sector == sector) {
                println!(
                    "[LFU] Buffer for sector {} found in left segment. Counter: {}",
                    sector, buffer.counter
                );
                buffer.counter += 1;
                return;
            }

            // Middle segment
            if let Some(pos) = self.middle.iter().position(|b| b.sector == sector) {
                let mut buffer = self.middle.remove(pos).unwrap();
                println!(
                    "[LFU] Buffer for sector {} moved from middle to left. Counter: {}",
                    sector, buffer.counter
                );
                buffer.counter += 1;
                self.move_to_left(buffer);
                return;
            }

            // Right segment
            if let Some(pos) = self.right.iter().position(|b| b.sector == sector) {
                let mut buffer = self.right.remove(pos).unwrap();
                println!(
                    "[LFU] Buffer for sector {} moved from right to left. Counter: {}",
                    sector, buffer.counter
                );
                buffer.counter += 1;
                self.move_to_left(buffer);
                return;
            }

            // Buffer not found
            println!("[LFU] Buffer for sector {} not found. Adding to left.", sector);
            self.add_to_left(Buffer { sector, counter: 1 });
        }

        fn move_to_left(&mut self, buffer: Buffer) {
            self.left.push_front(buffer);
            if self.left.len() > self.left_max {
                let buffer = self.left.pop_back().unwrap();
                self.move_to_middle(buffer);
            }
        }

        fn move_to_middle(&mut self, buffer: Buffer) {
            self.middle.push_front(buffer);
            if self.middle.len() > self.middle_max {
                let buffer = self.middle.pop_back().unwrap();
                self.move_to_right(buffer);
            }
        }

        fn move_to_right(&mut self, buffer: Buffer) {
            self.right.push_front(buffer);
            if self.right.len() > self.total_buffers - self.left_max - self.middle_max {
                if let Some((min_index, _)) = self
                    .right
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, buf)| (buf.counter, buf.sector))
                {
                    println!(
                        "[LFU] Buffer for sector {} evicted from cache due to minimum counter {}.",
                        self.right[min_index].sector, self.right[min_index].counter
                    );
                    self.right.remove(min_index);
                }
            }
        }

        fn add_to_left(&mut self, buffer: Buffer) {
            self.move_to_left(buffer);
        }
    }
}