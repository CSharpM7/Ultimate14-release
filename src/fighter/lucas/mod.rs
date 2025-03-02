mod acmd;
mod status;
//mod agent;
mod opff;
mod vtable;

mod pkfreeze;
mod pkfire;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
    vtable::install();
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("lucas");
    acmd::install(agent);
    status::install(agent);
    //agent::install(agent);
    opff::install(agent);
    agent.install();

    pkfreeze::install();
    //pkfire::install();
}

use smash::app::BattleObjectModuleAccessor;
pub unsafe fn pk_lifeup(module_accessor: &mut BattleObjectModuleAccessor, heal: f32) {
    use crate::imports::imports_agent::*;
    DamageModule::heal(module_accessor, -heal, 0);

    SoundModule::play_se(module_accessor, Hash40::new("se_item_healball_lifeup"), true, false, false, false, app::enSEType(0));
    EffectModule::req_follow(
        module_accessor,
        Hash40::new("sys_recovery"),
        Hash40::new("top"),
        &VECTOR_ZERO,
        &VECTOR_ZERO,
        0.9,
        false,
        0,
        0,
        0,
        0,
        0,
        false,
        false
    );
}