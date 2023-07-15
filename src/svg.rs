use std::{error::Error, io::Cursor, path::PathBuf};

use base64::{prelude::BASE64_STANDARD, Engine};
use image::{io::Reader, ImageOutputFormat};
use resvg::usvg::{fontdb, Options, TreeParsing, TreeTextToPath};

use crate::config::Config;

// Constants
const IMAGE_WIDTH: u32 = 750;
const IMAGE_HEIGHT: u32 = 600;

// Embeds
const FREESERIF_FONT: &[u8] = include_bytes!("include/freeserif.ttf");
const IMAGE_TEMPLATE: &'static str = include_str!("include/template.svg");

/// Handle the creation of SVGs for each image
pub struct Svg {
    pub status: i32,
}

impl Svg {
    pub fn new(status: i32) -> Self {
        Self { status }
    }

    pub fn image(&self, config: &Config) -> PathBuf {
        let mut images_dir = config.images_dir.clone();

        self.create_image(config).expect("Could not create image");

        images_dir.push(format!("{}.png", self.status));
        images_dir.to_path_buf()
    }

    // Manually create the image if it doesn't exist
    fn create_image(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::new();

        let mut raw_file = config.images_dir.clone();
        raw_file.push(format!("{}_raw.png", self.status));
        let mut img_file = config.images_dir.clone();
        img_file.push(format!("{}.png", self.status));

        let svg = Reader::open(raw_file)?.decode()?;
        let writer = BASE64_STANDARD;
        let mut cursor = Cursor::new(&mut buffer);

        svg.write_to(&mut cursor, ImageOutputFormat::Png)?;
        let base64_encoded = writer.encode(buffer);

        let svg_image_string = format!("data:image/png;base64,{base64_encoded}");

        // TODO: message
        let image = IMAGE_TEMPLATE
            .replace("DATA_IMAGE_URL", &svg_image_string)
            .replace("__STATUS", &self.status.to_string());
        let mut font_db = fontdb::Database::new();
        font_db.load_font_data(FREESERIF_FONT.to_vec());

        let mut tree = resvg::usvg::Tree::from_str(&image, &Options::default())?;
        tree.convert_text(&font_db);

        let mut pixmap =
            resvg::tiny_skia::Pixmap::new(IMAGE_WIDTH, IMAGE_HEIGHT)
                .ok_or(Box::<dyn Error>::from("Failed to create internal Pixmap"))?;
        resvg::Tree::from_usvg(&tree)
            .render(resvg::usvg::Transform::default(), &mut pixmap.as_mut());

        pixmap.save_png(&img_file)?;
        Ok(())
    }
}
