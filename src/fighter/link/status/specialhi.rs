use crate::imports::imports_agent::*;

pub unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_was_hold = StatusModule::prev_status_kind(fighter.module_accessor,0) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD;
    let turn = *FIGHTER_STATUS_ATTR_START_TURN;
    let mask = 0;

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_HOLD_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_HOLD_INT,
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
        mask as u64,
        turn as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}


pub unsafe extern "C" fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_Ground = situation_kind == SITUATION_KIND_GROUND;
    WorkModule::set_flag(fighter.module_accessor, is_Ground,*FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
    WorkModule::set_float(fighter.module_accessor,0.0,*FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    WorkModule::set_float(fighter.module_accessor, 0.0, link::SPECIAL_HI_CHARGE);
    
    if !is_Ground {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, link::STATUS_KIND_SPECIAL_HI_JUMP);
    }
    0.into()
}

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(),Hash40::new("special_hi_start").into(),false.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_hi").into());
    fighter.sub_set_ground_correct_by_situation(false.into());

    let rslash_charge_spd = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_charge_spd_div"));
    let air_mul = if situation_kind == *SITUATION_KIND_AIR {1.5} else {1.0};
    MotionModule::set_rate(fighter.module_accessor, rslash_charge_spd*air_mul);

    return fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_start_loop as *const () as _))
}

unsafe extern "C" fn special_hi_main_start_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(),false.into());
        return 1.into();
    }

    if StatusModule::is_changing(fighter.module_accessor) 
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let prev_situation_kind = StatusModule::prev_situation_kind(fighter.module_accessor);
        let ground_flag = situation_kind == *SITUATION_KIND_GROUND;
        WorkModule::set_flag(fighter.module_accessor, ground_flag,*FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
        //if prev_situation_kind == *SITUATION_KIND_GROUND && situation_kind == *SITUATION_KIND_AIR {
            fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(),Hash40::new("special_hi_start").into(),true.into());
            fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_hi").into());
            fighter.sub_set_ground_correct_by_situation(true.into());
        //}
    }
    0.into()
}

pub unsafe extern "C" fn special_hi_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK;

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_HOLD_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_HOLD_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        /* 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        */
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn special_hi_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_hold"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_hi_hold_sub(fighter, L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_hold_sub as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_hold_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_hold_sub(fighter: &mut L2CFighterCommon,arg2: L2CValue) -> L2CValue {
    if !arg2.get_bool() {
        return 0.into();
    }
    let rslash_charge_spd_div = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_charge_spd_div"));
    WorkModule::add_float(fighter.module_accessor, 1.0 / rslash_charge_spd_div, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let rslash_hold_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame"));
    let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    //println!("Hold Charge: {charge}");

    if !MotionModule::is_end(fighter.module_accessor) && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            let frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
            if frame < rslash_hold_frame_max as f32 {
                return 0.into();
            }
        }
    }
    fighter.change_status_req(link::STATUS_KIND_SPECIAL_HI_JUMP, true);
    1.into()
}

unsafe extern "C" fn special_hi_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {  
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR {
        fighter.change_status_req(link::STATUS_KIND_SPECIAL_HI_JUMP, false);
        return 1.into();
    }
    0.into()
}


pub unsafe extern "C" fn special_hi_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn = 0;
    let mask = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_HOLD_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_HOLD_INT,
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
        mask as u64,
        turn as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}


pub unsafe extern "C" fn special_hi_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_Ground = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);

    let charge = WorkModule::get_float(fighter.module_accessor,*FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let max_charge = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32;
    let boost = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_attack_up"));

    let ratio = charge/max_charge;
    let attackup = (boost-1.0);
    let attackmul = 1.0 + (attackup*ratio);
    //println!("Jump Init Charge: {charge}");
    
    WorkModule::set_float(fighter.module_accessor, ratio, link::SPECIAL_HI_CHARGE);
    
    AttackModule::set_power_mul_status(fighter.module_accessor, attackmul);

    0.into()
}

pub unsafe extern "C" fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("rslash_landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    special_hi_jump_launch(fighter);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    let rslash_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, rslash_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
    fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
    
    let prev_situation = //StatusModule::prev_situation_kind(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND) {*SITUATION_KIND_GROUND} else {*SITUATION_KIND_AIR};
    let motion = if prev_situation == *SITUATION_KIND_GROUND
        {Hash40::new("special_hi")} else {Hash40::new("special_air_hi")};

    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);    
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true,0);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, 
            Hash40::new("special_air_hi"),
        true, 0.0);
    }
    
    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_KEEP));
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_hi_jump_main as *const () as _));
    return fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_jump_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_jump_launch(fighter: &mut L2CFighterCommon) {
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
    GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
}

