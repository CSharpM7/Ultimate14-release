use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 98, 0, 30, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_greenshell_trace"), Hash40::new("arrow"), 0, 0, 1.5, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_ALPHA(agent,0.5);
    }
    loop {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("all"), 0, 0, 0, 0, 0, 0, 0.5, true);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

unsafe extern "C" fn sound_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::SET_TAKEOUT_SE(agent, Hash40::new("se_link_special_n03"));
        macros::PLAY_SE(agent, Hash40::new("se_link_special_n03"));
    }
}

unsafe extern "C" fn effect_stick(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) != hash40("hit_stick") {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("all"), -0.0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_greenshell_trace"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_smash_flash_s"), true, true);
    }
}
unsafe extern "C" fn sound_stick(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_unira_stick")); //se_item_unira_stick //se_lucina_special_s04s
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_fly", game_fly,Priority::Low);
    agent.acmd("effect_fly", effect_fly,Priority::Low);
    agent.acmd("sound_fly", sound_fly,Priority::Low);

    agent.acmd("effect_hitstick", effect_stick,Priority::Low);
    agent.acmd("effect_stick", effect_stick,Priority::Low);
    agent.acmd("sound_hitstick", sound_stick,Priority::Low);
    agent.acmd("sound_stick", sound_stick,Priority::Low);
}