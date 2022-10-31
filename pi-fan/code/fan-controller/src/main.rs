use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::{fs, num::ParseIntError, fmt::Error};

const TEMP_FILE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";
const TRIGGER_TEMP_IN_CELSIUS: i32 = 60;
const TIME_WAIT_IN_MILLISECONDS: u64 = 5 * 1000;
const GPIO: u64 = 17;

fn get_temperature(file_path: &str) -> Result<i32, &str> {
    let temp = match fs::read_to_string(file_path) {
         Ok(line) =>  match line.trim().parse::<i32>() {
             Ok(temp) => Ok(temp/ 1000),
             ParseIntError => Err("Cannot parse int.")
         },
         Err(error) => Err("Cannot open file.")
     };
 
     return temp;
 }
 

fn main() {
    let my_pin = Pin::new(GPIO);
    my_pin.with_exported(|| {
        my_pin.set_direction(Direction::Out).unwrap();
        loop {
            match get_temperature(TEMP_FILE_PATH) {
                Ok(temp) => {
                    if temp < TRIGGER_TEMP_IN_CELSIUS { 
                        println!("{}", "[ON] Fan");
                        my_pin.set_value(0).unwrap();
    
                    }
                    else {
                        println!("{}", "[OFF] Fan");
                        my_pin.set_value(1).unwrap();
                
                    }
                    println!("Temperature is = {}C", temp);
                    sleep(Duration::from_millis(TIME_WAIT_IN_MILLISECONDS));
                },
                Err(error) => println!("{}", error)
            };
        }
     
    }).unwrap();
}