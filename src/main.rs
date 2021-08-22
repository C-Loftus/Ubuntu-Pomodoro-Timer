extern crate libnotify;
mod lib;
use lib::Config;
use std::{thread};
extern crate fstrings;
use fstrings::format_args_f;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let relative_path: &str = "Music/mus.mp3";
    let mut sound =  dirs::home_dir().unwrap();
    sound.push(relative_path);

    let mut conf: Config = Config::new();
    let matches = lib::gen_args();
    Config::parse_lens(&mut conf, &matches);
    libnotify::init("Timer").unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml

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
    let mut cycle_len =  conf.cycles.to_string();         

    if conf.cycles == u64::MAX {
        cycle_len = String::from("endless");
    }

    let n = libnotify::Notification::new(
        &format_args_f!("{conf.on_len} min,{cycle_len} cycles,{conf.break_len} min brk,{conf.long_break_len} min lbrk: per {conf.long_break_freq} cycles")
                .to_string(),
            Some(""),
            None);
    n.show().unwrap();
    thread::sleep(std::time::Duration::from_secs(2));


    for i in 1..conf.cycles {      
        println!("starting {} minute cycle {}", conf.on_len, i);
        // work
        let n = libnotify::Notification::new(
                    &format_args_f!("Starting {conf.on_len} minute session #{i}, out of {cycle_len} total cycles")
                            .to_string(),
                        Some(""),
                        None);
        n.show().unwrap();
        thread::sleep(std::time::Duration::from_secs(conf.on_len * 60));


        // break
        let long_break = i % conf.long_break_freq == 0;

            // AUDIO
                // code duplication for the notifs isn't ideal but its the easiest way to do it with 
                // Rust scoping that I'm aware of
                let file = BufReader::new(File::open(&sound).unwrap());
                let source = Decoder::new(file).unwrap();
                stream_handle.play_raw(source.convert_samples()).unwrap();

                // The sound plays in a separate audio thread,
                // so we need to keep the main thread alive while it's playing.
                std::thread::sleep(std::time::Duration::from_secs(5));

        if long_break {
            let n = libnotify::Notification::new(
                &format_args_f!("Finished. Take a long break for {conf.long_break_len} minutes")
                        .to_string(),
                    Some(""),
                    None);
                    n.show().unwrap();
                    println!("starting break {} for {} minutes", i, conf.long_break_len);
            thread::sleep(std::time::Duration::from_secs(conf.long_break_len * 60));
        }
        else {
            let n = libnotify::Notification::new(
                &format_args_f!("Finished. Take break #{i} for {conf.break_len} minutes")
                        .to_string(),
                    Some(""),
                    None);
                    n.show().unwrap();
                    println!("starting break {} for {} minutes", i, conf.break_len);
            thread::sleep(std::time::Duration::from_secs(conf.break_len * 60));
        }

                 // AUDIO
                // code duplication for the notifs isn't ideal but its the easiest way to do it with 
                // Rust scoping that I'm aware of
                

                let file = BufReader::new(File::open(&sound).unwrap());
                let source = Decoder::new(file).unwrap();
                stream_handle.play_raw(source.convert_samples()).unwrap();

                // The sound plays in a separate audio thread,
                // so we need to keep the main thread alive while it's playing.
                std::thread::sleep(std::time::Duration::from_secs(5));   


    }
 
}


// TODO:
// Reduce duplicate code through Boxes /  rodio buffers