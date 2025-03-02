mod aerials;
mod smashes;
mod specials;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    smashes::install(agent);
    specials::install(agent);
}