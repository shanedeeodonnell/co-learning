use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;
use hc_zome_profiles_types::{AgentProfile};

mod utils;

entry_defs![
    Anchor::entry_def(),
    Path::entry_def()
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
pub fn ui_send_ping(name:String) -> ExternResult<()> {
    debug!("ui_send_ping called");
    let players_response = call(
        CallTargetCell::Local,
        "profiles".into(), 
        "get_all_profiles".into(), 
        None, 
        ())?;
    
   let extern_players =  match players_response {
        ZomeCallResponse::Ok(content) => Ok(content),
        _ => Err(WasmError::Guest("no players, or something...".into())),
    }?;

    let players:Vec<AgentProfile> = extern_players.decode()?;

    // Send a signal to all other players online
    let this_agent_pub_key = agent_info()?.agent_initial_pubkey.into();
    let others: Vec<AgentProfile> = players.into_iter().filter(|x| x.agent_pub_key != this_agent_pub_key).collect();

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

#[hdk_extern]
pub fn ui_send_direct_ping(agent_pub_key:AgentPubKeyB64) -> ExternResult<()> {
    debug!("ui_send_direct_ping called");    

    let players_response = call(
        CallTargetCell::Local,
        "profiles".into(), 
        "get_agent_profile".into(), 
        None, 
        agent_pub_key.clone())?;
    
   let extern_player =  match players_response {
        ZomeCallResponse::Ok(content) => Ok(content),
        _ => Err(WasmError::Guest("no player, or something...".into())),
    }?;

    let player : AgentProfile = extern_player.decode()?;

    call_remote(
        agent_pub_key.clone().into(),
        "ping".into(),
        "receive_ping".into(),
        None,
        player.profile.nickname,
    )?;
    debug!("Called remote receive_ping from {:?}", agent_pub_key);
    Ok(())
}

// MAKE UI
// - User name input and login
// - Button to send signal
// - Alert on emit signal
#[hdk_extern]
pub fn receive_ping(payload: String)  -> ExternResult<()> {
    emit_signal(payload.clone())?;
    debug!("External Call: receive_ping from {:?}", payload);
    Ok(())
}