unsafe extern "C" fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rslash_charge_spd = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_charge_spd_div"));
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let prev_situation_kind = StatusModule::prev_situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);

    MotionModule::set_rate(fighter.module_accessor, rslash_charge_spd);
    if frame > 6.0 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
    }
    //Attack Cancels
    if CancelModule::is_enable_cancel(fighter.module_accessor) || true {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }
        if fighter.sub_transition_group_check_air_special().get_bool() {
            return 1.into();
        }
        if fighter.sub_transition_group_check_air_attack().get_bool() {
            return 1.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
        return 1.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn sub_special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let prev_situation_kind = StatusModule::prev_situation_kind(fighter.module_accessor);

    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

    if situation_kind != SITUATION_KIND_GROUND || speed_y > 0.0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI {
                let rslash_end_air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_end_air_accel_x_mul"));
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI, rslash_end_air_accel_x_mul);
                app::sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
                let rslash_air_max_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_air_max_x_mul"));
                let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
                let rslash_end_air_max_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_end_air_max_x"));
                let mul_x_speed_max = ((rslash_end_air_max_x / air_speed_x_stable) / rslash_air_max_x_mul);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, mul_x_speed_max);
                app::sv_kinetic_energy::mul_x_speed_max(fighter.lua_state_agent);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);

                let charge = WorkModule::get_float(fighter.module_accessor, link::SPECIAL_HI_CHARGE);
                //println!("Ratio Charge: {charge}");
                let maxboost = 1.0;

                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    speed_y+(maxboost*charge),
                );
            }
        }
        else{
            special_hi_end_physics(fighter, false);
        }
    }
    0.into()
}

pub unsafe extern "C" fn special_hi_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {

    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true,0);
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, 
            Hash40::new("special_hi_fall"),
        true, 0.0);
    }
    0.into()

}

pub unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK);
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        //*FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_HI_FALL,
        *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut motion = Hash40::new("special_hi_fall");
    MotionModule::change_motion(
        fighter.module_accessor,
        motion,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    special_hi_end_physics(fighter,true);

    //GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PEACH_CLIFF_HANG_DATA_SPECIAL_HI as u32);

    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("rslash_landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_end_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST){
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
        SET_SPEED_EX(fighter,0.0,2.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
    }
    0.into()
}

unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        return 1.into();
    }

    //Attack Cancels
    if CancelModule::is_enable_cancel(fighter.module_accessor) || true {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }
        if fighter.sub_transition_group_check_air_special().get_bool() {
            return 1.into();
        }
        if fighter.sub_transition_group_check_air_attack().get_bool() {
            return 1.into();
        }
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    
    let squat_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let stick_y = fighter.global_table[0x1b].get_f32();
    if stick_y < squat_threshold {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        return 1.into();
    }

    if special_hi_end_wall_check(fighter) {
        return 1.into()
    }
    //special_hi_end_physics(fighter,false);
    
    0.into()
}
pub unsafe extern "C" fn special_hi_end_wall_check(fighter: &mut L2CFighterCommon) -> bool {
    let enter_cliff = GroundModule::can_entry_cliff(fighter.module_accessor) != 0;
    if enter_cliff {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, false);
        return true;
    }
    let no_attach = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME);
    let wall_jump_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_jump_stick_x"));
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    if no_attach <= 0 {
        if wall_jump_stick_x.abs() <= stick_x.abs() {
            let lr = PostureModule::lr(fighter.module_accessor);
            let attach_side = if lr>0.0 {*GROUND_TOUCH_FLAG_RIGHT} else { *GROUND_TOUCH_FLAG_LEFT };

            if GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_side)) 
            && GroundModule::is_touch(fighter.module_accessor, attach_side as u32) {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACH_WALL, false);
                return true;
            }
        }
    }
    return false;
}
pub unsafe extern "C" fn special_hi_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_hi_end_physics(fighter,false);
    0.into()
}

pub unsafe extern "C" fn special_hi_end_physics(fighter: &mut L2CFighterCommon, init: bool) {
    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    //let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_accel_y_mul = 0.3;
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_mul = 1.2;
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_speed_x_stable_mul = 0.6;
    let limit_y = -air_speed_y_stable*air_accel_y_mul;

    if speed_y <= 0.0 {
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y*air_accel_y_mul,
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            limit_y
        );
    }
    if init {
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y*air_accel_y_mul,
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            limit_y
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y*air_accel_y_mul,
        );
    }
    if speed_y <= -(limit_y.abs()){
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    //println!("SpeedY: {speed_y} / {limit_y}");
}

pub unsafe extern "C" fn special_hi_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *WEAPON_KIND_LINK_PARASAIL, ArticleOperationTarget(0));
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_CLIFF_HANG_DATA_DEFAULT as u32);
    0.into()
}

pub unsafe extern "C" fn special_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)(fighter);
    
    let prev_status = StatusModule::prev_status_kind(fighter.module_accessor,0);
    let hi = *FIGHTER_STATUS_KIND_SPECIAL_HI;
    let hihold = *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD;
    let hijump = link::STATUS_KIND_SPECIAL_HI_JUMP;
    let hiend = *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END;
    let specialhi = [hi,hihold,hijump,hiend];

    if specialhi.contains(&prev_status) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true,0);
        }
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, 
                Hash40::new("special_hi_landing"),
            true, 0.0);
        }
    }
    return original;
}

pub unsafe extern "C" fn special_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(0));
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, empty_status);
    
    agent.status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, special_hi_hold_pre);
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, special_hi_hold_main);
    
    agent.status(Init, link::STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_init);
    agent.status(Pre, link::STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_pre);
    agent.status(Main, link::STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_main);
    agent.status(Exec, link::STATUS_KIND_SPECIAL_HI_JUMP, empty_status);

    agent.status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_init);
    agent.status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_pre);
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
    agent.status(Exec, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_exec);
    agent.status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_end);
    
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, special_landing_init);
    agent.status(End, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, special_landing_end);
}