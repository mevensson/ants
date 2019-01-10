use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

#[derive(Debug)]
pub struct Settings {
    pub values: HashMap<String, String>,
}

impl Settings {
    pub fn parse<R>(mut reader: R) -> Result<Settings, Box<dyn Error>>
    where
        R: BufRead,
    {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line != "turn 0" {
            return Err(From::from("Settings does not start with 'turn 0'"));
        }
        Ok(Settings {
            values: HashMap::new(),
        })
    }
}

pub fn run(_settings: Settings) -> Result<(), Box<dyn Error>> {
    Ok(())
}
