use image::{GenericImageView, Pixel};
use image::imageops::FilterType;

pub fn display_album_cover(path: &str, size: u32) {
    let img = image::open(path).unwrap();
    let scaled = img.resize(size, size, FilterType::Nearest);
    let scaled_pixels = scaled.pixels().map(|p| p.2.to_rgb()).collect::<Vec<_>>();

    let grid = (0..size as usize)
        .map(|i| scaled_pixels[i*size as usize..(i+1)*size as usize]
            .iter()
            .map(|rgb| format!("\u{1b}[48;2;{:?};{:?};{:?}m  ", rgb[0], rgb[1], rgb[2]))
            .collect::<Vec<_>>().join("")
        ).collect::<Vec<_>>().join("\u{1b}[0m\n");

    println!("{}\u{1b}[0m", grid);
}
