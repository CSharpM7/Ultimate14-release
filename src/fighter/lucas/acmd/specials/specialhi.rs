use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkt_hold"), Hash40::new("rot"), 0, 1, 0, 0, 0, 0, 1, true);
        //EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT(agent, Hash40::new("lucas_pkt_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..16 {
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 0.5, 0.2, 1, 0.9);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 1, 0.5, 0.2, 1, 0);
            macros::BURN_COLOR_NORMAL(agent);
            macros::FLASH(agent, 0, 0, 0.1, 0.8);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 1, 0, 0, 0.1, 0);
            macros::COL_NORMAL(agent);
        }
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkt_attack"), false, false);
        //EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.9, true);
    }
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 0.7, 0.2, 1, 0.6);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 1, 0.7, 0.2, 1, 0);
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.8, 0.7, 1, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 1, 0.8, 0.7, 1, 0);
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //agent.acmd("effect_specialairhi", effect_specialairhi,Priority::Low);
}