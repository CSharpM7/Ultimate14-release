use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("toonlink_arrow_max"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent,0.8,1.0,0);
    }
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent,0.7,1.0,0.25);
            if agent.is_grounded() {
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            if agent.is_grounded() {
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 4.0);
    }
}
unsafe extern "C" fn effect_specialnend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {

        let is_max = WorkModule::is_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX);
        let eff = if is_max {Hash40::new("sys_staff_shot")} else {Hash40::new("sys_smash_flash")};
        
        macros::EFFECT(agent, eff, Hash40::new("haver"), 0, -0.3, 3.3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        if agent.is_grounded() {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_toonlinkbowchargemax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_killsword_flash"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("effect_specialn", effect_specialn,Priority::Low);
    agent.acmd("effect_specialairn", effect_specialn,Priority::Low);

    agent.acmd("effect_specialnend", effect_specialnend,Priority::Low);
    agent.acmd("effect_specialairnend", effect_specialnend,Priority::Low);

    agent.acmd("sound_toonlinkbowchargemax", sound_toonlinkbowchargemax,Priority::Low);
    //agent.acmd("game_specialairs", game_specials,Priority::Low);
}
