use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, -1);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        super::super::super::set_book(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, -1);
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_special_s"));
        macros::PLAY_SE(agent, Hash40::new("se_richter_special_s01"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //agent.acmd("effect_specials", effect_specials);
    agent.acmd("game_specialn", game_specialn,Priority::Low);
    agent.acmd("game_specialairn", game_specialn,Priority::Low);
    agent.acmd("effect_specialn", effect_specialn,Priority::Low);
    agent.acmd("effect_specialairn", effect_specialn,Priority::Low);
    agent.acmd("sound_specialn", sound_specialn,Priority::Low);
    agent.acmd("sound_specialairn", sound_specialn,Priority::Low);
    agent.acmd("expression_specialn", expression_specialn,Priority::Low);
    agent.acmd("expression_specialairn", expression_specialn,Priority::Low);
    Agent::new("kirby")
        .acmd("game_richterspecialn", game_specialn,Priority::Low)
        .acmd("game_richterspecialairn", game_specialn,Priority::Low)
        .acmd("effect_richterspecialn", effect_specialn,Priority::Low)
        .acmd("effect_richterspecialairn", effect_specialn,Priority::Low)
        .acmd("sound_richterspecialn", sound_specialn,Priority::Low)
        .acmd("sound_richterspecialairn", sound_specialn,Priority::Low)
        .acmd("expression_richterspecialn", expression_specialn,Priority::Low)
        .acmd("expression_richterspecialairn", expression_specialn,Priority::Low)
    .install(); 
}