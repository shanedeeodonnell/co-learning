extern crate hc_zome_profiles;
use hdk::prelude::*;
use hc_zome_profiles_types::{self, AgentProfile};

// Keeping track of which externs are called from where needs some indentifier
// - ui for calls from user input
// - no prefix for external zome call
#[hdk_extern]
pub fn ui_send_ping(name:String) -> ExternResult<()> {
  debug!("ui_send_ping called");
    let players = hc_zome_profiles::get_all_profiles(())?;

    // Send a signal to all other players online
    let mut others: Vec<AgentProfile> = vec![];

    for player in players {
        if player.agent_pub_key != agent_info()?.agent_initial_pubkey.into() {
            others.push(player.clone());
        }
    }
    
    for other in others {
        call_remote(
            other.agent_pub_key.into(),
            "ping".into(),
            "receive_ping".into(),
            None,
            &name
        )?;
    }

    debug!("Called remote receive_ping from {:?}", name);

    Ok(())
}