# Worldbox Image-to-Map Converter  

An Open Source Rust-based tool that converts images into Worldbox: God Simulator maps. \
This utility transforms pixel data into a `.wbox` map file using a customizable palette, making it easy to create custom maps from images. \
This tool built to be fast and customizable to the user's needs. \
If you have improvements, feel free to contribute to this project! 

## Features  
- Convert images into Worldbox-compatible maps  
- Supports easily customizable color palettes for accurate biome and terrain mapping
- Provides an Adobe Photoshop color file (A palette of all colors for usage in Image editing programs)
- Setting the enabled/disabled state of World Laws
- Optional freeze map support to mark specific areas as frozen  
- Drag-and-drop support for easy usage  
- Outputs a `.wbox` file ready to be loaded into Worldbox  

## Installation (Using Prebuilt Binary)
# Currently Supporting Windows/Linux
Download the appropriate build from [this link](https://github.com/IgnizGitHub/HerzMapper/releases/tag/v1.0.0) \
Extract the contents of the zipped files into a folder. \
Check out the [usage info below](https://github.com/IgnizGitHub/HerzMapper?tab=readme-ov-file#usage) for further steps.


## Installation (From Source Code)
Only do this if you want to compile the program yourself.

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Git (optional, for cloning)

Clone the repository and build the project with Cargo:

```bash
git clone https://github.com/IgnizGitHub/HerzMapper.git
cd HerzMapper
cargo build --release
```

## Usage  

Run the program with all defaults, using our example image in command line.

```sh
herzmapper.exe images/example.png 
```

### CLI Arguments  

| Argument        | Short | Default                 | Description |
|---------------|------|------------------------|-------------|
| `--input`     | None | *(Required)*           | Path to the input image file (ex: `images/example.png`). (Required) |
| `--palette`   | `-p` | `palettes/no-special.txt` | Path to the palette file defining color mappings. (Required)  |
| `--map-data`  | `-m` | `map_data.json`         | JSON file containing additional map data. (Required) |
| `--output`    | `-o` | `map.wbox`              | Output file name for the converted map. |
| `--world-laws`  | `-w` | `worldlaws/default.txt` | Path to world laws file allowing for enabling/disabling certain world laws |
| `--freeze-map` | `-f` | *(Optional)*           | Secondary Image where white pixels represent frozen areas. Must be same-size as your Input Image |
| `--no-pause`  | `-n` | Enabled                 | Disables pause before exit. Useful if using in an automated setup. |

### Drag-and-Drop Support  
Simply drag an image file onto the executable to automatically process it using the default palette and settings.

## All Custom Example  
Convert an image using all possible custom options and save it as `custom_map.wbox`:
(Requires a jpg with 'customimage.jpg' as the title.)

```sh
herzmapper.exe customimage.jpg  --palette my_palette.txt --map-data custom_data.json --world_laws worldlaws/gaia.txt --freeze-map images/frozen.png --no-pause --output custom_map.wbox
```
or with all short-hands

```sh
herzmapper.exe customimage.jpg  --p my_palette.txt --m custom_data.json --w worldlaws/gaia.txt --f images/frozen.png --n --o custom_map.wbox
```

## Contributing  
Feel free to open issues or submit pull requests to improve this project!
