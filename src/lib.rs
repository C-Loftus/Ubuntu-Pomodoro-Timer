use clap::{App, Arg, ArgMatches};


pub struct Config {
    pub help: bool,
    pub on_len: u64,
    pub break_len: u64,
    pub cycles: u64
}


impl Config {
    pub fn new()-> Config{
        return Config {help: false, on_len: 10, break_len : 10, cycles: u64::MAX}
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
    
        cnf.on_len = match matches.value_of("work time") {
            None => cnf.on_len,
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
    }

}