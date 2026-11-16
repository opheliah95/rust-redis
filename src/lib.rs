use std::time::{Duration, Instant};

pub struct Data {
    pub value: String,
    pub ex: u64,
    pub px: u64,
    start: Instant,
    end: Instant,
}

impl Data {
    pub fn new (value: String, ex: u64, px: u64) -> Data {
        Data {
            value:value,
            ex: ex,
            px: px,
            start: Instant::now(),
            end: Instant::now()
        }
    }
    pub fn init_timer( &mut self) {
        self.set_start();
    }

    pub fn check_timer(&mut self) {
        //eprintln!("struct ex={} px={}", self.ex, self.px);
        if (self.ex == 0 && self.px == 0) {
            return;
        }
        
        if Instant::now() > self.end{
            self.value = "$-1\r\n".to_string();
        }
    }
    
    fn set_start(&mut self) {
        self.start = Instant::now();
        // when func will end
        self.set_expire();
    }

    fn set_expire(&mut self) {
        if self.ex != 0 {
            self.end = self.start + Duration::from_secs(self.ex);
        }

        if self.px != 0 {
            self.end = self.start + Duration::from_millis(self.px);
        }

        eprintln!("struct ex={} px={}", self.ex, self.px)
    }


}
