use crate::imports::imports_agent::*;

pub unsafe extern "C" fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_SAMUSD_SPECIAL_N} else {*FIGHTER_STATUS_KIND_SPECIAL_N};
    let original: L2CValue = smashline::original_status(Init, fighter, original_status)(fighter);

    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    if charge >= charge_max {
        if is_kirby {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_F);
        }
        else {
            let charge_eff = VarModule::get_int(fighter.battle_object, samusd::instance::int::CHARGE_EFF) as u32;
            EffectModule::kill(fighter.module_accessor, charge_eff, false, false);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("samusd_cshot_bullet_sub_a"), false, false);
            VarModule::set_int(fighter.battle_object, samusd::instance::int::CHARGE_EFF,0);

            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F);
        }
    }

    original
}

pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_SAMUSD_SPECIAL_N} else {*FIGHTER_STATUS_KIND_SPECIAL_N};
    let original: L2CValue = smashline::original_status(Main, fighter, original_status)(fighter);

    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    if charge >= charge_max {
        //fighter.sub_change_motion_by_situation(Hash40::new("special_n_f_max").into(), Hash40::new("special_air_n_f_max").into(),false.into());
        MotionModule::set_rate(fighter.module_accessor, 4.0);
    }

    original
}
pub unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    if charge >= charge_max {
        /* 
        let frame = MotionModule::frame(fighter.module_accessor);
        if frame > 6.0 {
            StatusModule::change_status_request(fighter.module_accessor,samus::STATUS_KIND_SPECIAL_N_F2,false);
        }*/
    }

    0.into()
}

pub unsafe extern "C" fn special_n_h_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut is_dark = utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_SAMUSD;
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if is_kirby {
        is_dark = WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_SAMUSD;
    }
    if !is_dark {
        return smashline::original_status(Exit, fighter, *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_H)(fighter);
    }

    EFFECT_OFF_KIND(fighter,Hash40::new("samusd_cshot_bullet_sub_a"),false,false);
    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_SAMUSD_SPECIAL_N_H} else {*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H};
    let original: L2CValue = smashline::original_status(Exit, fighter, original_status)(fighter);
    if is_kirby { 
        PLAY_SE(fighter,Hash40::new("se_samusd_escape_ex"));
        return original;
    }
    //I could translate this shit but nah
    EFFECT_OFF_KIND(fighter,Hash40::new("samusd_cshot_max"),false,false);
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    if charge >= charge_max {  
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_cshot_bullet_sub_a"), Hash40::new("handl"), &Vector3f{x: 1.0, y: 0.0, z: 0.0}, &Vector3f{x: -91.2728, y: -1.7974, z: 176.373}, 0.25, true, 0, 0, 0, 0,0, false,false) as i32;
        
        VarModule::set_int(fighter.battle_object, samusd::instance::int::CHARGE_EFF,handle);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);

        PLAY_SE(fighter,Hash40::new("se_samusd_escape_ex"));
    }
    
    original
}
pub unsafe extern "C" fn special_n_f2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut is_dark = utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_SAMUSD;
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if is_kirby {
        is_dark = WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_SAMUSD;
    }
    if !is_dark {
        return smashline::original_status(Exec, fighter, *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_F)(fighter);
    }

    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    let is_max = charge >= charge_max;
    if is_max {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_attack(fighter.module_accessor,0,false) {
            let screen = EffectModule::req_screen(fighter.module_accessor,Hash40::new("bg_criticalhit"),false,false,false);
            EffectModule::set_billboard(fighter.module_accessor, screen as u32, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
            EFFECT_GLOBAL_BACK_GROUND(fighter.lua_state_agent);

            SlowModule::set_whole(fighter.module_accessor, 8, 40);
            CAM_ZOOM_IN_arg5(fighter, 2.0, 0.0, 1.5, 0.0, 0.0);
            QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
            PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        }
        else if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_attack(fighter.module_accessor,1,false) {
            SlowModule::clear_whole(fighter.module_accessor);;
            CameraModule::reset_all(fighter.module_accessor);
            CAM_ZOOM_OUT(fighter);
            EffectModule::remove_screen(fighter.module_accessor,Hash40::new("bg_criticalhit"),-1);
        }
    }
    0.into()
}
pub unsafe extern "C" fn special_n_f2_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    SlowModule::clear_whole(fighter.module_accessor);
    CameraModule::reset_all(fighter.module_accessor);
    CAM_ZOOM_OUT(fighter);
    EffectModule::remove_screen(fighter.module_accessor,Hash40::new("bg_criticalhit"),-1);
    EFFECT_OFF_KIND(fighter,Hash40::new("bg_criticalhit"),false,false);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);

    agent.status(Exit, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, special_n_h_exit);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, special_n_f2_exec);
    agent.status(Exit, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, special_n_f2_exit);

    Agent::new("kirby")
        .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SAMUSD_SPECIAL_N, special_n_init)
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SAMUSD_SPECIAL_N, special_n_main)
        .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SAMUSD_SPECIAL_N, special_n_exec)

        .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_H, special_n_h_exit)
        .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_F, special_n_f2_exec)
        .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_F, special_n_f2_exit)
    .install();
}