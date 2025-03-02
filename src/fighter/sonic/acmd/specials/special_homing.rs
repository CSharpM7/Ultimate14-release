use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialnhomingstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 35.0, 0.0, 5.0, 30.0, Some(0.0), Some(5.0), Some(40.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
}
unsafe extern "C" fn game_specialnhoming(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        MotionModule::set_rate(agent.module_accessor, 80.0);

        let bone = if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_HOMING_FLAG_IS_KIRBY) {Hash40::new("hip")} else {Hash40::new("rot")};
        macros::ATTACK(agent, 0, 0, bone, 11.0, 67, 55, 0, 55, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);

        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        AttackModule::set_captured_same_time_attack(agent.module_accessor, 0, true);
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
    }
}
unsafe extern "C" fn game_specialnhit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -0.0, y: 0.5, z: 0.0});
    }
    macros::FT_MOTION_RATE(agent, 0.6);
}
pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialnhomingstart", game_specialnhomingstart,Priority::Low);
    agent.acmd("game_specialnhoming", game_specialnhoming,Priority::Low);
    agent.acmd("game_specialnhit", game_specialnhit,Priority::Low);
}