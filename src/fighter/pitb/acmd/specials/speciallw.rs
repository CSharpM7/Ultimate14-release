use crate::imports::imports_acmd::*;
unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, pitb::GENERATE_ARTICLE_ORBITER, false,-1);
    }
}
unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let mut no_shield = false;
    if macros::is_excute(agent) {
        no_shield = WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_LW_FLAG_NO_SHIELD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        if !no_shield {
            PLAY_SE(agent, Hash40::new("vc_pitb_special_l02"));
        }
        else {
            PLAY_VC(agent, Hash40::new("vc_pitb_special_l01"),0.3);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
    }
}
unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let mut no_shield = false;
    if macros::is_excute(agent) {
        no_shield = WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_LW_FLAG_NO_SHIELD);
    }
    let eff_x = 11.5;
    let eff_y = 13.0;
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        if !no_shield {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, pitb::ORBITER_SPAWN_Y, pitb::ORBITER_SPAWN_X, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("bitr"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        if no_shield {
            macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, pitb::ORBITER_SPAWN_Y, pitb::ORBITER_SPAWN_X+2.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("bitr"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_guardian_shield_end"), Hash40::new("bitl"), 0, 0, 0, 0, 0, 0, 1.0, true);

        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("bitr"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}
unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let mut no_shield = false;
    if macros::is_excute(agent) {
        no_shield = WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_LW_FLAG_NO_SHIELD);
    }
    if macros::is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("weapon") as i64, 0x11242751f5);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        //VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_none") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_none") as i64);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }    
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        let rumble_type = if no_shield {Hash40::new("rbkind_nohits")} else {Hash40::new("rbkind_lightthrowitem")};
        ControlModule::set_rumble(agent.module_accessor, rumble_type, 1, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_none") as i64);
        VisibilityModule::set_default_int64(agent.module_accessor, hash40("weapon") as i64);
    }
}

unsafe extern "C" fn game_speciallwblank(agent: &mut L2CAgentBase) {
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_speciallw", game_speciallw,Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw,Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw,Priority::Low);
    agent.acmd("sound_specialairlw", sound_speciallw,Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw,Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw,Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw,Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw,Priority::Low);

    agent.acmd("game_speciallwblank", empty_acmd,Priority::Low);
    agent.acmd("game_specialairlwblank", empty_acmd,Priority::Low);
    agent.acmd("sound_speciallwblank", sound_speciallw,Priority::Low);
    agent.acmd("sound_specialairlwblank", sound_speciallw,Priority::Low);
    agent.acmd("effect_speciallwblank", effect_speciallw,Priority::Low);
    agent.acmd("effect_specialairlwblank", effect_speciallw,Priority::Low);
    agent.acmd("expression_speciallwblank", expression_speciallw,Priority::Low);
    agent.acmd("expression_specialairlwblank", expression_speciallw,Priority::Low);
}