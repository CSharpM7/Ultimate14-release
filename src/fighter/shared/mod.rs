pub mod samus;
pub mod villagers;
pub mod belmonts;
pub mod pits;
pub mod aegis;

pub fn install() {
    pits::install();
    samus::install();
    //villagers::install();
    belmonts::install();
    //aegis::install();
}