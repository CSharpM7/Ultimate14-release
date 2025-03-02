use smash::{lib::lua_const::*, hash40};

mod acmd;
mod agent_init;
mod status;
mod opff;

mod arrow;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
    unsafe {
        crate::vars::lucina::GENERATE_ARTICLE_BOW = *FIGHTER_LUCINA_ARTICLE_TERM + smashline::clone_weapon("link", *WEAPON_KIND_LINK_BOW, "lucina","bow",true);
        smashline::clone_weapon("link", *WEAPON_KIND_LINK_BOWARROW, "lucina","bowarrow",true);
    	crate::vars::lucina::GENERATE_ARTICLE_BOWARROW = crate::vars::lucina::GENERATE_ARTICLE_BOW + 1;
    }
    param_config::disable_villager_pocket(*FIGHTER_KIND_LUCINA, vec![-1].clone(), *WEAPON_KIND_LINK_BOWARROW);
}
pub fn dev_install() {
    #[cfg(feature = "dev")]
    unsafe {
        crate::vars::lucina::GENERATE_ARTICLE_BOW = *FIGHTER_LUCINA_ARTICLE_TERM + 0;
    	crate::vars::lucina::GENERATE_ARTICLE_BOWARROW = *FIGHTER_LUCINA_ARTICLE_TERM + 1;
    }
    let agent = &mut smashline::Agent::new("lucina");
    agent_init::install(agent);
    acmd::install(agent);
    status::install(agent);
    opff::install(agent);
    agent.install();

    arrow::install();
}