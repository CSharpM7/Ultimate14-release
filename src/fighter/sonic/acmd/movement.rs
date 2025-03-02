use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_jumpfront(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }

    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sonic_spintrace"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_RATE(agent,1.2);
        
        BURN_COLOR(agent,0.3, 0.8, 10.5, 0.3);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sonic_spintrace"),false,false);
        COL_NORMAL(agent);
    }
}
unsafe extern "C" fn effect_jumpback(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }

    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sonic_spintrace"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_RATE(agent,1.2);
        
        BURN_COLOR(agent,0.3, 0.8, 10.5, 0.3);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sonic_spintrace"),false,false);
        COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("effect_jumpfront", effect_jumpfront,Priority::Low);
    agent.acmd("effect_jumpfrontmini", effect_jumpfront,Priority::Low);
    agent.acmd("effect_jumpaerialfront", effect_jumpfront,Priority::Low);

    agent.acmd("effect_jumpback", effect_jumpback,Priority::Low);
    agent.acmd("effect_jumpbackmini", effect_jumpback,Priority::Low);
    agent.acmd("effect_jumpaerialback", effect_jumpback,Priority::Low);
}