use hdk::prelude::*;
use player_profile::JoinInfo;

mod player_profile;
mod profile_anchor;
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
    Anchor::entry_def(),
    Path::entry_def(),
    player_profile::PlayerProfile::entry_def()
];

#[hdk_extern]
pub fn join_with_code(payload: String) -> ExternResult<EntryHash> {
    debug!("{:?}", payload);
    let join_info = JoinInfo{
        anchorcode: ACCESS_CODE.into(),
        nickname: payload
    };
    player_profile::join_with_code(join_info)
}

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

    let players: Vec<PlayerProfile> = player_profile::get_all_player_profiles(ACCESS_CODE.into())?;

    // Send a signal to all other players online
    let mut others: Vec<PlayerProfile> = vec![];

    for player in players {
        if player.player_id != agent_info()?.agent_initial_pubkey {
            others.push(player.clone());
        }
    }

    let payload: String = String::from("blue");
    
    for other in others {
        call_remote(
            other.player_id.into(),
            zome_info()?.name,
            "receive_ping".into(),
            None,
            payload.clone()
        )?;
    }

    debug!("Called remote receive_ping with colour {:?}", payload);

    Ok(())
}

// MAKE UI
// - User name input and login
// - Button to send signal
// - Alert on emit signal
#[hdk_extern]
pub fn receive_ping(payload: String)  -> ExternResult<()> {
    emit_signal(payload.clone())?;
    debug!("External Call: receive_ping with colour {:?}", payload);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    PingReceived
}




