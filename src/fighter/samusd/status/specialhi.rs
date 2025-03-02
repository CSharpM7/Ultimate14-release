use crate::imports::imports_agent::*;

pub const FLAG_SET_DIR: i32 = 0x2100000C; //When on, this sets joint rotation during startup.
pub const FLAG_RUSH_AIR: i32 = 0x2100000D; //True if Special Hi is used in air
pub const FLAG_RUSH_BRAKE: i32 = 0x2100000E;

pub const FLOAT_RUSH_INPUT_X: i32 = 0x1000007;
pub const FLOAT_RUSH_INPUT_Y: i32 = 0x1000008;
pub const FLOAT_RUSH_DEGREE: i32 = 0x1000009;

pub const INT_RUSH_FRAME: i32 = 0x11000005;
pub const INT_END_TYPE: i32 = 0x11000006; //Controls whether DSamus lands, continues on ground, or falls
pub const END_TYPE_LANDING: i32 = 0;
pub const END_TYPE_GROUND: i32 = 1;
pub const END_TYPE_AIR: i32 = 2;
pub const INT_END_NEXT_STATUS: i32 = 0x11000007;
pub const INT_CONST_LANDING_FRAME: i32 = 0x11000008;

pub const PARAM_RUSH_SPEED: f32 = 3.0;
pub const PARAM_RUSH_FRAME_MAX: i32 = 24; //Frame to transition into end status
pub const PARAM_RUSH_BRAKE_FRAME: i32 = 24; //Frame to start braking during rush. Make this greater than Max to prevent breaking
pub const PARAM_RUSH_BRAKE: f32 = 0.2; //Brake amount
pub const PARAM_SET_ANGLE_FRAME: f32 = 33.0; //Frame during start up to set angle and turn FLAG_SET_DIR on

pub unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_WAIT_TAKEOFF;
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
        *GROUND_CORRECT_KIND_AIR as u32,
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn special_hi_kinetic(fighter: &mut L2CFighterCommon) {
    let energy_stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let stop_speed_x = lua_bind::KineticEnergy::get_speed_x(energy_stop);
    let energy_gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let grav_speed_y = lua_bind::KineticEnergy::get_speed_y(energy_gravity);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*0.5;
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*0.6;

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_FREE,
        speed_x,
        speed_y,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.02,
        0.02
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {      
        KineticModule::clear_speed_all(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        ); 
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);

    WorkModule::off_flag(fighter.module_accessor, FLAG_SET_DIR);
    WorkModule::off_flag(fighter.module_accessor, FLAG_RUSH_AIR);
    WorkModule::off_flag(fighter.module_accessor, FLAG_RUSH_BRAKE);
    WorkModule::set_float(fighter.module_accessor, 0.0, FLOAT_RUSH_DEGREE);
    WorkModule::set_int(fighter.module_accessor, 0, INT_RUSH_FRAME);

    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("sjump_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_hold"), L2CValue::Hash40s("special_hi_hold_air"), false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    special_hi_kinetic(fighter);
    
    return fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _));
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(samusd::STATUS_KIND_SPECIAL_HI_RUSH.into(),false.into());
        return 0.into();
    }
    
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_hold"), L2CValue::Hash40s("special_hi_hold_air"), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }

    0.into()
}

