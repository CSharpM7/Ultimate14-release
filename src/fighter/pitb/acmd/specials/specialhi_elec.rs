use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn game_specialhi_start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    let isGround = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_histart").hash;
    let motionFrames = if isGround {9.0} else {7.0};
    FT_MOTION_RATE(agent,motionFrames/(18.0-1.0));

    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) 
    && isGround {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 4);
    }
}


pub unsafe extern "C" fn effect_specialhi_start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("pitb_gouwan_dashstart"), Hash40::new("top"), -7, 12, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_gouwan_dash"), Hash40::new("havel"), 0, 0, 3, -0.569, -0.001, 0.028, 1, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        let isGround = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_histart").hash;
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_gouwan_dash_ring"), Hash40::new("havel"), 0, 0, 2.2, 0, 0, 0, 1, true);
        if isGround{
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub unsafe extern "C" fn sound_specialhi_start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pitb_special_s01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pitb_rnd_special_h"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pitb_special_s02"));
    }
}

pub unsafe extern "C" fn expression_specialhi_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}




pub unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);

        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BACK_ANGLE);
        
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 80, 100, 85, 0, 6.5, 0.0, 7.0, 10.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 85, 0, 6.5, 0.0, 7.0, 10.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);

        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 85, 0, 6.5, 0.0, 14.0, 10.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 367, 100, 85, 0, 6.0, 0.0, 7.0, 10.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {        
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {        
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 8.0, 70, 90, 0, 65, 6.3, 0.0, 17.0, 10.0, Some(0.0), Some(12.0), Some(10.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {        
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_CONTINUE);

        let stop_rise = Vector3f{x: 1.0, y: 0.75, z: 1.0};
        KineticModule::mul_speed(agent.module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {        
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    /*
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {        
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
    } */
}

pub unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_gouwan_dash_arc"), Hash40::new("top"), -2, 15, 2, 10, -40, 93, 1.2, true);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("havel"), 0, 0, 2, 0, 0, 0, 1, true);
    }
}

pub unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pitb_rnd_special_h"));
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pitb_special_s03"));
    }
}


pub unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhistart", game_specialhi_start,Priority::Low);
    agent.acmd("sound_specialhistart", sound_specialhi_start,Priority::Low);
    agent.acmd("effect_specialhistart", effect_specialhi_start,Priority::Low);
    agent.acmd("expression_specialhistart", expression_specialhi_start,Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhi_start,Priority::Low);
    agent.acmd("sound_specialairhistart", sound_specialhi_start,Priority::Low);
    agent.acmd("effect_specialairhistart", effect_specialhi_start,Priority::Low);
    agent.acmd("expression_specialairhistart", expression_specialhi_start,Priority::Low);

    agent.acmd("game_specialhi", game_specialhi,Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi,Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi,Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi,Priority::Low);
}