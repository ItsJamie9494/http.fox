use std::error::Error;
use std::str::FromStr;

use httpfox::{config::Config, png::Png};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    for f in config.images_dir.read_dir()? {
        let file = f?;

        let _filename = file.path();
        let file_name = _filename
            .file_stem()
            .ok_or(Box::<dyn Error>::from("Could not get File Name"))?
            .to_str()
            .ok_or(Box::<dyn Error>::from("Could not get File Name"))?;

        if file_name.contains("raw") && file.path().extension().is_some_and(|ext| ext == "png") {
            println!("Converting image {file_name}");

            let status_code = i32::from_str(&file_name.replace("_raw", ""))?;

            match Png::new(status_code) {
                Some(png) => {
                    png.image(&config);
                    println!("Successfully converted image {file_name}");
                }
                None => {
                    println!("Image {file_name} os not a valid status image, skipping")
                }
            }
        }
    }

    Ok(())
}
