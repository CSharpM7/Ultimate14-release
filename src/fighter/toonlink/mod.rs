mod acmd;
mod status;
//mod agent_init;
//mod opff;
mod vtable;

mod boomerang;
mod bowarrow;

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
    let agent = &mut smashline::Agent::new("toonlink");
    acmd::install(agent);
    status::install(agent);
    //agent_init::install(agent);
    //opff::install(agent);
    agent.install();

    boomerang::install();
    bowarrow::install();
}