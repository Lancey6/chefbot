use rustircbot::bot::Bot;
use rustircbot::command;
use rustircbot::events;

use bitcoin;
use tinychat;

pub fn init_commands( bot : &mut Bot ) {
  bot.add_cmd( "^!t(iny)?c(hat)?", Box::new( tinychat::GetTcCb ) );
  bot.add_code_cmd( "001", "", Box::new( tinychat::WelcomeCb ) );
  //bot.add_cmd( "-names", Box::new( tinychat::Names ) );
  //bot.add_raw_cmd( "", Box::new( tinychat::PrintRaw ) );
  
  bot.help.add_help( "!tinychat", "Shows the TinyChat URL\nAliases: !tc, !tinyc, !tchat" );
  bot.init_help( "^!help", "!help" );
}