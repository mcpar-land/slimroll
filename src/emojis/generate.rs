use crate::EmojiError::{self, *};
use nsvg::image::{RgbaImage, PNG};

fn fmt_svg(c: char) -> String {
	format!(
		r#"
<svg viewBox="0 0 64 64" style="background-color: green">
	<style>
		.char {{
			fill: white;
		}}
	</style>
	<rect x="10" y="10" width="32" height="32" stroke="red" />
	<text x="0" y="32" class="char">adf{}</text>
</svg>
"#,
		c
	)
}

pub fn emoji_for_char(c: char) -> Result<RgbaImage, EmojiError> {
	println!("{}", fmt_svg(c));
	let img = nsvg::parse_str(&fmt_svg(c), nsvg::Units::Pixel, 96.0)
		.or(Err(ImageGenerationError))?
		.rasterize(1.0)
		.or(Err(ImageGenerationError))?;
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
