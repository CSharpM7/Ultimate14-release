mod specialn;
//mod specialn_kirby;

mod specials;
//mod specialhi;
//mod speciallw;


pub fn install(agent: &mut smashline::Agent) {
    specialn::install(agent);
    //specialn_kirby::install(agent);
    specials::install(agent);
    //specialhi::install(agent);
    //speciallw::install(agent);
}