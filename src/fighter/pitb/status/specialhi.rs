use crate::imports::imports_agent::*;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_END_FLOAT_SDIR: i32 = 0x1000007;
//pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR: i32 = 0x2100000D;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_TURNING: i32 = 0x2100000F;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BURN: i32 = 0x2100000C;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE: i32 = 0x2100000E;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_BRAKE_X: i32 = 0x1000008;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_BRAKE_Y: i32 = 0x1000009;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_DISTANCE: i32 = 0x100000B;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_MOVE: i32 = 0x100000C;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_TURN_DIR: i32 = 0x1000007;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_TURN_DIR_INIT: i32 = 0x100000A;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_ROT_Y: i32 = 0x100000D;

pub const FLIGHT_GRAVITY_ACCEL_Y_MUL: f32 = 0.05;
pub const FLIGHT_GRAVITY_MAX_Y_MUL: f32 = 0.01;

pub const FLIGHT_BRAKE_MUL: f32 = 2.0;
pub const FLIGHT_X_MUL: f32 = 0.95;
pub const FLIGHT_Y_MUL: f32 = 0.85;
pub const FLIGHT_MOTION_SPEED_MIN: f32 = 0.5;
pub const FLIGHT_MOTION_SPEED_MAX: f32 = 1.0;

pub const LANDING_LIGHT_MUL: f32 = 0.5;

pub const FALL_ACCEL_X_MUL: f32 = 0.75;
pub const FALL_MAX_X_MUL: f32 = 0.75;

pub unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_START_RECHARGE);

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(),FIGHTER_KINETIC_TYPE_MOTION_AIR.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
    
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    
    let is_air = StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR;
    WorkModule::set_flag(fighter.module_accessor, is_air,*FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH.into(), false.into());
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_transition_group_check_air_attack().get_bool() {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("pitb_ikaros_wing_flare"), true, true);
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 1.0, y: 0.75 , z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL); 

            FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            return 1.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() ||
        fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    } 

    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), true.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(),FIGHTER_KINETIC_TYPE_MOTION_AIR.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }            
    return 0.into();
}
pub unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_jump = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
    if !is_jump {
        let move_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
        let move_speed_y = lua_bind::KineticEnergy::get_speed_y(move_energy);
        let motion_y = MotionModule::trans_move_speed(fighter.module_accessor).value[1];

        //If rising via motion, or triggered via acmd...
        if motion_y > 0.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), true.into());
            
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            //WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
            //WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);

        }
    }

    0.into()
}

pub unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
	return 0.into();
}
pub unsafe extern "C" fn special_hi_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn special_hi_rush_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_from = VarModule::get_float(fighter.battle_object,pitb::instance::float::SPECIAL_HI_START_FRAME).ceil() as i32;
    WorkModule::set_int(fighter.module_accessor, start_from,*FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);

    let lr = PostureModule::lr(fighter.module_accessor);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_accel_y_mul = FLIGHT_GRAVITY_ACCEL_Y_MUL;
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let speed_y_max_mul = FLIGHT_GRAVITY_MAX_Y_MUL;
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        speed_y,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0,
        speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y*air_accel_y_mul
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable*speed_y_max_mul
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    special_hi_rush_exec(fighter);
    
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FREE,
        speed_x,
        speed_y,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_x,
        speed_y,
    );
    let air_speed_x_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("air_brake_x"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let brake_mul = FLIGHT_BRAKE_MUL;
    let stable_x_mul_x = FLIGHT_X_MUL;
    let stable_x_mul_y = FLIGHT_Y_MUL;
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_brake*brake_mul,
        air_speed_x_brake*brake_mul,
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable*stable_x_mul_x,
        air_speed_x_stable*stable_x_mul_y,
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable*stable_x_mul_x,
        air_speed_x_stable*stable_x_mul_y,
    );
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let accel_x_mul_x = FLIGHT_X_MUL;
    let accel_x_mul_y = FLIGHT_Y_MUL;
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        air_accel_x_mul*accel_x_mul_x,
        air_accel_x_mul*accel_x_mul_y
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        air_accel_x_add*accel_x_mul_x,
        air_accel_x_add*accel_x_mul_y
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    //SET_SPEED_EX(fighter,speed_x*lr,speed_y,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    0.into()
}

pub unsafe extern "C" fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, lr, FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_TURN_DIR_INIT);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_TURNING);

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, true, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));    

    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_hi_rush_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_rush_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_rush_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);
    }
    0.into()
}
unsafe extern "C" fn special_hi_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        special_hi_rush_init(fighter);
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame")) as f32;
        WorkModule::set_float(fighter.module_accessor, landing_frame*LANDING_LIGHT_MUL, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END.into(), false.into());
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) || true {
        if fighter.sub_transition_group_check_air_attack().get_bool() {
            FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            return 1.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() ||
        fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    } 

    let rush_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);
    let fly_frame_max = pitb::SPECIAL_HI_FUEL_MAX;//WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("stop_y_frame"));
    if rush_frame >= fly_frame_max {
        VarModule::on_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_COOLDOWN);

        let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
        WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END.into(), false.into());
        return 1.into();
    }
    else if rush_frame >= fly_frame_max-60 
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BURN){
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BURN);
    }
    
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_magnitude = sv_math::vec2_length(stick_x, stick_y);
    let motion_rate = lerp(FLIGHT_MOTION_SPEED_MIN,FLIGHT_MOTION_SPEED_MAX,stick_magnitude.min(1.0));
    MotionModule::set_rate(fighter.module_accessor, motion_rate);
    
    0.into()
}

