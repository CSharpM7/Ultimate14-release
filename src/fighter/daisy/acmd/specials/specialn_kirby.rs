use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn effect_daisyspecialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(agent.lua_state_agent, 1.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
    }
}
pub unsafe extern "C" fn expression_daisyspecialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item_flip") as i64, hash40("smash_item_racket") as i64);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
		VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item_flip") as i64, hash40("smash_item_none") as i64);
		ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}


pub unsafe extern "C" fn effect_daisyspecialnhit(agent: &mut L2CAgentBase) {
    let mut isTurn=false;
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0);
        isTurn = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_n_turn").hash
        || MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_air_n_turn").hash;
        if !isTurn{
            frame(agent.lua_state_agent, 8.0);
        }
        else{
            frame(agent.lua_state_agent, 20.0);
        }
    }
}

pub unsafe extern "C" fn sound_daisyspecialnhit(agent: &mut L2CAgentBase) {
    let mut isTurn=false;
    if macros::is_excute(agent) {
        isTurn = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_n_turn").hash
        || MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_air_n_turn").hash;
        if !isTurn{
            frame(agent.lua_state_agent, 7.0);
        }
        else{
            frame(agent.lua_state_agent, 19.0);
        }
    }
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_daisy_daisyspecial_n01"));
        macros::PLAY_SE(agent, Hash40::new("se_daisy_smash_s03"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_daisy_attack04"));
    }
}

pub unsafe extern "C" fn expression_daisyspecialnhit(agent: &mut L2CAgentBase) {
    let mut isTurn=false;
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item_flip") as i64, hash40("smash_item_racket") as i64);

        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);

        isTurn = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_n_turn").hash
        || MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_air_n_turn").hash;
        if !isTurn{
            frame(agent.lua_state_agent, 8.0);
        }
        else{
            frame(agent.lua_state_agent, 20.0);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 25, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 29.0);
	if macros::is_excute(agent) {
		VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item_flip") as i64, hash40("smash_item_none") as i64);
		ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
	}
}

pub fn install(agent: &mut smashline::Agent) {
    Agent::new("kirby")
    .acmd("effect_daisyspecialn", effect_daisyspecialn,Priority::Low)
    .acmd("effect_daisyspecialairn", effect_daisyspecialn,Priority::Low)
    .acmd("expression_daisyspecialairn", expression_daisyspecialn,Priority::Low)

    .acmd("effect_daisyspecialnhit", effect_daisyspecialnhit,Priority::Low)
    .acmd("effect_daisyspecialnturn", effect_daisyspecialnhit,Priority::Low)
    .acmd("effect_daisyspecialairnhit", effect_daisyspecialnhit,Priority::Low)
    .acmd("effect_daisyspecialairnturn", effect_daisyspecialnhit,Priority::Low)
    .acmd("sound_daisyspecialnhit", sound_daisyspecialnhit,Priority::Low)
    .acmd("sound_daisyspecialnturn", sound_daisyspecialnhit,Priority::Low)
    .acmd("sound_daisyspecialairnhit", sound_daisyspecialnhit,Priority::Low)
    .acmd("sound_daisyspecialairnturn", sound_daisyspecialnhit,Priority::Low)
    .acmd("expression_daisyspecialnhit", expression_daisyspecialnhit,Priority::Low)
    .acmd("expression_daisyspecialnturn", expression_daisyspecialnhit,Priority::Low)
    .acmd("expression_daisyspecialairnhit", expression_daisyspecialnhit,Priority::Low)
    .acmd("expression_daisyspecialairnturn", expression_daisyspecialnhit,Priority::Low)
    .install();
}