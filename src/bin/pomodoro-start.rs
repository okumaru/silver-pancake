use silver_pancake::MyTime;
use std::env;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    // collect input arguments
    let args: Vec<String> = env::args().collect();

    // init sistem time
    let now = SystemTime::now();

    // set default value count to 1500 (25 min)
    // unit value in second
    let mut count: Option<i64> = Some(1500);

    if args.len() > 1 {
        // Input arguments are in minutes;
        // converts them to seconds by multiplying by 60.
        count = Some(args[1].parse::<i64>().unwrap() * 60)
    }

    // loop until count value to zero
    while Some(0) <= count {
        // clear terminal
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // system timeouts every 1 second,
        // What’s odd is,
        // that one second in lib Duration feels quicker
        // so I add 5000000 nanosecond
        sleep(Duration::new(1, 5000000));

        // validate sistem time
        match now.elapsed() {
            Ok(_) => {
                // get current count value
                let data = count.unwrap();

                // print current count value
                MyTime::from_number(data as u64).print();

                // reduce data count value
                count = Some(data - 1);
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
            }
        }
    }
}
