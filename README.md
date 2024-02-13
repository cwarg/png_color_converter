# png_color_converter

Translates the colors of a given 16x16 png to a different hue, while retaining the saturation and lightness. Original purpose for this project was to convert Minecraft textures from one color to another while maintaining the shading/detail such as creating a ruby texture from a sapphire texture.

## Examples
Sapphire.png was made manually and ruby, emerald, and amethyst were created with this program. These show up quite small, but if you were to open them up in IntelliJ, GIMP, or some other program you can see that the 16x16 texture maintained it's quality.

![](./resources/sapphire.png) -> ![](./resources/ruby.png)\
![](./resources/sapphire.png) -> ![](./resources/emerald.png)\
![](./resources/sapphire.png) -> ![](./resources/amethyst.png)

## Installation
```shell
cargo install png_color_converter
```

## Usage
```shell
png_color_converter --input-file <Path to input png to convert.> --output-file <Output path (including file name) where you'd like the converted file to be saved.> --color <Choose one of the following: red, orange, yellow, green, blue, indigo, violet...>
```

## Acknowledgements
- See [resources.txt](./resources/resources.txt) for articles that I used for different formulas and examples.
