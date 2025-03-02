use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK);
    }
}
unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_dash_start"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_link_dash_start"), 20);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_link_attack02"));
    }
}
unsafe extern "C" fn expression_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shield"), 9.0, 70, 60, 0, 55, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        //ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 70, 60, 0, 55, 4.5, 0.0, 6.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
       
       damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6);
       //HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
       //HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
    }
}
unsafe extern "C" fn effect_specials1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_speedbooster"), Hash40::new("top"), 0, 10, 5, 0, 0, 0, 0.85, true);
    }
    for i in 0..i32::MAX {
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_specials1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_link_special_s01"));
    }
    for i in 0..i32::MAX {
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_link_step_left_l"));
        }
        wait(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_link_step_right_l"));
        }
        wait(agent.lua_state_agent, 17.0);
    }
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}
unsafe extern "C" fn sound_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_dash_stop"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_link_dash_stop"), 15);
    }
}
unsafe extern "C" fn expression_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


unsafe extern "C" fn game_specials2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 13.0, 361, 82, 0, 55, 4.0, 7.2, 0.0, 0.0, Some(-1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn effect_specials2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_sword_flare"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 14, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("link_sword_flare"), false, false);
    }
}
unsafe extern "C" fn sound_specials2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_dash_stop"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_link_dash_stop"), 15);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_m"));
        macros::PLAY_SE(agent, Hash40::new("vc_link_attack04"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_landing03_soubi_03"));
    }
}
unsafe extern "C" fn expression_specials2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialairs1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shieldb"), 9.0, 70, 55, 0, 65, 4.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 70, 60, 0, 55, 4.5, 0.0, 6.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(agent.module_accessor, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        //AttackModule::set_optional_hit_sound(agent.module_accessor, 0, Hash40::new("se_link_shieldguard"));
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::set_power(agent.module_accessor, 0, 7.0, false);
    }
    frame(agent.lua_state_agent, 40.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor,1.0);
        AttackModule::clear_all(agent.module_accessor);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FIRST) {
            CancelModule::enable_cancel(agent.module_accessor);
        }
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor,*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
unsafe extern "C" fn effect_specialairs1(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn sound_specialairs1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
    }
}
unsafe extern "C" fn expression_specialairs1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
}

unsafe extern "C" fn effect_specialairs2(agent: &mut L2CAgentBase) {
    /*
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    } */
    for _ in 0..2 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}
unsafe extern "C" fn sound_specialairs2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_link_shieldguard"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 0.8, 0);
        macros::PLAY_SE(agent, Hash40::new("se_link_dash_turn"));
    }
}
unsafe extern "C" fn expression_specialairs2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialsstart", game_specialsstart,Priority::Low);
    agent.acmd("effect_specialsstart", effect_specialsstart,Priority::Low);
    agent.acmd("sound_specialsstart", sound_specialsstart,Priority::Low);
    agent.acmd("expression_specialsstart", expression_specialsstart,Priority::Low);

    agent.acmd("game_specials1", game_specials1,Priority::Low);
    agent.acmd("effect_specials1", effect_specials1,Priority::Low);
    agent.acmd("sound_specials1", sound_specials1,Priority::Low);

    agent.acmd("game_specials2", game_specials2,Priority::Low);
    agent.acmd("effect_specials2", effect_specials2,Priority::Low);
    agent.acmd("sound_specials2", sound_specials2,Priority::Low);
    agent.acmd("expression_specials2", expression_specials2,Priority::Low);

    agent.acmd("effect_specialsend", effect_specialsend,Priority::Low);
    agent.acmd("sound_specialsend", sound_specialsend,Priority::Low);
    agent.acmd("expression_specialsend", expression_specialsend,Priority::Low);

    agent.acmd("game_specialairs1", game_specialairs1,Priority::Low);
    agent.acmd("effect_specialairs1", effect_specialairs1,Priority::Low);
    agent.acmd("sound_specialairs1", sound_specialairs1,Priority::Low);
    agent.acmd("expression_specialairs1", expression_specialairs1,Priority::Low);

    //agent.acmd("game_specialairs2", game_specialairs2,Priority::Low);
    agent.acmd("effect_specialairs2", effect_specialairs2,Priority::Low);
    agent.acmd("sound_specialairs2", sound_specialairs2,Priority::Low);
    agent.acmd("expression_specialairs2", expression_specialairs2,Priority::Low);
}