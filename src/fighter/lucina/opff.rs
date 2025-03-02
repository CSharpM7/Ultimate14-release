use crate::imports::imports_agent::*;


pub unsafe extern "C" fn parallel_nerf(boma: &mut BattleObjectModuleAccessor,status_kind:i32) {
    if [*FIGHTER_STATUS_KIND_ATTACK_DASH,
    *FIGHTER_STATUS_KIND_ATTACK_S3,*FIGHTER_STATUS_KIND_ATTACK_LW3,
    *FIGHTER_STATUS_KIND_ATTACK_HI4,*FIGHTER_STATUS_KIND_ATTACK_LW4,
    *FIGHTER_STATUS_KIND_ATTACK_AIR,
    *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END,*FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX,
    *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
        AttackModule::set_power_mul_status(boma, 0.9);
    }
}
pub unsafe extern "C" fn mask(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor) {
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    if frame > 2.0 && !ArticleModule::is_exist(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK) {
        let entry = get_entry(fighter) as usize;
        lucina::instance::flag::IS_MASKED[entry] = true;
        ArticleModule::generate_article(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, false, -1);
        ArticleModule::set_frame(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, 50.0);
        ArticleModule::set_rate(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, 0.0);
    }
}

pub unsafe extern "C" fn lucina_frame(fighter: &mut L2CFighterCommon) {
    let boma = &mut *fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);

    parallel_nerf(boma,status_kind);
    //mask(fighter,boma);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main,  lucina_frame);
}