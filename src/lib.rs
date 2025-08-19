pub struct MyTime {
    //minut: u64,
    //second: u64,
    time: String,
}

impl MyTime {
    pub fn from_number(input: u64) -> MyTime {
        let minute: u64 = input / 60;
        let second: u64 = input % 60;

        MyTime {
            //minut: minute,
            //second: second,
            time: format!("{:02}:{:02}", minute, second),
        }
    }

    pub fn print(&self) {
        println!("{}", self.time)
    }
}
