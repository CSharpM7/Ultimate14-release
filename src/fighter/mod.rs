pub mod common;
pub mod shared;
use common::functions::*;

#[cfg(not(feature = "mario"))]
mod mario;
#[cfg(not(feature = "link"))]
mod link;
#[cfg(not(feature = "samusd"))]
mod samusd;
#[cfg(not(feature = "kirby"))]
mod kirby;
#[cfg(not(feature = "peach"))]
mod peach;
#[cfg(not(feature = "daisy"))]
mod daisy;
#[cfg(not(feature = "mariod"))]
mod mariod;
#[cfg(not(feature = "lucina"))]
mod lucina;
#[cfg(not(feature = "ganon"))]
mod ganon;
#[cfg(not(feature = "chrom"))]
mod chrom;
#[cfg(not(feature = "pit"))]
mod pit;
#[cfg(not(feature = "pitb"))]
mod pitb;
#[cfg(not(feature = "lucas"))]
mod lucas;
#[cfg(not(feature = "sonic"))]
mod sonic;
#[cfg(not(feature = "toonlink"))]
mod toonlink;
#[cfg(not(feature = "simon"))]
mod simon;
#[cfg(not(feature = "richter"))]
mod richter;
#[cfg(not(feature = "shizue"))]
mod shizue;

#[no_mangle]
pub fn smashline_install() {
    println!("[smashline_14::main] Reloading...");
    println!(" ");
    install();
}

pub fn install() {
    #[cfg(not(feature = "dev"))]
    common::install();
        
    #[cfg(not(feature = "devhook"))]
    shared::install();
    
    #[cfg(not(feature = "mario"))]
    mario::install();
    #[cfg(not(feature = "link"))]
    link::install();
    #[cfg(not(feature = "samusd"))]
    samusd::install();
    #[cfg(not(feature = "kirby"))]
    kirby::install();
    #[cfg(not(feature = "peach"))]
    peach::install();
    #[cfg(not(feature = "daisy"))]
    daisy::install();
    #[cfg(not(feature = "mariod"))]
    mariod::install();
    #[cfg(not(feature = "lucina"))]
    lucina::install();
    #[cfg(not(feature = "ganon"))]
    ganon::install();
    #[cfg(not(feature = "chrom"))]
    chrom::install();
    #[cfg(not(feature = "pit"))]
    pit::install();
    #[cfg(not(feature = "pitb"))]
    pitb::install();
    #[cfg(not(feature = "lucas"))]
    lucas::install();
    #[cfg(not(feature = "sonic"))]
    sonic::install();
    #[cfg(not(feature = "toonlink"))]
    toonlink::install();
    #[cfg(not(feature = "simon"))]
    simon::install();
    #[cfg(not(feature = "richter"))]
    richter::install();
    #[cfg(not(feature = "shizue"))]
    shizue::install();
}