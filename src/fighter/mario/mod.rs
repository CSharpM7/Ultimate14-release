use smash::{lib::lua_const::*, hash40};

mod acmd;
mod agent_init;
mod opff;
mod status;

mod cappy;

pub const HAVE: bool = false;
pub const FORCE_FLY: bool = true;
pub const SHOOT: bool = false;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}

pub fn hook_install() {
    unsafe {
        crate::vars::mario::GENERATE_ARTICLE_CAPTOSS = *FIGHTER_MARIO_ARTICLE_TERM + smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "mario", "captoss",false);
    }
    param_config::disable_villager_pocket(*FIGHTER_KIND_MARIO, vec![-1].clone(), *WEAPON_KIND_KOOPAJR_CANNONBALL);
    //vtable::install();
}

pub fn dev_install() {
    #[cfg(feature = "dev")]
    unsafe {
        crate::vars::mario::GENERATE_ARTICLE_CAPTOSS = *FIGHTER_MARIO_ARTICLE_TERM + 0;
    }

    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    cappy::install();
}
