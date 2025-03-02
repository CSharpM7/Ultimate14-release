mod jabs;
mod tilts;
mod aerials;
mod specials;
mod throws;

pub fn install(agent: &mut smashline::Agent) {
    jabs::install(agent);
    tilts::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
}