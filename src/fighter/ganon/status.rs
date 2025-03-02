mod specialhi;
mod specials;

pub fn install(agent: &mut smashline::Agent) {
    specials::install(agent);
    specialhi::install(agent);
}