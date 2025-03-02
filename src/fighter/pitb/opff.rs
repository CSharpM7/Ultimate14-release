use crate::imports::imports_agent::*;

unsafe fn shield_update(fighter: &mut L2CFighterCommon) {
    let shield_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_NO_SHIELD_FRAME);
    if shield_frame == 1 {
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("rot"), 0.0, 2.5, 1, 0, 0, 0, 1.0, true);
    }
}
unsafe fn wing_update(fighter: &mut L2CFighterCommon, status_kind: i32) {
    let status_frame = fighter.global_table[STATUS_FRAME].get_f32();

    let prev_kind = StatusModule::prev_status_kind(fighter.module_accessor,0);
    let burned_wings = [*FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END,*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind)
    || ([*FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END,*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&prev_kind) && status_frame < 15.0);

    if !burned_wings {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wing_normal"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("wing_normal"), true);
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("effect_fallspecial"));
    }
    
    let recover_wings = status_kind != FIGHTER_STATUS_KIND_FALL_SPECIAL &&
    [*FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END,*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&prev_kind)
    && MotionModule::frame(fighter.module_accessor) < 1.0; //uh oh

    if recover_wings {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, -1, 1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter,1.5);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, -1, -1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter,1.5);
    }
}
unsafe fn fuel_update(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_START_RECHARGE) {
        let charge = VarModule::get_float(fighter.battle_object,pitb::instance::float::SPECIAL_HI_START_FRAME)-1.0;
        let recharge = if fighter.is_grounded() {0.75} else {0.25};
        VarModule::add_float(fighter.battle_object,pitb::instance::float::SPECIAL_HI_START_FRAME,-recharge);
        if charge <= 0.0 {
            VarModule::off_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_START_RECHARGE);
            VarModule::set_float(fighter.battle_object,pitb::instance::float::SPECIAL_HI_START_FRAME,0.0);
        }
        else if charge <= (pitb::SPECIAL_HI_FUEL_MAX as f32 * 0.75) {
            if VarModule::is_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_COOLDOWN) {
                WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
                VarModule::off_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_COOLDOWN);
            }
        }
    }
}
pub unsafe extern "C" fn pitb_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        shield_update(fighter);
        fuel_update(fighter);
        //wing_update(fighter,info.status_kind);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main,  pitb_frame);
}