use crate::imports::imports_status::*;
pub const SPECIAL_S_JUMP_START_FRAME : f32 = 15.0;


pub unsafe extern "C" fn special_s_pre(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let mask = if sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S} else {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N};
    let power_up = if sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S} else {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N};

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        //*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (mask) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        power_up as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_s_situation_changed(fighter: &mut L2CFighterCommon, init: bool, spin_status: bool) {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

    if !spin_status {
        let motion_g = if !is_kirby {Hash40::new("special_s_start")} else {Hash40::new("special_n_start")};
        let motion_a = if !is_kirby {Hash40::new("special_air_s_start")} else {Hash40::new("special_air_n_start")};
        let motion = if fighter.is_grounded() {motion_g} else {motion_a};
        if init {
            let start_frame = if init && !fighter.is_grounded() {3.0} else {0.0};
            if !is_kirby {
                MotionModule::change_motion(fighter.module_accessor, motion, start_frame, 1.0, false, 0.0, false, false);
            }
            else {
                FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor,motion,start_frame, 1.0, false, 0.0, false, false);
            }
        }
        else {
            if !is_kirby {
                fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), (!init).into());
            }
            else {
                FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(fighter.module_accessor,motion,-1.0,1.0, 0.0, false, false);
            }
        }
    }
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(), FIGHTER_KINETIC_TYPE_MOTION_AIR.into());
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::clear_speed_all(fighter.module_accessor);
    fighter.sub_set_ground_correct_by_situation(true.into());
    let correct = if fighter.is_grounded() {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(correct));
}

pub unsafe extern "C" fn special_s_set_tornado_pos(fighter: &mut smashline::L2CFighterCommon) {
    if !fighter.is_grounded() {
        let offset_y = -9.0;
        let pos = *PostureModule::pos(fighter.module_accessor);
        let hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        let ray_check = GroundModule::ray_check_hit_pos(
            fighter.module_accessor,
            &Vector2f{x: pos.x, y: pos.y},
            &Vector2f{x: 0.0, y: offset_y},
            hit_pos,
            true
        );
        let new_y = if ray_check {hit_pos.y} else {pos.y+offset_y};
        WorkModule::set_float(fighter.module_accessor,new_y-pos.y,sonic::SPECIAL_TORNADO_OFFSET_Y);
    }
    else {
        WorkModule::set_float(fighter.module_accessor,0.0,sonic::SPECIAL_TORNADO_OFFSET_Y);
    }
}
pub unsafe extern "C" fn special_s_common(fighter: &mut smashline::L2CFighterCommon, spin_status: bool) {
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            special_s_situation_changed(fighter,false,spin_status);
        }
        else {
            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        }
    }

    if WorkModule::is_flag(fighter.module_accessor,sonic::SPECIAL_TORNADO_SPAWN_TORNADO) {
        WorkModule::off_flag(fighter.module_accessor,sonic::SPECIAL_TORNADO_SPAWN_TORNADO);
        special_s_set_tornado_pos(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor,sonic::SPECIAL_TORNADO_SPAWN_EFF) 
    && WorkModule::get_int(fighter.module_accessor,sonic::SPECIAL_TORNADO_EFF) == 0{
        WorkModule::off_flag(fighter.module_accessor,sonic::SPECIAL_TORNADO_SPAWN_EFF);
        
        let offset = WorkModule::get_float(fighter.module_accessor,sonic::SPECIAL_TORNADO_OFFSET_Y);
        let eff = EffectModule::req_follow(
            fighter.module_accessor,
            Hash40::new("sys_club_tornado"), 
            Hash40::new("top"),
            &Vector3f{x: 0.0, y: offset, z: 0.0},
            &VECTOR_ZERO,
            1.5,
            true,
        0,0,0,0,0,false,false) as u32;
        LAST_EFFECT_SET_COLOR(fighter,0.1,0.75,2.0);
        println!("Eff: {eff}");
        WorkModule::set_int(fighter.module_accessor,eff as i32,sonic::SPECIAL_TORNADO_EFF);
    }
}

pub unsafe extern "C" fn special_s_main(fighter: &mut smashline::L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0,sonic::SPECIAL_TORNADO_EFF);
    WorkModule::on_flag(fighter.module_accessor,sonic::INSTANCE_TORNADO_DISABLE);
    special_s_situation_changed(fighter,true,false);
    crate::fighter::sonic::set_ball_mode(fighter,false);
  
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_status_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    /*
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }
    } 
    */
    if MotionModule::is_end(fighter.module_accessor) {
        let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
        let next_status = if !is_kirby {FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD} else {FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START};
        fighter.change_status(next_status.into(), false.into());
        return 1.into();
    }
    special_s_common(fighter,false);

    0.into()
}

