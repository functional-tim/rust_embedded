#[derive(Clone, Copy, Debug)]
pub enum Counter {
    AddFiveMinutes,
    AddOneMinute,
    Reset,
    None,
}

pub struct Controller {
    pub counter: u16,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            counter: 0,
        }
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }

    pub fn add_one_minute(&mut self) {
        if self.counter <= 1860 {
            self.counter += 60;
        }
    }

    pub fn add_five_minutes(&mut self) {
        if self.counter <= 1620 {
            self.counter += 300;
        }
    }

    pub fn count_down(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }
    }

    pub fn step(&mut self, counter: Counter) {
        match counter {
            Counter::AddFiveMinutes => self.add_five_minutes(),
            Counter::AddOneMinute => self.add_one_minute(),
            Counter::Reset => self.reset(),
            Counter::None => (),
        }
        self.count_down();
    }
}
