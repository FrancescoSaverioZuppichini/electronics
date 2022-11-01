use lazy_static::lazy_static;
use std::fmt::Display;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};
use thiserror::Error;

lazy_static! {
    static ref TEMP_FILE_PATH: String = "/sys/class/thermal/thermal_zone0/temp".to_string();
    static ref TRIGGER_TEMP_IN_CELSIUS: u64 = 60;
    static ref TIME_WAIT_IN_MILLISECONDS: u64 = 5 * 1000;
    static ref GPIO: u64 = 17;
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Can't read temperature from file: {0}")]
    IO(String),
    #[error("Can't convert to interger: {0}")]
    Conversion(String),
    #[error("Can't set pin value: {0}")]
    GPIO(String),
}

type Result<T> = std::result::Result<T, MyError>;

enum PinValue {
    On,
    Off,
}

impl From<PinValue> for u8 {
    fn from(pin: PinValue) -> Self {
        match pin {
            PinValue::On => 0_u8,
            PinValue::Off => 1_u8,
        }
    }
}

impl Display for PinValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinValue::On => write!(f, "0"),
            PinValue::Off => write!(f, "1"),
        }
    }
}

fn get_temperature(file_path: &str) -> Result<u64> {
    let line = fs::read_to_string(file_path);
    match line {
        Ok(line) => line
            .trim()
            .parse::<u64>()
            .map_err(|e| MyError::Conversion(e.to_string())),
        Err(e) => Err(MyError::IO(e.to_string())),
    }
}

fn set_pin_value(pin: &Pin, value: PinValue) -> Result<()> {
    pin.set_direction(Direction::Out).unwrap();
    pin.with_exported(|| pin.set_value(value.into()))
        .map_err(|e| MyError::GPIO(e.to_string()))
}

fn main() {
    let my_pin = Pin::new(*GPIO);

    loop {
        match get_temperature(&*TEMP_FILE_PATH) {
            Ok(temp) => {
                println!("Temperature is = {}C", temp);
                if temp < *TRIGGER_TEMP_IN_CELSIUS {
                    if let Err(e) = set_pin_value(&my_pin, PinValue::On) {
                        println!("Error while setting pin to {}: {}", PinValue::Off, e)
                    } else {
                        println!("[ON] Fan");
                    }
                } else {
                    if let Err(e) = set_pin_value(&my_pin, PinValue::Off) {
                        println!("Error while setting pin to {}: {}", PinValue::On, e)
                    } else {
                        println!("[OFF] Fan");
                    }
                }
            }
            Err(e) => println!("Error while reading temp: {}", e),
        }
        sleep(Duration::from_millis(*TIME_WAIT_IN_MILLISECONDS));
    }
}
