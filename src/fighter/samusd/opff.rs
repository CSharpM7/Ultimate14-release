use crate::imports::imports_agent::*;

pub unsafe extern "C" fn specials_force_homing(boma: &mut BattleObjectModuleAccessor, status_kind: i32,situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if (&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
    ]).contains(&status_kind) 
    && frame < 6.0 {
        StatusModule::change_status_request_from_script(boma,
            if situation_kind == *SITUATION_KIND_AIR{*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A} else {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G},
        false);
    }
}



pub unsafe extern "C" fn samusd_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        let boma = &mut *info.boma;
        specials_force_homing(boma,info.status_kind,info.situation_kind,info.frame);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main,  samusd_frame);
}