use crate::imports::imports_agent::*;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma,0);
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let entry = get_entry(fighter) as usize;
    let mut has_mask = lucina::instance::flag::IS_MASKED[entry];

    if is_damage_status(boma)
    || prev_status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END {
        WorkModule::set_int(fighter.module_accessor,0,*FIGHTER_MARTH_INSTANCE_WORK_ID_INT_TERM);
        super::status::specials::remove_articles(fighter);
    }
    if (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(fighter.module_accessor)) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
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