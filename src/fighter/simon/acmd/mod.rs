mod ground;
mod tilts;
mod aerials;

pub fn install(agent: &mut smashline::Agent,whip: &mut smashline::Agent) {
    ground::install(agent);
    tilts::install(agent,whip);
    aerials::install(agent,whip);
}
