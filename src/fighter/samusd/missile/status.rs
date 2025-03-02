use crate::imports::imports_agent::*;


unsafe extern "C" fn missile_homing_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let remaining_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_INT_LIFE);
    //let max_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    //println!("Life: {life} MaxLife: {max_life} CurrentLife: {current_life}");
    if remaining_life < 10 {
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("samusd_missile_homing"), false, true);
    }

    smashline::original_status(Exec, weapon, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING)(weapon)
}
unsafe extern "C" fn missile_homing_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_missile"), hash40("h_life"));
    WorkModule::set_int(weapon.module_accessor, life,*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    smashline::original_status(Main, weapon, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING)(weapon)
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Exec, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, missile_homing_exec);
    agent.status(Main, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, missile_homing_main);
}
