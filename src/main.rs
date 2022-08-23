use clap::Parser;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage};
use std::path::{Path, PathBuf};

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Resize {
        // #[clap(forbid_empty_values = true, long, short)]
        paths: Vec<String>,
        #[clap(forbid_empty_values = true, long)]
        width: u32,
        #[clap(forbid_empty_values = true, long)]
        height: u32,
        #[clap(long, short)]
        output_directory: Option<String>,
    },
    Rescale {
        // #[clap(forbid_empty_values = true, long, short)]
        paths: Vec<String>,
        #[clap(short, long)]
        ratio: f32,
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
            for path in paths {
                let output_path = get_output_path(&path, &output_directory);
                resize_image(path, output_path, width, height).unwrap()
            }
        }
        Action::Rescale {
            paths,
            ratio,
            output_directory,
        } => {
            for path in paths {
                let output_path = get_output_path(&path, &output_directory);
                rescale_image(path, output_path, ratio).unwrap()
            }
        } // _ => return_error("Subcommand does not exist".to_string()).unwrap(),
    };
}

fn get_output_path(path: &str, output_directory: &Option<String>) -> PathBuf {
    let input_path = Path::new(path);
    if let Some(output_directory) = output_directory {
        let output_path = Path::new(output_directory).join(format!(
            "{}_transformed",
            input_path.file_stem().unwrap().to_str().unwrap()
        ));
    } else {
        let output_path = input_path.with_file_name(format!(
            "{}_transformed.jpg",
            input_path.file_stem().unwrap().to_str().unwrap(),
        ));
    }
    return output_path;
}

fn resize_image(path: String, output_path: PathBuf, width: u32, height: u32) -> Result<(), String> {
    let img = image::open(path.clone()).unwrap();
    let img = img.resize(width, height, image::imageops::Triangle);
    img.save(output_path).unwrap();
    Ok(())
}

fn rescale_image(path: String, output_path: PathBuf, ratio: f32) -> Result<(), String> {
    if ratio > 1.0 {
        return Err("Ratio must be less than 1.0".to_string());
    }
    let img = image::open(path.clone()).unwrap();
    let (width, height) = img.dimensions();
    let img = img.resize(
        (ratio * width as f32) as u32,
        (ratio * height as f32) as u32,
        image::imageops::Triangle,
    );
    img.save(output_path).unwrap();
    Ok(())
}

// fn return_error(msg: String) -> Result<(), String> {
//     Err(msg)
// }
