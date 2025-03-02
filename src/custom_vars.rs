
use smash::lib::lua_const::BATTLE_OBJECT_CATEGORY_FIGHTER;

use {
    smash::{
        app:: *,
    },
    //custom_var::*,
    crate::imports::imports_agent::*
};
//use crate::imports::imports_agent::*;

#[skyline::hook(offset = 0x3af300)]
pub unsafe extern "C" fn battleobjectmoduleaccessor__initialize_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: *const u64) {
    original!()(module_accessor, param_1);
    let object_id = (*module_accessor).battle_object_id;
    if object_id != 0x50000000 {
        let cat = smash::app::utility::get_category(&mut *module_accessor);
        if cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            GetVarManager::reset_var_module_by_object_id(object_id, false);
        }
    }
}
#[skyline::hook(offset = 0x3afa10)]
pub unsafe extern "C" fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    let object_id = (*module_accessor).battle_object_id;
    let cat = smash::app::utility::get_category(&mut *module_accessor);
    if cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        GetVarManager::reset_var_module_by_object_id(object_id, false);
    }
}

#[skyline::hook(offset = 0x3afe00)]
pub unsafe extern "C" fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    let cat = smash::app::utility::get_category(&mut *module_accessor);
    let object_id = (*module_accessor).battle_object_id;
    if cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        GetVarManager::reset_var_module_by_object_id(object_id, false);
    }
    original!()(module_accessor, param_1)
}
#[skyline::hook(offset = 0x3af720)]
pub unsafe extern "C" fn battleobjectmoduleaccessor__finalize_modules(module_accessor: *mut BattleObjectModuleAccessor) {
    let cat = smash::app::utility::get_category(&mut *module_accessor);
    let object_id = (*module_accessor).battle_object_id;
    if cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        GetVarManager::remove_var_module_by_object_id(object_id);
    }
    original!()(module_accessor)
}
#[skyline::hook(offset = 0x33a0a40)]
pub unsafe extern "C" fn weapon_init_hook(weapon: &mut smash::app::Weapon, param_2: u64) {
    original!()(weapon, param_2);
    //MiscModule::get_vars_from_pocket(weapon.battle_object.module_accessor);
}

pub fn install() {
    skyline::install_hooks!(
        //battleobjectmoduleaccessor__initialize_modules,
        battleobjectmoduleaccessor__start_modules,
        battleobjectmoduleaccessor__end_modules,
        //battleobjectmoduleaccessor__finalize_modules,
        //weapon_init_hook,
    );
}
