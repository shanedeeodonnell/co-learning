use hdk::prelude::*;

mod player_profile;
mod game_code;
mod utils;

use crate::{
    player_profile::{PlayerProfile}
};

pub const ACCESS_CODE: &str = "PINGS";

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct Hashed<T> {
//     pub hash: EntryHashB64,
//     pub content: T,
// };

entry_defs![
    // Anchor::entry_def(),
    // Path::entry_def(),
    player_profile::PlayerProfile::entry_def()
    // game_session::GameSession::entry_def(),
];

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions = GrantedFunctions::new();
    functions.insert((zome_info()?.name, "receive_ping".into()));
    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions,
        tag: "".into(),
    };
    create_cap_grant(grant)?;
    Ok(InitCallbackResult::Pass)
}

// Keeping track of which externs are called from where needs some indentifier
// - ui for calls from user input
// - no prefix for external zome call
#[hdk_extern]
pub fn ui_send_ping(_: ()) -> ExternResult<()> {

    let players: Vec<PlayerProfile> = player_profile::get_player_profiles_for_game_code(ACCESS_CODE.into())?;

    let other: PlayerProfile;
    if players[0].player_id == agent_info()?.agent_initial_pubkey {
        other = players[1].to_owned();
    } else {
        other = players[0].to_owned();
    }

    let payload: Ping = Ping { colour: PingColour::Blue };
    
    call_remote(
        other.player_id.into(),
        zome_info()?.name,
        "receive_ping".into(),
        None,
        payload
    )?;

    debug!("Called remote receive_ping with colour {:?}", payload.colour);

    Ok(())
}

// MAKE UI
// - User name input and login
// - Button to send signal
// - Alert on emit signal
#[hdk_extern]
pub fn receive_ping(payload: Ping)  -> ExternResult<()> {
    emit_signal(payload.clone())?;
    debug!("External Call: receive_ping with colour {:?}", payload.colour);
    Ok(())
}

// #[hdk_extern]
// pub fn get_ping(entry_hash: EntryHashB64) -> ExternResult<Ping> {
//     let element = get(EntryHash::from(entry_hash), GetOptions::default())?.ok_or(WasmError::Guest(String::from("Post not found")))?;

//     let ping: Ping = element.entry().to_app_option()?.ok_or(WasmError::Guest(String::from("Malformed ping")))?;

//     Ok(ping)
// }

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    PingReceived
}

#[derive(Clone, Serialize, Deserialize, Debug, Copy)]
pub struct Ping {
    pub colour: PingColour,
}

#[derive(Clone, Serialize, Deserialize, Debug, Copy)]
pub enum PingColour {
    Red,
    Blue,
    Green,
    White,
    Black
}


