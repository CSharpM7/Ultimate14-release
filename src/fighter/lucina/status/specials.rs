use crate::imports::imports_agent::*;

unsafe fn special_s_reset_control(fighter: &mut L2CFighterCommon) {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_spd_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("start_spd_x_mul"));

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let accel_x_mul = 0.375;
    let speed_x_max_mul = 1.0;
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        speed_x*start_spd_x_mul,
        0.0,
        0.0,
        0.0,
        0.0
    );
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
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * speed_x_max_mul,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * speed_x_max_mul,
        0.0
    );
    /*
    sv_kinetic_energy!(
        mul_x_accel_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        accel_x_mul
    );
    sv_kinetic_energy!(
        mul_x_accel_add,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        accel_x_mul
    );
    sv_kinetic_energy!(
        mul_x_speed_max,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_x_max_mul
    ); */
}
pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    let customize_special_hi_no = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_HI_NO);
    let start_spd_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("start_spd_x_mul"));
    let air_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_spd_y"));

    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if status_kind != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {
        speed_x *= start_spd_x_mul;
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    if situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else if situation_kind == *SITUATION_KIND_AIR {
        if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {
            //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

            special_s_reset_control(fighter);

            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP) {
                WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
                speed_y = air_spd_y;
            }
        }
        else {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_STOP_RESET_TYPE_AIR,
                speed_x,
                0.0,
                0.0,
                0.0,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
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
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    0.into()
}
pub unsafe extern "C" fn special_s_common_loop(fighter: &mut L2CFighterCommon, status: i32) -> i32 {
    /*
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    } */
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        let air_kinetic =  if status == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {FIGHTER_KINETIC_TYPE_FALL} else {FIGHTER_KINETIC_TYPE_AIR_STOP};
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),air_kinetic.into());
        let motion_g = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
        let motion_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
        if status == *FIGHTER_STATUS_KIND_SPECIAL_S {
            fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), true.into());
        }
        else {
            fighter.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(), Hash40::new_raw(motion_a).into(), true.into());
            if status == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {
                if CancelModule::is_enable_cancel(fighter.module_accessor) 
                && fighter.is_grounded() {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(),false.into());
                    return 1;
                }
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1;
        }
    }
    let new_status = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
    if new_status > 0 {
        WorkModule::set_int(fighter.module_accessor,0, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
        fighter.change_status(new_status.into(), false.into());
        return 1;
    }
    0
}
pub unsafe extern "C" fn special_s_common_exec(fighter: &mut L2CFighterCommon) -> i32 {
    let charge = WorkModule::get_int(fighter.module_accessor,lucina::SPECIAL_S_CHARGE);
    if charge > 0 {
        WorkModule::inc_int(fighter.module_accessor,lucina::SPECIAL_S_CHARGE);
        if (charge+1 == lucina::SPECIAL_S_CHARGE_MAX) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
            macros::PLAY_SE(fighter, Hash40::new("se_item_killsword_flash")); //se_item_killsword_flash //se_item_sensorbomb_search
        }
        if charge+1 >= lucina::SPECIAL_S_HOLD_MAX || ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::set_int(fighter.module_accessor,*FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
            return 1;
        }
    }

    0
}
pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {    
    let charge_max = 60;//WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame_max")) as i32;
    //WorkModule::set_int(fighter.module_accessor,charge_max, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHARGE_COUNT);
    WorkModule::set_int(fighter.module_accessor,0, lucina::SPECIAL_S_CHARGE);
    WorkModule::set_int(fighter.module_accessor,0, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);

    ArticleModule::generate_article(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, false, -1); 
    if ArticleModule::is_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW) {
        ArticleModule::change_status_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
        let bow_g = Hash40::new("n_start");
        let bow_a = Hash40::new("n_air_start");
        let bow_motion = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {bow_g} else {bow_a};
        ArticleModule::change_motion(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, bow_motion,false,-1.0);
    }

    ArticleModule::generate_article(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW, false, -1); 
    if ArticleModule::is_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW) {
        let article = ArticleModule::get_article(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article);
        WorkModule::set_int64(fighter.module_accessor, article_id as i64, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
    }

    
    let motion_g = hash40("special_s_start");
    let motion_a = hash40("special_air_s_start");
    WorkModule::set_int64(fighter.module_accessor, motion_g as i64, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, motion_a as i64, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    fighter.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(), Hash40::new_raw(motion_a).into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());

    return fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _));
}
pub unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let common = special_s_common_loop(fighter,*FIGHTER_STATUS_KIND_SPECIAL_S);
    if common == 1 {
        return common.into();
    }
    let exec = special_s_common_exec(fighter);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2.into(), false.into());
        return 1.into();
    }

    0.into()
}
pub unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return special_s_common_exec(fighter).into();
}


