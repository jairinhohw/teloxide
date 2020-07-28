use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without
/// sound) stored on the Telegram servers.
///
/// By default, this animated MPEG-4 file will be sent by the user with an
/// optional caption. Alternatively, you can use `input_message_content` to send
/// a message with the specified content instead of the animation.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedmpeg4gif).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier for the MP4 file.
    pub mpeg4_file_id: String,

    /// Title for the result.
    pub title: Option<String>,

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the video animation.
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedMpeg4Gif {
    pub fn new<S1, S2>(id: S1, mpeg4_file_id: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            mpeg4_file_id: mpeg4_file_id.into(),
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(val.into());
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode<S>(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}
