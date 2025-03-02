use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn expression_attachwall(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

pub unsafe extern "C" fn sound_attachwall(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_link_step_left_s_ft"));
        macros::STOP_SE(fighter, Hash40::new("se_link_step_right_s_ft"));
    }
}


pub unsafe extern "C" fn expression_attachwallclimb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

pub unsafe extern "C" fn sound_attachwallclimb(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_link_step_right_s_ft"));
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_link_step_left_s_ft"));
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("sound_attachwall", sound_attachwall,Priority::Low);
    agent.acmd("expression_attachwall", expression_attachwall,Priority::Low);
    agent.acmd("sound_attachwallclimb", sound_attachwallclimb,Priority::Low);
    //agent.acmd("expression_attachwallclimb", expression_attachwallclimb,Priority::Low);
}