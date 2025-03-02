use crate::imports::imports_agent::*;
use {
    std::collections::HashSet,
    once_cell::sync::Lazy,
};

pub static CLONED_SLOTS: Lazy<HashSet<(i32,i32)>> = Lazy::new(|| {
    let mut vec = HashSet::new();
    vec.insert((*FIGHTER_KIND_MARIO,*WEAPON_KIND_KOOPAJR_CANNONBALL));
    vec.insert((*FIGHTER_KIND_LUCINA,*WEAPON_KIND_LINK_BOWARROW));
    vec.insert((*FIGHTER_KIND_PITB,*WEAPON_KIND_MIISWORDSMAN_WAVE));

    vec
});

pub unsafe extern "C" fn is_cloned_article(object_boma: *mut BattleObjectModuleAccessor) -> bool {
    let object_kind = utility::get_kind(&mut *object_boma);
    let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);

    for article in CLONED_SLOTS.iter() {
        if owner_kind == article.0 && object_kind == article.1 {
            return true;
        }
    }
    return false;
}

//To prevent crashes when Isabelle/Villager pocket
pub unsafe extern "C" fn ac_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    
    if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let object_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        if object_id == 0 || object_id == OBJECT_ID_NULL {return;}
        let object_boma = sv_battle_object::module_accessor(object_id);
        if is_cloned_article(object_boma) {
            //Change Villager status
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE,false);
            WorkModule::set_int(fighter.module_accessor, 0x50000000, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);

            //Remove object
            let weapon = get_fighter_common_from_accessor(object_boma);
            smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            let pos = *PostureModule::pos(object_boma);
            EffectModule::req(
                object_boma,
                Hash40::new("sys_erace_smoke"),
                &Vector3f{x:pos.x,y:pos.y+2.0,z:pos.z},
                &Vector3f{x:0.0,y:0.0,z:0.0},
                0.625,
                0,
                -1,
                false,
                0
            );
            
        }
    }
}

pub fn install() {
    Agent::new("murabito")
        .on_line(Main, ac_update)
        .install();
    Agent::new("shizue")
        .on_line(Main, ac_update)
        .install();
}