use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {

    // init sistem time
    let now = SystemTime::now();

    // set default value count to 120 (2 min)
    let mut count = Some(120);

    // loop until count value to zero
    while count.unwrap() >= 0 {

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
                let second = count.unwrap();

                // print current count value
                println!("{}", second);

                // reduce data count value
                count = Some(second - 1);
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
            }
        }
    }
}
