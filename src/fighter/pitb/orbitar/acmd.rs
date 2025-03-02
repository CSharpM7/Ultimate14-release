use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 70, 40, 0, 50, 0.32, 0.0, -7.0, 1.0, Some(0.0), Some(7.0), Some(1.0), 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.5, 0.0, 10, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
        //AttackModule::set_no_attacker_log(agent.module_accessor,0,false,false);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 40, 0, 20, 0.32, 0.0, -7.0, 1.0, Some(0.0), Some(7.0), Some(1.0), 0.5, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.5, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
        AttackModule::set_down_only(agent.module_accessor,1, true);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_shoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pitb_special_l01"));
    }
}

unsafe extern "C" fn effect_shoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let rot_y = if lr > 0.0 {300.0} else {240.0} ; //120?
        macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_guardian_shield"), Hash40::new("top"), 0, 0, 0, 0, rot_y, 0, 2.25, true);
        LAST_EFFECT_SET_COLOR(agent,0.25, 0.5, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        let eff = EffectModule::get_last_handle(agent.module_accessor) as u32;
        WorkModule::set_int(agent.module_accessor,eff as i32,*WEAPON_MIISWORDSMAN_WAVE_INSTANCE_WORK_ID_INT_EFFECT_TYPE);
    }
}
unsafe extern "C" fn sound_hit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
}
unsafe extern "C" fn sound_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pitb_special_l02"));
    }
}

unsafe extern "C" fn effect_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let rot_y = if lr > 0.0 {300.0} else {240.0} ; //120?
        macros::EFFECT(agent, Hash40::new("pitb_guardian_break"), Hash40::new("top"), 0, 0, 0, 0, rot_y, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent,0.25, 0.5, 1.0);
        let eff = EffectModule::get_last_handle(agent.module_accessor) as u32;
        WorkModule::set_int(agent.module_accessor,eff as i32,*WEAPON_MIISWORDSMAN_WAVE_INSTANCE_WORK_ID_INT_EFFECT_TYPE);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_respawn", game_shoot,Priority::Low);
    agent.acmd("game_shoot", game_shoot,Priority::Low);
    agent.acmd("sound_shoot", sound_shoot,Priority::Low);
    agent.acmd("effect_shoot", effect_shoot,Priority::Low);

    agent.acmd("sound_hit", sound_hit,Priority::Low);
    agent.acmd("sound_break", sound_break,Priority::Low);
    agent.acmd("effect_break", effect_break,Priority::Low);
}