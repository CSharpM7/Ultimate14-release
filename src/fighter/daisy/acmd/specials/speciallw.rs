use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_IS_DAIKON_GENERATABLE) == 1 {
            let item = WorkModule::get_int(agent.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND);
            if item == *ITEM_KIND_NONE {
                ArticleModule::generate_article(agent.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON, false, -1);
            }
            else if item == *ITEM_KIND_BOMBHEI{
                ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_BOMBHEI), 0, 0, false, false);
            }
            else if item == *ITEM_KIND_DOSEISAN {
                ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_DOSEISAN), 0, 0, false, false);
            }
            else if item == *ITEM_KIND_BEAMSWORD {
                ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_RIPSTICK), 0, 0, false, false);
            }
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_speciallw", game_speciallw,Priority::Low);
}