use crossterm::style::{Color, Stylize};
mod data_loader;

fn main() {
    println!("Hello, world!");
    let dataset = data_loader::load_file_u8(
        &include_bytes!("../train-labels")[..],
        &include_bytes!("../train-images")[..],
    );
    for pair in dataset.data {
        let (image, label) = (pair.0, pair.1);
        println!("image {label}");
        for row in image.data {
            for pixel in row {
                print!(
                    "{}",
                    "  ".on(Color::Rgb {
                        r: pixel,
                        g: pixel,
                        b: pixel,
                    })
                );
            }
            println!()
        }
        todo!("neural network stuff");
    }
}
