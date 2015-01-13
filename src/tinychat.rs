use rustirc::message::Message;
use rustircbot::command::Cmd;
use rustirc::client::Client;

pub struct GetTcCb;
impl Cmd for GetTcCb {
  fn on_cmd( &mut self, msg : Message, cnt : &mut Client ) {
    if msg.is_public( ) {
      match msg.target( ) {
        Some( chan ) => cnt.send_msg(
          Message::privmsg( chan, "http://tinychat.com/bakedfurs" ) ),
        None         => (),
      }
    } else {
      match msg.nick( ) {
        Some( nick ) => cnt.send_msg(
          Message::privmsg( nick.as_slice( ), "http://tinychat.com/bakedfurs" ) ),
        None         => (),
      }
    }
  }
}

pub struct WelcomeCb;
impl Cmd for WelcomeCb {
  fn on_cmd( &mut self, msg : Message, cnt : &mut Client ) {
    cnt.send_str( "MODE Chef +b" );
    cnt.send_str( "PRIVMSG Nickserv identify ReallyGreatVacation" );
  }
}