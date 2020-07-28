/// A placeholder, currently holds no information. Use [@BotFather](https://t.me/botfather) to set up your game.
///
/// [The official docs](https://core.telegram.org/bots/api#callbackgame).
use serde::{Deserialize, Serialize};

/// A placeholder, currently holds no information.
///
/// Use [@Botfather] to set up your game.
///
/// [@Botfather]:  https://t.me/botfather
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallbackGame;
