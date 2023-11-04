use std::process::Command;
use ini::Ini;
use std::io::{self, Write, Read};

fn main() {
    let ini_path = "wrapper.ini";
    let conf = match Ini::load_from_file(ini_path) {
        Ok(conf) => conf,
        Err(_) => {
            let mut conf = Ini::new();
            conf.with_section(None::<String>)
                .set("exec", "./dsda-doom.exe")
                .set("args", "");
            conf.write_to_file(ini_path).unwrap();
            conf
        }
    };

    let exec_path = conf.general_section().get("exec").unwrap().to_owned();
    let exec_args = conf.general_section().get("args").unwrap().to_owned();

    let mut new_args = Vec::new();
    new_args.push(exec_path);

    // Split the arguments string into separate arguments and add them to new_args
    new_args.extend(exec_args.split_whitespace().map(|s| s.to_owned()));

    let output = Command::new(&new_args[0])
        .args(&new_args[1..])
        .output()
        .expect("Failed to execute command");

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    // Add a pause before the program exits
    pause();
}

fn pause() {
    let mut stdout = io::stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    io::stdin().read(&mut [0]).unwrap();
}
