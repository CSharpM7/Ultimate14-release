use crate::imports::imports_agent::*;

pub unsafe extern "C" fn art_cancel(is_flame: bool,fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,status_kind:i32) {
    if /*[*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_DASH,
    *FIGHTER_STATUS_KIND_ATTACK_S3,*FIGHTER_STATUS_KIND_ATTACK_HI3,*FIGHTER_STATUS_KIND_ATTACK_LW3,
    *FIGHTER_STATUS_KIND_ATTACK_S4,*FIGHTER_STATUS_KIND_ATTACK_HI4,*FIGHTER_STATUS_KIND_ATTACK_LW4,
    *FIGHTER_STATUS_KIND_ATTACK_AIR,
    *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END,*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2,*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3,*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX,
    *FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_ROY_STATUS_KIND_SPECIAL_S2,*FIGHTER_ROY_STATUS_KIND_SPECIAL_S3,*FIGHTER_ROY_STATUS_KIND_SPECIAL_S4].contains(&status_kind) 
    */
    VarModule::is_flag(fighter.battle_object, element::instance::flag::SPECIAL_CANCEL)
    //&& !VarModule::is_flag(fighter.battle_object, element::instance::flag::SPECIAL_CANCEL_DISABLED)
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        let mut change=false;

        let special_n = if is_flame {
            [*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_ATTACK].contains(&status_kind)
        }
        else {
            [*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END].contains(&status_kind)
        };
        let special_s = if is_flame {
            false
        }
        else {
            [*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD].contains(&status_kind)
        };
        println!("Cancel? S: {special_s} N: {special_n}");
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        if (0 != (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N)) && !special_n {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,true);
            change=true;
        }
        if (0 != (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S)) && !special_s  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,true);
            change=true;
        }
        if (0 != (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI))  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,true);
            change=true;
        }
        if (0 != (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW))  {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,true);
            change=true;
        }
        if change {
            VarModule::on_flag(fighter.battle_object, element::instance::flag::SPECIAL_CANCEL_DISABLED);

            let sfx_handle = SoundModule::play_se(boma, Hash40::new("se_eflame_special_s01_flick"), true, false, false, false, app::enSEType(0));
            let lr = PostureModule::lr(boma);
            //SoundModule::set_se_vol(boma sfx_handle as i32, 0.5, 0);

            let flash_y_offset = WorkModule::get_param_float(boma, hash40("height"), 0)*1.0;
            let eff_pos = *PostureModule::pos(boma);
            let eff_hash = if is_flame {Hash40::new("eflame_promrevolt_flash")} else {Hash40::new("sys_smash_flash")};
            let eff = EffectModule::req(boma, eff_hash, &Vector3f{x: eff_pos.x-(0.75*lr), y: eff_pos.y+flash_y_offset, z: eff_pos.z}, &Vector3f{x: 0.0, y: -90.0*(lr-1.0), z: 0.0}, 1.1, 0,-1,false,0) as u32;
            EffectModule::set_rate(boma, eff, 0.75);
        }
    }
}
pub unsafe extern "C" fn art_cancel_debuff(is_flame: bool,fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,status_kind:i32) {
    let eflame_art = is_flame && [*FIGHTER_STATUS_KIND_SPECIAL_N,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_HOLD,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_ATTACK,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_END,
    *FIGHTER_STATUS_KIND_SPECIAL_S,
    *FIGHTER_STATUS_KIND_SPECIAL_HI,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_JUMP,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_LOOP,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END,
    *FIGHTER_STATUS_KIND_SPECIAL_LW,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_STANDBY,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT,
    ].contains(&status_kind);
    let elight_art = !is_flame && [*FIGHTER_STATUS_KIND_SPECIAL_N,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_HOLD,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END,
    *FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END,
    *FIGHTER_STATUS_KIND_SPECIAL_HI,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2,
    *FIGHTER_STATUS_KIND_SPECIAL_LW,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_STANDBY,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT,
    ].contains(&status_kind);

    if (eflame_art || elight_art) {
        if VarModule::is_flag(fighter.battle_object, element::instance::flag::SPECIAL_CANCEL_DISABLED) {
            AttackModule::set_power_mul_status(boma, 0.8);
        }
    }
}
pub unsafe extern "C" fn quick_switch(is_flame: bool,fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,status_kind:i32) {
    if [*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_DASH,
    *FIGHTER_STATUS_KIND_ATTACK_S3,*FIGHTER_STATUS_KIND_ATTACK_HI3,*FIGHTER_STATUS_KIND_ATTACK_LW3,
    *FIGHTER_STATUS_KIND_ATTACK_S4,*FIGHTER_STATUS_KIND_ATTACK_HI4,*FIGHTER_STATUS_KIND_ATTACK_LW4,
    *FIGHTER_STATUS_KIND_ATTACK_AIR,
    *FIGHTER_STATUS_KIND_THROW].contains(&status_kind) {
        let mut can_swap = VarModule::is_flag(fighter.battle_object, element::instance::flag::SWAP_DISABLE);
        if !can_swap {
            let has_attack = AttackModule::is_attack(boma,0,false);
            if has_attack {
                can_swap=true;
                VarModule::on_flag(fighter.battle_object, element::instance::flag::SWAP_DISABLE);
            }
        }
        if can_swap {
            let cat1 = ControlModule::get_command_flag_cat(boma, 0);
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) 
            && (0 != (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW)) {
                println!("Req cancel");
                let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
                let frame = MotionModule::frame(boma);
                if frame >= cancel_frame-15.0 {
                    println!("In frames");
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,true);
                }
            }
        }
    }
}

pub unsafe extern "C" fn aegis_frame(fighter: &mut L2CFighterCommon) {
    let boma = &mut *fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let is_flame = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_EFLAME;
    art_cancel(is_flame,fighter,boma,status_kind);
    art_cancel_debuff(is_flame,fighter,boma,status_kind);
    quick_switch(is_flame,fighter,boma,status_kind);
}

pub fn install() {
    Agent::new("eflame")
        .on_line(Main, aegis_frame)
        .install();
    Agent::new("elight")
        .on_line(Main, aegis_frame)
        .install();
}