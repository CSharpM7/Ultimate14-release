use crate::imports::imports_agent::*;

unsafe extern "C" fn attack_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !super::is_richter(fighter.module_accessor) {smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH)(fighter);}
    
    smashline::original_status(Main, fighter, *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32)(fighter)
}

unsafe extern "C" fn specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !super::is_richter(fighter.module_accessor) {smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);}
    VarModule::on_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_S);

    KineticModule::clear_speed_all(fighter.module_accessor);

    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s1"), L2CValue::Hash40s("special_air_s1"), false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(),FIGHTER_KINETIC_TYPE_MOTION_AIR.into());
    fighter.sub_set_ground_correct_by_situation(true.into());

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(specials_main_loop as *const () as _))
}

unsafe extern "C" fn specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() ||
        fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        } 
    } 
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s1"), L2CValue::Hash40s("special_air_s1"), true.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(),FIGHTER_KINETIC_TYPE_MOTION_AIR.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL) {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL) {
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY)
        && !fighter.is_grounded() {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            fighter.set_cliff_hangdata(14.0,20.0,-9.5,9.5);

            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALLED);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            //KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }
    return 0.into();
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, attack_dash_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, empty_status);
}