unsafe extern "C" fn special_hi_rush_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    
    //TURN//
    let is_turning = WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_TURNING);
    if !is_turning {
        let turn_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
        if (stick_x*lr <= turn_stick && stick_x.abs() >= turn_stick) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_TURNING);
            WorkModule::set_float(fighter.module_accessor, -lr,FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_TURN_DIR);
            WorkModule::set_float(fighter.module_accessor, 0.0,FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_ROT_Y);
            /*
            TurnModule::set_turn(
                fighter.module_accessor,
                Hash40::new("landing"), //landing?
                lr,
                false,
                false,
                true
            ); */
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1c872c7ad2), true);
            PostureModule::reverse_lr(fighter.module_accessor);
            //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_turn"), 0.0, 0.0, true, 0.0, false, false);
        }
    }
    else {
        if !TurnModule::is_turn(fighter.module_accessor) {
            println!("No turn?");
            let target_dir = WorkModule::get_float(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_TURN_DIR);
            PostureModule::set_lr(fighter.module_accessor, target_dir);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_TURNING);
            KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }

    //GRAV//
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_accel_y_mul = FLIGHT_GRAVITY_ACCEL_Y_MUL;
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let speed_y_max_mul = FLIGHT_GRAVITY_MAX_Y_MUL;
    //If ascending/descending, decrease gravity control
    if stick_y.abs() > 0.1 {
        let mut grav_y = {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
        };
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
        grav_y = lerp(grav_y,0.0,0.2);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0,
            grav_y
        );
    }
    else {
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y*air_accel_y_mul
        );
    }
    0.into()
}

pub unsafe extern "C" fn special_hi_rush_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rush_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);
    VarModule::set_float(fighter.battle_object,pitb::instance::float::SPECIAL_HI_START_FRAME,rush_frame as f32);
    println!("Fuel: {rush_frame}");
    EFFECT_OFF_KIND(fighter,Hash40::new("pitb_ikaros_wing_flare"),true,false);
    0.into()
}


pub unsafe extern "C" fn special_hi_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR;
    WorkModule::set_flag(fighter.module_accessor,is_air,*FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
    
    if !is_air {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        return 0.into();
    }
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*0.5;
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*0.5;
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        speed_x,
        0.0,
        0.0,
        0.0,
        0.0
    );
    
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        air_accel_x_mul * FALL_ACCEL_X_MUL
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        air_accel_x_add * FALL_ACCEL_X_MUL
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * FALL_MAX_X_MUL,
        0.0
    );
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        speed_y,
        0.0,
        0.0,
        0.0
    );

    0.into()
}
pub unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}
pub unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = WorkModule::is_flag(fighter.module_accessor,*FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("landing_heavy"), L2CValue::Hash40s("special_air_hi_end"), false.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_end_main_loop as *const () as _))
}
pub unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = WorkModule::is_flag(fighter.module_accessor,*FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
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
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        let situation = StatusModule::situation_kind(fighter.module_accessor);
        if is_air && situation == *SITUATION_KIND_GROUND
        || !is_air && situation == *SITUATION_KIND_AIR {
            fighter.change_status_by_situation(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }

    if !is_air {
        let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        if MotionModule::frame(fighter.module_accessor) >= landing_frame {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }

    0.into()
}
pub unsafe extern "C" fn special_hi_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = WorkModule::is_flag(fighter.module_accessor,*FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
    if is_air && fighter.global_table[0xB].get_i32() == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        WorkModule::set_float(fighter.module_accessor,FALL_ACCEL_X_MUL,*FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_ACCEL_MUL);
        WorkModule::set_float(fighter.module_accessor,FALL_MAX_X_MUL,*FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }
    0.into()
}

pub unsafe extern "C" fn special_hi_end_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter,Hash40::new("pitb_ikaros_wing_flare"),false,false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wing_normal"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("wing_normal"), true);
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("effect_fallspecial"));
    0.into()
}


pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, empty_status);

    agent.status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_pre);
    agent.status(Init, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_init);
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_main);
    agent.status(Exec, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_exec);
    agent.status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_end); 
    agent.status(Exit, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_end); 
    
    agent.status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_end_pre);
    agent.status(Init, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_end_init);
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_end_main);
    agent.status(Exec, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, empty_status);
    agent.status(ExecStop, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, empty_status);
    agent.status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_end_end); 
    agent.status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_end_exit); 
    
    agent.status(Exit, *FIGHTER_STATUS_KIND_FALL_SPECIAL, special_hi_end_exit); 
}