use holo_hash::{EntryHashB64};
use hdk::prelude::*;


pub fn _try_get_and_convert<T: TryFrom<Entry>>(
    entry_hash: EntryHash,
) -> ExternResult<T> {
    match get(entry_hash.clone(), GetOptions::default())? {
        Some(element) => _try_from_element(element),
        None => Err(WasmError::Guest(String::from("Something went wrong."))),
    }
}

pub fn _try_from_element<T: TryFrom<Entry>>(element: Element) -> ExternResult<T> {
    match element.entry() {
        element::ElementEntry::Present(entry) => T::try_from(entry.clone()).or(Err(WasmError::Guest(String::from("Cannot conver entry")))),
        _ => Err(WasmError::Guest(String::from("Could not convert element"))),
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hashed<T> {
    pub hash: EntryHashB64,
    pub content: T,
}