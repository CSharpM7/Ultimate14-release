mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucina_bowarrow");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}
