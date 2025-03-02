use crate::imports::imports_agent::*;

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_loop as *const () as _))
}
unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
	ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("shieldb"));
	ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("shield_b"));

	fighter.status_AttackAir_Main()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
}