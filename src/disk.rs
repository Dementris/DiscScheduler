pub mod disk {
    pub struct Disk {
        pub tracks_num: u32,
        pub sectors_per_track: u32,
        pub next_track_seek_time: u32,
        pub rewind_seek_time: u32,
        pub rotation_latency_time: u32,
        pub sector_access_time: u32,
    }

    impl Disk {
        pub fn new(
            tracks_num: u32,
            sectors_per_track: u32,
            next_track_seek_time: u32,
            rewind_seek_time: u32,
            rotation_latency_time: u32,
            sector_access_time: u32,
        ) -> Self {
            Self {
                tracks_num,
                sectors_per_track,
                next_track_seek_time,
                rewind_seek_time,
                rotation_latency_time,
                sector_access_time,
            }
        }

        pub fn simulate_access(&self, track: u32, sector: u32) -> u32 {
            let seek_time = track * self.next_track_seek_time;
            let rotational_latency = self.rotation_latency_time;
            let access_time = self.sector_access_time;
            seek_time + rotational_latency + access_time
        }
    }
}