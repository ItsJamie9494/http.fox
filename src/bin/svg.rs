use std::error::Error;
use std::str::FromStr;

use httpfox::{config::Config, svg::Svg};

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

        if file_name.contains("raw") {
            println!("Converting image {}", file_name);

            let status_code = i32::from_str(&file_name.replace("_raw", ""))?;

            let svg = Svg::new(status_code);

            svg.image(&config);
        }
    }

    Ok(())
}
