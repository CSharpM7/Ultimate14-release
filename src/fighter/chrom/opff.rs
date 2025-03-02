use crate::imports::imports_agent::*;


pub unsafe extern "C" fn parallel_nerf(boma: &mut BattleObjectModuleAccessor,status_kind:i32) {
    if [*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_DASH,
    *FIGHTER_STATUS_KIND_ATTACK_S3,*FIGHTER_STATUS_KIND_ATTACK_HI3,*FIGHTER_STATUS_KIND_ATTACK_LW3,
    *FIGHTER_STATUS_KIND_ATTACK_S4,*FIGHTER_STATUS_KIND_ATTACK_HI4,*FIGHTER_STATUS_KIND_ATTACK_LW4,
    *FIGHTER_STATUS_KIND_ATTACK_AIR,
    *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END,*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2,*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3,*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX,
    *FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_ROY_STATUS_KIND_SPECIAL_S2,*FIGHTER_ROY_STATUS_KIND_SPECIAL_S3,*FIGHTER_ROY_STATUS_KIND_SPECIAL_S4].contains(&status_kind) {
        AttackModule::set_power_mul_status(boma, 0.9);
    }
}

pub unsafe extern "C" fn chrom_frame(fighter: &mut L2CFighterCommon) {
    let boma = &mut *fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);

    parallel_nerf(boma,status_kind);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main,  chrom_frame);
}