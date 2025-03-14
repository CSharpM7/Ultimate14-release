use crate::imports::imports_status::*;
use super::*;
//Unused

pub unsafe extern "C" fn captoss_haved_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    //GetVarManager::reset_var_module_by_object_id(weapon.battle_object_id, false);

    macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x1df7907ec3));
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REMOVE_SELF);
    0.into()
}

pub unsafe extern "C" fn captoss_haved_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn captoss_haved_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(captoss_haved_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_haved_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REMOVE_SELF)
    {
        weapon.clear_lua_stack();
        macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    0.into()
}

pub unsafe extern "C" fn captoss_haved_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let founder = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    0.into()
}


pub fn install(agent: &mut smashline::Agent) {  
    agent.status(Init, mario_cappy::CAPTOSS_STATUS_KIND_HAVED, captoss_haved_init);
    agent.status(Pre, mario_cappy::CAPTOSS_STATUS_KIND_HAVED, captoss_haved_pre);
    agent.status(Main, mario_cappy::CAPTOSS_STATUS_KIND_HAVED, captoss_haved_main);
    agent.status(Exec, mario_cappy::CAPTOSS_STATUS_KIND_HAVED, captoss_haved_exec);
}