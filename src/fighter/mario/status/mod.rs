mod capjump;
mod capcatch;
mod capdive;
mod specials;
mod appeal;
mod wait;

pub fn install(agent: &mut smashline::Agent) {
    capjump::install(agent);
    capdive::install(agent);
    capcatch::install(agent);
    specials::install(agent);
    appeal::install(agent);
    wait::install(agent);
}