use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}
unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_fly_miracle_start"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, -1, 1, 0, 0, 0, 1, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, -1, -1, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pitb_fly_miracle_start"), true, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_fly_miracle"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, -1, 1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent,0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, -1, -1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent,0.2);
    }
}
unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pitb_jump01"));
        macros::PLAY_SE(agent, Hash40::new("vc_pitb_attack02"));
    }
}
unsafe extern "C" fn expression_specialhistart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
}
unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {        
        EFFECT_OFF_KIND(agent,Hash40::new("pitb_ikaros_wing_flare"),false,false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, -1, 1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent,0.35);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, -1, -1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent,0.35);
    }
    for feather in 0..i32::MAX{
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 15, -10, 0, 0, 0, 1, true);
        }
        for fire in 0..2{
            if macros::is_excute(agent) {

                if WorkModule::is_flag(agent.module_accessor, 0x2100000C){
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("s_wingl1"), 0, -1, 1, 0, 0, 0, 0.5, false);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("s_wingr1"), 0, -1, -1, 0, 0, 0, 0.5, false);
                }
            }
            wait(agent.lua_state_agent, 7.0);
        }
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    for feather in 0..i32::MAX{
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_pitb_jump02"));
        }
        wait(agent.lua_state_agent, 45.0);
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    for feather in 0..i32::MAX{
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 45.0);
    }
}

unsafe extern "C" fn game_specialairhiend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("pit_wing_normal"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("wing_normal"), false);
    }
}
unsafe extern "C" fn effect_specialairhiend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_fallspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("pit_wing_normal"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("wing_normal"), false);
    }
}

unsafe extern "C" fn game_landingfallspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("pit_wing_normal"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("wing_normal"), true);
    }
}
unsafe extern "C" fn effect_landingfallspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if StatusModule::prev_status_kind(agent.module_accessor,0) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, -1, 1, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent,1.5);
            macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, -1, -1, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent,1.5);
        }
    }
}


pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhistart", game_specialhistart,Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart,Priority::Low);
    agent.acmd("effect_specialhistart", effect_specialhistart,Priority::Low);
    agent.acmd("effect_specialairhistart", effect_specialhistart,Priority::Low);
    agent.acmd("sound_specialhistart", sound_specialhistart,Priority::Low);
    agent.acmd("sound_specialairhistart", sound_specialhistart,Priority::Low);
    agent.acmd("expression_specialhistart", expression_specialhistart,Priority::Low);
    agent.acmd("expression_specialairhistart", expression_specialhistart,Priority::Low);

    agent.acmd("game_specialhi", game_specialhi,Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi,Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi,Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi,Priority::Low);
    
    agent.acmd("game_specialairhiend", game_specialairhiend,Priority::Low);
    agent.acmd("effect_specialairhiend", effect_specialairhiend,Priority::Low);

    
    agent.acmd("game_fallspecial", game_fallspecial,Priority::Low);
    agent.acmd("game_landingfallspecial", game_landingfallspecial,Priority::Low);
    agent.acmd("effect_landingfallspecial", effect_landingfallspecial,Priority::Low);
    
}