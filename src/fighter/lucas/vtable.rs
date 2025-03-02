use crate::imports::imports_agent::*;
use wubor_utils::app::*;

const ON_SEARCH_EVENT_OFFSET: usize = 0x68d8a0;
const ON_SEARCH_EVENT_KIRBY_OFFSET: usize = 0xb9d8d0;

#[skyline::hook(offset = ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn on_search_cmn_lucas(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    on_search_lucas(fighter, log,false);
    original!()(vtable, fighter, log)
}
#[skyline::hook(offset = ON_SEARCH_EVENT_KIRBY_OFFSET)]
unsafe extern "C" fn on_search_kirby(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;

    on_search_lucas(fighter, log,true);
    original!()(vtable, fighter, log)
}

unsafe fn on_search_lucas(fighter: &mut Fighter, log: u64, is_kirby: bool) {
    let kind = fighter.battle_object.kind as i32;
    if kind == *FIGHTER_KIND_LUCAS 
    || kind == *FIGHTER_KIND_KIRBY {
        let module_accessor = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(module_accessor);
        if (!is_kirby && status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE)
        || (is_kirby && status_kind == *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE)
        {
            let collision_log = *(log as *const u64).add(0x10/0x8);
            let collision_log = collision_log as *const CollisionLogScuffed;

            let frame = MotionModule::frame(module_accessor);
            let heal_frame = WorkModule::get_float(module_accessor,lucas::SPECIAL_N_HEAL_FRAME);
            if frame < heal_frame || heal_frame <= 0.0 {
                let target_id = (*collision_log).opponent_object_id;
                if target_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                    let target_boma = sv_battle_object::module_accessor(target_id as u32);
                    let target_team = FighterUtil::get_team_color(target_boma);
                    let owner_team = FighterUtil::get_team_color(module_accessor);
                    //println!("Search {target_team} = {owner_team}?");
                    if target_team == owner_team {
                        let heal = WorkModule::get_float(module_accessor, lucas::SPECIAL_N_HEAL) * 1.2;
                        println!("Search heal: {heal}");
                        super::pk_lifeup(&mut *target_boma, heal);
                    }
                    WorkModule::set_float(module_accessor,frame+0.1,lucas::SPECIAL_N_HEAL_FRAME);
                }
            }
            else {
                SearchModule::clear_all(module_accessor);
            }
        }
    }
}
pub fn install() {
	skyline::install_hooks!(
        on_search_cmn_lucas,
        on_search_kirby
    );
}