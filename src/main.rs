use sys_info::*;
use whoami::username;

fn main() {
    match hostname() {
        Ok(hostname) => println!(" {}@{}\t|", username(), hostname),
        Err(e) => eprintln!("Error: {}", e),
    }

    match os_type() {
        Ok(os) => println!(" OS\t\t\t| {}", os),
        Err(e) => eprintln!("Error: {}", e),
    }

    match os_release() {
        Ok(release) => println!(" Kernel\t\t\t| {}", release),
        Err(e) => eprintln!("Error: {}", e),
    }

    match cpu_num() {
        Ok(num) => println!(" CPU\t\t\t| {}", num),
        Err(e) => eprintln!("Error: {}", e),
    }

    match cpu_speed() {
        Ok(speed) => println!(" CPU clock\t\t| {} MHz", speed),
        Err(e) => eprintln!("Error: {}", e),
    }

    match mem_info() {
        Ok(mem) => println!(" Memory\t\t\t| {}/{} KB", mem.total, mem.free),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("╰───────────────────────╯")
}
