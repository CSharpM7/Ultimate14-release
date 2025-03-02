use crate::imports::imports_agent::*;

pub unsafe fn spawn_homing_effect(fighter: &mut L2CFighterCommon) {
    let target = WorkModule::get_int(fighter.module_accessor, toonlink::SPECIAL_S_TARGET_ID) as u32;
    if !sv_battle_object::is_active(target) {
        return;
    }
    let target_boma = sv_battle_object::module_accessor(target);
    let target_pos = *PostureModule::pos(target_boma);
    let mut eff_offset = Vector3f::zero();
    ModelModule::joint_global_offset_from_top(target_boma, Hash40{hash: hash40("rot")}, &mut eff_offset);
    let eff_pos = Vector3f{x: target_pos.x, y: target_pos.y + eff_offset.y, z: target_pos.z};
    
    let eff_current = WorkModule::get_int(fighter.module_accessor,toonlink::SPECIAL_S_HOMING_EFF_ID) as u32;

    if EffectModule::is_exist_effect(fighter.module_accessor, eff_current) {
        EffectModule::set_pos(fighter.module_accessor, eff_current, &eff_pos);
    }
    else {
        let eff = EffectModule::req(
            fighter.module_accessor,
            Hash40::new("sys_attack_impact"),
            &eff_pos,
            &VECTOR_ZERO,
            1.5,
            0, -1, false, 0,
        ) as u32;
        
        EffectModule::set_rgb(target_boma,eff,0.8,1.0,0.0);
        EffectModule::set_rate(target_boma,eff,0.5);
        EffectModule::set_billboard(target_boma,eff, true);
        EffectModule::set_disable_render_offset_last(target_boma);
        let sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_toonlink_special_n04"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, sfx_handle as i32, 1.2, 0);
        
        WorkModule::set_int(fighter.module_accessor,eff as i32,toonlink::SPECIAL_S_HOMING_EFF_ID);
    }
}


pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    WorkModule::set_int(fighter.module_accessor,OBJECT_ID_NULL as i32,toonlink::SPECIAL_S_TARGET_ID);
    WorkModule::set_int(fighter.module_accessor,OBJECT_ID_NULL as i32,toonlink::SPECIAL_S_TARGET_ID_PREV);
    WorkModule::on_flag(fighter.module_accessor,toonlink::SPECIAL_S_FLAG_CAN_SEARCH);
    original
}

/* 
Some of this should be in main but whatever
*/
pub unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = &mut *fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    
    
    if WorkModule::is_flag(boma,toonlink::SPECIAL_S_FLAG_CAN_SEARCH)  {
        if WorkModule::is_flag(boma,toonlink::SPECIAL_S_FLAG_SEARCH) {
            if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::off_flag(boma,toonlink::SPECIAL_S_FLAG_SEARCH);
                WorkModule::off_flag(boma,toonlink::SPECIAL_S_FLAG_CAN_SEARCH);
                SearchModule::clear(fighter.module_accessor, 0);
                MotionModule::set_rate(boma,1.0);
                println!("Fast Forward");
            }
            else if SearchModule::is_search(fighter.module_accessor, 0) {
                SearchModule::set_size(fighter.module_accessor, 0, 35.0);
            }
        }
    }
    
    if !WorkModule::is_flag(fighter.module_accessor, toonlink::SPECIAL_S_FLAG_SEARCH) 
    && SearchModule::is_search(fighter.module_accessor, 0) {
        //SearchModule::clear(fighter.module_accessor, 0);
        //spawn_homing_effect(fighter);
    }
    let target_curr = WorkModule::get_int(fighter.module_accessor, toonlink::SPECIAL_S_TARGET_ID) as u32;
    let target_prev = WorkModule::get_int(fighter.module_accessor, toonlink::SPECIAL_S_TARGET_ID_PREV) as u32;
    if target_curr != target_prev {
        spawn_homing_effect(fighter);
        WorkModule::set_int(fighter.module_accessor,target_curr as i32,toonlink::SPECIAL_S_TARGET_ID_PREV);
    }

    0.into()
}

pub unsafe extern "C" fn  special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Main, fighter, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END)(fighter);
    let charge = WorkModule::get_float(fighter.module_accessor,*FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let max_charge = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32;
    if charge >= max_charge {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
    }
    original
}
unsafe extern "C" fn special_hi_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge = WorkModule::get_float(fighter.module_accessor,*FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let max_charge = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32;
    if charge >= max_charge {
        if AttackModule::is_attack(fighter.module_accessor,0,false) 
        && !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) 
        && MotionModule::frame(fighter.module_accessor) < 40.0 {
            let max_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_speed_max"), 0);
            let max_speed_mul = 0.8;
            let accel: f32 = 0.09;//WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_add"), 0)+WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_mul"), 0);
            let accel_mul = 1.0;
            let lr = PostureModule::lr(fighter.module_accessor);
            let stick_x = ControlModule::get_stick_x(fighter.module_accessor)*lr;
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_MOVE_GROUND,
                max_speed*max_speed_mul*0.5*stick_x,
                0.0,
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                max_speed*max_speed_mul,
                0.0
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                max_speed*max_speed_mul,
                0.0
            );
            sv_kinetic_energy!(
                controller_set_accel_x_add,
                fighter,
                accel*accel_mul
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        else if !AttackModule::is_attack(fighter.module_accessor,0,false) 
        && KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    0.into()
}
pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec);
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
    agent.status(Exec, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_exec);
}