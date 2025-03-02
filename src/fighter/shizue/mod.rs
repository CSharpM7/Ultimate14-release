mod acmd;
mod status;
mod agent_init;
//mod opff;

mod clayrocket;
mod pot;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("shizue");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    //opff::install(agent);
    agent.install();

    clayrocket::install();
    pot::install();
}