use crate::imports::imports_agent::*;

unsafe extern "C" fn special_lw_init(agent: &mut L2CFighterCommon) -> L2CValue {
    /*
    if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR {
        let sum_speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let mul_x = 1.0;
        let start_accel_x = 0.0025;
        let accel_y = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_y"), 0);
        let speed_x = sum_speed_x*mul_x;
        
        sv_kinetic_energy!(
            reset_energy,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL); */
    let exists = ArticleModule::is_exist(agent.module_accessor, pitb::GENERATE_ARTICLE_ORBITER);
    println!("Exist: {}",exists);
    let no_shield = ArticleModule::is_exist(agent.module_accessor, pitb::GENERATE_ARTICLE_ORBITER) || WorkModule::get_int(agent.module_accessor, *FIGHTER_PIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_NO_SHIELD_FRAME) > 0;
    println!("Init No Shield: {}",no_shield);

    WorkModule::set_flag(agent.module_accessor, no_shield, *FIGHTER_PIT_STATUS_SPECIAL_LW_FLAG_NO_SHIELD);


    return 0.into();
}

unsafe extern "C" fn special_lw_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        agent.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u32,
        0
    );

    0.into()
}
unsafe extern "C" fn special_lw_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let no_shield = WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_LW_FLAG_NO_SHIELD);
    let cat = if no_shield {0} else {*FIGHTER_LOG_ACTION_CATEGORY_ATTACK};
    let kind = if no_shield {0} else {*FIGHTER_LOG_ATTACK_KIND_SPECIAL_LW};
    smash_script::notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d),cat,kind,true);

    let motion_g = if !no_shield {hash40("special_lw")} else {hash40("special_lw_blank")};
    let motion_a = if !no_shield {hash40("special_air_lw")} else {hash40("special_air_lw_blank")};

    WorkModule::set_int64(agent.module_accessor, motion_g as i64, *FIGHTER_PIT_STATUS_SPECIAL_LW_INT_MOTION_GROUND);
    WorkModule::set_int64(agent.module_accessor, motion_a as i64, *FIGHTER_PIT_STATUS_SPECIAL_LW_INT_MOTION_AIR);
    agent.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(), Hash40::new_raw(motion_a).into(), false.into());
    agent.sub_set_ground_correct_by_situation(true.into());
    agent.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_FALL.into());
    agent.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("pit_rbit"), false);
    if agent.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(agent.module_accessor)
    && StatusModule::is_situation_changed(agent.module_accessor) {
        agent.sub_set_ground_correct_by_situation(false.into());
        agent.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_FALL.into());
        let motion_g = WorkModule::get_int64(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_LW_INT_MOTION_GROUND);
        let motion_a = WorkModule::get_int64(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_LW_INT_MOTION_AIR);
        agent.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(), Hash40::new_raw(motion_a).into(), true.into());
    }
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    0.into()
} 
pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_init);
    //agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    //agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, empty_status);
    //agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exit);
}