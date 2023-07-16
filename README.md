# http.fox

HTTP Foxxos

## Adding New Foxes

1. Add a raw Fox image in the format `<CODE>_raw.png` to the folder `/static/images/raw/`
    - For example, `/static/images/raw/404_raw.png`
    - Each picture must be a 600x395 image. You may use a tool like GIMP to scale your image and add a black background.
2. Run the command `cargo run --bin png` to regenerate all images, or `cargo run --bin png <CODE>` to generate a single image.
3. In the file `/src/status.json`, move the code from the Unimplemented list to the Implemented list.
4. Add a new file in `/templates/statuses` in the format `CODE.html.tera`. More details about the status file format can be found in [the README.md](/templates/statuses/README.md)