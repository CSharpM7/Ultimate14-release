use crate::imports::imports_agent::*;

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_grounded() {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_catch"), 0.0, 1.0, false, 0.0, false, false);
    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}
unsafe extern "C" fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_grounded() {
        return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
    return 0.into()
}


unsafe extern "C" fn special_air_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor,0) != *FIGHTER_STATUS_KIND_SPECIAL_S {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0 as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0,
    );
    0.into()
}

unsafe extern "C" fn special_air_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_S_DISABLE);
    if StatusModule::prev_status_kind(fighter.module_accessor,0) != *FIGHTER_STATUS_KIND_SPECIAL_S {
        //Special Hi//
        fighter.change_status(0x1E3.into(), false.into());
        return fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _));
        //return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    }
    //let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    //return original

    let explosion_speed_coef = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("explosion_speed_coef"));
    WorkModule::set_float(fighter.module_accessor,explosion_speed_coef,*FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    let stick_lr = PostureModule::set_stick_lr(fighter.module_accessor,0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    WorkModule::set_int(fighter.module_accessor,situation, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(),FIGHTER_KINETIC_TYPE_MOTION_AIR.into());

    return fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_main_loop as *const () as _));
}
unsafe extern "C" fn special_air_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(),false.into());
        return 1.into();
    }
    /*
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        }
    } */
    let init_situation = *SITUATION_KIND_AIR; //WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
    if init_situation != fighter.global_table[0x16].get_i32() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF) {
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            //fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
            let new_status_g = if init_situation == *SITUATION_KIND_GROUND {FIGHTER_STATUS_KIND_WAIT} else {FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL};
            let new_status_a = if init_situation == *SITUATION_KIND_GROUND {FIGHTER_STATUS_KIND_FALL} else {FIGHTER_STATUS_KIND_FALL};
            fighter.change_status_by_situation(new_status_g.into(), new_status_a.into(), false.into());
        }
    }
    
    //Cliff check
    if GroundModule::is_ottotto(fighter.module_accessor, 1.5) {
        if GrabModule::is_grab(fighter.module_accessor, 0) {
            MotionModule::set_frame(fighter.module_accessor, 30.0, true);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_air_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor,0) != *FIGHTER_STATUS_KIND_SPECIAL_S {
        return 0.into();
    }
    return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exit);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_air_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_air_s_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_air_s_exec);
} 