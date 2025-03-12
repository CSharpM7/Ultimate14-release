use crate::imports::imports_agent::*;
use smash_rs::app::*;

mod acmd;
mod agent_init;
mod opff;
mod status;
mod axe;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
    //params::install();
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("richter");
    let whip = &mut smashline::Agent::new("richter_whip");
    
    acmd::install(agent,whip);
    status::install(agent);
    agent_init::install(agent);
    opff::install(agent);
    agent.install();
    
    axe::install();
}

const DEFAULT_COLOR: i32 = 0;
pub unsafe fn is_richter(module_accessor: *mut smash::app::BattleObjectModuleAccessor) -> bool{
    let entry_id = sv_battle_object::entry_id((*module_accessor).battle_object_id) as u32;
    let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
    let color = app::lua_bind::FighterInformation::fighter_color(info) as i32;

    #[cfg(feature = "dev")]
    return color==DEFAULT_COLOR;
    #[cfg(feature = "devhook")]
    return color==DEFAULT_COLOR;

    let to_return = color < 8;
    return color < 8;
}
pub unsafe fn is_alucard(module_accessor: *mut smash::app::BattleObjectModuleAccessor) -> bool {
    return MotionModule::is_anim_resource(module_accessor, Hash40::new("special_s3"));
}

const LEAFSHIELD_DISABLE_GROUPS: [WorkId; 6] = [
    //transition_groups::CHECK_GROUND_SPECIAL,
    //transition_groups::CHECK_AIR_SPECIAL,
    transition_groups::CHECK_GROUND_ESCAPE,
    transition_groups::CHECK_AIR_ESCAPE,
    transition_groups::CHECK_GROUND_GUARD,
    transition_groups::CHECK_GROUND_ATTACK,
    transition_groups::CHECK_GROUND_CATCH,
    transition_groups::CHECK_AIR_ATTACK,
    //transition_groups::CHECK_AIR_CLIFF,
];
const LEAFSHIELD_DISABLE_INDIVI: [WorkId; 5] = [
    transition_terms::CONT_ATTACK_DASH,
    //transition_terms::CONT_CATCH_DASH,
    //transition_terms::CONT_CATCH_TURN,
    transition_terms::CONT_SPECIAL_N,
    transition_terms::CONT_SPECIAL_S,
    transition_terms::CONT_SPECIAL_HI,
    transition_terms::CONT_SPECIAL_LW,
    //transition_terms::CONT_CLIFF_ATTACK,
    //transition_terms::CONT_CLIFF_ESCAPE
];
pub unsafe fn set_book(module_accessor: *mut smash::app::BattleObjectModuleAccessor, state: bool) {
    let mut has_book = smash::app::lua_bind::WorkModule::is_flag(module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_CROSS);
    let battle_object = get_battle_object_from_id((*module_accessor).battle_object_id);
    let fighter_kind = utility::get_kind(&mut *module_accessor);
    if fighter_kind == *FIGHTER_KIND_RICHTER || true {
        if state == has_book {
            return;
        }
        println!("New State: {state}");
        smash::app::lua_bind::WorkModule::set_flag(module_accessor,state,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_CROSS);
    }
    else if fighter_kind == *FIGHTER_KIND_KIRBY {
        has_book = VarModule::is_flag(battle_object,kirby::instance::flag::RICHTER_BOOK);
        if state == has_book {
            return;
        }
        VarModule::set_flag(battle_object,kirby::instance::flag::RICHTER_BOOK,state);
    }
    else {
        return;
    }

    let rs_accessor = module_accessor as *mut smash_rs::app::BattleObjectModuleAccessor;
    let work = (*rs_accessor).work();
    if !state {
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.unable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.unable_transition_term_forbid_indivi(*x);
        }
    }
    else {
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.enable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.enable_transition_term_forbid_indivi(*x);
        }
    }
}