unsafe extern "C" fn special_hi_set_direction(fighter: &mut L2CFighterCommon) {
    let isGrounded = special_hi_ray_check(fighter);
    let sit = if isGrounded {*SITUATION_KIND_GROUND} else {*SITUATION_KIND_AIR};
    let kinetic = if isGrounded {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
    StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(sit), false);
    WorkModule::set_flag(fighter.module_accessor,!isGrounded,FLAG_RUSH_AIR);

    //Get angle//
    let mut lr: f32 = PostureModule::lr(fighter.module_accessor);
    let mut lr_reversed = 1.0;
    let mut stick_x: f32 = ControlModule::get_stick_x(fighter.module_accessor);
    let mut stick_y: f32 = ControlModule::get_stick_y(fighter.module_accessor);

    if (stick_x*lr <= -0.1 && stick_x.abs() >= 0.2) {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        lr*=-1.0;
    }

    //If in deadzone, go up
    if (stick_x.abs() < 0.1 && stick_y.abs() < 0.1)
    {
        stick_x = 0.0; //if isGrounded {lr} else {0.0};
        stick_y = 1.0;
    }
    //If on ground, and aiming the stick towards the ground, limit y to 0
    if (isGrounded && stick_y < 0.0)
    {
        stick_y = 0.0;
        if (stick_x.abs() <0.1) {stick_x = lr;}
        stick_x = sv_math::vec2_normalize(stick_x, stick_y).x;
    }

    WorkModule::set_float(fighter.module_accessor, stick_x,FLOAT_RUSH_INPUT_X);
    WorkModule::set_float(fighter.module_accessor, stick_y,FLOAT_RUSH_INPUT_Y);

    let normalized = sv_math::vec2_normalize(stick_x, stick_y);
    let arctangent = normalized.y.atan2(normalized.x.abs());
    let mut degree = arctangent.to_degrees();
    let mut flip = 0.0;
    if (degree > 90.0 && degree < 270.0)
    {
        degree=(degree-180.0);
        flip = 180.0;
    }
    WorkModule::set_float(fighter.module_accessor, degree,FLOAT_RUSH_DEGREE);
}
unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_y.abs() < 0.05 {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }

    let frame = MotionModule::frame(fighter.module_accessor);
    let set_frame = PARAM_SET_ANGLE_FRAME;
    if !WorkModule::is_flag(fighter.module_accessor, FLAG_SET_DIR) 
    && frame > set_frame {
        WorkModule::on_flag(fighter.module_accessor, FLAG_SET_DIR);
        special_hi_set_direction(fighter);
    }
    else if WorkModule::is_flag(fighter.module_accessor, FLAG_SET_DIR) {
        let degree_target = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
        let end_frame = MotionModule::end_frame(fighter.module_accessor)-1.0;
        let t = (frame-set_frame)/(end_frame-set_frame);
        let degree = lerp(0.0,degree_target,t*0.5);
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: -degree, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }
    
    0.into()
}

pub unsafe extern "C" fn special_hi_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0 as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}


unsafe extern "C" fn special_hi_rush_kinetic(fighter: &mut L2CFighterCommon) {
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let accel_y_mul = if WorkModule::is_flag(fighter.module_accessor, FLAG_RUSH_BRAKE) {0.1} else {0.0};
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_accel_y*accel_y_mul
    ); 

    let brake = if WorkModule::is_flag(fighter.module_accessor, FLAG_RUSH_BRAKE) {PARAM_RUSH_BRAKE} else {0.0};//fire_rush_brake;
    let rad = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE).to_radians();
    //looks like it does sin/cos to find something
	if WorkModule::is_flag(fighter.module_accessor, FLAG_RUSH_BRAKE)
	&& !StopModule::is_stop(fighter.module_accessor) 
	&& !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
		sv_kinetic_energy!(
			set_brake,
			fighter,
			FIGHTER_KINETIC_ENERGY_ID_STOP,
			brake*rad.cos().abs(),
			brake*rad.sin().abs()
		);
		sv_kinetic_energy!(
			set_accel,
			fighter,
			FIGHTER_KINETIC_ENERGY_ID_STOP,
			0.0,
			0.0
		);
	}
}
unsafe extern "C" fn special_hi_rush_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut degree = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
    let isGrounded = StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND && degree.abs() < 10.0;//special_hi_ray_check(fighter);
    let sit = if isGrounded {*SITUATION_KIND_GROUND} else {*SITUATION_KIND_AIR};
    let kinetic = if isGrounded {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
    let correct = if isGrounded {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP} else {*GROUND_CORRECT_KIND_AIR};
    StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(sit), false);
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if isGrounded {
        if degree.abs() < 10.0 {
            degree = 0.0;
        }
        WorkModule::set_float(fighter.module_accessor, 0.0,FLOAT_RUSH_DEGREE);
    }
    if special_hi_ray_check(fighter){
        special_hi_snap_to_ground(fighter);
        if isGrounded {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }

    //Set speed
    let rad = degree.to_radians();
    let speed_init = PARAM_RUSH_SPEED;
    let speed_x=rad.cos()*speed_init*PostureModule::lr(fighter.module_accessor);
    let speed_y=rad.sin()*speed_init;
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
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    //If in not the air and under a certain degree, clear gravity with no accel 

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_FREE,
        speed_x,
        speed_y,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        speed_y
    );
    special_hi_rush_kinetic(fighter);
    
    special_hi_set_joint_rotation(fighter);
    0.into()
}

