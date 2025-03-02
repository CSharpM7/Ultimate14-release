// status imports
use crate::imports::imports_agent::*;

pub unsafe extern "C" fn fighter_guard_off_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("Just: {}",WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
        let shield_size = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_size"));
        let shield_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_scale"), 0);
        let shield_radius = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);

        let just_size = (shield_size * shield_scale) * shield_radius * 1.5;
        println!("Size: {}",just_size);

        ATTACK(fighter, 0, 0, Hash40::new("throw"), 0.0, 361, 0, 10, 60, just_size, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_GUARD_DAMAGE)(fighter)
}


unsafe extern "C" fn fighter_guard_damage_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let currentFrame = MotionModule::frame(fighter.module_accessor);
    let just = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD);
    if just
    {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
            StopModule::end_stop(fighter.module_accessor);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
        }
        else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
            StopModule::end_stop(fighter.module_accessor);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH, true);
        }
    }

    smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_GUARD_DAMAGE)(fighter)
}
pub fn install(agent: &mut smashline::Agent) {
    //agent.status(Init, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, fighter_guard_off_init);
    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, fighter_guard_damage_exec);
}