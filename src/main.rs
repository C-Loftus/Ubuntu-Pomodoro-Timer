extern crate libnotify;
use std::process::exit;
mod lib;
use lib::Config;
use std::thread;
use clap::{Arg, App};
extern crate fstrings;
use fstrings::format_args_f;

fn main() {
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


    if matches.is_present("help") {
        print!("Temp help message");
        exit(0);
    }

    Config::parse_lens(&mut conf, &matches);

    libnotify::init("Timer").unwrap();

    for i in 1..conf.cycles {
        let mut cycle_len =  conf.cycles.to_string(); 
        if conf.cycles == u64::MAX {
            cycle_len = String::from("endless");
        }
        
        let n = libnotify::Notification::new(
                    &format_args_f!("Starting {conf.on_len} minute session #{i}, out of {cycle_len} total cycles")
                            .to_string(),
                        Some(""),
                        None);
        n.show().unwrap();
        thread::sleep(std::time::Duration::from_secs(conf.on_len * 60));

        let n = libnotify::Notification::new(
                    &format_args_f!("Finished. Take a break for {conf.break_len} minutes")
                            .to_string(),
                        Some(""),
                        None);
        n.show().unwrap();
        thread::sleep(std::time::Duration::from_secs(conf.on_len * 60));

        thread::sleep(std::time::Duration::from_secs(conf.break_len * 60));
        
    }
 
}


