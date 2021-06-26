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
    .arg(Arg::with_name("break time")
             .short("b")
             .long("breakt")
             .takes_value(true)
             .help("How long for your standard short breaks"))
    .arg(Arg::with_name("work time")
            .short("w")
            .long("workt")
            .takes_value(true)
            .help("How long to work"))
    .arg(Arg::with_name("cycles")
             .short("c")
             .long("cycles")
             .takes_value(true)
             .help("How many work/break cycles"))
    .arg(Arg::with_name("long break frequency")
             .short("f")
             .long("longfreq")
             .takes_value(true)
             .help("Generate a longer break after n cycles"))
    .arg(Arg::with_name("long break length")
             .short("l")
             .long("lbreakt")
             .takes_value(true)
             .help("The length of your long break"))
    .get_matches();

    Config::parse_lens(&mut conf, &matches);

    libnotify::init("Timer").unwrap();


    let has_freq = matches.is_present("long break frequency");
    let has_len = matches.is_present("long break length");  
    if !has_freq && !has_len {
        // will never have a long break if not specified since anything mod 1 never equals 0
        conf.long_break_freq = 1;
    }
    else if !has_freq && has_len {
        // standard pomodoro
        conf.long_break_freq = 5;
    }
    else if has_freq && !has_len {
        // default of 15 min long break
        conf.long_break_len = 15;
    }


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

        let long_break = i % conf.long_break_freq == 0;
        if long_break {
            thread::sleep(std::time::Duration::from_secs(conf.long_break_len * 60));
        }
        else {
            thread::sleep(std::time::Duration::from_secs(conf.break_len * 60));
        }

        
    }
 
}


