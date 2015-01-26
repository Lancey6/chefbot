extern crate hyper;
extern crate rustirc;
extern crate rustircbot;
extern crate serialize;

use rustirc::client::Client;
use rustirc::info::IrcInfo;
use rustircbot::bot::Bot;

mod commands;
mod tinychat;

fn main() {
    let mut infobox = Box::new( IrcInfo::gen( "Chef", "Chef",
      "Chef Bot by Lancey",
      vec![ "#bakedfurs", "#thefuture" ] ) );
    let mut bot = Bot::connect( "irc.furnet.org", 6667, "", infobox );
    bot.set_help_info( "Chef Bot", "Lancey", "0.1.2" );

    commands::init_commands( &mut bot );

    bot.start( );
}
