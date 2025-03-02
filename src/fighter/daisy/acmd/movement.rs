use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn game_fuwafuwa_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
	    //KineticModule::suspend_energy(agent.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let lr = PostureModule::lr(agent.module_accessor);
        if ControlModule::get_stick_x(agent.module_accessor).signum() != lr
        && ControlModule::get_stick_x(agent.module_accessor).abs() > 0.5 {
            PostureModule::reverse_lr(agent.module_accessor);
            PostureModule::update_rot_y_lr(agent.module_accessor);
        } 
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0,y: 0.5,z: 0.0});
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
	    KineticModule::resume_energy(agent.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

pub unsafe extern "C" fn effect_fuwafuwa_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT_FLW_POS(agent, Hash40::new("daisy_levitation"), Hash40::new("toer"), 1, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(agent, Hash40::new("daisy_flower_petals"), Hash40::new("toer"), 0, 1, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}

pub unsafe extern "C" fn sound_fuwafuwa_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_daisy_attackhard_h02"));
        macros::PLAY_SE(agent, Hash40::new("se_daisy_dash_stop"));
    }
}

pub unsafe extern "C" fn sound_fuwafuwa(agent: &mut L2CAgentBase) {
    /* 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        macros::PLAY_STATUS(agent, Hash40::new("se_daisy_jump04"));
    }
    */
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_fuwafuwastart", game_fuwafuwa_start,Priority::Low);
    agent.acmd("effect_fuwafuwastart", effect_fuwafuwa_start,Priority::Low);
    agent.acmd("sound_fuwafuwastart", sound_fuwafuwa_start,Priority::Low);
    agent.acmd("sound_fuwafuwa", sound_fuwafuwa,Priority::Low);
}