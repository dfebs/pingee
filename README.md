# Pingee
![Pingee Logo](pingee_logo.png)
A tool to display PNGs built in Rust.

## About Pingee
This project was created to learn two things:
- Rust
- How to navigate and display PNG files.

This is not meant to be an exhaustive, perfect PNG library by any stretch. It is simply a tool I made to learn the ins and outs of PNGs. Feel free to pull it down and learn stuff!

The process of decompressing the image data and displaying it were done using libraries, but everything else was done by hand. 

## Running the code
1. `git clone https://github.com/dfebs/pingee`
1. `cd pingee`
1. `cargo run -- gpru.png`

A very tiny window should appear which displays a 2x2 image. Feel free to try using the other PNGs in the repo, or include your own! This program does not currently display all types of PNGs. Below are the supported PNG types.

## Supported PNG types
The following types of PNGs can be displayed
- RGB
- RGB with transparency
    - Partially supported, will display the image but transparency will not be applied in `minifb`
- Indexed
