# Image-to-ascii in Rust

Just a simple project that converts input from an image format into a .txt with a possibility of 
generating an html preview.

## Motivation

I'm learning Rust and thought this would be an easy enough project to do and learn.

## Installation

If you have cargo dependencies should be installed automatically by running
```bash
$ cargo run
```
## Usage

The 'ita' cmd utility accepts the following arguments. 

```
-i|-image      [image path]   Input image
-o|-output     [file name]    Output file name
-r|-resolution [integer]      Resolution (reverse linear scale, it's the divider of the original image resolution, the bigger the number the smaller the ascii output)
-p|-preview                   Flag that is used to generate an HTML preview (the name of the file will be the same as the input image file with an html extension)
```

## Example
```bash
$ cargo run -- -i image.png -o image.txt -r 5 -p  
```
The given example analyzes the image.png file and outputs the ascii art to image.txt. It will resize the original image to a fifth of it's original size and it will generate an HTML preview.

## Tests

I'd like to write basic tests soon just for practice

## Contributors

Nenad Lukic - [lnenad](http://github.com/lnenad)

## License

[WTFPL](http://www.wtfpl.net/)