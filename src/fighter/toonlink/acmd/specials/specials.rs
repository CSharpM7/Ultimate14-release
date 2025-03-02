use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    //FT_MOTION_RATE_RANGE(agent,1.0,25.0,30.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, false, -1);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, toonlink::SPECIAL_S_FLAG_SEARCH);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK);
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            MotionModule::set_rate(agent.module_accessor,0.25);
            macros::SEARCH(agent, 0, 0, Hash40::new("top"), 0.0, 0.0, 5.0, 30.0, Some(0.0), Some(5.0), Some(40.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    //FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, toonlink::SPECIAL_S_FLAG_SEARCH);
        //WorkModule::off_flag(agent.module_accessor, toonlink::SPECIAL_S_FLAG_CAN_SEARCH);
        MotionModule::set_rate(agent.module_accessor,1.0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

unsafe extern "C" fn effect_specials1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 9, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specials1", game_specials1,Priority::Low);
    agent.acmd("game_specialairs1", game_specials1,Priority::Low);
    agent.acmd("effect_specials1", effect_specials1,Priority::Low);
    agent.acmd("effect_specialairs1", effect_specials1,Priority::Low);
}
