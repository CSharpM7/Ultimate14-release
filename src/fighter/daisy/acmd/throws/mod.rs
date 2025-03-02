mod throwf;
mod throwb;
mod throwhi;
mod throwlw;
mod catch;

pub fn install(agent: &mut smashline::Agent) {
    catch::install(agent);
    throwf::install(agent);
    throwb::install(agent);
    throwhi::install(agent);
    throwlw::install(agent);
}