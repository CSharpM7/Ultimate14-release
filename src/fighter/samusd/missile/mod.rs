mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("samusd_missile");
    acmd::install(agent);
    //status::install(agent);
    agent.install();
}