unsafe fn special_hi_ray_check(fighter: &mut L2CFighterCommon) -> bool {
    return GroundModule::ray_check(fighter.module_accessor, 
        &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, 
        &Vector2f{ x: 0.0, y: -5.5}, true) == 1; //?
}
unsafe fn special_hi_snap_to_ground(fighter: &mut L2CFighterCommon) {
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    let hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    let ray_check = GroundModule::ray_check_hit_pos(
        fighter.module_accessor,
        &Vector2f{x: pos_x, y: pos_y},
        &Vector2f{x: 0.0, y: -6.0},
        hit_pos,
        true
    );
    PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: hit_pos.y, z: pos_z});
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);
    SET_SPEED_EX(fighter,speed_x*lr,-0.01,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
}
unsafe fn special_hi_find_end_type(fighter: &mut L2CFighterCommon) {
    let degree = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
    let ground_correct_force = special_hi_ray_check(fighter);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && !ground_correct_force {
        WorkModule::set_int(fighter.module_accessor,END_TYPE_AIR, INT_END_TYPE);
    }
    else if degree.abs() < 20.0 {
        WorkModule::set_int(fighter.module_accessor,END_TYPE_GROUND, INT_END_TYPE);
    }
    else {
        WorkModule::set_int(fighter.module_accessor,END_TYPE_LANDING, INT_END_TYPE);
    }
}

unsafe extern "C" fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //special_hi_rush_init(fighter);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_hi"), 0.0,1.0,false,0.0,false,false);

    let degree = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
    if degree > 10.0 {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        special_hi_rush_kinetic(fighter);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        special_hi_rush_substatus(fighter);
    }
    //fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_hi_rush_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_rush_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_rush_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rush_frame = WorkModule::get_int(fighter.module_accessor, INT_RUSH_FRAME);
    if !StopModule::is_stop(fighter.module_accessor)
	&& !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        WorkModule::add_int(fighter.module_accessor, 1,INT_RUSH_FRAME);
    }
    let fire_rush_xlu_frame = 16;//WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("fire_rush_xlu_frame"));
    if fire_rush_xlu_frame < rush_frame+1 {
        GroundModule::set_passable_check(fighter.module_accessor, false);
    }
    0.into()
}

unsafe extern "C" fn special_hi_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let rush_frame = WorkModule::get_int(fighter.module_accessor, INT_RUSH_FRAME);
    let fire_rush_frame = PARAM_RUSH_FRAME_MAX;//WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("fire_rush_frame"));
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if fire_rush_frame < rush_frame {
        special_hi_find_end_type(fighter);
        fighter.change_status(samusd::STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 0.into();
    }
    if !fighter.global_table[IS_STOP].get_bool(){
        special_hi_rush_substatus(fighter);
        let to_return = special_hi_rush_handle_bound(fighter);
        if to_return == 1 {
            return to_return.into();
        }
    }

    if (!StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor)) {
        if situation == *SITUATION_KIND_AIR {
            WorkModule::on_flag(fighter.module_accessor,FLAG_RUSH_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        special_hi_rush_kinetic(fighter);
    }
    0.into()
}


unsafe extern "C" fn special_hi_rush_handle_bound(fighter: &mut L2CFighterCommon) -> i32 {
    let rush_frame = WorkModule::get_int(fighter.module_accessor, INT_RUSH_FRAME);
    let degree = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
    let status_frame = fighter.global_table[STATUS_FRAME].get_f32();
    
    //Groundcast to find ceil/wall
    /* 
    let air_flag = WorkModule::is_flag(fighter.module_accessor,FLAG_RUSH_AIR);
    if air_flag && !WorkModule::is_flag(fighter.module_accessor, FLAG_RUSH_BRAKE) {
        if GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_UP) as u32) {
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            
            let pos_x = PostureModule::pos_x(fighter.module_accessor);
            let pos_y = PostureModule::pos_y(fighter.module_accessor);
            let pos_z = PostureModule::pos_z(fighter.module_accessor);
            let mut hit_pos = Vector2f{x: 0.0, y: 0.0};
            let mut normal = &mut Vector2f{x: 0.0, y: 0.0};
            let mut ray = Vector2f{x: speed_x*2.0, y: speed_y*2.0};

            let ray_check = GroundModule::ray_check_hit_pos_normal(
                fighter.module_accessor,
                &Vector2f{x: pos_x, y: pos_y},
                &ray,
                &mut hit_pos,
                normal,
                true
            );
            if ray_check==1 {
                if (normal.x.abs() > 0.8 || normal.y.abs() > 0.8) {
                    special_hi_find_end_type(fighter);
                    fighter.change_status(samusd::STATUS_KIND_SPECIAL_HI_BOUND.into(), false.into());
                    return 1.into();
                }
            }
            WorkModule::on_flag(fighter.module_accessor, FLAG_RUSH_BRAKE);
        }
    }
    */

    //Check for landing
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
    && WorkModule::is_flag(fighter.module_accessor,FLAG_RUSH_AIR) 
    && (speed_y <= 0.5 || rush_frame > 6)
    {
        let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let angle = sv_math::vec2_angle(normal_x, normal_y, speed_x, speed_y);
        let fire_crush_angle: f32 = 20.0;//WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fire_crush_angle"));
        let rad: f32 = (fire_crush_angle + 90.0).to_radians();
        //If angle is too step, then land. Otherwise, continue forward
        if rad < angle {
            special_hi_find_end_type(fighter);
            fighter.change_status(samusd::STATUS_KIND_SPECIAL_HI_END.into(), false.into());
            return 1.into();
        }
        else
        {
            let speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let lr = PostureModule::lr(fighter.module_accessor);
            WorkModule::set_float(fighter.module_accessor, 0.0, FLOAT_RUSH_DEGREE);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                speed.abs()*lr,
                -0.02
            );
            WorkModule::off_flag(fighter.module_accessor,FLAG_RUSH_AIR);
            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);

        }
    }
    return 0.into();
}