pub unsafe extern "C" fn special_s_hold_main(fighter: &mut smashline::L2CFighterCommon) -> L2CValue {  

    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let motion = if !is_kirby {Hash40::new("special_s_hold")} else {Hash40::new("special_n_spin")};
    if !is_kirby {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        //FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor,motion, 0.0, 1.0, false, 0.0, false, false);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("sonic_special_n_spin"), 0.0, 1.0, false, 0.0, false, false);
    }

    crate::fighter::sonic::set_ball_mode(fighter,true);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_hold_main_status_loop as *const () as _))
}

unsafe extern "C" fn special_s_hold_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    /*
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }
    } 
    */
    if MotionModule::is_end(fighter.module_accessor) {
        let next_status = if !is_kirby {FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP} else {FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_REBOUND};
        fighter.change_status(next_status.into(), false.into());
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            special_s_situation_changed(fighter,false,true);
        }
        else {
            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        }
    }
    special_s_common(fighter,true);

    0.into()
}

pub unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn special_s_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    special_s_teleport(fighter);
    0.into()
}
pub unsafe extern "C" fn special_s_teleport(fighter: &mut smashline::L2CFighterCommon) {
    let damage = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
    if damage > 0.0  || true {
        let rot = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_offset_from_top(fighter.module_accessor, Hash40::new("rot"), rot);
        if rot.x.abs() > 2.0 {
            let pos = *PostureModule::pos(fighter.module_accessor);
            let hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
            let ray_check = GroundModule::ray_check_hit_pos(
                fighter.module_accessor,
                &Vector2f{x: pos.x, y: pos.y},
                &Vector2f{x: rot.x-pos.x, y: 0.0},
                hit_pos,
                true
            );
            let new_x = if ray_check {hit_pos.x} else {pos.x+rot.x};
            PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: new_x, y: pos.y, z: pos.z});
        }
    }
}
pub unsafe extern "C" fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next_status = fighter.global_table[0xB].get_i32();
    let continue_special = [*FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD,*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START].contains(&next_status);
    
    if !continue_special {
        crate::fighter::sonic::set_ball_mode(fighter,false);
    }

    0.into()
}

pub unsafe extern "C" fn special_s_jump_pre(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let mask = if sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S} else {*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N};
    let power_up = if sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S} else {*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N};
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
        //*FS_SUCCEEDS_KEEP_EFFECT
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (mask) as u64,
        0,
        power_up as u32,
        0
    );
    0.into()
}
pub unsafe extern "C" fn special_s_jump_main(fighter: &mut smashline::L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor,sonic::INSTANCE_TORNADO_DISABLE);
    special_s_set_tornado_pos(fighter);
    let lr = PostureModule::lr(fighter.module_accessor);
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;

    let motion = if !is_kirby {Hash40::new("special_s_hold_jump")} else {Hash40::new("special_n_hit")};
    if !is_kirby {
        MotionModule::change_motion(fighter.module_accessor, motion, SPECIAL_S_JUMP_START_FRAME, 1.0, false, 0.0, false, false);
    }
    else {
        FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor,motion,SPECIAL_S_JUMP_START_FRAME, 1.0, false, 0.0, false, false);
    }
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
    StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
    if GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_DOWN) as u32) 
    || fighter.is_grounded() {
        let offset = 0.0*lr;
        let pos = *PostureModule::pos(fighter.module_accessor);
        let hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        let ray_check = GroundModule::ray_check_hit_pos(
            fighter.module_accessor,
            &Vector2f{x: pos.x, y: pos.y},
            &Vector2f{x: lr, y: 0.0},
            hit_pos,
            true
        );
        let new_x = if ray_check {hit_pos.x} else {pos.x+offset};
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: new_x, y: pos.y, z: pos.z});
    }
    KineticModule::clear_speed_all(fighter.module_accessor);
    
    let accel = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let distance = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_aerial_y"), 0);
    let jump_speed = KineticUtility::get_jump_speed_y(distance, accel);
    let jump_mul = if is_kirby {1.3} else {1.0};
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
        jump_speed*jump_mul
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y*1.1
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_x_limit"));
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_MOVE_AIR,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * 1.0,
        0.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        air_accel_x_mul * 0.5
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        air_accel_x_add * 0.5
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_jump_main_status_loop as *const () as _))
}

