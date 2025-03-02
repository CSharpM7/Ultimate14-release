//mod specials;
mod specialhi;


pub fn install(agent: &mut smashline::Agent) {
    //specials::install(agent);
    specialhi::install(agent);
}