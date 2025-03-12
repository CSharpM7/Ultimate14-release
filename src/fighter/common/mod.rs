pub mod functions;
mod status;
pub mod opff;
pub mod vtables;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
    functions::install();
    //vtables::install();
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("fighter");
    status::install(agent);
    opff::install(agent);
    agent.install();
}