use crate::imports::imports_agent::*;

/// Prevents up b from being used again in air when it has been disabled by up-b fall
unsafe extern "C" fn should_use_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS) {
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
    //|| (motion_kind == Hash40::new("jump_aerial_f").hash || motion_kind == Hash40::new("jump_aerial_b").hash) 
    {
        if !VarModule::is_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_COOLDOWN) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
        }

        let charge = VarModule::get_float(fighter.battle_object,pitb::instance::float::SPECIAL_HI_START_FRAME);
        if !VarModule::is_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_START_RECHARGE)
        && (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(boma))
        && ![*FIGHTER_STATUS_KIND_SPECIAL_HI,*FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH,*FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind)
        && charge > 0.0 {
            VarModule::on_flag(fighter.battle_object,pitb::instance::flag::SPECIAL_HI_START_RECHARGE);
            EffectModule::remove_common(fighter.module_accessor, Hash40::new("effect_fallspecialcommon"));
        }
    }
    if (&[*FIGHTER_STATUS_KIND_REBIRTH,*FIGHTER_STATUS_KIND_DEAD]).contains(&status_kind) {
        VarModule::set_float(fighter.battle_object,pitb::instance::float::SPECIAL_HI_START_FRAME,0.0);
    }
    

    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi as *const () as _));   
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   

}
pub unsafe extern "C" fn agent_end(fighter: &mut L2CFighterCommon)
{
    if LinkModule::is_link(fighter.module_accessor,*LINK_NO_CONSTRAINT) {
        println!("Unlink");
        LinkModule::unlink(fighter.module_accessor, *LINK_NO_CONSTRAINT);
    }
}
pub fn install(agent: &mut smashline::Agent) {
    agent.on_init(agent_start);
    agent.on_start(agent_start);
    agent.on_end(agent_end);
}