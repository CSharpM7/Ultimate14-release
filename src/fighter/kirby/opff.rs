use crate::imports::imports_agent::*;

pub unsafe extern "C" fn lucas_cooldown(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if ![*FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N,*FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_END,
    *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE,*FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_HOLD].contains(&status_kind) {
        VarModule::countdown_int(fighter.battle_object,kirby::instance::float::LUCAS_COOLDOWN,0);
    }
}
pub unsafe extern "C" fn richter_book(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor) {
    let copy_kind = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    let is_richter = copy_kind == *FIGHTER_KIND_RICHTER;
    let is_richter_book = WorkModule::is_flag(boma,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_CROSS);//VarModule::is_flag(fighter.battle_object,kirby::instance::flag::RICHTER_BOOK);

    if is_richter_book {
        if !is_richter {
            crate::fighter::richter::set_book(boma,false);
        }
        else if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE) {
            crate::fighter::richter::set_book(boma,false);
        }
    }
}

pub unsafe extern "C" fn kirby_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        let boma = &mut *info.boma;
        lucas_cooldown(fighter,boma,info.status_kind);
        richter_book(fighter,boma);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //agent.on_line(Main,  kirby_frame);
}