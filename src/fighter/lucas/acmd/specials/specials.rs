use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucas_004"));
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_s03"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucas_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_s03"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("effect_specials", effect_specials,Priority::Low);
    agent.acmd("sound_specials", sound_specials,Priority::Low);
    agent.acmd("sound_specialairs", sound_specials,Priority::Low);
}