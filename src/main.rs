extern crate image;
extern crate time;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::collections::HashMap;

use image::GenericImage;
use image::imageops::FilterType;

struct ArgumentList<'a> {
    arguments: &'a HashMap<&'a str, ArgumentDefinitions<'a>>
}

#[derive(Debug)]
struct ArgumentDefinitions<'a> {
    short: &'a str,
    long: &'a str,
    alternate: Option<&'a str>
}

impl<'b> ArgumentDefinitions<'b> {
    fn new (short: &'b str, long: &'b str, alternate: Option<&'b str>) -> ArgumentDefinitions<'b> {
        match alternate {
            Some(value) => ArgumentDefinitions{short, long, alternate: Some(value)},
            None => ArgumentDefinitions{short, long, alternate: None},
        }
    }
}

fn main() {
    let mut needed_arguments: HashMap<&str, ArgumentDefinitions> = HashMap::new();
    needed_arguments.insert("image", ArgumentDefinitions::new("i","image", None));
    needed_arguments.insert("output", ArgumentDefinitions::new("o","output", None));
    needed_arguments.insert("resolution", ArgumentDefinitions::new("r","resolution", None));
    needed_arguments.insert("preview", ArgumentDefinitions::new("p","preview", None));
    let argument_list: ArgumentList = ArgumentList{arguments: &needed_arguments};
    let arguments = parse_arguments(&argument_list);
    let image_path = match arguments.get("image") {
        Some(val) => val,
        None => panic!("No image argument provided")
    };

    let output = match arguments.get("output") {
        Some(val) => val,
        None => panic!("No output argument provided")
    };

    let resolution = match arguments.get("resolution") {
        Some(val) => val.parse::<u32>().unwrap(),
        None => 3
    };
    let start = time::precise_time_ns();
    let contents = convert_to_ascii(image_path, resolution, output);
    let end = (time::precise_time_ns() - start) / 1000000;

    match arguments.get("preview") {
        Some(val) => generate_preview(&output.replace(".txt", ".html"), &contents),
        None => Ok(String::from("ASD"))
    };

    println!("Completed in: {}ms", end);
}

fn parse_arguments(argument_list: &ArgumentList) -> HashMap<String, String> {
    let mut named_arguments: HashMap<String, String> = HashMap::new();
    let mut unnamed_arguments: Vec<String> = Vec::new();
    let mut argument_name: String = String::new();
    let mut received_args: Vec<_> = env::args().collect();
    let mut i = 0;
    received_args.remove(0);

    for (index, argument) in received_args.iter().enumerate() {
        println!("{:?}: \"{:?}\"", index, argument);
    }
        /*for (argument_name, argument_ids) in argument_list.arguments.iter() {
            println!("{:?}: \"{:?}\"", book, review);
        }

        for (index, argument) in .enumerate() {
            if argument.starts_with("--") {
                argument_name = argument[2..argument.len()].to_owned();
                i = index + 1;
                continue;
            }
            if argument.starts_with("-") {
                argument_name = argument[1..argument.len()].to_owned();
                i = index + 1;
                continue;
            }
            if index == i && argument_name != "" {
                named_arguments.entry(argument_name.to_owned()).or_insert(argument.to_owned());
            }
        }
    */
    named_arguments
}

fn convert_to_ascii(image_url: &str, resolution: u32, output: &str) -> String {
    let img = image::open(&Path::new(image_url)).unwrap();
    let character_set: [&str; 11] = ["@", "#", "0", "O", "L", ";", ":", ".", ",", "'", " "];
    let mut art = String::new();
    let mut last_y = 0;

    let small_img = img.resize(img.width() / resolution, img.height() / resolution, FilterType::Nearest);

    println!("Original size: {:?}   Reduced: {:?}", img.dimensions(), small_img.dimensions());

    for pixel in small_img.pixels() {
        if last_y != pixel.1 {
            art.push_str("\n");
            last_y = pixel.1;
        }

        let pixel_data = pixel.2.data;
        let brightness:f64 = ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
        let character_position = ((brightness/255.0) * (character_set.len()  - 1) as f64 ).round() as usize;
        art.push_str(character_set[character_position])
    }

    match write_to_file(output,&art) {
        Ok(_) => (),
        _ => ()
    };

    art
}

fn write_to_file(file_name: &str, contents: &String) -> Result<String, Box<Error>> {
    let mut file = File::create(file_name)?;
    if true == false {
        return Err(Box::from("Is not a directory!"));
    }

    file.write_all(contents.as_bytes())?;

    Ok("Done".into())
}

fn generate_preview(file_name: &str, contents: &String) -> Result<String, Box<Error>> {
    let mut src = File::open("preview.html")?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    drop(src);  // Close the file early

    let new_data = data.replace("{templateContents}", &*contents);

    let mut dst = File::create(&file_name)?;
    dst.write(new_data.as_bytes())?;

    Ok(String::from("Done"))
}
