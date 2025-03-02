use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn game_homing(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 85, 50, 0, 65, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
}

pub unsafe extern "C" fn effect_homing(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_missile_straight"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 1, true);
    }
}


pub unsafe extern "C" fn game_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

pub unsafe extern "C" fn effect_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samusd_atk_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3.5, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_homing", game_homing,Priority::Low);
    agent.acmd("effect_homing", effect_homing,Priority::Low);
    //agent.acmd("game_hburst", game_hburst,Priority::Low);
    agent.acmd("effect_hburst", effect_hburst,Priority::Low);
}