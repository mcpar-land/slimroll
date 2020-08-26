use crate::EmojiError::{self, *};
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use resvg::Image;
use usvg::{FitTo, Options, SystemFontDB};

fn fmt_svg(c: char) -> String {
	format!(
		r#"
<svg viewBox="0 0 64 64" width="64" height="64" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
	<style>
		.char {{
			fill: black;
			font-size: 64px;
			font-family: monospace;
			text-align: center;
		}}
	</style>
	<rect x="1" y="1" rx="15" width="62" height="62" stroke-width="2" stroke="black" fill="white" />
	<text
		x="32"
		y="52"
		text-anchor="middle"
		dominant-baseline="middle"
		alignment-baseline="middle"
		class="char"
	>{}</text>
</svg>
"#,
		c
	)
}

lazy_static! {
	static ref opts: Options = {
		let mut o = Options::default();
		o.keep_named_groups = true;
		o.fontdb.load_system_fonts();
		o
	};
}

/// Returns base64 image data
pub fn emoji_for_char(
	c: char,
	bar: Option<&ProgressBar>,
) -> Result<Image, EmojiError> {
	// println!("{}", fmt_svg(c));
	let tree = usvg::Tree::from_str(&fmt_svg(c), &opts)
		.expect("Failed to make tree from str");
	let img = resvg::render(&tree, FitTo::Height(64), None)
		.ok_or(ImageGenerationError)?;
	if let Some(b) = bar {
		b.inc(1);
	}
	Ok(img)
}

pub fn png_base64(image: &Image) -> Result<String, EmojiError> {
	let mut buf: Vec<u8> = vec![];
	{
		let mut encoder =
			png::Encoder::new(&mut buf, image.width(), image.height());
		encoder.set_color(png::ColorType::RGBA);
		encoder.set_depth(png::BitDepth::Eight);
		let mut writer = encoder
			.write_header()
			.or(Err(EmojiError::ImageGenerationError))?;
		writer
			.write_image_data(image.data())
			.or(Err(EmojiError::ImageGenerationError))?;
	};
	Ok(format!("data:image/png;base64,{}", base64::encode(&buf)))
}

pub fn number_emojis() -> Result<[String; 11], EmojiError> {
	let bar = ProgressBar::new(10);
	bar.tick();
	let res = Ok([
		png_base64(&emoji_for_char('0', None)?)?,
		png_base64(&emoji_for_char('1', None)?)?,
		png_base64(&emoji_for_char('2', None)?)?,
		png_base64(&emoji_for_char('3', None)?)?,
		png_base64(&emoji_for_char('4', None)?)?,
		png_base64(&emoji_for_char('5', None)?)?,
		png_base64(&emoji_for_char('6', None)?)?,
		png_base64(&emoji_for_char('7', None)?)?,
		png_base64(&emoji_for_char('8', None)?)?,
		png_base64(&emoji_for_char('9', None)?)?,
		png_base64(&emoji_for_char('-', None)?)?,
	]);
	bar.finish();
	res
}

#[cfg(test)]
mod test {
	use super::*;
	use std::path::Path;

	#[test]
	fn gen_all() -> Result<(), Box<dyn std::error::Error>> {
		for i in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-'].iter() {
			let img = emoji_for_char(*i, None)?;
			let path_str = format!("./test/test_{}.png", i);
			let path = Path::new(&path_str);
			img.save_png(path)?;
			// println!("saved image: {:?}", path);
		}

		Ok(())
	}
}
