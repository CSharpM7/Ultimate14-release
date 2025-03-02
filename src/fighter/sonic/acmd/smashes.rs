use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 13.0, 361, 101, 0, 30, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 13.0, 361, 101, 0, 30, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 13.0, 361, 101, 0, 30, 2.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        let motion = MotionModule::motion_kind(agent.module_accessor);
        if motion != hash40("attack_s4") {
            let height = if motion == hash40("attack_s4_hi") {*ATTACK_HEIGHT_HIGH} else {*ATTACK_HEIGHT_LOW};
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(height), false);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.2, 4.0);
    }
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 125, 100, 125, 0, 5.5, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(-5.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 82, 72, 0, 80, 7.5, 4.0, 22.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    FT_MOTION_RATE_RANGE(agent,21.0,27.0,1.0);
    frame(agent.lua_state_agent, 27.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sphere"), 0, 0, 0, 0, -160, 270, 0.75, true, 1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.0, 23.5, 0.0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 360, false);
        LAST_EFFECT_SET_RATE(agent,0.75);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spinwind"), true, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spinblur_plain"), true, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spintrace"), false, true);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new_raw(0x13589f8893));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_sonic_attack06"));
        macros::PLAY_SE(agent, Hash40::new("se_sonic_attackair_n01"));
    }
    wait(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_sonic_smash_h01"));
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_landing02"));
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 30, 80, 0, 50, 4.75, 0.0, 4.75, 11.5, Some(0.0),Some(5.0),Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 30, 80, 0, 50, 4.75, 0.0, 4.75, -8.5,Some(0.0),Some(5.0),Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -2, 0, 1.5, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), -2.2, 8.5, 0, -51.924, -294.814, 114.417, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent,1.4);
        //macros::LAST_EFFECT_SET_ALPHA(agent, 0.55);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), -2.2, 5, 0, -30.709, -288.509, 168.141, 1.05, 0, 0, 0, 0, 0, 0, true);
        //macros::LAST_EFFECT_SET_ALPHA(agent, 0.55);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 3, 0, 1.5, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -1.5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_sonic_attack01"));
        macros::PLAY_SE(agent, Hash40::new("se_sonic_smash_s02"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_s"));
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_swing_m"));
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_landing02"));
    }
}

unsafe extern "C" fn expression_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn effect_attacklw4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for i in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
        }
    }
}
pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attacks4lw", game_attacks4,Priority::Low);
    agent.acmd("game_attacks4", game_attacks4,Priority::Low);
    agent.acmd("game_attacks4hi", game_attacks4,Priority::Low);
    
    agent.acmd("game_attackhi4", game_attackhi4,Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4,Priority::Low);
    agent.acmd("sound_attackhi4", sound_attackhi4,Priority::Low);
    //agent.acmd("expression_attackhi4", expression_attackhi4,Priority::Low);
    
    agent.acmd("game_attacklw4", game_attacklw4,Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4,Priority::Low);
    agent.acmd("sound_attacklw4", sound_attacklw4,Priority::Low);
    agent.acmd("expression_attacklw4", expression_attacklw4,Priority::Low);

    agent.acmd("effect_attacklw4charge", effect_attacklw4charge,Priority::Low);
}