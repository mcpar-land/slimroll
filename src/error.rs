#[derive(Debug)]
pub enum EmojiError {
	NoEmojiServer,
	ImageGenerationError,
	EmojiUploadFailed,
	EmojiDeletionFailed,
	CouldNotParseEmojiName,
	RollError(caith::RollError),
	BadCommand,
}
impl std::error::Error for EmojiError {}
impl std::fmt::Display for EmojiError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<regex::Error> for EmojiError {
	fn from(_: regex::Error) -> Self {
		Self::CouldNotParseEmojiName
	}
}

impl From<caith::RollError> for EmojiError {
	fn from(re: caith::RollError) -> Self {
		Self::RollError(re)
	}
}
