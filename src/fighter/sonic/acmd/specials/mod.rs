mod special_homing;
mod special_tornado;
mod specialhi;
mod speciallw;

pub fn install(agent: &mut smashline::Agent) {
    special_homing::install(agent);
    special_tornado::install(agent);
    specialhi::install(agent);
    speciallw::install(agent);
}