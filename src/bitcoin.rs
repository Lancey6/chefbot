use hyper::Client;
use hyper::status::StatusCode;
use serialize::{ Decodable, Decoder };
use serialize::json;
use std::collections::HashMap;

type BpiError;

struct BpiTime {
  updated     : String,
  updatedIso  : String,
  updatedUk   : String,
}

struct BpiData {
  code        : String,
  rate        : String,
  description : String,
  rateFloat   : float,
}

struct BpiResponse {
  time        : BpiTime,
  disclaimer  : String,
  bpi         : HashMap < String, BpiData >,
}

impl < S : Decoder < E >, E > Decodable < S, E > for BpiResponse {
  fn decode ( decoder : &mut S ) -> Result < BpiResponse, E > {
    decoder.read_struct( "root", 0, |decoder| {
      let time : BpiTime;
      let data : HashMap <BpiData> = Vec::new( );

      // decode the time from the json
      decoder.read_struct( "time", 0, |decoder| {
        time = BpiTime {
          updated     : try! ( decoder.read_struct_field( "updated", 0, |decoder| Decodable::decode( decoder ) ) ),
          updatedIso  : try! ( decoder.read_struct_field( "updatedISO", 0, |decoder| Decodable::decode( decoder ) ) ),
          updatedUk   : try! ( decoder.read_struct_field( "updateduk", 0, |decoder| Decodable::decode( decoder ) ) ),
        };
      } );

      // decode the data from the json
      decoder.read_map( "bpi", 0, |bpimap| {
        for ( code, bpi ) in bpimap.iter( ) {
          data.insert( code, BpiData {
            code        : try! ( bpi.read_struct_field( "code", 0, |decoder| Decodable::decode( decoder ) ) ),
            rate        : try! ( bpi.read_struct_field( "rate", 0, |decoder| Decodable::decode( decoder ) ) ),
            description : try! ( bpi.read_struct_field( "description", 0, |decoder| Decodable::decode( decoder ) ) ),
            rateFloat   : try! ( bpi.read_struct_field( "rate_float", 0, |decoder| Decodable::decode( decoder ) ) ),
          } );
        }
      } );

      // assemble the response struct
      Ok ( BpiResponse {
        time        : time,
        disclaimer  : try! ( decoder.read_struct_field( "disclaimer", 0, |decoder| Decodable::decode( decoder ) ) ),
        bpi         : data,
      } )
    } )
  }
}

fn get_bpi( sym : & str ) -> Result < BpiResponse, BpiError > {
  let mut client  = Client::new( );
  let url         = if sym.is_empty( ) {
    String::from_str( "http://api.coindesk.com/v1/bpi/currentprice.json" )
  } else {
    format! ( "http://api.coindesk.com/v1/bpi/currentprice/{}.json", sym )
  }
  let mut resp    = client.get( url ).send( );
  match resp {
    Ok ( body ) => {
      println! ( " ~ got bpi response from api.coindesk.com" );
      match body.status {
        StatusCode::Ok => Ok( json::decode( body ).unwrap( ) ),
        _              => Err( BpiError ),
      }
    },
    Err ( e )   => {
      println! ( "!! error getting bpi - http error: {}", e );
      Err( BpiError )
    },
  }
}

pub struct GetBpiIndex;
impl Cmd for GetBpiIndex {
  fn on_cmd( &mut self, msg : Message, cnt : &mut Client ) {
    // get parameters
    let params : Vec < &str > = msg.trailing( ).unwrap( ).words( ).collect( );
    let sym = if params.len( ) == 1 {
      "USD"
    } else {
      params[1]
    }
    let bpires = get_bpi( sym );

    if msg.is_public( ) {
      match bpires {
        Ok ( res ) => {
          match msg.param( 1 ) {
            Some ( chan ) => {
              let rate = match res.bpi.get( &(sym.to_string( )) ) {
                Some ( r ) => r.rateFloat,
                None       => 0.0,
              };
              let out  = format! ( "1 XBT = {} {}", rate, sym );
              cnt.message( chan, out.as_slice( ) );
            },
            None          => (),
          }
        },
        Err ( _ ) => {
          match msg.param( 1 ) {
            Some ( chan ) => {
              let out = format! ( "Couldn't get BPI for {}", sym );
              cnt.message( chan, out.as_slice( ) );
            },
            None          => (),
          }
        },
      }
    } else {
      match bpires {
        Ok ( res ) => {
          match msg.nick( ) {
            Some ( nick ) => {
              let rate = match res.bpi.get( &(sym.to_string( )) ) {
                Some ( r ) => r.rateFloat,
                None       => 0.0,
              };
              let time = res.time.updated.clone( );
              let outr = format! ( "1 XBT = {} {}", rate, sym );
              let outt = format! ( "Last updated {}", time );
              cnt.message( nick, outr.as_slice( ) );
              cnt.message( nick, outt.as_slice( ) );
            },
            None          => (),
          }
        },
        Err ( _ ) => {
          match msg.nick( ) {
            Some ( nick ) => {
              let out = format! ( "Couldn't get BPI for {}", sym );
              cnt.message( nick, out.as_slice( ) );
            },
            None          => (),
          }
        },
      }
    }
  }
}