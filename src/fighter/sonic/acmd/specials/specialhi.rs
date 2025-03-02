use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhisquat(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, false,ArticleOperationTarget(0));
    }
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,5.0,8.0);
    frame(agent.lua_state_agent,5.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        ArticleModule::set_frame(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, 0.0);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, 1.0);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, true,ArticleOperationTarget(0));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, -1, *FIGHTER_SONIC_STATUS_SPECIAL_HI_WORK_INT_ADVANCE_COUNTER);
    }
}
unsafe extern "C" fn effect_specialhisquat(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 4.300);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn game_fall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 48, 90, 0, 30, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhisquat", game_specialhisquat,Priority::Low);
    agent.acmd("effect_specialhisquat", effect_specialhisquat,Priority::Low);
    
    smashline::Agent::new("sonic_gimmickjump")
    .acmd("game_fall", game_fall,Priority::Low)
    .install();
}