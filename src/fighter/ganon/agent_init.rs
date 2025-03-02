use crate::imports::imports_agent::*;

unsafe extern "C" fn should_CHECK_SPECIAL_S_UNIQ(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if VarModule::is_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_S_DISABLE) {
        false.into()
    } else {
        true.into()
    }
}
unsafe extern "C" fn should_use_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    
    if (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(boma))
    || is_damage_status(boma)
    //|| (motion_kind == Hash40::new("jump_aerial_f").hash || motion_kind == Hash40::new("jump_aerial_b").hash) 
    {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
        VarModule::off_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_S_DISABLE);

        let charge = VarModule::get_float(fighter.battle_object,ganon::instance::float::SPECIAL_HI_START_FRAME);
        if !VarModule::is_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_HI_START_RECHARGE)
        && (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(boma))
        && ![0x1E3,0x1E4,0x1E5].contains(&status_kind)
        && charge > 0.0 {
            VarModule::on_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_HI_START_RECHARGE);
        }
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK_S4_START,*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,*FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,*FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind) 
    {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if [*FIGHTER_STATUS_KIND_REBIRTH,*FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) 
    {
        VarModule::off_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_HI_START_RECHARGE);
        VarModule::set_float(fighter.battle_object,ganon::instance::float::SPECIAL_HI_START_FRAME,0.0);
    }

    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_CHECK_SPECIAL_S_UNIQ as *const () as _));   
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi as *const () as _));   
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   

}
pub fn install(agent: &mut smashline::Agent) {
    agent.on_init(agent_start);
    agent.on_start(agent_start);
}