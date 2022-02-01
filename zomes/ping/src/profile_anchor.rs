use hdk::prelude::*;

pub const PROFILE_ANCHOR: &str = "PROFILE";

#[hdk_extern]
pub fn create_profile_anchor(short_unique_code: String) -> ExternResult<EntryHash> {
    let anchor = anchor(PROFILE_ANCHOR.into(), short_unique_code)?;
    Ok(anchor)
}

#[hdk_extern]
pub fn get_profile_anchor(anchor_code: String) -> ExternResult<EntryHash> {
    let path: Path = (&Anchor {
        anchor_type: PROFILE_ANCHOR.into(),
        anchor_text: Some(anchor_code),
    }).into();
    path.path_entry_hash()
}
