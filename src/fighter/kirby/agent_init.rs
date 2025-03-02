use crate::imports::imports_agent::*;

unsafe extern "C" fn should_use_special_tornado_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_SONIC {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor,sonic::INSTANCE_TORNADO_DISABLE) {
            return false.into();
        } 
    }
    true.into()
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    
    if (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(boma))
    || (is_damage_status(boma) && status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FALL)
    //|| (motion_kind == Hash40::new("jump_aerial_f").hash || motion_kind == Hash40::new("jump_aerial_b").hash) 
    {
        WorkModule::off_flag(fighter.module_accessor,sonic::INSTANCE_TORNADO_DISABLE);
    }
    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    if !sonic::SPECIAL_TORNADO_SIDEB {
        fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(should_use_special_tornado_callback as *const () as _));  
    } 
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   

}
pub fn install(agent: &mut smashline::Agent) {
    agent.on_init(agent_start);
    agent.on_start(agent_start);
}