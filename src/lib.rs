use clap::{App, Arg, ArgMatches};


pub struct Config {
    pub on_len: u64,
    pub break_len: u64,
    pub cycles: u64,
    pub long_break_freq: u64,
    pub long_break_len: u64,
}


impl Config {
    pub fn new()-> Config{
        // default values
        return Config {
            on_len: 10, 
            break_len : 10, 
            cycles: u64::MAX,
            long_break_freq: 1,
            long_break_len: 10,
        }
    }

    pub fn parse_lens(cnf: &mut Config, matches: &ArgMatches) -> () {

        cnf.on_len = match matches.value_of("work time") {
            None => cnf.on_len,
            Some(s) => match s.parse::<u64>() {
                Ok(o) => o,
                Err(_) => panic!("Invalid break argument. Input an integer")
            }
        };
    
        cnf.break_len = match matches.value_of("break time") {
            None => cnf.break_len,
            Some(s) => match s.parse::<u64>() {
                Ok(o) => o,
                Err(_) => panic!("Invalid break argument. Input an integer")
            }
        };
    
        cnf.cycles = match matches.value_of("cycles") {
            None => cnf.cycles,
            Some(s) => match s.parse::<u64>() {
                Ok(o) => o,
                Err(_) => panic!("Invalid cycles argument. Input an integer")
            }
        };

        cnf.long_break_freq = match matches.value_of("long break frequency") {
            None => cnf.long_break_freq,
            Some(s) => match s.parse::<u64>() {
                Ok(o) => o,
                Err(_) => panic!("Invalid cycles argument. Input an integer")
            }
        };

        cnf.long_break_len = match matches.value_of("long break length") {
            None => cnf.long_break_len,
            Some(s) => match s.parse::<u64>() {
                Ok(o) => o,
                Err(_) => panic!("Invalid cycles argument. Input an integer")
            }
        };


    }

}