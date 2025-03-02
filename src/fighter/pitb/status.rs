mod specialhi;
mod speciallw;

pub fn install(agent: &mut smashline::Agent) {
    speciallw::install(agent);
    specialhi::install(agent);
}