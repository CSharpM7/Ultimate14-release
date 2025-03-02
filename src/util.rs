use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        lua2cpp::*,
        phx::*,
    },
    smash_rs::*,
    smash_script::*,
    smashline::*,
    singletons::FighterManager,
};

pub unsafe fn get_table_value(table: *mut smash_rs::lib::L2CTable, key: &str) -> smash_rs::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash_rs::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash_rs::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}