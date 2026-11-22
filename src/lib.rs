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
pub struct ListTable {
    lists:Vec<List>
}

impl ListTable {
    pub fn new () -> ListTable {
        ListTable { lists: Vec::new() }
    }
    pub fn list_exist(&mut self, name: String) -> bool {

       return self.lists.iter().any(|list| list.name == name)
    }
}

pub struct List {
    pub name: String,
    pub elements: Vec<String>
}

impl List {
    pub fn new() -> List {
        List {
            name: "".to_string(),
            elements: Vec::new()
        }
    }

    // change list name
    pub fn update_name(&mut self, name:&str ) {
        self.name = name.to_string();
    }

    // add element to list
    pub fn add_to_list(&mut self, content: String) {
        self.elements.push(content)
    }

    // display after an element is added to list
    pub fn display(&mut self) -> String {
        return format!("(integer){}", self.elements.len());
    }
}