unsafe extern "C" fn special_s_jump_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !fighter.is_grounded() {
        //fighter.sub_air_check_dive();
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
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            }
            else {
                let landingLag = 16.0;
                WorkModule::set_float(fighter.module_accessor, landingLag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
            return 1.into();
        }
        else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
        && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }

    let decay_rate: f32 = 0.075;
    let status_frame = fighter.global_table[STATUS_FRAME].get_f32();
    let eff = WorkModule::get_int(fighter.module_accessor,sonic::SPECIAL_TORNADO_EFF) as u32;
    if eff != 0 {
        EffectModule::set_alpha(fighter.module_accessor,eff,1.0-(status_frame*decay_rate));
        EffectModule::detach(fighter.module_accessor,eff,5);
    }
    0.into()
}

pub unsafe extern "C" fn special_s_hold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next_status = fighter.global_table[0xB].get_i32();
    let continue_special = [*FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP,*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_REBOUND].contains(&next_status);
    
    crate::fighter::sonic::set_ball_mode(fighter,false);

    //macros::EFFECT_OFF_KIND(fighter, Hash40::new("spintrace"), true, false);
    let eff = WorkModule::get_int(fighter.module_accessor,sonic::SPECIAL_TORNADO_EFF) as u32;
    if !continue_special && eff != 0 {
        EffectModule::kill(fighter.module_accessor,eff,false,false);
    }
    else {
        EffectModule::detach_all(fighter.module_accessor,5);
    }

    let offset = WorkModule::get_float(fighter.module_accessor,sonic::SPECIAL_TORNADO_OFFSET_Y);
    let eff_handle = if continue_special {Hash40::new("sys_v_smoke_a")} else {Hash40::new("sys_down_smoke")};
    let eff_size = if continue_special {1.25} else {0.9};
    let pos = *PostureModule::pos(fighter.module_accessor);
    let eff_end = EffectModule::req(
        fighter.module_accessor,
        eff_handle,
        &Vector3f{x:pos.x,y:pos.y+offset,z:pos.z},
        &Vector3f{x:0.0,y:0.0,z:0.0},
        eff_size,
        0,
        -1,
        true,
        0
    );
    LAST_EFFECT_SET_COLOR(fighter,0.1,0.75,2.0);

    SoundModule::stop_loop_se_all(fighter.module_accessor, 0);
    
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    let special_status = if sonic::SPECIAL_TORNADO_SIDEB {*FIGHTER_STATUS_KIND_SPECIAL_S} else {*FIGHTER_STATUS_KIND_SPECIAL_N};

    agent.status(Pre, special_status, special_s_pre);
    agent.status(Main, special_status, special_s_main);
    agent.status(Exec, special_status, empty_status);
    agent.status(End, special_status, empty_status);
    agent.status(Exit, special_status, special_s_exit);
    //agent.status(FixCamera, special_status, empty_status);
    agent.status(CheckDamage, special_status, special_s_damage);

    agent.status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, special_s_jump_pre);
    agent.status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, special_s_hold_main);
    agent.status(Exec, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, empty_status);
    agent.status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, empty_status);
    agent.status(Exit, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, special_s_hold_exit);
    agent.status(CheckDamage, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, special_s_damage);

    agent.status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, special_s_jump_pre);
    agent.status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, special_s_jump_main);
    agent.status(Exec, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, empty_status);
    agent.status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, empty_status);
    
    if !sonic::SPECIAL_TORNADO_SIDEB {
        Agent::new("kirby")
        .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, special_s_pre)
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, special_s_main)
        .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, empty_status)
        .status(End, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, empty_status)
        .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, special_s_hold_exit)
        .status(CheckDamage, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N, special_s_damage)
        
        .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START, special_s_jump_pre)
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START, special_s_hold_main)
        .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START, empty_status)
        .status(End, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START, empty_status)
        .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START, special_s_hold_exit)
        .status(CheckDamage, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START, special_s_damage)
        
        .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_REBOUND, special_s_jump_pre)
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_REBOUND, special_s_jump_main)
        .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_REBOUND, empty_status)
        .status(End, *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_REBOUND, empty_status)
        .install();
    }
}
