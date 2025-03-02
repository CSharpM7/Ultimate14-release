use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    let has_mask = ArticleModule::is_exist(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        let entry = get_entry(agent) as usize;
        lucina::instance::flag::IS_MASKED[entry] = !has_mask;
        if !has_mask {
            ArticleModule::set_frame(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, 50.0);
            ArticleModule::set_rate(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, 0.0);
        }
        else {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK) {
            ArticleModule::set_frame(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, 50.0);
            ArticleModule::set_rate(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, 0.0);
        }
    }
}

pub unsafe extern "C" fn game_entry(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let entry = get_entry(agent) as usize;
        lucina::instance::flag::IS_MASKED[entry] = false;
    }
}

pub unsafe extern "C" fn game_results(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let entry = get_entry(agent) as usize;
        let has_mask = lucina::instance::flag::IS_MASKED[entry];
        if has_mask && !ArticleModule::is_exist(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, false, -1);
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, Hash40::new("appeal_lw"), false, 50.0);
            ArticleModule::set_frame(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, 50.0);
            ArticleModule::set_rate(agent.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, 0.0);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_appeallwl", game_appeallw,Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw,Priority::Low);

    agent.acmd("game_entryl", game_entry,Priority::Low);
    agent.acmd("game_entryr", game_entry,Priority::Low);

    agent.acmd("game_win1", game_results,Priority::Low);
    agent.acmd("game_win2", game_results,Priority::Low);
    agent.acmd("game_win2a", game_results,Priority::Low);
    agent.acmd("game_win2b", game_results,Priority::Low);
    agent.acmd("game_win3", game_results,Priority::Low);
    agent.acmd("game_win3a", game_results,Priority::Low);
    agent.acmd("game_win3b", game_results,Priority::Low);
    agent.acmd("game_lose", game_results,Priority::Low);
}