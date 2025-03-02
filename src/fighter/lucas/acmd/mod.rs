mod aerials;
mod specials;
//mod smashes;
//mod throws;
//mod ground;
mod tilts;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    specials::install(agent);
    //smashes::install(agent);
    //throws::install(agent);
    //ground::install(agent);
    tilts::install(agent);
}