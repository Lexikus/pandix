use std::time::SystemTime;

pub struct Tick {
    time_until_start: SystemTime,
    previous_time: SystemTime,
    delta_time: f32,
}

pub(crate) fn update(tick: &mut Tick) {
    let time = SystemTime::now();

    tick.delta_time = match time.duration_since(tick.previous_time) {
        Ok(time) => (time.as_millis() as f64 / 1000.0) as f32,
        Err(_) => 0.0,
    };

    tick.previous_time = SystemTime::now();
}

impl Tick {
    pub(crate) fn new() -> Tick {
        Tick {
            time_until_start: SystemTime::now(),
            previous_time: SystemTime::now(),
            delta_time: 0.0,
        }
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn time(&self) -> f32 {
        let time = SystemTime::now();
        match time.duration_since(self.time_until_start) {
            Ok(time) => time.as_secs() as f32,
            Err(_) => 0.0,
        }
    }
}
