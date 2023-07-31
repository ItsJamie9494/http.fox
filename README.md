# http.fox

HTTP Foxxos

## Adding New Foxes

1. Add a raw Fox image in the format `<CODE>_raw.png` to the folder `/static/images/raw/`
    - For example, `/static/images/raw/404_raw.png`
    - Each picture must be a 600x395 image. You may use a tool like GIMP to scale your image and add a black background.
2. Run the command `cargo run --bin png` to regenerate all images, or `cargo run --bin png <CODE>` to generate a single image.
3. In the file `/src/status.json`, move the code from the Unimplemented list to the Implemented list.
4. Add a new file in `/data` in the format `CODE.md`. This file contains a brief summary of the status code, and should not contain anything else.
5. Credit yourself! Add an entry to `/src/credits.json`

## Configuration

1. Create a new file at `config.json`, or set the `HTTPFOX_CONFIG` variable to point to a config.json file
2. Copy the contents of `config.json.example` over
3. Modify them for your instance!