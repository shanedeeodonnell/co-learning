use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;

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

#[hdk_extern]
pub fn ui_send_direct_ping(agent_pub_key:AgentPubKeyB64) -> ExternResult<()> {
  debug!("ui_send_direct_ping called");    

        call_remote(
            agent_pub_key.clone().into(),
            "ping".into(),
            "receive_ping".into(),
            None,
            agent_pub_key.clone()
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