use crate::imports::imports_acmd::*;

unsafe fn effect_aura(agent: &mut L2CAgentBase) {
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("clavicler"), 2, 0, 0.5, 0, 0, 0, 2, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 2, 0, -0.5, 0, 0, 0, 2, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
}

unsafe extern "C" fn effect_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        effect_aura(agent);
    }
    frame(agent.lua_state_agent, 1.0);
    loop {
        if macros::is_excute(agent) {
            if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.73, 0, 0, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            }
            macros::BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, -3, -90, 0, 0, 1.1, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, 3, -90, 0, 0, 1.1, true);
            //macros::FLASH(agent, 0.3, 0.5, 1.0, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn sound_specialhihold(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_samusd_special_h01"));
    }
}

unsafe extern "C" fn expression_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        //macros::RUMBLE_HIT_STATUS(agent, Hash40::new("rbkind_spinattack"), 0);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("rbkind_spinattack"), 0);
        sv_animcmd::RUMBLE_HIT_STATUS(agent.lua_state_agent);
        agent.clear_lua_stack();

        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        //AREA_WIND_2ND_arg10(agent,0, 1, 80, 2, 0.8, 0, 15, 40, 30, 60);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        //AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 16.0, 60, 60, 0, 70, 7.2, 2.5, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 10.0, 60, 50, 0, 85, 5.0, 2.5, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let back = 10.0*PostureModule::lr(agent.module_accessor);
    if macros::is_excute(agent) {
        effect_aura(agent);

        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_dash_attack"), Hash40::new("rot"), 0.0,-4.0,2.0, 0,0,0, 1.1, true);
        //LAST_EFFECT_SET_ALPHA(agent,0.75);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //let degree = WorkModule::get_float(agent.module_accessor, FLOAT_RUSH_DEGREE);
        macros::EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("rot"), 0.0, -4.0, -6.0, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    loop {
        for i in 0..2{
            if macros::is_excute(agent) {
                macros::BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
            }
            wait(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::BURN_COLOR_NORMAL(agent);
            }
            wait(agent.lua_state_agent, 1.0);
        }
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_dash_attack"), Hash40::new("rot"), 0.0,-4.0,2.0, 0,0,0, 1.1, true);
        }
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samusd_special_n03"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_fly"), 20, true, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_specialhifall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn effect_specialhifall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        effect_aura(agent);
        macros::BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
        //macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        //EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        //EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
    }
}

unsafe extern "C" fn game_specialhilandingf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
}

unsafe extern "C" fn effect_specialhilandingf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        effect_aura(agent);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialhilandingf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_samusd_special_n03"));
        macros::PLAY_SE(agent, Hash40::new("se_samusd_dash_stop"));
    }
}

unsafe extern "C" fn expression_specialhilandingf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
unsafe extern "C" fn game_specialhilandinglw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 42, 70, 0, 80, 6.5, 0.0, 3.0, 10.0, Some(0.0), Some(3.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 42, 70, 0, 80, 6.5, 0.0, 3.0, -10.0, Some(0.0), Some(3.0), Some(-11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialhilandinglw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 3, 0, 2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 3, 0, 2, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        effect_aura(agent);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
    }
}

unsafe extern "C" fn sound_specialhilandinglw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_samusd_special_n03"));
        macros::PLAY_SE(agent, Hash40::new("se_samusd_landing02"));
    }
}

unsafe extern "C" fn expression_specialhilandinglw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairhiwallf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn sound_specialairhiwallf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_samusd_special_n03"));
        macros::PLAY_SE(agent, Hash40::new("se_common_down_m_01"));
    }
}

unsafe extern "C" fn expression_specialairhiwallf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairhiceil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.2);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}
unsafe extern "C" fn sound_specialairhiceil(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_samusd_special_n03"));
        macros::PLAY_SE(agent, Hash40::new("se_common_down_m_01"));
    }
}

unsafe extern "C" fn expression_specialairhiceil(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhihold", empty_acmd,Priority::Low);
    agent.acmd("effect_specialhihold", effect_specialhihold,Priority::Low);
    agent.acmd("sound_specialhihold", sound_specialhihold,Priority::Low);
    agent.acmd("expression_specialhihold", expression_specialhihold,Priority::Low);
    agent.acmd("game_specialhiholdair", empty_acmd,Priority::Low);
    agent.acmd("effect_specialhiholdair", effect_specialhihold,Priority::Low);
    agent.acmd("sound_specialhiholdair", sound_specialhihold,Priority::Low);
    agent.acmd("expression_specialhiholdair", expression_specialhihold,Priority::Low);
    
    agent.acmd("game_specialhi", game_specialhi,Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi,Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi,Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi,Priority::Low);
    
    agent.acmd("game_specialhifall", game_specialhifall,Priority::Low);
    agent.acmd("effect_specialhifall", effect_specialhifall,Priority::Low);
    
    agent.acmd("game_specialhilandingf", game_specialhilandingf,Priority::Low);
    agent.acmd("effect_specialhilandingf", effect_specialhilandingf,Priority::Low);
    agent.acmd("sound_specialhilandingf", sound_specialhilandingf,Priority::Low);
    agent.acmd("expression_specialhilandingf", expression_specialhilandingf,Priority::Low);

    agent.acmd("game_specialhilandinglw", game_specialhilandinglw,Priority::Low);
    agent.acmd("effect_specialhilandinglw", effect_specialhilandinglw,Priority::Low);
    agent.acmd("sound_specialhilandinglw", sound_specialhilandinglw,Priority::Low);
    agent.acmd("expression_specialhilandinglw", expression_specialhilandinglw,Priority::Low);
    
    agent.acmd("game_specialairhiwallf", game_specialairhiwallf,Priority::Low);
    //agent.acmd("effect_specialairhiwallf", effect_specialairhiwallf,Priority::Low);
    agent.acmd("sound_specialairhiwallf", sound_specialairhiwallf,Priority::Low);
    agent.acmd("expression_specialairhiwallf", expression_specialairhiwallf,Priority::Low);

    //agent.acmd("game_specialairhiceil", game_specialairhiceil,Priority::Low);
    //agent.acmd("effect_specialairhiceil", effect_specialairhiceil,Priority::Low);
    agent.acmd("sound_specialairhiceil", sound_specialairhiceil,Priority::Low);
    agent.acmd("expression_specialairhiceil", expression_specialairhiceil,Priority::Low);
}