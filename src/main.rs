extern crate imagetoascii;
extern crate time;
extern crate image;

use imagetoascii::cmd::Parser;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use image::GenericImage;
use image::imageops::FilterType;

fn main() {
    let (arguments, flags) = Parser::new().parse();
    let image_path = arguments.get("image").or(arguments.get("i")).expect("No image argument provided");

    println!("{:?}{:?}", arguments, flags);

    let output = arguments.get("output").or(arguments.get("o")).expect("No output argument provided");

    let resolution = match arguments.get("resolution").or(arguments.get("r")) {
        Some(val) => val.parse::<u32>().unwrap(),
        None => 5
    };

    let start = time::precise_time_ns();
    let contents = convert_to_ascii(image_path, resolution, output);
    let end = (time::precise_time_ns() - start) / 1000000;

    if flags.contains(&"preview".to_owned()) || flags.contains(&"p".to_owned()) {
        generate_preview(&output.replace(".txt", ".html"), &contents);
    };

    println!("Completed in: {}ms", end);
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
