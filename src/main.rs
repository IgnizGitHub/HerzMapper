use std::{
    collections::{HashMap, HashSet},
    fs,
    fs::File,
    io::Write,
    io::stdin,
    time::Instant,
};
use clap::Parser;
use image::{open, imageops::resize, ImageBuffer, Rgb, ImageReader};
use kiddo::{KdTree, SquaredEuclidean};
use rayon::prelude::*;
use serde_json::{json, Value};
use flate2::{write::ZlibEncoder, Compression};
use anyhow::{Context, Result};

// Command-line arguments for the program
#[derive(Parser)]
#[command(name = "Image Processor")]
#[command(about = "Processes an image using a color palette and compresses output", long_about = None)]
struct Args {
    // Path to the input image file (ex: "images/example.png")
    #[arg(value_name = "IMAGE_FILE", help = "Specify the input image file (ex: images/example.png)")]
    input: Option<String>, // Now optional to allow drag-and-drop

    // Path to the palette file containing color mappings (ex: "palettes/all.txt")
    #[arg(short, long, value_name = "PALETTE_FILE", default_value = "palettes/no-special.txt", help = "Specify the color palette file (ex: palette/all.txt)")]
    palette: String,

    // Path to the JSON map data file. Defaults to "map_data.json" if not provided
    #[arg(short, long = "map-data", default_value = "map_data.json", value_name = "MAP_JSON", help = "Specify the JSON map data file")]
    map_data: String,

    // Output file where the processed data will be saved. Defaults to "map.wbox"
    #[arg(short, long, default_value = "map.wbox", value_name = "OUTPUT_FILE", help = "Specify the output file name")]
    output: String,

    // Optional path to a world laws file (ex: worldlaws/default.txt)
    #[arg(short, long, value_name = "WORLD_LAWS_FILE",  default_value = "worldlaws/default.txt", help = "Path to a world laws .txt file")]
    world_laws: String,

    // Optional freeze map image: white pixels in this image will be marked as frozen
    #[arg(short, long, value_name = "FREEZE_MAP_IMAGE", help = "Specify an optional freeze map image file (ex: images/frozen.png)")]
    freeze_map: Option<String>,

    // Disables pause before exiting
    #[arg(short, long, value_name = "NO_PAUSE", default_value_t = true, action = clap::ArgAction::SetFalse, help = "Specify this if you don't want the program to pause before exit")]
    no_pause: bool,
}

// Resizes an image to the nearest multiple of 64, ensuring it's at least 128x128
fn resize_to_nearest_64(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let new_width = ((width + 63) / 64).max(2) * 64;
    let new_height = ((height + 63) / 64).max(2) * 64;
    resize(&img, new_width, new_height, image::imageops::FilterType::Nearest)
}

// Converts a hex color string (e.g., "#RRGGBB") to an RGB tuple
fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    u32::from_str_radix(hex.trim_start_matches('#'), 16).ok().map(|c| {
        (
            ((c >> 16) & 0xff) as u8,
            ((c >> 8) & 0xff) as u8,
            (c & 0xff) as u8,
        )
    })
}