unsafe extern "C" fn special_hi_set_joint_rotation(fighter: &mut L2CFighterCommon) {
    let degree = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: -degree, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})

}
unsafe extern "C" fn special_hi_rush_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_hi_set_joint_rotation(fighter);
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    let isGround = situation == *SITUATION_KIND_GROUND;

    let rush_frame = WorkModule::get_int(fighter.module_accessor, INT_RUSH_FRAME);
    let fire_rush_brake_frame = 0.1;
    if rush_frame > PARAM_RUSH_BRAKE_FRAME 
    && !WorkModule::is_flag(fighter.module_accessor, FLAG_RUSH_BRAKE)
	&& !StopModule::is_stop(fighter.module_accessor) 
	&& !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        WorkModule::on_flag(fighter.module_accessor, FLAG_RUSH_BRAKE);
        special_hi_rush_kinetic(fighter);
    }
    0.into()
}
unsafe extern "C" fn special_hi_rush_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_hi_set_joint_rotation(fighter);
    0.into()
}
unsafe extern "C" fn special_hi_rush_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_cshot_bullet"), false,false);
    0.into()
}


pub unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0 as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn special_hi_end_enable_control(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR { return };

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let speed_x_max_mul = 0.5;
    let accel_x_mul = 0.375;
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
    WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_ACCEL_MUL);
    WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
}

pub unsafe extern "C" fn special_hi_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_type = WorkModule::get_int(fighter.module_accessor, INT_END_TYPE);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let reset_type_x = if end_type != END_TYPE_AIR {*ENERGY_CONTROLLER_RESET_TYPE_MOVE_GROUND} else {*ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST};
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        reset_type_x,
        speed_x,
        0.0,
        0.0,
        0.0,
        0.0
    );
    if end_type != END_TYPE_AIR {
        if end_type == END_TYPE_LANDING {
            speed_x = 0.0;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
        WorkModule::set_float(fighter.module_accessor, 0.0, FLOAT_RUSH_DEGREE);
        WorkModule::off_flag(fighter.module_accessor,FLAG_RUSH_AIR);
        special_hi_snap_to_ground(fighter);
    }
    else {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y*0.5,
            0.0,
            0.0,
            0.0
        );
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let accel_y_mul = 1.0;
        let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y*accel_y_mul
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_speed_y_stable
        );
        special_hi_end_enable_control(fighter);
    }
    0.into()
}
pub unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("sjump_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    let end_type = WorkModule::get_int(fighter.module_accessor, INT_END_TYPE);
    let mot = match end_type {
        END_TYPE_GROUND => Hash40::new("special_hi_landing_f"),
        END_TYPE_LANDING => Hash40::new("special_hi_landing_lw"),
        END_TYPE_AIR => Hash40::new("special_hi_fall"),
        _ => Hash40::new("special_hi_fall"),
    };
    let mut next_status = match end_type {
        END_TYPE_GROUND => *FIGHTER_STATUS_KIND_WAIT,
        END_TYPE_LANDING => *FIGHTER_STATUS_KIND_WAIT,
        END_TYPE_AIR => *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        _ => *FIGHTER_STATUS_KIND_FALL_SPECIAL,
    };
    WorkModule::set_int(fighter.module_accessor, next_status.into(),INT_END_NEXT_STATUS.into());
    if end_type != END_TYPE_AIR {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
        WorkModule::set_float(fighter.module_accessor, 0.0, FLOAT_RUSH_DEGREE);
		KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
    }
    MotionModule::change_motion(fighter.module_accessor, mot, 0.0,1.0,false,0.0,false,false);
    
    return fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_end_main_loop as *const () as _));
}

unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let next_status = WorkModule::get_int(fighter.module_accessor, INT_END_NEXT_STATUS);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(next_status.into(),false.into());
        return 0.into();
    }
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    if MotionModule::frame(fighter.module_accessor) > 1.0 &&
    (!StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor)) {
        if situation == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        } 
        else
        {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into()); 
        }
    }

    0.into()
}

//Enable kinetics, smooth degree and joint rotation towards 0.0
unsafe extern "C" fn special_hi_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_type = WorkModule::get_int(fighter.module_accessor, INT_END_TYPE);

    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL &&
    StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND &&
    end_type == END_TYPE_AIR {
        special_hi_end_enable_control(fighter);
    }

    let mut degree = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
    if degree > 0.0 {
        degree -= 4.0;
        WorkModule::set_float(fighter.module_accessor, degree.max(0.0), FLOAT_RUSH_DEGREE);
        special_hi_set_joint_rotation(fighter);
    }
    0.into()
}

pub unsafe extern "C" fn special_hi_bound_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0 as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}
pub unsafe extern "C" fn special_hi_bound_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut degree = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
    let wall = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT)as u32);
    WorkModule::set_int(fighter.module_accessor, 
        if wall {END_TYPE_GROUND} else {END_TYPE_AIR},
        INT_END_TYPE);

        
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    let mut hit_pos = Vector2f{x: 0.0, y: 0.0};
    let mut normal = &mut Vector2f{x: 0.0, y: 0.0};
    let mut ray = Vector2f{x: 0.0, y: 20.0};
    
    let mut eff_degree = 0.0;
    if !wall {
        WorkModule::set_float(fighter.module_accessor, 0.0,FLOAT_RUSH_DEGREE);
        eff_degree = 180.0;
    }
    else {
        ray = Vector2f{x: speed_x*15.0, y: speed_y*15.0};
        eff_degree = degree;
        if PostureModule::lr(fighter.module_accessor) < 0.0 {
            eff_degree+=90.0;
        }
    }

    let ray_check = GroundModule::ray_check_hit_pos_normal(
        fighter.module_accessor,
        &Vector2f{x: pos_x, y: pos_y},
        &ray,
        &mut hit_pos,
        normal,
        true
    );
    let arctangent = normal.y.atan2(normal.x);
    eff_degree = arctangent.to_degrees()-90.0;
    if ray_check == 0 {
        hit_pos = Vector2f{x: pos_x, y: pos_y+5.0};
    }
    PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: hit_pos.x, y: hit_pos.y-1.0, z: pos_z});
    
    let effect = EffectModule::req(
        fighter.module_accessor,
        Hash40::new("sys_crown_collision"),
        &Vector3f{x: hit_pos.x, y: hit_pos.y, z: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: eff_degree},
        1.0,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rot(
        fighter.module_accessor,
        effect,
        &Vector3f{x: 0.0, y: 0.0, z: eff_degree}
    );


    
    let speed_y_mul = if wall {0.375} else {0.0};
    let speed_y_add = if wall {1.0} else {0.5};
    let speed_x_mul = if wall {-0.375} else {0.0};

    let new_x =speed_x*speed_x_mul;
    let new_y = (speed_y*speed_y_mul)+speed_y_add;
    
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_FREE,
        new_x,
        new_y,
        0.0,
        0.0,
        0.0
    );
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_accel_mul = if wall {1.0} else {0.0};
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        -air_accel_y*air_accel_mul
    );
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
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0,
        -air_accel_y*air_accel_mul
    );
    if !wall {  
        KineticModule::clear_speed_all(fighter.module_accessor);
    }

    0.into()
}
pub unsafe extern "C" fn special_hi_bound_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //VarModule::on_flag(fighter.battle_object, fighter::instance::flag::DISABLE_JUMP);

    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("sjump_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    let end_type = WorkModule::get_int(fighter.module_accessor, INT_END_TYPE);
    let mot = match end_type {
        END_TYPE_GROUND => Hash40::new("special_air_hi_wall"),
        _ => Hash40::new("special_air_hi_ceil"),
    };
    let rate = match end_type {
        END_TYPE_GROUND => 1.25,
        _ => 0.5,
    };
    MotionModule::change_motion(fighter.module_accessor, mot, 0.0,rate,false,0.0,false,false);  
    
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);

    return fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_bound_main_loop as *const () as _));
}

