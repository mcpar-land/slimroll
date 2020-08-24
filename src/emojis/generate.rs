use crate::EmojiError::{self, *};
use image::{ImageBuffer, Pixel, Rgb, Rgba, RgbaImage};
use imageproc::{drawing, rect::Rect};
use lazy_static::lazy_static;
use rusttype::{Font, Scale};
use std::fs;

lazy_static! {
	static ref FONT: Font<'static> = {
		let raw =
			fs::read("./resources/DMMono-Medium.ttf").expect("Font not found");

		Font::try_from_vec(raw).expect("Coult not load font")
	};
}

static SCALE: Scale = Scale { x: 100.0, y: 100.0 };

pub fn emoji_for_char(c: char) -> Result<RgbaImage, EmojiError> {
	let mut img = ImageBuffer::from_fn(64, 64, |_, _| Rgba([255, 0, 0, 255]));

	let glyph = FONT.glyph(c).scaled(SCALE);

	let bounding_box = glyph.exact_bounding_box().ok_or(ImageGenerationError)?;

	println!(
		"{} : {} x {}",
		c,
		bounding_box.width(),
		bounding_box.height()
	);

	let width = bounding_box.width();
	let height = bounding_box.height() - FONT.v_metrics(SCALE).ascent;

	// let x = 32 - (width as u32 / 2);
	// let y = 32 - (height as u32 / 1);

	// img = drawing::draw_filled_rect(
	// 	&mut img,
	// 	Rect::at(x as i32, y as i32).of_size(width as u32, height as u32),
	// 	Rgba([0, 255, 255, 255]),
	// );

	img = drawing::draw_text(
		&mut img,
		Rgba([255, 255, 255, 255]),
		0,
		0,
		SCALE,
		&FONT,
		&c.to_string(),
	);

	Ok(img)
}

#[cfg(test)]
mod test {
	use super::*;
	use std::path::Path;

	#[test]
	fn gen_all() -> Result<(), Box<dyn std::error::Error>> {
		for i in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].iter() {
			let img = emoji_for_char(*i)?;
			let path_str = format!("./test/test_{}.png", i);
			let path = Path::new(&path_str);
			img.save(path)?;
			println!("saved image: {:?}", path);
		}

		Ok(())
	}
}
