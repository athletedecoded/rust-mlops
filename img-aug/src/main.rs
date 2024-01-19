use image::imageops::FilterType;
use image::ImageFormat;
use rayon::prelude::*;
use std::fmt;
use std::fs::File;
use std::time::{Duration, Instant};

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

// Function to perform image augmentations
fn main() {
    let img = image::open("assets/test.jpg").unwrap();
    let augmentations = [
        ("blur", img.blur(10.0)),
        ("flip", img.fliph()),
        ("cw90", img.rotate90()),
        ("gauss", img.resize(256, 256, FilterType::Gaussian)),
        ("bright", img.brighten(50)),
        ("grayed", img.grayscale()),
    ];
    // Parallelize over the augmentations
    let timer = Instant::now();
    augmentations.par_iter().for_each(|(augmentation, fxn)| {
        let aug_img = fxn;
        let mut output = File::create(format!("assets/img-{}.png", augmentation)).unwrap();
        aug_img.write_to(&mut output, ImageFormat::Png).unwrap();
    });
    println!(
        "Parallel augmentations completed in {}",
        Elapsed::from(&timer)
    );

    // Serial walk over the augmentations
    // let timer = Instant::now();
    // for (augmentation, fxn) in augmentations.iter()
    // {
    //     let aug_img = fxn;
    //     let mut output = File::create(format!("assets/img-{}.png", augmentation)).unwrap();
    //     aug_img.write_to(&mut output, ImageFormat::Png).unwrap();
    // }
    // println!("Serial augmentations completed in {}", Elapsed::from(&timer));
}
