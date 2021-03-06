use std::fs::File;
use std::io::Read;

const PATH: &str = "/proc/uptime";

// On MacOS you can get the uptime in seconds from this
// shell command: sysctl -n kern.boottime | cut -c14-18

fn read_uptime() -> Result<u64, String> {
    let mut file = match File::open(PATH) {
        Ok(file) => file,
        Err(err) => return Err(format!("'{}': {}", PATH, err.to_string())),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    };
    let value: u64 = match contents.split(".").next() {
        Some(text) => match text.parse() {
                Ok(value) => value,
                Err(err) => return Err(err.to_string()),
           }
        None => return Err("No uptime value".to_string())
    };
    Ok(value)
}

fn strict_uptime(secs : u64) -> String {
    const SECS_PER_DAY : u64 = 86400;
    const SECS_PER_HOUR : u64 = 3600;
    const SECS_PER_MINUTE : u64 = 60;
    const SEPARATOR : char = '\u{f104}';
    let days = secs / SECS_PER_DAY;
    let remainder = secs % SECS_PER_DAY;
    let hours = remainder / SECS_PER_HOUR;
    let minutes = remainder % SECS_PER_HOUR / SECS_PER_MINUTE;
    format!("{:}{}{:>02}{}{:>02}", days, SEPARATOR, hours, SEPARATOR, minutes)
}

fn main() {
    match read_uptime() {
        Ok(value) => println!("{}", strict_uptime(value)),
        Err(err) => eprintln!("strict_uptime: error {}", err.to_string()),
    }
}
