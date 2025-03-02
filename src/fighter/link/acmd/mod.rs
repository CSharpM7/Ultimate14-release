mod aerials;
mod specials;
mod smashes;
mod ground;
//mod throws;
//mod tilts;

mod movement;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    specials::install(agent);
    smashes::install(agent);
    ground::install(agent);
    //throws::install(agent);
    //tilts::install(agent);

    movement::install(agent);
}