// Compresses the given JSON data string to a .wbox file using zlib compression
fn compress_to_wbox(json_data: &str, output_path: &str) -> std::io::Result<()> {
    let output_file = File::create(output_path)?;
    let mut encoder = ZlibEncoder::new(output_file, Compression::fast());
    encoder.write_all(json_data.as_bytes())?;
    encoder.finish()?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }

    if args.no_pause {
        println!("Press Enter to exit...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }
}


fn run() -> Result<()> {

    let args = Args::parse();
    let start = Instant::now();

    // Load the palette file ("palette.txt") where each line is "id #RRGGBB"
    let palette_content = fs::read_to_string(&args.palette)
        .with_context(|| format!("Failed to read palette file: {}", args.palette))?;
    let (mut palette_ids, mut palette_points) = (Vec::new(), Vec::new());
    for line in palette_content.lines() {
        if let Some((id, hex)) = line.split_once(' ') {
            if let Some((r, g, b)) = hex_to_rgb(hex) {
                palette_ids.push(id.to_string());
                palette_points.push([r as f64, g as f64, b as f64]);
            }
        }
    }

    // Build a kd-tree from the palette (3-dimensional points, storing u64 indices)
    let mut kdtree: KdTree<f64, 3> = KdTree::new();
    for (i, point) in palette_points.iter().enumerate() {
        kdtree.add(point, i as u64);
    }
    println!("Palette loaded in {:?}", start.elapsed());

    // Ensure we have a valid input path
    let input_path = args.input.as_ref().context("No input file provided")?;

    let mut img = ImageReader::open(input_path)?
        .with_guessed_format()?
        .decode()?
        .into_rgb8();
    
    img = resize_to_nearest_64(img);
    
    // Extract unique colors from the image
    let unique: HashSet<(u8, u8, u8)> = img.pixels().map(|p| (p[0], p[1], p[2])).collect();

    // In parallel, map each unique color to its nearest palette color
    let mapping: HashMap<_, _> = unique.into_par_iter().map(|col| {
        let query = [col.0 as f64, col.1 as f64, col.2 as f64];
        let nn: kiddo::NearestNeighbour<f64, u64> = kdtree.nearest_one::<SquaredEuclidean>(&query);
        let idx = nn.item as usize;
        let pal = palette_points[idx];
        let new = ((pal[0] as u8), (pal[1] as u8), (pal[2] as u8));
        (col, (palette_ids[idx].clone(), new))
    }).collect();

    // Replace each pixel with its nearest palette color in parallel
    img.as_mut().par_chunks_mut(3).for_each(|pixel| {
        let key = (pixel[0], pixel[1], pixel[2]);
        if let Some((_, new)) = mapping.get(&key) {
            pixel[0] = new.0;
            pixel[1] = new.1;
            pixel[2] = new.2;
        }
    });
    println!("Image processed in {:?}", start.elapsed());

    // Save the processed image
    img.save("output.jpg").expect("Failed to save output.jpg");
    println!("Image saved in {:?}", start.elapsed());

    // Update map_data JSON
    let mut map_data: Value = fs::read_to_string(&args.map_data)
        .with_context(|| format!("Failed to read map data file: {}", args.map_data))?
        .parse()
        .context("JSON parse error")?;

    // tileMap set here
    if let Some(tile_map) = map_data.get_mut("tileMap").and_then(|v| v.as_array_mut()) {
        for id in mapping.values().map(|(id, _)| id.clone()).collect::<HashSet<_>>() {
            tile_map.push(json!(id));
        }
    } else {
        eprintln!("tileMap array not found in JSON");
    }
    
    // Run-length Encoding for the tileArray and tileAmounts
    // Don't ask me how it works I don't know
    let (w, h) = img.dimensions();
    let tmap: Vec<_> = map_data["tileMap"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();
    let pidx = tmap
        .iter()
        .enumerate()
        .map(|(i, id)| (id, i))
        .collect::<HashMap<_, _>>();
    let rgb_to_id = mapping.iter().fold(HashMap::new(), |mut m, (_, (id, col))| {
        m.insert(*col, id);
        m
    });
    let (tile_array, tile_amounts): (Vec<_>, Vec<_>) = (0..h).rev()
        .map(|y| {
            (0..w).fold((Vec::new(), Vec::new()), |(mut tiles, mut counts), x| {
                let p = img.get_pixel(x, y).0;
                let tuple = (p[0], p[1], p[2]);
                let idx = *pidx
                    .get(rgb_to_id.get(&tuple).expect("Color not found"))
                    .expect("ID not in palette index");
                if tiles.last() == Some(&idx) {
                    *counts.last_mut().unwrap() += 1;
                } else {
                    tiles.push(idx);
                    counts.push(1);
                }
                (tiles, counts)
            })
        })
        .unzip();

    map_data["height"] = json!(img.height() / 64);
    map_data["width"] = json!(img.width() / 64);
    map_data["tileArray"] = json!(tile_array);
    map_data["tileAmounts"] = json!(tile_amounts);

    //Process our World Laws and Append them to the list
    let laws = fs::read_to_string(&args.world_laws)?;
    let list = {
        if let Some(list) = map_data.get_mut("worldLaws")
            .and_then(|wl| wl.get_mut("list"))
            .and_then(Value::as_array_mut) {
            list
        } else {
            map_data["worldLaws"] = json!({ "list": [] });
            map_data["worldLaws"]["list"].as_array_mut().unwrap()
        }
    };
    
    list.extend(
        laws.lines()
            .filter_map(|l| l.split_once(' '))
            .map(|(k, v)| {
                let k = k.trim();
                if v.trim().eq_ignore_ascii_case("true") {
                    json!({ "name": k })
                } else {
                    json!({ "name": k, "boolVal": false })
                }
            })
    );
    

    // Optionally process freeze_map image to add frozen_tiles to map_data
    if let Some(freeze_map_path) = &args.freeze_map {
        println!("Processing freeze map: {}", freeze_map_path);
        let freeze_img = open(freeze_map_path)
            .with_context(|| format!("Failed to open freeze map: {}", freeze_map_path))?
            .into_rgb8();
        let mut frozen_tiles = Vec::new();
        // Record the index of every white pixel (RGB == 255,255,255)
        for (i, pixel) in freeze_img.pixels().enumerate() {
            if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255 {
                frozen_tiles.push(i as u32);
            }
        }
        map_data["frozen_tiles"] = json!(frozen_tiles);
        println!("Frozen tiles added: {}", frozen_tiles.len());
    }

    // Serialize JSON to string and compress to output file
    let json_string =
        serde_json::to_string_pretty(&map_data).expect("Failed to serialize map_data JSON");

    println!("JSON updated in {:?}", start.elapsed());

    compress_to_wbox(&json_string, &args.output)
        .with_context(|| format!("Failed to compress output to: {}", args.output))?;
    println!("Compression successful. Output written to {}", args.output);

    println!("Total execution time: {:?}", start.elapsed());

    Ok(())
}


