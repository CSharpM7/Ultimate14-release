use crate::imports::imports_agent::*;

pub unsafe extern "C" fn remove_articles(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POT,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    
    0.into()
}
pub unsafe extern "C" fn attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackAir();

    remove_articles(fighter)
}
pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();

    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shizue_turnip_2"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shizue_turnipflip_2"), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}

pub unsafe extern "C" fn attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec();
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shizue_turnip_2"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shizue_turnipflip_2"), false);

    if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("attack_air_f").hash {
        let frame = MotionModule::frame(fighter.module_accessor);
        if frame > 40.0 && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG) {
            let moneybag = get_article_boma(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG);
            PostureModule::set_link_scale(moneybag, 1.75*(1.0-((frame-40.0)/10.0)), false);
        }
    }

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_end);
    //agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, remove_articles);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exec);
}