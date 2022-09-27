use std::{
    io::{stdout, stdin, Write, Stdout},
    thread::sleep,
    time::Duration
};

fn get_input(stdout: &mut Stdout) -> Option<u32> {
    let mut time = String::new();

    print!("Enter how long the countdown should last [0+]: ");
    stdout.flush().unwrap();

    stdin()
        .read_line(&mut time)
        .expect("Failed to read line from user");

    match time.trim().parse() {
        Ok(num) => { if num > 0 { Some(num) } else { None } },
        Err(_) => None,
    }
}

fn main() {
    let mut stdout = stdout();
    
    let mut time: u32;

    loop {
        time = match get_input(&mut stdout) {
            Some(num) => num,
            None => { 
                println!("Invalid input value!");
                continue
            },
        };
        break;
    }

    println!();
    while time > 0 {
        print!("\r       \r{}", time);
        stdout.flush().unwrap();

        time -= 1;
        sleep(Duration::from_secs(1));
    }

    println!();
}