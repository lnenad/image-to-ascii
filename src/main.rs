extern crate image;
extern crate imagetoascii;
extern crate time;

use imagetoascii::cmd::Parser;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use image::imageops::FilterType;
use image::GenericImageView;

fn main() {
    let (arguments, flags) = Parser::new().parse();

    if arguments.len() == 0 {
        print_help();
        return;
    }

    let image_path = arguments.get("image").or(arguments.get("i"));

    if image_path.is_none() {
        println!("No image was provided, use '-i' or '-input' to specify");
        print_help();
        return;
    }

    let output_path = arguments.get("output").or(arguments.get("o"));

    let resolution = match arguments.get("resolution").or(arguments.get("r")) {
        Some(val) => val.parse::<u32>().unwrap(),
        None => 5,
    };

    let preview = flags.contains(&"preview".to_owned()) || flags.contains(&"p".to_owned());

    run(image_path.unwrap(), resolution, output_path, preview);
}

fn run(image_path: &str, resolution: u32, output_path: Option<&String>, preview: bool) {
    // let image_path_txt =
    let start = time::precise_time_ns();
    let art = convert_to_ascii(&image_path, resolution, output_path.is_some());
    let end = (time::precise_time_ns() - start) / 1000000;

    match output_path {
        Some(op) => {
            let file_name = if op.ends_with(".txt") {
                op.to_owned()
            } else {
                format!("{}{}", op, ".txt".to_owned())
            };
            match write_to_file(&file_name, &art) {
                Ok(_) => (),
                _ => (),
            };

            if preview {
                let preview_file_name = &file_name.replace(".txt", ".html");
                match generate_preview(preview_file_name, &art) {
                    Ok(_) => {
                        println!("Output HTML File: {}", preview_file_name);
                        match open::that(preview_file_name) {
                            Ok(_) => (),
                            Err(err) => eprint!("Error occured while opening file: {}", err),
                        };
                    }
                    Err(err) => eprintln!("Error ocurred while generating file: {}", err),
                };
            };
            println!("Completed in: {}ms", end);
        }
        None => println!("{}", art),
    }
}

fn print_help() {
    println!(concat!(
        "-i|-image      [image path]   The input image file name\n",
        "-o|-output     [file name]    Output file name, if none is provided then will print to the stdout. Note: if the output file does not end in '.txt', that will be appended to it.\n",
        "-r|-resolution [integer]      Resolution: Perfomed with reverse linear scale, it's the divider of the original image resolution, the bigger the number the smaller the ascii output\n",
        "-p|-preview                   Flag that is used to generate an HTML preview and will open in your OS's default application (the name of the file will be the same as the input image file with an html extension)"
    ));
}

fn convert_to_ascii(image_path: &str, resolution: u32, print_info: bool) -> String {
    if print_info {
        println!("Input File: {}", image_path);
    }
    let img = image::open(&Path::new(image_path)).unwrap();
    let character_set: [&str; 11] = ["@", "#", "0", "O", "L", ";", ":", ".", ",", "'", " "];
    let mut art = String::new();
    let mut last_y = 0;

    let small_img = img.resize(
        img.width() / resolution,
        img.height() / resolution,
        FilterType::Nearest,
    );

    if print_info {
        println!(
            "Original size: {:?}   Reduced: {:?}",
            img.dimensions(),
            small_img.dimensions()
        );
    }

    for pixel in small_img.pixels() {
        if last_y != pixel.1 {
            art.push_str("\n");
            last_y = pixel.1;
        }

        let pixel_data = pixel.2;
        let brightness: f64 =
            ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
        let character_position =
            ((brightness / 255.0) * (character_set.len() - 1) as f64).round() as usize;
        art.push_str(character_set[character_position])
    }

    art
}

fn write_to_file(file_name: &String, contents: &String) -> Result<String, Box<dyn Error>> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;

    Ok("Done".into())
}

fn generate_preview(file_name: &String, contents: &String) -> Result<String, Box<dyn Error>> {
    let html = concat!(
        "<!DOCTYPE html>\n",
        "<!-- saved from url=(0053)https://www.google.rs/_/chrome/newtab?espv=2&ie=UTF-8 -->\n",
        "<html lang=\"en-RS\"><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\">\n",
        "</head>\n",
        "<body style=\"font-size: 2.5pt;\">\n",
	    "   <pre style=\"color: black; letter-spacing: 1px;font-weight: bold;\">{templateContents}</pre>\n",
        "</body>\n",
        "</html>",
    );
    let new_data = html.replace("{templateContents}", &*contents);

    let mut dst = File::create(&file_name)?;
    dst.write(new_data.as_bytes())?;

    Ok(String::from("Done"))
}
