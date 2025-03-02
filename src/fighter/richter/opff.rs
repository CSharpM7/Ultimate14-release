use crate::imports::imports_agent::*;

unsafe fn book_update(fighter: &mut L2CFighterCommon) {
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE) {
        super::set_book(fighter.module_accessor,false);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
}
pub unsafe extern "C" fn richter_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        book_update(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main, richter_frame);
}