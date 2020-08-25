#[derive(Debug)]
pub enum EmojiError {
	NoEmojiServer,
	ImageGenerationError,
	EmojiUploadFailed,
}
impl std::error::Error for EmojiError {}
impl std::fmt::Display for EmojiError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}
