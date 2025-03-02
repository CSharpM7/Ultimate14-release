mod acmd;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("pit");
    //acmd::install(agent);
    agent.install();
}