pub unsafe extern "C" fn special_s_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {    
    if ArticleModule::is_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW) {
        let bow_g = Hash40::new("n");
        let bow_a = Hash40::new("n_air");
        let bow_motion = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {bow_g} else {bow_a};
        ArticleModule::change_motion(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, bow_motion,false,-1.0);
    }

    let motion_g = hash40("special_s_hold");
    let motion_a = hash40("special_air_s_hold");
    WorkModule::set_int64(fighter.module_accessor, motion_g as i64, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, motion_a as i64, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    fighter.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(), Hash40::new_raw(motion_a).into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    if StopModule::is_stop(fighter.module_accessor){
        WorkModule::inc_int(fighter.module_accessor,lucina::SPECIAL_S_CHARGE);
    }

    return fighter.sub_shift_status_main(L2CValue::Ptr(special_s_hold_main_loop as *const () as _));
}
pub unsafe extern "C" fn special_s_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let common = special_s_common_loop(fighter,*FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2);
    if common == 1 {
        return common.into();
    }
    /*
    WorkModule::inc_int(fighter.module_accessor,lucina::SPECIAL_S_CHARGE);
    let charge = WorkModule::get_int(fighter.module_accessor,lucina::SPECIAL_S_CHARGE);
    let max_charge = 60;
    if charge >= max_charge {
        println!("End status");
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3.into(), false.into());
        return 1.into();
    } */

    0.into()
}
pub unsafe extern "C" fn special_s_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return special_s_common_exec(fighter).into();
}

pub unsafe extern "C" fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if ArticleModule::is_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW) {
        let charge = WorkModule::get_int(fighter.module_accessor,lucina::SPECIAL_S_CHARGE) as f32;
        let charge_max = 60.0;
        let ratio = charge/charge_max;
        ArticleModule::set_float(fighter.module_accessor,lucina::GENERATE_ARTICLE_BOWARROW,ratio,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE);
        ArticleModule::shoot_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
 
    if ArticleModule::is_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW) {
        let bow_g = Hash40::new("n_end");
        let bow_a = Hash40::new("n_air_end");
        let bow_motion = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {bow_g} else {bow_a};
        ArticleModule::change_motion(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, bow_motion,false,-1.0);
    }
    
    let motion_g = hash40("special_s");
    let motion_a = hash40("special_air_s");
    WorkModule::set_int64(fighter.module_accessor, motion_g as i64, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, motion_a as i64, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    fighter.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(), Hash40::new_raw(motion_a).into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    //fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());

    return fighter.sub_shift_status_main(L2CValue::Ptr(special_s_end_main_loop as *const () as _));
}

pub unsafe extern "C" fn special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_grounded(){
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            special_s_reset_control(fighter);
        }
    }

    let common = special_s_common_loop(fighter,*FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3);
    if common == 1 {
        return common.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    0.into()
}

pub unsafe extern "C" fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
pub unsafe extern "C" fn remove_articles(fighter: &mut L2CFighterCommon) {
    ArticleModule::remove_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    //figure out if the arrow was shot or nah
    ArticleModule::remove_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("handfalchion"),true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("waistfalchion"),false);
}
pub unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor,0,lucina::SPECIAL_S_CHARGE);
    remove_articles(fighter);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, empty_status);
    agent.status(ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_S, empty_status);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exit);
    
    agent.status(Init, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, empty_status);
    agent.status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, special_s_hold_main);
    agent.status(Exec, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, special_s_hold_exec);
    agent.status(ExecStop, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, empty_status);
    agent.status(Exit, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, special_s_exit);

    agent.status(Init, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, special_s_init);
    agent.status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, special_s_end_main);
    agent.status(Exec, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, empty_status);
    agent.status(ExecStop, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, empty_status);
    agent.status(Exit, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, special_s_exit);
    agent.status(End, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, special_s_end);
    /*
    Agent::new("kirby")
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_S, kirby_main)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_S_END, special_s_end_main)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_S_END_MAX, special_s_end_main)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_S_END, kirby_end_main)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_S_END_MAX, kirby_end_max_main) 
    .install();*/
}