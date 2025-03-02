mod specialn;
mod specials;
mod specialhi;

pub fn install(agent: &mut smashline::Agent) {
    specialn::install(agent);
    specials::install(agent);
    specialhi::install(agent);
}