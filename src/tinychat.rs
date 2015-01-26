use std::sync::atomic::Ordering;
use rustirc::message::Message;
use rustircbot::command::Cmd;
use rustirc::client::Client;

pub struct GetTcCb;
impl Cmd for GetTcCb {
  fn on_cmd( &mut self, msg : Message, cnt : &mut Client ) {
    if msg.is_public( ) {
      match msg.target( ) {
        Some( chan ) => cnt.message( chan, "http://tinychat.com/bakedfurs" ),
        None         => (),
      }
    } else {
      match msg.nick( ) {
        Some( nick ) => cnt.message( nick.as_slice( ), "http://tinychat.com/bakedfurs" ),
        None         => (),
      }
    }
  }
}

pub struct WelcomeCb;
impl Cmd for WelcomeCb {
  fn on_cmd( &mut self, msg : Message, cnt : &mut Client ) {
    cnt.send_str( "MODE Chef +B" );
    cnt.identify( "ReallyGreatVacation" );
  }
}

pub struct Names;
impl Cmd for Names {
  fn on_cmd( &mut self, msg : Message, cnt : &mut Client ) {
    let mut outstring = String::from_str( "Users here : " );
    match msg.param( 1 ) {
      Some( chan )  => {
        let info     = cnt.get_info( );
        let namelist = info.get_channel_names( chan.to_string( ) ).unwrap( );
        for name in namelist.iter( ) {
          outstring.push_str( name.as_slice( ) );
        }
        cnt.message( chan, outstring.as_slice( ) );
      },
      None          => (),
    }
  }
}

pub struct PrintRaw;
impl Cmd for PrintRaw {
  fn on_cmd( &mut self, msg : Message, cnt : &mut Client ) {
    println! ( " > {}", msg.raw );
  }
}