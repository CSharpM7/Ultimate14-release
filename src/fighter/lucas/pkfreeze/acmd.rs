use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_move(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {
		//macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, false);
		//macros::LAST_EFFECT_SET_COLOR(agent, 0.5, 1.0, 0.3);
	}
    for _ in 0..i32::MAX {
		if macros::is_excute(agent) {
			//macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
			//macros::LAST_EFFECT_SET_COLOR(agent, 0.5, 1.0, 0.3);
		}
    	wait(agent.lua_state_agent, 6.0);
	}
}
unsafe extern "C" fn effect_tame(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_hold"),false,false);
    }
}

unsafe extern "C" fn game_bang(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 34, 60, 0, 35, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x27936db96d));
    }
}
unsafe extern "C" fn effect_bang(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        /*
        if WorkModule::is_flag(agent.module_accessor, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_FLAG_MAX){
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
			macros::LAST_EFFECT_SET_COLOR(agent, 0.5, 1.0, 0.3);
        } 
        */
        let scale = 0.5;//ModelModule::scale(agent.module_accessor) * 2.0;
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_status_speed_up"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, scale, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, scale, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.5, 1.0, 0.3);
    }
}
unsafe extern "C" fn sound_bang(agent: &mut L2CAgentBase) {
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_bang", game_bang,Priority::Low);
    agent.acmd("effect_bang", effect_bang,Priority::Low);
    agent.acmd("sound_bang", sound_bang,Priority::Low);

    agent.acmd("effect_move", effect_move,Priority::Low);
    agent.acmd("effect_tame", effect_tame,Priority::Low);
}