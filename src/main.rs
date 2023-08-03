use rodio::{Decoder, OutputStream, Sink};
use std::fs;
use std::io::{BufReader, Error};
use std::{fs::File, path::Path};

fn print_help() {
    let first_argument = std::env::args().nth(0).unwrap();
    let executable_name = Path::new(&first_argument)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    println!("Oggly-rusty - by Aragubas");
    println!("Usage: {} <optional arguments> file_path", executable_name);
    println!("Arguments:");
    println!("  --volume: Sets volume (float; default=1.0)");
    println!("  --speed: Sets speed (float; default=1.0)");
}

fn do_play(file_name: String, volume: f32, speed: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open(file_name).unwrap());
    let source = Decoder::new(file).unwrap();

    // Speed, Volume
    sink.set_volume(volume);
    sink.set_speed(speed);
    sink.append(source);

    sink.sleep_until_end();
}


fn file_exists(path: &String) -> Result<(), String> {
    let io_path = Path::new(path);
    match fs::metadata(io_path) {
        Ok(metadata) => {
            if !metadata.is_file() {
                let mut error_message = String::from("Specified path ");
                error_message.push_str(path);
                error_message.push_str(" is not a file");

                return Err(error_message);
            }
        }
        Err(error) => {
            let mut error_message = String::from("Could not open ");
            error_message.push_str(path);

            return Err(error_message);
        }
    };

    Ok(())
}


fn main() -> Result<(), String> {
    if std::env::args().count() == 1 {
        // Print help, invalid argument count
        print_help();
        return Err(String::from("Invalid argument count"));
    } else if std::env::args().count() == 2 {
        // Expects the only argument to be audio path
        let argument = match std::env::args().nth(1) {
            Some(string) => string,
            _ => String::from(""),
        };

        // Check if argument is help
        if argument == "help"
            || argument == "h"
            || argument == "--help"
            || argument == "/?"
            || argument == "-h"
        {
            print_help();
            return Ok(());
        }

        // Check if audio file exists and if the argument is a file
        // and not a path
        match file_exists(&argument) {
            Ok(_) => { }
            Err(err) => { return Err(err); }
        };

        do_play(argument, 1.0, 1.0);
    }

    // Parse Arguments
    let mut next_is_volume = false;
    let mut next_is_speed = false;
    let mut volume = 1.0;
    let mut speed = 1.0;
    let mut file_path = String::from("");

    for i in 1..=std::env::args().count() - 1 {
        let argument = std::env::args().nth(i).unwrap();

        if next_is_volume {
            let parsed_value = argument.parse::<f32>();
            volume = match parsed_value {
                Ok(value) => value,
                Err(_) => {
                    return Err(String::from("Could not parse volume, value is invalid"));
                }
            };

            next_is_volume = false;
            continue;
        }

        if next_is_speed {
            let parsed_value = argument.parse::<f32>();
            speed = match parsed_value {
                Ok(value) => value,
                Err(_) => {
                    return Err(String::from("Could not parse speed, value is invalid"));
                }
            };

            next_is_speed = false;
            continue;
        }

        if argument == "--volume" {
            next_is_volume = true;
            continue;
        } else if argument == "--speed" {
            next_is_speed = true;
            continue;
        } else {
            // Last Argument
            if i == std::env::args().count() - 1 {
                if next_is_speed || next_is_volume {
                    return Err(String::from(
                        "Argument count invalid, last argument must be file path",
                    ));
                }

                file_path = argument;
            } else {
                let mut error_message = String::from("Argument '");
                error_message.push_str(&argument);
                error_message.push_str("' is invalid");
    
                return Err(error_message);
            }
        }
    }

    if file_path == "" {
        return Err(String::from("No input file specified"));
    }

    // Check if input file exists
    match file_exists(&file_path) {
        Ok(_) => { }
        Err(err) => { return Err(err); }
    };

    do_play(file_path, volume, speed);

    Ok(())
}
