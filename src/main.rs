use std::env::args;
use std::process::exit;
use libc::{reboot, sync, RB_POWER_OFF, RB_AUTOBOOT};
use std::thread::sleep;
use std::time::Duration;



fn main() {
    let args: Vec<String> = args().collect();

    unsafe { sync(); }

    let mut timeout: u64 = 0;

    let mut rb = 0;

    for arg in &args[1..] {
        match arg.as_str() {
            "p" | "poweroff" | "off" => rb = RB_POWER_OFF,
            "r" | "reboot" => rb = RB_AUTOBOOT,
            "h" | "help" => help(),
            other => {
                if let Ok(num) = other.parse::<u64>() {
                    timeout = num;
                } else {
                    eprintln!("powman: unknown option: {}", other);
                    eprintln!("try 'powman help' to more info");
                    exit(1);
                }
            }
        }
    }

    if timeout != 0 {
        eprintln!("=> timer: {}s", timeout);
        sleep(Duration::from_secs(timeout));
    }
    unsafe {
        reboot(rb);
    }
}


fn help() -> ! {
    println!("powman - power manager");

    println!("! powman does not prepare the system for shutdown !");

    println!("Usage:    (default timeout = 0)");
    println!("    powman r|reboot [timeout] : reboot");
    println!("    powman p|poweroff|off [timeout] : poweroff");
    println!("    powman h|help : print this help");
    exit(0)
}

