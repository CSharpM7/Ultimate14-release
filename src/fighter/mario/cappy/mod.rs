mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario_captoss");
    acmd::install(agent);
    status::install(agent);

    agent.install();
}