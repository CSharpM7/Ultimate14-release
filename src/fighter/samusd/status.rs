mod specialhi;
mod specialn;

pub fn install(agent: &mut smashline::Agent) {
    specialn::install(agent);
    specialhi::install(agent);
}