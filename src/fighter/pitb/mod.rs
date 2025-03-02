use smash::{lib::lua_const::*, hash40};

mod acmd;
mod status;
mod opff;
mod agent_init;

mod orbitar;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
    unsafe {
        crate::vars::pitb::GENERATE_ARTICLE_ORBITER = *FIGHTER_PITB_ARTICLE_TERM + smashline::clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "pitb","orbitar",false);
        println!("[smashline_14::main] Pitb Article: {} > {}",*FIGHTER_PITB_ARTICLE_TERM,crate::vars::pitb::GENERATE_ARTICLE_ORBITER);
    }
    param_config::disable_villager_pocket(*FIGHTER_KIND_PITB, vec![-1].clone(), *WEAPON_KIND_LUIGI_FIREBALL);
}
pub fn dev_install() {
    #[cfg(feature = "dev")]
    unsafe {
        crate::vars::pitb::GENERATE_ARTICLE_ORBITER = *FIGHTER_PITB_ARTICLE_TERM + 0;
    }
    let agent = &mut smashline::Agent::new("pitb");
    acmd::install(agent);
    status::install(agent);
    opff::install(agent);
    agent_init::install(agent);
    agent.install();

    orbitar::install();
}