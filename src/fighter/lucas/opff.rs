use crate::imports::imports_agent::*;

pub unsafe extern "C" fn counter_hurtbox(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind != *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD{
        //macros::HIT_NO(fighter, 11, *HIT_STATUS_NORMAL);
        /*
        for i in 0..12 {
            let hit_status = if i==11 {*HIT_STATUS_OFF} else {*HIT_STATUS_NORMAL};
            macros::HIT_NO(fighter, i, hit_status);
        } */
    }
    else{
        /*
        for i in 0..12 {
            let hit_status = if i==11 {*HIT_STATUS_NORMAL} else {*HIT_STATUS_OFF};
            macros::HIT_NO(fighter, i, hit_status);
        } */
        //macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
    }
}

pub unsafe extern "C" fn cooldown(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let is_kirby = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let special_n_status = if is_kirby {
        [*FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N,*FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_END,
    *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE,*FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_HOLD]
    }
    else {
        [*FIGHTER_STATUS_KIND_SPECIAL_N,*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE,*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD]
    };
    if !special_n_status.contains(&status_kind) {
        WorkModule::count_down_int(boma,lucas::SPECIAL_N_COOLDOWN,0);
    }
}

pub unsafe extern "C" fn lucas_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        let boma = &mut *info.boma;
        counter_hurtbox(fighter,boma,info.status_kind);
        cooldown(boma,info.status_kind);
    }
}
pub unsafe extern "C" fn kirby_lucas_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        let boma = &mut *info.boma;

        let copy_kind = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        if copy_kind == *FIGHTER_KIND_LUCAS {
            cooldown(boma,info.status_kind);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main,  lucas_frame);
    Agent::new("kirby")
        .on_line(Main,  kirby_lucas_frame)
    .install();
}