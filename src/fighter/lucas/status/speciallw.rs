use crate::imports::imports_agent::*;

pub unsafe extern "C" fn counter_hurtbox(fighter: &mut L2CFighterCommon,active: bool) {
    if active {
        for i in 0..12 {
            let hit_status = if i==11 {*HIT_STATUS_OFF} else {*HIT_STATUS_NORMAL};
            macros::HIT_NO(fighter, i, hit_status);
        }
    }
    else{
        for i in 0..12 {
            let hit_status = if i==11 {*HIT_STATUS_NORMAL} else {*HIT_STATUS_OFF};
            macros::HIT_NO(fighter, i, hit_status);
        }
    }
}

pub unsafe extern "C" fn  special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    AbsorberModule::clear_all(fighter.module_accessor);
    return 0.into();
}

pub unsafe extern "C" fn special_lw_hold_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    counter_hurtbox(fighter,true);
    let original: L2CValue = smashline::original_status(Init, fighter, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD)(fighter);

    let eff = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_EFFECT_HANDLE) as u32;
    EffectModule::kill(fighter.module_accessor, eff, false, false);
    SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_lucas_special_l01"), 0);
    //SoundModule::play_se(fighter.module_accessor, Hash40::new("se_lucas_smash_l04"), true, false, false, false, app::enSEType(0));
    WorkModule::set_int(fighter.module_accessor, 0,*FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_EFFECT_HANDLE); 

    original
}


pub unsafe extern "C" fn  special_lw_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //WorkModule::set_int(fighter.module_accessor,300,*FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_TIME);
    return smashline::original_status(Pre, fighter, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD)(fighter);
}

pub unsafe extern "C" fn  special_lw_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let wait_mtrans_kind = if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {*SITUATION_KIND_AIR} else {*SITUATION_KIND_GROUND};
    WorkModule::set_int(fighter.module_accessor,wait_mtrans_kind,*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND);
    DamageModule::set_damage_lock(fighter.module_accessor,true);
    WorkModule::set_int(fighter.module_accessor,30,*FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_TIME);

    fighter.sub_change_motion_by_situation(Hash40::new("special_lw_hold").into(), Hash40::new("special_air_lw_hold").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());

    fighter.main_shift(special_lw_hold_main_loop)
}

unsafe extern "C" fn special_lw_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    AbsorberModule::clear_all(fighter.module_accessor);
    DamageModule::set_damage_lock(fighter.module_accessor,true);

    let mut wait_mtrans_kind = WorkModule::get_int(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND);
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::situation_kind(fighter.module_accessor) == wait_mtrans_kind {
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_hold").into(), Hash40::new("special_air_lw_hold").into(), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_LUCAS_AIR_STOP_X_NORMAL_MAX_SPECIAL_LW.into());
        wait_mtrans_kind = if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {*SITUATION_KIND_AIR} else {*SITUATION_KIND_GROUND};
        WorkModule::set_int(fighter.module_accessor,wait_mtrans_kind,*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND);
    }
    
    let status_frame = fighter.global_table[STATUS_FRAME].get_f32();
    //if WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_TIME, 0) {
    if status_frame >= 30.0 {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, false);
        return 1.into();
    }

    let reflect_size = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("phy_magnet_size"));
    let reflect_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("phy_magnet_offset_y"));
    let shield_pos = &Vector3f{x: 0.0, y:reflect_y-4.0, z:0.0};

    let init_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("time"));
    let frame = WorkModule::get_int(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_TIME);
    //let ratio = frame as f32 / init_frame as f32;
    let ratio = 1.0;
    let vector_ratio = Vector3f{x: ratio, y: ratio, z:ratio};
    let effect =  WorkModule::get_int(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_EFFECT_HANDLE) as u32;
    EffectModule::set_scale(fighter.module_accessor,effect,&vector_ratio);
    ModelModule::set_joint_scale(fighter.module_accessor, smash::phx::Hash40::new("throw"), &vector_ratio);  
    ModelModule::set_joint_translate(fighter.module_accessor, smash::phx::Hash40::new("throw"), shield_pos,false,false); 

    0.into()
}

