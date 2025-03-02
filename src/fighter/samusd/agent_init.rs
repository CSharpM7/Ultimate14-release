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
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_cshot_bullet_sub_b"), false,false);

    let charge_eff = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);
    EffectModule::set_pos(fighter.module_accessor, charge_eff as u32, &VECTOR_ZERO);

    if (&[*FIGHTER_STATUS_KIND_REBIRTH]).contains(&status_kind) || lua_bind::FighterManager::is_result_mode(singletons::FighterManager()) {
        let charge_eff = VarModule::get_int(fighter.battle_object, samusd::instance::int::CHARGE_EFF);
        effect!(fighter, MA_MSC_EFFECT_REMOVE, charge_eff);
    }
    if (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(boma))
    || (is_damage_status(boma) && status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FALL)
    //|| (motion_kind == Hash40::new("jump_aerial_f").hash || motion_kind == Hash40::new("jump_aerial_b").hash) 
    {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
        VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_JUMP);
    }
    if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::DISABLE_JUMP) {
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        if (&[*FIGHTER_STATUS_KIND_JUMP_AERIAL]).contains(&status_kind) {
            return false.into();
        }
    }
    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi as *const () as _));   
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   

}
pub fn install(agent: &mut smashline::Agent) {
    agent.on_init(agent_start);
    agent.on_start(agent_start);
}