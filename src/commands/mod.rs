mod prelude {
    pub use serenity::framework::standard::CommandResult;
    pub use serenity::framework::standard::macros::command;
    pub use serenity::model::prelude::*;
    pub use serenity::prelude::*;
}

mod ping;
mod note;

use ping::*;
use note::*;

use serenity::framework::standard::macros::*;

#[group]
#[commands(ping, note, notes)]
struct General;
