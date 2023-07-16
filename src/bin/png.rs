use std::error::Error;
use std::str::FromStr;

use httpfox::{config::Config, png::Png};

fn make_image(config: &Config, file_name: &str) -> Result<(), Box<dyn Error>> {
    println!("Converting image {file_name}");

    let status_code = i32::from_str(&file_name.replace("_raw", ""))?;

    match Png::new(&config, status_code) {
        Some(png) => {
            png.image();
            println!("Successfully converted image {file_name}");
        }
        None => {
            println!("Image {file_name} is not a valid status image, skipping")
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let code = &args[1];

        make_image(&config, code)?;

        return Ok(());
    }

    for f in config.raw_images_dir.read_dir()? {
        let file = f?;

        let _filename = file.path();
        let file_name = _filename
            .file_stem()
            .ok_or(Box::<dyn Error>::from("Could not get File Name"))?
            .to_str()
            .ok_or(Box::<dyn Error>::from("Could not get File Name"))?;

        if file_name.contains("raw") && file.path().extension().is_some_and(|ext| ext == "png") {
            make_image(&config, file_name)?;
        }
    }

    Ok(())
}
