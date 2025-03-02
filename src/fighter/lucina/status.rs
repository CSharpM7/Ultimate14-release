use crate::imports::imports_agent::*;

pub mod specials;

unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    1.into()
}

pub fn install(agent: &mut smashline::Agent) {
    specials::install(agent);
    agent.status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end);
}