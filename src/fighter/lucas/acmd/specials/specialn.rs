use crate::imports::imports_acmd::*;

unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucas_003"));
        //macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_kirby_copy_lucas"));
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let is_kirby = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
        let vc = if is_kirby {Hash40::new("vc_kirby_002")} else {Hash40::new("vc_lucas_appeal03")}; //kirby_final01
        macros::PLAY_SE_REMAIN(agent, vc);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_n05"));
    }
}

unsafe extern "C" fn effect_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.5, 1.0, 0.3);
    }
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
        }
        macros::FLASH(agent, 0.01, 0.5, 1, 0.4);
    }
    for i in 0..i32::MAX {
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}
unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 0.0, 0.0, 12.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
}
unsafe extern "C" fn effect_specialnfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.5, 1.0, 0.3);
        //macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.01, 0.5, 1, 0.4);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 3.0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_hold"), false, false);
    }
}
pub fn install(agent: &mut smashline::Agent) {
    //agent.acmd("effect_specials", effect_specials);
    agent.acmd("sound_specialnstart", sound_specialnstart,Priority::Low);
    agent.acmd("sound_specialairnstart", sound_specialnstart,Priority::Low);
    agent.acmd("effect_specialnhold", effect_specialnhold,Priority::Low);
    agent.acmd("effect_specialairnhold", effect_specialnhold,Priority::Low);
    agent.acmd("game_specialnfire", game_specialnfire,Priority::Low);
    agent.acmd("game_specialairnfire", game_specialnfire,Priority::Low);
    agent.acmd("effect_specialnfire", effect_specialnfire,Priority::Low);
    agent.acmd("effect_specialairnfire", effect_specialnfire,Priority::Low);

    Agent::new("kirby")
        .acmd("sound_lucasspecialnstart", sound_specialnstart,Priority::Low)
        .acmd("sound_lucasspecialairnstart", sound_specialnstart,Priority::Low)
        .acmd("effect_lucasspecialnhold", effect_specialnhold,Priority::Low)
        .acmd("effect_lucasspecialairnhold", effect_specialnhold,Priority::Low)
        .acmd("game_lucasspecialnfire", game_specialnfire,Priority::Low)
        .acmd("game_lucasspecialairnfire", game_specialnfire,Priority::Low)
        .acmd("effect_lucasspecialnfire", effect_specialnfire,Priority::Low)
        .acmd("effect_lucasspecialairnfire", effect_specialnfire,Priority::Low)
    .install();
}