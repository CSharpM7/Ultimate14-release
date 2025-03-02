pub mod guard_off;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    guard_off::install(agent);
    //escape::install(agent);
    cliff::install(agent);
}