extern crate libnotify;
use std::{env, process};
mod lib;
use lib::Config;
use std::thread;
use std::time::Duration;
use clap::{Arg, App};

// https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut conf: Config = Config::new();


    let matches = App::new("Pomodoro")
    .version("0.1.0")
    .author("C-Loftus")
    .about("Pomodoro")
    .arg(Arg::with_name("help")
             .short("h")
             .long("help")
             .takes_value(false)
             .help("Displays help info"))
    .arg(Arg::with_name("break time")
             .short("b")
             .long("breakt")
             .takes_value(true)
             .help("Five less than your favorite number"))

    .arg(Arg::with_name("work time")
            .short("w")
            .long("workt")
            .takes_value(true)
            .help("How long to work"))
    .arg(Arg::with_name("cycles")
             .short("c")
             .long("cycles")
             .takes_value(true)
             .help("How many cycles to do"))
    .get_matches();

    conf.on_len = match matches.value_of("break time") {
        None => conf.on_len,
        Some(s) => match s.parse::<u64>() {
            Ok(o) => o,
            Err(_) => panic!("Invalid break argument. Input an integer")
        }
    };
    print!("{}", conf.on_len);
}

//     libnotify::init("Timer").unwrap();
//     loop {
//         let n = libnotify::Notification::new("Starting work session __ out of ___ ",
//         Some(""),
//         None);
//         n.show().unwrap();
//         thread::sleep(std::time::Duration::from_secs(conf.on_len * 60));

//         let n = libnotify::Notification::new("Finished. Take a break for",
//         Some(""),
//         None);
//         n.show().unwrap();
//         thread::sleep(std::time::Duration::from_secs(conf.break_len * 60));
        
//     }
// }


// loop {



//     thread::sleep(Duration::from_millis(4000));

// }
// libnotify::uninit();
// }


