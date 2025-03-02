mod acmd;
mod status;
//mod agent;
//mod opff;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("simon");
    let whip = &mut smashline::Agent::new("simon_whip");
    acmd::install(agent,whip);
    status::install(agent);
    //agent::install(agent);
    //opff::install(agent);
    agent.install();
    whip.install();
}