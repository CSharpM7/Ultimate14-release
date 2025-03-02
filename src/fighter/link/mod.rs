use smash::{lib::lua_const::*, hash40};

mod acmd;
mod status;
mod agent_init;
//mod opff;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
    param_config::update_int_2(-(*WEAPON_KIND_LINK_PARASAIL), vec![-1].clone(), (hash40("article_use_type"),0 as u64, 1));
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("link");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    //opff::install(agent);
    agent.install();
}