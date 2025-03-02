use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_raijin_hold"), Hash40::new("armr"), 2, 0, 0, 0, 0, 0, 1, true);
        //EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {

}
unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


unsafe extern "C" fn sound_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
}



pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhi", game_specialhi,Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi,Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi,Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialhi,Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi,Priority::Low);
    agent.acmd("sound_specialairhi", sound_specialhi,Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi,Priority::Low);
    agent.acmd("expression_specialairhi", expression_specialhi,Priority::Low);
}