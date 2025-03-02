use crate::imports::imports_agent::*;
use super::*;


unsafe extern "C" fn shield_guard_size(fighter: &mut L2CFighterCommon) {
    let wait_shield_scale_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("wait_shield_scale_x"));
    let wait_shield_scale_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("wait_shield_scale_y"));
    let wait_shield_scale_z = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("wait_shield_scale_z"));
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("shield"), &Vector3f{x: wait_shield_scale_x,y: wait_shield_scale_y,z: wait_shield_scale_z});
}

pub unsafe extern "C" fn shield_guard_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield_attack = [*FIGHTER_STATUS_KIND_GUARD,*FIGHTER_STATUS_KIND_GUARD_ON].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0));
    if !shield_attack {
        return smashline::original_status(Exec, fighter, *FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD)(fighter);
    }
    shield_guard_size(fighter);
    return 0.into()
}

pub unsafe extern "C" fn shield_guard_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield_attack = [*FIGHTER_STATUS_KIND_GUARD,*FIGHTER_STATUS_KIND_GUARD_ON].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0));
    if !shield_attack {
        return smashline::original_status(Pre, fighter, *FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD)(fighter);
    }
    
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        //*FIGHTER_KINETIC_TYPE_GUARD_ON,
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        /* 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLOAT,
        */
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_VISIBILITY,
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0,
    );

    0.into()
}

pub unsafe extern "C" fn shield_guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, link::status::flag::SHIELD_ATTACK_SUCCEEDED);
    let shield_attack = [*FIGHTER_STATUS_KIND_GUARD,*FIGHTER_STATUS_KIND_GUARD_ON].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0));
    if !shield_attack {
        return smashline::original_status(Main, fighter, *FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD)(fighter);
    }

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(shield_guard_main_loop)
}

unsafe extern "C" fn shield_guard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor,*COLLISION_KIND_MASK_ALL)
    || AttackModule::is_infliction_status(fighter.module_accessor,*COLLISION_KIND_MASK_HIT) 
    || AttackModule::is_infliction_status(fighter.module_accessor,*COLLISION_KIND_MASK_ATTACK) {
        let can_se = !VarModule::is_flag(fighter.battle_object, link::status::flag::SHIELD_ATTACK_SUCCEEDED);
        if can_se {
            VarModule::on_flag(fighter.battle_object, link::status::flag::SHIELD_ATTACK_SUCCEEDED);
            //macros::PLAY_SE(fighter, Hash40::new("se_link_shieldguard"));
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return 1.into();
    }
    let cancelFrame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)), true);
    let guardFrame = 39.0;
    let guard_flag = MotionModule::frame(fighter.module_accessor) >= guardFrame;
    let can_guard = guard_flag && 
    (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD));
    
    if can_guard {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
        return 1.into();
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor){
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    /*
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        let new_status = if GroundModule::is_miss_foot(fighter.module_accessor) {FIGHTER_STATUS_KIND_MISS_FOOT} else {FIGHTER_STATUS_KIND_DAMAGE_FALL};
        fighter.change_status(new_status.into(), false.into());
    }
    */

    0.into()
}

pub unsafe extern "C" fn guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(guard_main_loop as *const () as _))
}

unsafe extern "C" fn guard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD.into(), false.into());
    }

    fighter.status_Guard_Main()
}

pub unsafe extern "C" fn guard_on_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_on_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(guard_on_main_loop as *const () as _))
}

unsafe extern "C" fn guard_on_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) != FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD {
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD.into(), false.into());
        }
    }
    fighter.status_GuardOn_Main()
}

pub fn install(agent: &mut smashline::Agent) {
    //agent.status(Main, *FIGHTER_STATUS_KIND_ATTACH_WALL, shield_guard_init);
    agent.status(Pre, *FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD, shield_guard_pre);
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD, shield_guard_main);
    agent.status(End, *FIGHTER_LINK_STATUS_KIND_SHIELD_GUARD, empty_status);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD, guard_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on_main);
}