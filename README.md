# Worldbox Image-to-Map Converter  

A Rust-based tool that converts images into Worldbox: God Simulator maps. \
This utility transforms pixel data into a `.wbox` map file using a customizable palette, making it easy to create custom maps from images.  

## Features  
- Convert images into Worldbox-compatible maps  
- Supports easily customizable color palettes for accurate biome and terrain mapping  
- Optional freeze map support to mark specific areas as frozen  
- Drag-and-drop support for easy usage  
- Outputs a `.wbox` file ready to be loaded into Worldbox  

## Installation  

Currently this is in an Alpha Testing phase \
The built executable is in Pre-Releases with a version for `Windows` machines. \
Source code and other platform support will be released after some testing. \
As well as simple guide for building from source code.


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
| `--freeze-map` | `-f` | *(Optional)*           | Secondary Image where white pixels represent frozen areas. Must be same-size as your Input Image |
| `--no-pause`  | `-n` | Enabled                 | Disables pause before exit. Useful if using in an automated setup. |

### Drag-and-Drop Support  
Simply drag an image file onto the executable to automatically process it using the default palette and settings.

## All Custom Example  
Convert an image using all possible custom options and save it as `custom_map.wbox`:
(Requires a jpg with 'customimage.jpg' as the title.)

```sh
herzmapper.exe customimage.jpg  --palette my_palette.txt --map-data custom_data.json --freeze-map images/frozen.png --no-pause --output custom_map.wbox
```
or with all short-hands

```sh
herzmapper.exe customimage.jpg  --p my_palette.txt --m custom_data.json --f images/frozen.png --n --o custom_map.wbox
```

## Contributing  
Feel free to open issues or submit pull requests to improve this project!
