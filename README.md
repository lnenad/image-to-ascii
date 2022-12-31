# Image-to-ascii in Rust

Just a simple project that converts input from an image format into a .txt with a possibility of 
generating an html preview.

## Motivation

I'm learning Rust and thought this would be an easy enough project to do and learn.

## Installation

If you have cargo dependencies should be installed automatically by building

```bash
$ cargo build --release
```

From there, navigate to the `target/release` and execute the binary

```bash
$ imagetoascii.exe
```

If no arguments are provided, the help menu in the next section is printed

## Usage

The 'ita' cmd utility accepts the following arguments. 

```
-i|-image      [image path]   The input image file name
-o|-output     [file name]    Output file name, if none is provided then will print to the stdout. Note: if the output file does not end in '.txt', that will be appended to it.
-r|-resolution [integer]      Resolution: Perfomed with reverse linear scale, it's the divider of the original image resolution, the bigger the number the smaller the ascii output
-p|-preview                   Flag that is used to generate an HTML preview and will open in your OS's default application (the name of the file will be the same as the input image file with an html extension)
```

## Example
The program can be run directly from cargo
```bash
$ cargo run -- -i image.png -o image.txt -r 5 -p  
```

Or the program can be run by building and executing the image
```bash
$ cargo build --release
$ cd ./target/relase
$ imagetoascii -i image.png -o image.txt -r 5 -p  
```

The given example analyzes the image.png file and outputs the ascii art to image.txt. It will resize the original image to a fifth of it's original size and it will generate an HTML preview.

## Tests

I'd like to write basic tests soon just for practice

## Contributors

Nenad Lukic - [lnenad](https://github.com/lnenad)
Grant Lanham - [glanham-jr](https://github.com/glanham-jr)

## License

[WTFPL](http://www.wtfpl.net/)