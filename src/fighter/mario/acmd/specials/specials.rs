use crate::imports::imports_acmd::*;

pub const HAVE: bool = false;
pub const FORCE_FLY: bool = true;
pub const SHOOT: bool = false;

unsafe extern "C" fn game_specials(agent: &mut smashline::L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, mario::GENERATE_ARTICLE_CAPTOSS, false, -1);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 7.0+2.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(agent.module_accessor, 9.0+2.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(agent.lua_state_agent, 9.0+1.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        let damage_mul = 1.5; //WorkModule::get_param_float(agent.module_accessor, hash40("param_reflector"), hash40("attack_mul"));
        let speed_mul = 1.0; //WorkModule::get_param_float(agent.module_accessor, hash40("param_reflector"), hash40("speed_mul"));
        let life_mul = 1.0; //WorkModule::get_param_float(agent.module_accessor, hash40("param_reflector"), hash40("life_mul"));
        let reflect_max = 999;
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("throw"), 6.5, 0.0, 3.0, -1.0, 0.0, 3.0, -1.0, 
        damage_mul, speed_mul, 50, false, life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);

    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, mario::GENERATE_ARTICLE_CAPTOSS) 
        && FORCE_FLY {
            let cappy = get_article_boma(agent.module_accessor, mario::GENERATE_ARTICLE_CAPTOSS);
            StatusModule::change_status_force(cappy, mario_cappy::CAPTOSS_STATUS_KIND_FLY, false);
        }
        if SHOOT {
            ArticleModule::shoot_exist(agent.module_accessor, mario::GENERATE_ARTICLE_CAPTOSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        //shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}


unsafe extern "C" fn effect_specials(agent: &mut smashline::L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("mario_supermant_flash"), Hash40::new("top"), 0, 8.0, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_capjump(agent: &mut smashline::L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_item_boomerang_throw"));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_jump03"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SEQUENCE(agent, Hash40::new("seq_mario_rnd_jump"));
        PLAY_VC(agent, Hash40::new("vc_mario_appeal02"), 0.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
    }
}

unsafe extern "C" fn effect_capjump(agent: &mut smashline::L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_capjump(agent: &mut smashline::L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_bounce"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_capdive(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.75);
    FT_MOTION_RATE_RANGE(agent,1.0,12.0,8.0);
    frame(agent.lua_state_agent, 12.0);
    FT_MOTION_RATE_RANGE(agent,12.0,36.0,20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 45, 35, 0, 80, 3.5, 1.9, -1.6, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 45, 35, 0, 80, 3.0, 0.0, 5.0, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::set_power(agent.module_accessor, 0, 6.0,false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    FT_MOTION_RATE(agent,1.0);
}
unsafe extern "C" fn game_capdiveair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.125);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 45, 35, 0, 80, 3.5, 1.9, -1.6, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 45, 35, 0, 80, 3.0, 0.0, 5.0, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::set_power(agent.module_accessor, 0, 6.0,false);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
        WorkModule::set_float(agent.module_accessor, MotionModule::frame(agent.module_accessor),*FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
}
unsafe extern "C" fn effect_capdive(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_s_dash").hash {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
unsafe extern "C" fn sound_capdive(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_mario_throw_b01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        PLAY_VC(agent,Hash40::new("vc_mario_passive"),0.5);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_s_dash").hash {
            macros::PLAY_SE(agent, Hash40::new("se_mario_rise"));
        }
    }
}
unsafe extern "C" fn expression_capdive(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_s_dash").hash {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn sound_capcatch(agent: &mut smashline::L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_boomerang_catch"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_appeal_s06"));
    }
}

unsafe extern "C" fn effect_capcatch(agent: &mut smashline::L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_capcatch(agent: &mut smashline::L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specials", game_specials,Priority::Low);
    agent.acmd("game_specialairs", game_specials,Priority::Low);
    agent.acmd("effect_specials", effect_specials,Priority::Low);
    agent.acmd("effect_specialairs", effect_specials,Priority::Low);
    
    agent.acmd("sound_specialairsjump", sound_capjump,Priority::Low);
    agent.acmd("effect_specialairsjump", effect_capjump,Priority::Low);
    agent.acmd("expression_specialairsjump", expression_capjump,Priority::Low);
    
    agent.acmd("game_specialsdash", game_capdive,Priority::Low);
    agent.acmd("game_specialairsdash", game_capdiveair,Priority::Low);
    agent.acmd("sound_specialsdash", sound_capdive,Priority::Low);
    agent.acmd("sound_specialairsdash", sound_capdive,Priority::Low);
    agent.acmd("effect_specialsdash", effect_capdive,Priority::Low);
    agent.acmd("effect_specialairsdash", effect_capdive,Priority::Low);
    agent.acmd("expression_specialsdash", expression_capdive,Priority::Low);
    agent.acmd("expression_specialairsdash", expression_capdive,Priority::Low);

    agent.acmd("sound_specialsend", sound_capcatch,Priority::Low);
    agent.acmd("sound_specialairsend", sound_capcatch,Priority::Low);
    agent.acmd("effect_specialsend", effect_capcatch,Priority::Low);
    agent.acmd("effect_specialairsend", effect_capcatch,Priority::Low);
    agent.acmd("expression_specialsend", expression_capcatch,Priority::Low);
    agent.acmd("expression_specialairsend", expression_capcatch,Priority::Low);
}
