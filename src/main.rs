use std::process::Command;
use std::thread;
use std::time::Duration;
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
};

fn set_state(display: bool) {
    let mut flags = ES_CONTINUOUS | ES_SYSTEM_REQUIRED;
    if display {
        flags |= ES_DISPLAY_REQUIRED;
    }
    unsafe { SetThreadExecutionState(flags); }
}

fn clear_state() {
    unsafe { SetThreadExecutionState(ES_CONTINUOUS); }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut display = false;
    let mut duration_secs: Option<u64> = None;
    let mut command: Option<Vec<String>> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-d" => display = true,
            "-t" => {
                i += 1;
                duration_secs = Some(
                    args.get(i)
                        .expect("caffeinate: -t requires a number of seconds")
                        .parse()
                        .expect("caffeinate: -t value must be a positive integer"),
                );
            }
            "-i" => {
                i += 1;
                if i < args.len() {
                    command = Some(args[i..].to_vec());
                    break;
                } else {
                    eprintln!("caffeinate: -i requires a command");
                    std::process::exit(1);
                }
            }
            unknown => {
                eprintln!("caffeinate: unknown option '{unknown}'");
                eprintln!("Usage: caffeinate [-d] [-t seconds] [-i command [args...]]");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    set_state(display);

    // Ctrl+C handler: always clean up the execution state.
    ctrlc::set_handler(move || {
        println!("\ncaffeinate: released. System can sleep again.");
        clear_state();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");

    // -i: run a command and exit when it finishes.
    if let Some(cmd) = command {
        let (bin, cmd_args) = cmd.split_first().expect("caffeinate: empty command");
        println!("caffeinate: running `{}`", cmd.join(" "));

        let status = Command::new(bin)
            .args(cmd_args)
            .status()
            .unwrap_or_else(|e| {
                eprintln!("caffeinate: failed to run command: {e}");
                std::process::exit(1);
            });

        clear_state();
        std::process::exit(status.code().unwrap_or(1));
    }

    // -t: sleep for the given duration then exit.
    if let Some(secs) = duration_secs {
        println!(
            "caffeinate: keeping awake for {secs}s{}. Press Ctrl+C to stop early.",
            if display { " (display on)" } else { "" }
        );
        thread::sleep(Duration::from_secs(secs));
        println!("caffeinate: time's up. System can sleep again.");
        clear_state();
        return;
    }

    // No flags or just -d: run until Ctrl+C.
    println!(
        "caffeinate: keeping{}awake. Press Ctrl+C to stop.",
        if display { " display " } else { " " }
    );
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
