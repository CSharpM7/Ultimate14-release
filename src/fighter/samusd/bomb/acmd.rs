use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 45, 60, 22, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);

        AREA_WIND_2ND_RAD(agent, 0, 0.5, 0.02, 1000, 1, 0, 0, 16);
    }
}

unsafe extern "C" fn sound_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samusd_special_l01"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //PLAY_SE_REMAIN(agent,Hash40::new("se_common_spirits_floor_elec_loop"));
    }
}
unsafe extern "C" fn effect_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for i in 0..1000 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_pasaran_spark"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_COLOR(agent,0,0.5,1);
        }
        wait(agent.lua_state_agent, 12.0);
    }
}


unsafe extern "C" fn game_burstattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 85, 60, 0, 60, 10.38, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AREA_WIND_2ND_RAD(agent, 0, 0.5, 0.02, 1000, 1, 0, 0, 16);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 4.9);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 2.5);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_burstattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT_BRANCH_SITUATION(agent, Hash40::new("samusd_atk_bomb"), Hash40::new("samusd_atk_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("samusd_atk_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3.5, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_burstattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_common_bomb_s"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_fall", game_fall,Priority::Low);
    agent.acmd("sound_fall", sound_fall,Priority::Low);
    agent.acmd("effect_fall", effect_fall,Priority::Low);
    agent.acmd("game_burstattack", game_burstattack,Priority::Low);
    agent.acmd("effect_burstattack", effect_burstattack,Priority::Low);
}