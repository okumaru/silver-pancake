use std::env;
use std::thread;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

fn main() {
    // collect input arguments
    let args: Vec<String> = env::args().collect();

    let mut count = 0;
    let mut total_size = 1500;

    if args.len() > 1 {
        // Input arguments are in minutes;
        // converts them to seconds by multiplying by 60.
        total_size = args[1].parse::<u64>().unwrap() * 60
    }

    let pb = ProgressBar::new(total_size);

    pb.set_style(
        ProgressStyle::with_template("[{count_down}] {bar:40.cyan/blue} ")
            .unwrap()
            .with_key("count_down", |state: &ProgressState, w: &mut dyn Write| {
                let total = state.len().unwrap();
                let curr = state.elapsed().as_secs();
                let minute = (total - curr) / 60;
                let second = (total - curr) % 60;

                write!(w, "{:02}:{:02}", minute, second).unwrap()
            })
            .progress_chars("#>-"),
    );

    while count < total_size {
        let new = min(count + 1, total_size);
        count = new;
        pb.set_position(new);
        thread::sleep(Duration::new(1, 5000000));
    }

    pb.finish_with_message("downloaded");
}
