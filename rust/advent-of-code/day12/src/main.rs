mod mountains;
mod parse;

use parse::parse_file;

fn main() {
    let map = parse_file("input");
    map.save_map_image("img.bmp").unwrap();
}
