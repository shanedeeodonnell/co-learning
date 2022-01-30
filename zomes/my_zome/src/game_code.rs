use hdk::prelude::*;

pub const GAME_CODES_ANCHOR: &str = "GAME_CODES";

#[hdk_extern]
pub fn create_game_code_anchor(short_unique_code: String) -> ExternResult<EntryHash> {
    let anchor = anchor(GAME_CODES_ANCHOR.into(), short_unique_code)?;
    Ok(anchor)
}

#[hdk_extern]
pub fn get_game_code_anchor(game_code: String) -> ExternResult<EntryHash> {
    let path: Path = (&Anchor {
        anchor_type: GAME_CODES_ANCHOR.into(),
        anchor_text: Some(game_code),
    }).into();
    path.path_entry_hash()
}
