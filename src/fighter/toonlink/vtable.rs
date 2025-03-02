use crate::imports::imports_agent::*;
use wubor_utils::app::*;

const ON_SEARCH_EVENT_OFFSET: usize = 0xc29ae0;//0x68d8a0;

#[skyline::hook(offset = ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn on_search_cmn_toonlink(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    on_search_toonlink(fighter, log,false);
    original!()(vtable, fighter, log)
}
unsafe fn on_search_toonlink(fighter: &mut Fighter, log: u64, is_kirby: bool) {
    let kind = fighter.battle_object.kind as i32;
    if kind == *FIGHTER_KIND_TOONLINK  {
        let module_accessor = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
        {
            let collision_log = *(log as *const u64).add(0x10/0x8);
            let collision_log = collision_log as *const CollisionLogScuffed;
            let target_id = (*collision_log).opponent_object_id;
            
            let target_prev = WorkModule::get_int(module_accessor, toonlink::SPECIAL_S_TARGET_ID_PREV) as u32;
            if target_id != *BATTLE_OBJECT_ID_INVALID as u32
            && target_id != target_prev {
                println!("Found: {target_id}");
                let current_target = WorkModule::get_int(module_accessor,toonlink::SPECIAL_S_TARGET_ID) as u32;
                if current_target == *BATTLE_OBJECT_ID_INVALID as u32
                || !sv_battle_object::is_active(current_target) {
                    println!("Auto set new target");
                    WorkModule::set_int(module_accessor,target_id as i32,toonlink::SPECIAL_S_TARGET_ID);
                    //WorkModule::off_flag(module_accessor, toonlink::SPECIAL_S_FLAG_SEARCH);
                    return;
                }

                let current_target_boma = sv_battle_object::module_accessor(current_target);
                let current_target_pos = *PostureModule::pos(current_target_boma);

                let new_target_boma = sv_battle_object::module_accessor(target_id);
                let new_target_pos = *PostureModule::pos(new_target_boma);

                let pos = *PostureModule::pos(module_accessor);
                
                let current_dist = sv_math::vec2_distance(pos.x,pos.y,current_target_pos.x,current_target_pos.y);
                let new_dist = sv_math::vec2_distance(pos.x,pos.y,new_target_pos.x,new_target_pos.y);
                //println!("Dist: {dist}/{max_dist}");
                if new_dist <= current_dist {
                    println!("Set new target by distance");
                    WorkModule::set_int(module_accessor,target_id as i32,toonlink::SPECIAL_S_TARGET_ID);
                }

            }
        }
    }
}
pub fn install() {
	skyline::install_hooks!(
        on_search_cmn_toonlink
    );
}