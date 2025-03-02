use crate::imports::imports_agent::*;

unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP.into(),false.into());
        return 0.into();
    }
    
    0.into()
}
pub unsafe extern "C" fn special_hi_jump_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);

    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) 
    && speed_y < 1.0 {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }

    return 0.into();
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(Exec, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_exec);
}