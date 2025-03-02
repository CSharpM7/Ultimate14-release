use crate::imports::imports_agent::*;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let next_status_kind = StatusModule::status_kind_next(fighter.module_accessor);

    //Re-enable Cap movement on ground/death
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
    }
    
    true.into()
}

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}
pub unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    agent_start(fighter);
}
pub unsafe extern "C" fn agent_reset(fighter: &mut L2CFighterCommon) {
    agent_start(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_init(agent_init);
    agent.on_start(agent_reset);
}