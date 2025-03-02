mod acmd;
mod status;
mod opff;
mod agent_init;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("daisy");
    acmd::install(agent);
    status::install(agent);
    opff::install(agent);
    agent_init::install(agent);
    agent.install();
}