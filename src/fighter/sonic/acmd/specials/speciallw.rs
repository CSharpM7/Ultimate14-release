use crate::imports::imports_acmd::*;

unsafe extern "C" fn expression_speciallwstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
}
unsafe extern "C" fn game_speciallwdash1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT as u64, 0, Hash40::new("hip"), 1.0, 9, 15, 0, 70, 3.8, 0.0, 1.5, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(agent.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT, true);
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
    }
}

unsafe extern "C" fn effect_specialairlwstart(agent: &mut L2CAgentBase) {
    //frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spinblur"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_specialairlwstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_STATUS(agent, Hash40::new("se_sonic_rounddash"));
        macros::PLAY_SE(agent, Hash40::new("se_sonic_smash_l01"));
        //macros::PLAY_SE(agent, Hash40::new("se_sonic_smash_h01"));
    }
}

unsafe extern "C" fn effect_specialairlwhold(agent: &mut L2CAgentBase) {
    /*
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spincharge_hold"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }*/
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            //macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spinblur"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        if macros::is_excute(agent) {
            //macros::EFFECT_FLW_POS(agent, Hash40::new("sonic_spinwind_continuoushit"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        wait(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            //macros::EFFECT_FLW_POS(agent, Hash40::new("sonic_spinwind_continuoushit"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        wait(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            //macros::EFFECT_FLW_POS(agent, Hash40::new("sonic_spinwind_continuoushit"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        wait(agent.lua_state_agent, 10.0);
    } 
}

unsafe extern "C" fn sound_specialairlwhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_sonic_smash_h01"));
    }
}
pub fn install(agent: &mut smashline::Agent) {
    //agent.acmd("effect_specialairlwstart", empty_acmd,Priority::Low);
    agent.acmd("expression_speciallwstart", expression_speciallwstart,Priority::Low);
    agent.game_acmd(0x1b07509826, game_speciallwdash1,Priority::Low);

    agent.acmd("effect_specialairlwstart", effect_specialairlwstart,Priority::Low);
    agent.acmd("sound_specialairlwstart", sound_specialairlwstart,Priority::Low);
    agent.acmd("effect_specialairlwhold", effect_specialairlwhold,Priority::Low);
    agent.acmd("sound_specialairlwhold", sound_specialairlwhold,Priority::Low);
}