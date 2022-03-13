use image::Pixel;
use image::imageops::thumbnail;

pub fn image_string(path: &str, size: u32) -> String {
    let img = image::open(path).unwrap();

    format!("{}\u{1b}[0m",
        thumbnail(&img, size, size)
            .pixels()
            .map(|p| p.to_rgb())
            .collect::<Vec<_>>()
            .chunks(size as usize)
            .map(|r| 
                r
                    .into_iter()
                    .map(|rgb| format!("\u{1b}[48;2;{:?};{:?};{:?}m  ", rgb[0], rgb[1], rgb[2]))
                    .collect::<Vec<_>>()
                    .join("")
            )
            .collect::<Vec<_>>()
            .join("\u{1b}[0m\n")
    )
}
