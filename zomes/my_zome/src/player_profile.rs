use hdk::prelude::*;

use crate::game_code::{create_game_code_anchor, get_game_code_anchor};

pub const PLAYER_LINK_TAG: &str = "PLAYER";

#[hdk_entry(id = "player_profile", visibility = "public")]
#[derive(Clone)]
pub struct PlayerProfile {
    pub player_id: AgentPubKey,
    pub nickname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct JoinGameInfo {
    pub gamecode: String,
    pub nickname: String,
}

pub fn join_game_with_code(input: JoinGameInfo) -> ExternResult<EntryHash> {
    let anchor = create_game_code_anchor(input.gamecode)?;
    let player_profile_entry_hash = create_and_hash_entry_player_profile(input.nickname)?;
    create_link(
        anchor.clone().into(),
        player_profile_entry_hash.into(),
        LinkTag::new(String::from(PLAYER_LINK_TAG)),
    )?;
    Ok(anchor)
}

#[hdk_extern]
pub fn create_and_hash_entry_player_profile(nickname: String) -> ExternResult<EntryHash> {
    let agent = agent_info()?;

    let player_profile = PlayerProfile {

        player_id: agent.agent_initial_pubkey,
        nickname,
    };
    create_entry(&player_profile)?;

    hash_entry(&player_profile)
}

pub fn get_player_profiles_for_game_code(
    access_code: String,
) -> ExternResult<Vec<PlayerProfile>> {
    let anchor = get_game_code_anchor(access_code)?;
    let links: Vec<Link> = get_links(anchor, Some(LinkTag::new(String::from(PLAYER_LINK_TAG))))?;
    let mut players = vec![];
    for link in links {
        let element: Element = get(link.target, GetOptions::default())?
            .ok_or(WasmError::Guest(String::from("Entry not found")))?;
        let entry_option = element.entry().to_app_option()?;
        let entry: PlayerProfile = entry_option.ok_or(WasmError::Guest(
            "The targeted entry is not agent pubkey".into(),
        ))?;
        players.push(entry);
    }
    Ok(players)
}
