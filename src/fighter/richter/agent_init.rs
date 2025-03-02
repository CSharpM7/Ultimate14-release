use crate::imports::imports_agent::*;

/// Prevents up b from being used again in air when it has been disabled by up-b fall
unsafe extern "C" fn should_use_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let disable = VarModule::is_flag(fighter.battle_object,fighter::instance::flag::DISABLE_SPECIAL_S);
    if VarModule::is_flag(fighter.battle_object,fighter::instance::flag::DISABLE_SPECIAL_S) {
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
    
    if (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(boma))
    || is_damage_status(boma)
    {
        VarModule::off_flag(fighter.battle_object,fighter::instance::flag::DISABLE_SPECIAL_S);
    }

    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s as *const () as _));   
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _)); 

}
pub fn install(agent: &mut smashline::Agent) {
    agent.on_init(agent_start);
    agent.on_start(agent_start);
}