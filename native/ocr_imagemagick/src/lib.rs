use magick_rust::{MagickWand, PixelWand, magick_wand_genesis};
use std::sync::Once;
use std::fs;

pub fn jpg_combine(images: Vec<Vec<u8>>) -> Vec<u8> {
    println!("jpg combine!")
}

pub fn jpg_combine_reverse(images: Vec<Vec<u8>>) -> Vec<u8> {
    println!("jpg combine reverse!")
}

rustler::init!("Elixir.OcrImagemagick", [jpg_combine, jpg_combine_reverse]);
