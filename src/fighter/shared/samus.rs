use crate::imports::imports_agent::*;

pub unsafe extern "C" fn samus_special_n_f2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    let is_max = charge >= charge_max;
    let mut is_dark = utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_SAMUSD;
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if is_kirby {
        is_dark = WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_SAMUSD;
    }

    let attr = if is_max && is_dark {*FIGHTER_STATUS_ATTR_START_TURN} else {0} as u32;
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT)as u64,
        attr,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}
pub unsafe extern "C" fn samus_special_n_f2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let motion_prefix = if is_kirby {"samusd_"} else {""};
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ST_INIT);
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    let is_max = charge >= charge_max;
    let motion_g = if is_max 
    {Hash40::new(format!("{}special_n_f_max",motion_prefix).as_str())} else {Hash40::new(format!("{}special_n_f",motion_prefix).as_str())};

    let motion_a = if is_max 
    {Hash40::new(format!("{}special_air_n_f_max",motion_prefix).as_str())} else {Hash40::new(format!("{}special_air_n_f",motion_prefix).as_str())};

    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {motion_g} else {motion_a};

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART) && true {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
    }
    else{
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0,false, false);
    }

    let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));

    let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    
    //WorkModule::set_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT,0);
    //Something about slopes?
    fighter.sub_shift_status_main(L2CValue::Ptr(samus_special_n_f2_main_loop as *const () as _))
}

unsafe extern "C" fn samus_special_n_f2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return 0.into();
    }
    
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);

        let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
        let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
        let is_max = charge >= charge_max;

        let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
        let motion_prefix = if is_kirby {"samusd_"} else {""};
        let motion_g = if is_max {Hash40::new("special_n_f_max")} else {Hash40::new("special_n_f")};
        let motion_a = if is_max {Hash40::new("special_air_n_f_max")} else {Hash40::new("special_air_n_f")};
        let motion_g = if is_max 
        {Hash40::new(format!("{}special_n_f_max",motion_prefix).as_str())} else {Hash40::new(format!("{}special_n_f",motion_prefix).as_str())};
    
        let motion_a = if is_max 
        {Hash40::new(format!("{}special_air_n_f_max",motion_prefix).as_str())} else {Hash40::new(format!("{}special_air_n_f",motion_prefix).as_str())};
        //fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(),true.into()); //bruh?

        fighter.sub_set_ground_correct_by_situation(true.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("samus")
        //.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, samus_special_n_f2_pre) 
        //.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, samus_special_n_f2_main)
        .install();
    Agent::new("samusd")
        .status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, samus_special_n_f2_pre) 
        .status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, samus_special_n_f2_main)
        .install();
    Agent::new("kirby")
        .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_F, samus_special_n_f2_pre) 
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_F, samus_special_n_f2_main)
        .install();
}