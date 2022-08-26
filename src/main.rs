use clap::Parser;
use image::{GenericImageView, RgbImage};
use std::path::{Path, PathBuf};

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// Resize image to a fixed defined size
    Resize {
        /// Images input paths
        paths: Vec<String>,

        /// Output width
        #[clap(forbid_empty_values = true, short, long)]
        width: u32,

        /// Output height
        #[clap(forbid_empty_values = true, long)]
        height: u32,

        /// Output path to dump transformed images
        #[clap(long, short)]
        output_directory: Option<String>,
    },
    /// Rescale both dimensions of the image
    Rescale {
        /// Images input paths
        paths: Vec<String>,

        /// Ratio to resize image dimensions
        #[clap(short, long)]
        ratio: Option<f32>,

        /// New size of the shortest image dimension, the other dimension will be rescaled proportionally
        #[clap(long, short)]
        min_size: Option<u32>,

        /// Output path to dump transformed images
        #[clap(long, short)]
        output_directory: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Resize {
            paths,
            width,
            height,
            output_directory,
        } => {
            return_error("Resize not implemented yet".to_string()).unwrap();
            // for path in paths {
            //     let output_path = get_output_path(&path, &output_directory);
            //     resize_image(path, output_path, width, height).unwrap()
            // }
        }
        Action::Rescale {
            paths,
            ratio,
            min_size,
            output_directory,
        } => {
            for path in paths {
                // let output_path = get_output_path(&path, &output_directory);
                rescale_image(path, ratio, min_size, &output_directory).unwrap()
            }
        } // _ => return_error("Subcommand does not exist".to_string()).unwrap(),
    };
}

fn rescale_image(
    path: String,
    ratio: Option<f32>,
    min_size: Option<u32>,
    output_directory: &Option<String>,
) -> Result<(), String> {
    let input_path = Path::new(&path);
    let output_file_name: String;
    let mut new_height: u32 = 0;
    let mut new_width: u32 = 0;
    let mut output_image: image::DynamicImage =
        image::DynamicImage::ImageRgb8(RgbImage::new(10, 10));
    let mut output_path: PathBuf = PathBuf::new();
    match ratio {
        Some(ratio) => {
            let img = image::open(path.clone()).unwrap();
            let (width, height) = img.dimensions();
            new_width = (width as f32 * ratio) as u32;
            new_height = (height as f32 * ratio) as u32;
            output_image = img.resize(new_width, new_height, image::imageops::Triangle);
            // img.save(output_path).unwrap();
        }
        None => match min_size {
            Some(min_size) => {
                let img = image::open(path.clone()).unwrap();
                let (width, height) = img.dimensions();
                let scale = if width > height {
                    min_size as f32 / width as f32
                } else {
                    min_size as f32 / height as f32
                };
                new_width = (scale * width as f32) as u32;
                new_height = (scale * height as f32) as u32;
                output_image = img.resize(new_width, new_height, image::imageops::Triangle);
            }
            None => return_error("No ratio or min_size given".to_string()).unwrap(),
        },
    }

    output_file_name = format!(
        "{}_{}x{}.jpg",
        input_path.file_stem().unwrap().to_str().unwrap(),
        new_width,
        new_height
    );

    match output_directory {
        Some(output_directory) => {
            output_path = Path::new(output_directory).join(output_file_name);
        }
        None => {
            output_path = input_path.with_file_name(output_file_name);
        }
    }
    output_image.save(output_path).unwrap();

    Ok(())
}

fn return_error(msg: String) -> Result<(), String> {
    Err(msg)
}
