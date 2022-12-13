mod map_image;
mod mountains;
mod parse;

use std::path::Path;
use std::thread::available_parallelism;

use map_image::{save_image, MapImageOptions, MapType};
use parse::parse_file;

fn main() {
    let img_path = Path::new("img");
    let is_test = false;
    let (file_name, image_name) = match is_test {
        true => ("input.test", "img.test.bmp"),
        false => ("input", "img.bmp"),
    };
    
    let height_img = img_path.join(Path::new(image_name));
    let distance_img = img_path.join(Path::new(&format!("distance-{}", image_name)));
    let visited_img = img_path.join(Path::new(&format!("visited-{}", image_name)));

    let mut map = parse_file(file_name);

    map.process_shortest_distance();

    let height_opts = MapImageOptions {
        file_name: height_img.to_str().unwrap().to_string(),
        map_type: MapType::Height,
        draw_path: !is_test,
    };
    save_image(&map, height_opts).unwrap();

    let opts = MapImageOptions {
        file_name: distance_img.to_str().unwrap().to_string(),
        map_type: MapType::DistanceFromStart,
        draw_path: !is_test,
    };
    save_image(&map, opts).unwrap();

    let visited_opts = MapImageOptions {
        file_name: visited_img.to_str().unwrap().to_string(),
        map_type: MapType::Visited,
        draw_path: !is_test,
    };
    save_image(&map, visited_opts).unwrap();
}
