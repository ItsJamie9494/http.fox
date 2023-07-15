# http.fox

HTTP Foxxos

## Adding New Foxes

1. Add a raw Fox image in the format `<CODE>_raw.png` to the folder `/images/raw/`
    - For example, `/images/raw/404_raw.png`
1. Run the command `cargo run --bin png`
    - This will automatically regenerate all images
    <!-- - TODO: Add an argument for just one image -->
    <!-- - TODO: Add an SVG and PNG step, just in case images need adjustment -->
1. In the file `/src/status.json`, move the code from the Unimplemented list to the Implemented list.