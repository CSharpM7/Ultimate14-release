use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn game_specials_h(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) != *SITUATION_KIND_GROUND {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
        }
    }
}


pub unsafe extern "C" fn game_specials_s(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) != *SITUATION_KIND_GROUND {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specials_h", game_specials_h,Priority::Low);
    agent.acmd("game_specialairs_h", game_specialairs_h,Priority::Low);
    agent.acmd("game_specials_s", game_specials_s,Priority::Low);
    agent.acmd("game_specialairs_s", game_specialairs_s,Priority::Low);
}