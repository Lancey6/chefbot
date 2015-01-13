use rustircbot::bot::Bot;
use rustircbot::command;
use rustircbot::events;

use tinychat;

pub fn init_commands( bot : &mut Bot ) {
  bot.events.register_command(
    "!t(iny)?c(hat)?", Box::new( tinychat::GetTcCb ) );
  bot.events.register_command( "!ident", Box::new( tinychat::WelcomeCb ) );
}

