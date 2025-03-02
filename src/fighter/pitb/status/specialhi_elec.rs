use crate::imports::imports_agent::*;


pub unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}


pub unsafe extern "C" fn special_hi_rush_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    /* 
    sv_kinetic_energy!(
      clear_speed,
      fighter,
      FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    sv_kinetic_energy!(
      clear_speed,
      fighter,
      FIGHTER_KINETIC_ENERGY_ID_GRAVITY
    );
    sv_kinetic_energy!(
      clear_speed,
      fighter,
      KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN,
    );*/

    //Convert -1..1 stick_x to 0..1, then multiply by lr
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_shift_x_lr = (((ControlModule::get_stick_x(fighter.module_accessor)*lr)+1.0)/2.0);
    let mut speed_x = 0.0 //KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) 
    + (stick_shift_x_lr)*1.0;

    let param_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_arc_offset"));
    let param_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
    println!("paramy {}",param_speed_y);
    

    speed_x = (speed_x.max(0.0)).min(param_speed_x);
    let speed_y = (param_speed_y - (stick_shift_x_lr*0.25)).min(param_speed_y);
    SET_SPEED_EX(fighter, speed_x, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    //println!("LR: {} StickX {} Speed_X {} SpeedY {}",lr,stick_shift_x_lr,speed_x,speed_y);

    WorkModule::set_float(fighter.module_accessor,speed_x, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SPEED);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_CONTINUE);

    0.into()
}


pub unsafe extern "C" fn special_hi_rush_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //let speed_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
    //let accel = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_x"));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_CONTINUE)
    {
        let param_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
        let lr = PostureModule::lr(fighter.module_accessor);
        let speed_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SPEED);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, 1);//.min(param_speed_y);
        SET_SPEED_EX(fighter, speed_x, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    else{
        //WorkModule::on_flag(self, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
    }

    0.into()
}


unsafe extern "C" fn special_hi_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_SUPER_JUMP_PUNCH_AIR,
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        (*FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    0.into()
}



pub unsafe extern "C" fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);


    if StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SUPER_JUMP_PUNCH_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    //fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), false.into());
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_hi"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_hi"));

    return fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_rush_main_loop as *const () as _));
}

unsafe extern "C" fn special_hi_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    else if GroundModule::can_entry_cliff(fighter.module_accessor) == 1
    && ControlModule::get_stick_y(fighter.module_accessor) >= -0.7 {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE),
            L2CValue::Bool(false)
        );
        return 1.into()
    }

    fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_hi"));

    
    //if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR)
    let dive_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("stop_y_frame")) as f32;
    if MotionModule::frame(fighter.module_accessor) >= dive_frame
    {
        fighter.sub_air_check_dive();
        if (check_fastfall(fighter.module_accessor))
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = FIGHTER_STATUS_KIND_FALL_SPECIAL.into();
        fighter.change_status(status, false.into());
    }
    0.into()
}


pub unsafe extern "C" fn special_hi_rush_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn special_hi_rush_pos(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_fix_pos_slow_msc_common_table();
    return 0.into();
}

pub unsafe extern "C" fn special_hi_rush_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn special_hi_rush_leave_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut smashline::Agent) {
    /*
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);
    agent.status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_pre);
    agent.status(Init, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_init);
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_main);
    agent.status(Exec, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_exec);
    agent.status(ExecStop, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_exec_stop); */
}