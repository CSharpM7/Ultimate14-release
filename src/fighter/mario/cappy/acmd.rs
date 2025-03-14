use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 40, 0, 80, 2.6, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 40, 0, 60, 2.6, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

pub unsafe extern "C" fn game_turn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if StatusModule::prev_status_kind(agent.module_accessor, 0) != mario_cappy::CAPTOSS_STATUS_KIND_JUMP {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 70, 40, 0, 50, 3.0, 0.0, 1.0, -1.75, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

pub unsafe extern "C" fn game_hold(agent: &mut L2CAgentBase) {
    /*
    for _ in 1..i32::MAX {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 10, 2.0, 0.0, 1.0, -1.25, None, None, None, 0.5, 0.875, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    */
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 78, 40, 0, 40, 2.0, 0.0, 1.0, -1.25, None, None, None, 0.5, 0.875, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 10, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

pub unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    for _ in 1..i32::MAX {
        if is_excute(agent) {
            if StatusModule::prev_status_kind(agent.module_accessor, 0) != mario_cappy::CAPTOSS_STATUS_KIND_JUMP {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1.25, 0, 0, -90, 0, 0.4, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_COLOR(agent,1.0,1.0,0.625);
            }

            macros::EFFECT_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1.25, 0, 0, -90, 0, 0.4, 0, 1, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(agent,1.0,1.0,0.625);
            let trace_alpha = if StatusModule::prev_status_kind(agent.module_accessor, 0) != mario_cappy::CAPTOSS_STATUS_KIND_JUMP {0.5} else {0.125};
            LAST_EFFECT_SET_ALPHA(agent,trace_alpha);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

pub unsafe extern "C" fn sound_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if macros::is_excute(agent) {
        if !SoundModule::is_playing(agent.module_accessor, Hash40::new("se_item_boomerang_throw")){
            macros::PLAY_SE_REMAIN(agent, Hash40::new("se_item_boomerang_throw"));
        }
    }
}

pub unsafe extern "C" fn effect_hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    for _ in 1..i32::MAX {
        if is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_starring_trace"),Hash40::new("top"), 0, 1.25, -1.5, 0, -90, 0, 0.175, false);
        }
        for _ in 1..4 {
            if is_excute(agent) {
                if StatusModule::status_kind(agent.module_accessor) == mario_cappy::CAPTOSS_STATUS_KIND_HOP {
                    break;
                }
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1.25, 0, 0, -90, 0, 0.4, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_COLOR(agent,1.0,1.0,0.625);
            }
            wait(agent.lua_state_agent, 5.0);
        }
    }
}

pub unsafe extern "C" fn effect_jump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_piyo"), Hash40::new("rot"), 0, 2.75, 3.5, 0, 0, -10, 0.75, false);
        macros::EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("rot"), 0, 2.5, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe extern "C" fn sound_jump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_item_boomerang_throw"));
        macros::PLAY_SE(agent, Hash40::new("se_mario_jump03"));
    }
}

pub fn install(agent: &mut smashline::Agent) {  
    agent.acmd("game_fly", game_fly,Priority::Low);
    agent.acmd("game_hold", game_hold,Priority::Low);
    agent.acmd("game_turn", game_turn,Priority::Low);

    agent.acmd("effect_fly", effect_fly,Priority::Low);
    agent.acmd("effect_hold", effect_hold,Priority::Low);
    agent.acmd("effect_turn", effect_fly,Priority::Low);
    agent.acmd("effect_jump", effect_jump,Priority::Low);

    agent.acmd("sound_fly", sound_fly,Priority::Low);
    agent.acmd("sound_turn", sound_fly,Priority::Low);
    agent.acmd("sound_jump", sound_jump,Priority::Low);
}