unsafe extern "C" fn special_hi_bound_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    /* 
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }*/
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_DOWN.into(), FIGHTER_STATUS_KIND_DAMAGE_FALL.into(),false.into());
        return 0.into();
    }
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    if// MotionModule::frame(fighter.module_accessor) > 1.0 &&
    (!StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor)) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_DOWN.into(), FIGHTER_STATUS_KIND_DAMAGE_FALL.into(),false.into());
    }

    0.into()
}

unsafe extern "C" fn special_hi_bound_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_type = WorkModule::get_int(fighter.module_accessor, INT_END_TYPE);

    let mut degree = WorkModule::get_float(fighter.module_accessor, FLOAT_RUSH_DEGREE);
    if degree > 0.0 {
        degree -= 4.0;
        WorkModule::set_float(fighter.module_accessor, degree.max(0.0), FLOAT_RUSH_DEGREE);
        special_hi_set_joint_rotation(fighter);
    }
    0.into()
}


unsafe extern "C" fn damage_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == samusd::STATUS_KIND_SPECIAL_HI_BOUND {
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }

    let to_return = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DAMAGE_FALL)(fighter);
    
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == samusd::STATUS_KIND_SPECIAL_HI_BOUND {
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }

    to_return
}
pub fn install(agent: &mut smashline::Agent) {    
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, empty_status);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, empty_status);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, empty_status);

    agent.status(Init, samusd::STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_init);
    agent.status(Pre, samusd::STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_pre);
    agent.status(Main, samusd::STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_main);
    agent.status(Exec, samusd::STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_exec);
    agent.status(ExecStop, samusd::STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_execstop);
    agent.status(End, samusd::STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_end);
    agent.status(Exit, samusd::STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_end);

    agent.status(Init, samusd::STATUS_KIND_SPECIAL_HI_END, special_hi_end_init);
    agent.status(Pre, samusd::STATUS_KIND_SPECIAL_HI_END, special_hi_end_pre);
    agent.status(Main, samusd::STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
    agent.status(Exec, samusd::STATUS_KIND_SPECIAL_HI_END, special_hi_end_exec);
    agent.status(ExecStop, samusd::STATUS_KIND_SPECIAL_HI_END, special_hi_rush_execstop);
    agent.status(End, samusd::STATUS_KIND_SPECIAL_HI_END, empty_status);

    /*
    agent.status(Init, samusd::STATUS_KIND_SPECIAL_HI_BOUND, special_hi_bound_init);
    agent.status(Pre, samusd::STATUS_KIND_SPECIAL_HI_BOUND, special_hi_bound_pre);
    agent.status(Main, samusd::STATUS_KIND_SPECIAL_HI_BOUND, special_hi_bound_main);
    agent.status(Exec, samusd::STATUS_KIND_SPECIAL_HI_BOUND, special_hi_bound_exec);
    agent.status(ExecStop, samusd::STATUS_KIND_SPECIAL_HI_BOUND, special_hi_rush_execstop);
    agent.status(End, samusd::STATUS_KIND_SPECIAL_HI_BOUND, empty_status); 

    agent.status(Main, *FIGHTER_STATUS_KIND_DAMAGE_FALL, damage_fall_main);
    */

}