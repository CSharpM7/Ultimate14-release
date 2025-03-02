mod aerials;
mod specials;
//mod smashes;
mod throws;
mod tilts;
//mod ground;

//mod movement;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    specials::install(agent);
    //smashes::install(agent);
    throws::install(agent);
    tilts::install(agent);
    //ground::install(agent);
    //movement::install(agent);
    agent.install();
}