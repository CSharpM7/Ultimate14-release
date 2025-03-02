mod specials;
mod specialhi;
mod attackair;
mod attachwall;
mod guard;

pub fn install(agent: &mut smashline::Agent) {
    attackair::install(agent);
    specials::install(agent);
    specialhi::install(agent);

    attachwall::install(agent);
    guard::install(agent);
}