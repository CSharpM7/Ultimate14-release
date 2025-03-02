use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn game_toad(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn effect_toad(agent: &mut L2CAgentBase) {
}


pub unsafe extern "C" fn game_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 100, 30, 0, 5.5, 0.0, 4.5, 6.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -6, 1.25, 7, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true, *EF_FLIP_YZ, 1);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_catchwait", game_toad,Priority::Low);
    agent.acmd("game_catchpull", game_toad,Priority::Low);
    agent.acmd("effect_toad", effect_toad,Priority::Low);
    agent.acmd("game_catchattack", game_catchattack,Priority::Low);
    agent.acmd("effect_catchattack", effect_catchattack,Priority::Low);
}