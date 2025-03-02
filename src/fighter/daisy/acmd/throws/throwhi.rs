use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 17, 0, 90, 0, 0, 1, 0, 1, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 27, 0, 0, 0, 0, 1.1, 0, 1, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("effect_throwhi", effect_throwhi,Priority::Low);
}