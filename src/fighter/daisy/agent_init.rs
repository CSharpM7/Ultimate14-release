use crate::imports::imports_agent::*;

/// Prevents up b from being used again in air when it has been disabled by up-b fall
unsafe extern "C" fn should_use_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS) {
        false.into()
    } else {
        true.into()
    }
}

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    
    WorkModule::on_flag(boma,*FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
    WorkModule::off_flag(boma,*FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT_RAY_CHECK);
    if (&[*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START,*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT]).contains(&status_kind)
    {
        return false.into();
    }

    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   

}
pub fn install(agent: &mut smashline::Agent) {
    agent.on_init(agent_start);
    agent.on_start(agent_start);
}