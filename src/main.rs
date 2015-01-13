extern crate rustirc;
extern crate rustircbot;

use rustirc::client::Client;
use rustirc::info::IrcInfo;
use rustircbot::bot::Bot;

mod commands;
mod tinychat;

fn main() {
    let info    = IrcInfo::gen( "Chef", "Chef",
      "Chef Bot by Lancey",
      vec![ "#bakedfurs" ] );
    let mut bot = Bot::connect( "irc.furnet.org", 6667, "", info );

    commands::init_commands( &mut bot );

    bot.start( );
}
