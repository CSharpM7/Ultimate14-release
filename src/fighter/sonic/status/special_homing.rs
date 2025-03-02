use crate::imports::imports_agent::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask = if !sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S} else {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N};
    let power_up = if !sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S} else {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N};

    let kinetic = if fighter.is_grounded()
    || true
    || GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_DOWN) as u32) {*FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_N} else {*FIGHTER_KINETIC_TYPE_UNIQ};
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        kinetic,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | mask) as u64,
        0,
        power_up as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    WorkModule::on_flag(fighter.module_accessor,sonic::INSTANCE_HOMING_DISABLE);

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N} else {*FIGHTER_STATUS_KIND_SPECIAL_N};
    let original: L2CValue = smashline::original_status(Main, fighter, original_status)(fighter);
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_n_substatus as *const () as _));
    
    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_N { 
        return original;
    }
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    original
}
unsafe extern "C" fn special_n_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_frame_coeff = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_attack_frame_coeff"));
    let ratio = 1.0 / attack_frame_coeff;
    let mut advance_counter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_WORK_FLOAT_ADVANCE_COUNTER);
    advance_counter += ratio;
    WorkModule::set_float(fighter.module_accessor, advance_counter, *FIGHTER_SONIC_STATUS_SPECIAL_N_WORK_FLOAT_ADVANCE_COUNTER);
    let auto_attack_frame_min = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_start_enable_attack_frame"));
    let auto_attack_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_start_auto_attack_frame"));
    if auto_attack_frame_min as f32 <= advance_counter {
        WorkModule::set_float(fighter.module_accessor, auto_attack_frame_max as f32, *FIGHTER_SONIC_STATUS_SPECIAL_N_WORK_FLOAT_ADVANCE_COUNTER);
        fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
        fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    }
    0.into()
}
unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_N { 
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
    }
    0.into()
}
unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N} else {*FIGHTER_STATUS_KIND_SPECIAL_N};
    let original: L2CValue = smashline::original_status(End, fighter, original_status)(fighter);
    original
}

unsafe extern "C" fn special_n_homing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask = if !sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S} else {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N};
    let power_up = if !sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S} else {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N};

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_N_HOMING,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | mask) as u64,
        0,
        power_up as u32,
        0
    );
    0.into()
}


unsafe extern "C" fn special_n_fail_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask = if !sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S} else {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N};
    let power_up = if !sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S} else {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N};

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_N_FAIL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | mask) as u64,
        0,
        power_up as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_n_fail_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let motion = if !is_kirby {Hash40::new("special_n_homing")} else {Hash40::new("special_n_homing")};
    if !is_kirby {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor,motion, 0.0, 1.0, false, 0.0, false, false);
        //MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
    let brake_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_fail_brake_start_frame"));
    let counter_add = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_fail_advance_frame"));
    WorkModule::set_int(fighter.module_accessor,brake_frame, *FIGHTER_SONIC_STATUS_SPECIAL_N_FAIL_WORK_INT_ADVANCE_COUNTER);
    WorkModule::add_int(fighter.module_accessor,counter_add, *FIGHTER_SONIC_STATUS_SPECIAL_N_FAIL_WORK_INT_ADVANCE_COUNTER);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_n_fail_substatus(fighter);
    }
    
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_n_fail_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_fail_status_loop as *const () as _))
}
unsafe extern "C" fn special_n_fail_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::count_down_int(fighter.module_accessor,*FIGHTER_SONIC_STATUS_SPECIAL_N_FAIL_WORK_INT_ADVANCE_COUNTER,0);
    0.into()
}

unsafe extern "C" fn special_n_fail_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let mut next_status = 0;
    /*
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
     */
    let counter = WorkModule::get_int(fighter.module_accessor,*FIGHTER_SONIC_STATUS_SPECIAL_N_FAIL_WORK_INT_ADVANCE_COUNTER);
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        next_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_REBOUND} else {*FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_REBOUND};
    }
    else if counter <= 0 {
        next_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_CANCEL} else {*FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_CANCEL};
    }
    if next_status != 0 {
        fighter.change_status(next_status.into(),false.into());
    }

    0.into()
}

unsafe extern "C" fn special_n_cancel_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask = if !sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S} else {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N};
    let power_up = if !sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S} else {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N};

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_N_HOMING_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | mask) as u64,
        0,
        power_up as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_n_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let motion = if !is_kirby {Hash40::new("special_n_cancel")} else {Hash40::new("special_n_cancel")};
    if !is_kirby {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor,motion, 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        speed_x*0.5,
        0.0,
        0.0,
        0.0,
        0.0
    );
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let speed_x_max_mul = 0.5;
    let accel_x_mul = 0.5;
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        air_accel_x_mul * accel_x_mul
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        air_accel_x_add * accel_x_mul
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * speed_x_max_mul,
        0.0
    );
    println!("Cancel!");
    
    WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_ACCEL_MUL);
    WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_cancel_status_loop as *const () as _))
}

unsafe extern "C" fn special_n_cancel_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        let next_status = if !is_kirby {FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_LANDING} else {FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_LANDING};
        fighter.change_status(next_status.into(), false.into());
        return 1.into();
    }

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    let special_status = if sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_STATUS_KIND_SPECIAL_N} else {*FIGHTER_STATUS_KIND_SPECIAL_S};
    agent.status(Pre, special_status, special_n_pre);
    agent.status(Main, special_status, special_n_main);
    agent.status(Exec, special_status, special_n_exec);
    agent.status(End, special_status, special_n_end);
    agent.status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING, special_n_homing_pre);
    agent.status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, special_n_fail_pre);
    agent.status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, special_n_fail_main);
    agent.status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_pre);
    agent.status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_main);

    if sonic::SPECIAL_TORNADO_SIDEB {
        Agent::new("kirby")
        .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, special_n_pre)
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, special_n_main)
        .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, special_n_exec)
        .status(End, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, special_n_end)
        .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING, special_n_homing_pre)
        .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_FAIL, special_n_fail_pre)
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_FAIL, special_n_fail_main)
        .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_CANCEL, special_n_cancel_pre)
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_CANCEL, special_n_cancel_main)
        .install();
    }
}