pub unsafe extern "C" fn  special_lw_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_FLAG_BUTTON_RELEASE);
    //DamageModule::set_damage_mul(fighter.module_accessor,0.0);
    return 0.into();
}

pub unsafe extern "C" fn  special_lw_hold_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    SoundModule::stop_all_sound(fighter.module_accessor);
    counter_hurtbox(fighter,false);
    let table = param2.get_table() as *mut smash_rs::lib::L2CTable;

    let damage = get_table_value(table, "damage_").try_float().unwrap();
    let damage_add = get_table_value(table, "damage_add_").try_float().unwrap();
    let damage_add_reaction = get_table_value(table, "damage_add_reaction_").try_float().unwrap();
    println!(
        "Damage: {}\nDamageAdd: {}\nDamageAddReac: {}",
        damage,
        damage_add,
        damage_add_reaction
    );
    let attacker_id = get_table_value(table, "0x10d723eebb").try_integer().unwrap() as u32;
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let attacker_battle_object = &mut *get_battle_object_from_id(attacker_id);
    if attacker_battle_object.is_fighter() && !attacker_battle_object.is_weapon() {
        StopModule::set_other_stop(attacker_boma, 24, StopOtherKind(0));
    }
    WorkModule::set_float(fighter.module_accessor, damage_add, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);

    macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
    let sfx_handle = SoundModule::play_se(fighter.module_accessor,Hash40::new("se_lucas_attackair_l05"),
        true,false,false,false,app::enSEType(0),
    );
    SoundModule::set_se_vol(fighter.module_accessor, sfx_handle as i32, 3.0, 0);
    EffectModule::kill_kind(fighter.module_accessor,Hash40::new("lucas_psi_atk_lw"),false,false);
    let pos = *PostureModule::pos(fighter.module_accessor);
    let eff = EffectModule::req(
        fighter.module_accessor,
        Hash40::new("lucas_psi_atk_lw"),
        &Vector3f{x:pos.x,y:pos.y+4.5,z:pos.z+1.0},
        &Vector3f{x:-90.0,y:0.0,z:0.0},
        1.5,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rate(fighter.module_accessor,eff,0.45);

    StopModule::set_hit_stop_frame(fighter.module_accessor,5,true);
    DamageModule::end_damage_info_log(fighter.module_accessor);
    DamageModule::set_damage_lock(fighter.module_accessor,false);
    KineticModule::clear_speed_all(fighter.module_accessor);
    StatusModule::change_status_force(fighter.module_accessor, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
    1.into()
}

pub unsafe extern "C" fn  special_lw_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    counter_hurtbox(fighter,true);
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    DamageModule::set_damage_mul(fighter.module_accessor,1.0);
    ModelModule::set_joint_scale(fighter.module_accessor, smash::phx::Hash40::new("throw"), &VECTOR_ONE);
    return smashline::original_status(End, fighter, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD)(fighter);
}
pub unsafe extern "C" fn  special_lw_hold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    counter_hurtbox(fighter,false);
    0.into()
}

pub unsafe extern "C" fn  special_lw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw_hit").into(), Hash40::new("special_air_lw_hit").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());

    let damage = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE)*1.3;
    println!("Counter Power: {damage}");
    AttackModule::set_power_mul(fighter.module_accessor, damage);

    fighter.main_shift(special_lw_hit_main_loop)
}

unsafe extern "C" fn special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    AbsorberModule::clear_all(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }


    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }

    0.into()
}
pub fn install(fighter: &mut smashline::Agent) {
    fighter.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exec);

    fighter.status(Init, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD, special_lw_hold_init);
    fighter.status(Main, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD, special_lw_hold_main);
    fighter.status(Exec, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD, special_lw_hold_exec);
    fighter.status(CheckDamage, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD, special_lw_hold_damage);
    fighter.status(End, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD, special_lw_hold_end);
    fighter.status(Exit, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD, special_lw_hold_exit);

    fighter.status(Main, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_main);
}