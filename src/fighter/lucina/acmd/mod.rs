mod aerials;
mod specials;
mod jabs;
//mod smashes;
mod tilts;
mod ground;
mod taunts;

pub fn install(agent: &mut smashline::Agent) {
    //aerials::install(agent);
    specials::install(agent);
    //smashes::install(agent);
    tilts::install(agent);
    ground::install(agent);
    jabs::install(agent);
    taunts::install(agent);
}