use crate::imports::imports_agent::*;

pub const MAX_FRAME : f32 = 60.0;

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn = *FIGHTER_STATUS_ATTR_START_TURN;
    let mask = 0;

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
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0 as u64,
        0 as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_g = hash40("special_s_start");
    let motion_a = hash40("special_air_s1");
    fighter.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(),Hash40::new_raw(motion_a).into(),false.into());
    WorkModule::set_int64(fighter.module_accessor, motion_g as i64, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION);
    WorkModule::set_int64(fighter.module_accessor, motion_a as i64, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION_AIR);

    fighter.sub_set_ground_correct_by_situation(false.into());

    if fighter.is_grounded() {
        println!("Special S Grounded");
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        return fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FIRST);
        WorkModule::set_int(fighter.module_accessor,0,*FIGHTER_LINK_STATUS_BOOMERANG_CATCH_INT_MTRANS);

        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            1.0,
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_speed_y_stable
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable*1.1*lr,
        );
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            air_accel_x_mul * 0.2
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            air_accel_x_add * 0.2
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        return fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_main_loop as *const () as _))
    }
}

unsafe extern "C" fn special_air_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION_AIR);
    let mut to_return = 0;
    if motion == hash40("special_air_s1") {
        to_return = special_air_s_loop(fighter).into();
    }
    else if motion == hash40("special_air_s2") {
        to_return = special_air_s_landing_loop(fighter).into();
    }
    else if motion == hash40("landing_heavy") {
        to_return = special_air_s_landing_loop(fighter).into();
    }
    //Changed "status"
    let new_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION_AIR);
    if motion != new_motion {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK);
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(new_motion), 0.0, 1.0, false, 0.0, false, false);
    }
    
    to_return.into()
}


unsafe extern "C" fn special_air_s_loop(fighter: &mut L2CFighterCommon) -> i32 {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1;
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_transition_group_check_air_attack().get_bool() {
			//VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
            FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            return 1;
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() ||
        fighter.sub_air_check_fall_common().get_bool() {
			//VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
            return 1.into();
        } 
    } 
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_CATCH_INT_MTRANS) == 0 {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
    || StopModule::is_hit(fighter.module_accessor) {
        special_air_s_boost(fighter);
    }
    else if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_CATCH_INT_MTRANS,0);
    }

    if MotionModule::frame(fighter.module_accessor) > 27.0 {
        fighter.sub_air_check_dive();
    }

    if StatusModule::is_changing(fighter.module_accessor) 
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_grounded() {
            let special_land =  WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK);
            /*
            let landing_lag = if special_land {20.0} else {10.0};
            WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            let new_status = if special_land {FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL} else {FIGHTER_STATUS_KIND_LANDING};
            fighter.change_status(new_status.into(),false.into()); */
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if special_land {
                special_air_s_landing_light_init(fighter);
            }
            else {
                special_air_s_landing_init(fighter);
            }
            return 0;
        }
    }

    0
}
pub unsafe extern "C" fn special_air_s_boost(fighter: &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FIRST);

    let lr = PostureModule::lr(fighter.module_accessor);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*lr;
    SET_SPEED_EX(fighter,speed_x,1.5,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let sfx_debounce = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_CATCH_INT_MTRANS);
    if sfx_debounce == 0 {
        macros::PLAY_SE(fighter, Hash40::new("se_link_shieldguard"));
    }
    macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    WorkModule::set_int(fighter.module_accessor,5,*FIGHTER_LINK_STATUS_BOOMERANG_CATCH_INT_MTRANS);
    if MotionModule::frame(fighter.module_accessor) < 40.0 {
        //MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 40.0, true, true, false);
        MotionModule::set_rate(fighter.module_accessor,2.5);
    }
}
pub unsafe extern "C" fn special_s_attack(fighter: &mut L2CFighterCommon, param2: &L2CValue, param3: &L2CValue) -> L2CValue {
    let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION_AIR);
    if motion == hash40("special_air_s1") {
        //special_air_s_boost(fighter);
    }
    0.into()
}
unsafe extern "C" fn special_air_s_landing_light_init(fighter: &mut L2CFighterCommon) {
    let motion = hash40("special_air_s2");
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION_AIR);
}
unsafe extern "C" fn special_air_s_landing_init(fighter: &mut L2CFighterCommon) {
    let motion = hash40("landing_heavy");
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION_AIR);
}
unsafe extern "C" fn special_air_s_landing_loop(fighter: &mut L2CFighterCommon) -> i32 {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1;
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        /*
        if fighter.sub_transition_group_check_air_attack().get_bool() {
            FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            return 1;
        }
        */
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() ||
        fighter.sub_air_check_fall_common().get_bool() {
            return 1;
        } 
    } 

    if StatusModule::is_changing(fighter.module_accessor) 
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if !fighter.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1;
        }
    }
    0
}


unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION);
    let mut to_return = 0;
    if motion == hash40("special_s_start") {
        to_return = special_s_start_loop(fighter).into();
    }
    else if motion == hash40("special_s1") {
        to_return = special_s_loop(fighter).into();
    }
    else if motion == hash40("special_s2") {
        to_return = special_s_end_loop(fighter).into();
    }
    else if motion == hash40("special_s_end") {
        to_return = special_s_end_loop(fighter).into();
    }
    else if motion == hash40("special_s_cliff_jump") {
        to_return = special_s_jump_loop(fighter).into();
    }
    //Changed "status"
    let new_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION);
    if motion != new_motion {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK);
        AttackModule::clear_all(fighter.module_accessor);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_speedbooster"), false,false);
        let rate = if motion == hash40("special_s_cliff_jump") {0.75} else {1.0};
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(new_motion), 0.0, rate, false, 0.0, false, false);
    }
    
    to_return.into()
}

unsafe extern "C" fn special_s_start_loop(fighter: &mut L2CFighterCommon) -> i32 {
    if MotionModule::is_end(fighter.module_accessor) {
        special_s_init(fighter);
        return 0;
    }
    
    //Hit Wall
    let lr = PostureModule::lr(fighter.module_accessor);
    let wall_check = if lr > 0.0 {*GROUND_TOUCH_FLAG_RIGHT as u32} else {*GROUND_TOUCH_FLAG_LEFT as u32};
    if GroundModule::is_wall_touch_line(fighter.module_accessor, wall_check) {
        fighter.change_status(FIGHTER_STATUS_KIND_STOP_WALL.into(), false.into()); 
    }
    //Cliff check
    if GroundModule::is_ottotto(fighter.module_accessor, 1.5) {
        special_s_end_init(fighter);
        return 0;
    }

    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            special_s_attack_init(fighter);
            return 0;
        }
    }

    if StatusModule::is_changing(fighter.module_accessor) 
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if !fighter.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1;
        }
    }
    0
}

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) {
    let motion = hash40("special_s1");
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION);
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.5);
}
unsafe extern "C" fn special_s_loop(fighter: &mut L2CFighterCommon) -> i32 {
    if StatusModule::is_changing(fighter.module_accessor) 
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if !fighter.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1;
        }
    }


    //Respawn hitbox
    if !AttackModule::is_attack(fighter.module_accessor, 0, false) {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *WEAPON_ANIMCMD_GAME, Hash40::new("game_specials1"), -1);
    }

    //Hit Wall
    let lr = PostureModule::lr(fighter.module_accessor);
    let wall_check = if lr > 0.0 {*GROUND_TOUCH_FLAG_RIGHT as u32} else {*GROUND_TOUCH_FLAG_LEFT as u32};
    if GroundModule::is_wall_touch_line(fighter.module_accessor, wall_check) {
        fighter.change_status(FIGHTER_STATUS_KIND_STOP_WALL.into(), false.into()); 
    }

    //End motion
    let stick_x = fighter.global_table[STICK_X].get_f32()*lr;
    let turn_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("status_start_turn_stick_x"));
    let status_frame = fighter.global_table[STATUS_FRAME].get_f32();
    if GroundModule::is_ottotto(fighter.module_accessor, 1.5) 
    || status_frame >= MAX_FRAME {
        special_s_end_init(fighter);
        return 0;
    }
    else if (stick_x <= turn_stick && stick_x.abs() >= 0.2) 
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        special_s_end_init(fighter);
        return 0;
    }
    else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
        special_s_attack_init(fighter);
        return 0;
    }
    else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        special_s_jump_init(fighter);
        return 0;
    }
    
    0
}
unsafe extern "C" fn special_s_end_init(fighter: &mut L2CFighterCommon) {
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0);
    let motion = hash40("special_s_end");
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION);
}
unsafe extern "C" fn special_s_end_loop(fighter: &mut L2CFighterCommon) -> i32 {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1;
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        /*
        if fighter.sub_transition_group_check_air_attack().get_bool() {
            FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            return 1;
        }
        */
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() ||
        fighter.sub_air_check_fall_common().get_bool() {
            return 1;
        } 
    } 

    if StatusModule::is_changing(fighter.module_accessor) 
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if !fighter.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1;
        }
    }
    0
}
unsafe extern "C" fn special_s_attack_init(fighter: &mut L2CFighterCommon) {
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.25);
    let motion = hash40("special_s2");
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION);
}

unsafe extern "C" fn special_s_jump_init(fighter: &mut L2CFighterCommon) {
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_speedbooster"), false,false);
    ControlModule::clear_command(fighter.module_accessor, false);
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0);
    AttackModule::clear_all(fighter.module_accessor);
    let motion = hash40("special_s_cliff_jump");
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION);
}
unsafe extern "C" fn special_s_jump_loop(fighter: &mut L2CFighterCommon) -> i32 {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 1;
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        /*
        if fighter.sub_transition_group_check_air_attack().get_bool() {
            FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            return 1;
        }
        */
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() ||
        fighter.sub_air_check_fall_common().get_bool() {
            return 1;
        } 
    } 
    0
}

pub unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, empty_status);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_attack);
}