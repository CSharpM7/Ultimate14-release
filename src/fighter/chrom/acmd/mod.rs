mod aerials;
mod specials;
mod smashes;
//mod tilts;
mod ground;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    specials::install(agent);
    smashes::install(agent);
    //tilts::install(agent);
    ground::install(agent);
}