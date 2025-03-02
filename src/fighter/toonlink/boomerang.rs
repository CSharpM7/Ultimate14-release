use crate::imports::imports_agent::*;

pub unsafe extern "C" fn fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0,
    );
    0.into()
}
pub unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let angle = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let original = smashline::original_status(Main, weapon, *WN_LINK_BOOMERANG_STATUS_KIND_FLY)(weapon);
    if StatusModule::prev_status_kind(weapon.module_accessor,0) != toonlink::BOOMERANG_STATUS_KIND_HOMING {
        let owner = get_owner_boma(weapon);
        let mut new_target = WorkModule::get_int(owner,toonlink::SPECIAL_S_TARGET_ID) as u32;
        WorkModule::set_int(weapon.module_accessor,new_target as i32,toonlink::BOOMERANG_TARGET_ID);

        let target = WorkModule::get_int(weapon.module_accessor, toonlink::BOOMERANG_TARGET_ID) as u32;
        if target != OBJECT_ID_NULL {
            weapon.change_status(toonlink::BOOMERANG_STATUS_KIND_HOMING.into(),false.into());
            return 1.into();
        }
    }
    else {
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        WorkModule::set_float(weapon.module_accessor, angle, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    }
    original
}
pub unsafe extern "C" fn homing_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //smashline::original_status(Init, weapon, *WN_LINK_BOOMERANG_STATUS_KIND_FLY)(weapon)
    0.into()
}
pub unsafe extern "C" fn homing_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn homing_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 120, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);

	MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    
    let target = WorkModule::get_int(weapon.module_accessor, toonlink::BOOMERANG_TARGET_ID) as u32;
    if !sv_battle_object::is_active(target) {
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(),false.into());
        return 1.into();
    }
    //spawn_homing_effect(weapon);

	weapon.fastshift(L2CValue::Ptr(homing_main_loop as *const () as _))
}
pub unsafe extern "C" fn homing_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE,0) {
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_FLY.into(),false.into());
    }
    if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_ALL){
        WorkModule::on_flag(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_FLY.into(),false.into());
        return 1.into();
    }
    if WorkModule::is_flag(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION) {
        if StopModule::is_stop(weapon.module_accessor) {
            let penetrate = WorkModule::get_param_int(weapon.module_accessor, hash40("param_boomerang"), hash40("is_penetration")) == 1; 
            if !penetrate {
                weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(),false.into());
                return 1.into();
            }
        }
        else {
            weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_HOP.into(),false.into());
            return 1.into();
        }
    }
    0.into()
}
pub unsafe extern "C" fn homing_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let target = WorkModule::get_int(weapon.module_accessor, toonlink::BOOMERANG_TARGET_ID) as u32;
    if !sv_battle_object::is_active(target) {
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(),false.into());
        return 1.into();
    }
    let target_boma = sv_battle_object::module_accessor(target);
    let lr = PostureModule::lr(weapon.module_accessor);
    // Get positions
    let pos = *PostureModule::pos(weapon.module_accessor);
    let offset_y = 0.0;
    let other_pos = *PostureModule::pos(target_boma);
    let mut other_offset = Vector3f::zero();
    ModelModule::joint_global_offset_from_top(target_boma, Hash40{hash: hash40("hip")}, &mut other_offset);
    let other_offset_y = other_offset.y+0.75;

    let mut direction_full = Vector2f{x:other_pos.x-pos.x, y: (other_pos.y+other_offset_y)-(pos.y+offset_y)};
    let direction_len = sv_math::vec2_length(direction_full.x,direction_full.y);
    if (direction_len < 2.0) { 
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(),false.into());
        return 1.into();
    }
    let direction = Vector2f{x:direction_full.x/direction_len,y:direction_full.y/direction_len};
    let arctangent = direction.y.atan2(direction.x.abs());
    let mut degree = arctangent.to_degrees();
    if degree > 180.0 {
        degree = -(360.0-degree);
    }
    let is_behind = direction.x.signum() != PostureModule::lr(weapon.module_accessor);

    
    if is_behind {
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(),false.into());
        return 1.into();
    }
    else if degree >= toonlink::BOOMERANG_HOMING_MAX || degree <= -toonlink::BOOMERANG_HOMING_MAX {
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_FLY.into(),false.into());
        return 1.into();
    }

    let degree_current = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE).to_degrees();
    let mut degree_new = lerp(degree_current,degree,toonlink::BOOMERANG_HOMING_LERP);
    let degree_min = -toonlink::BOOMERANG_HOMING_MAX;
    if degree_new > toonlink::BOOMERANG_HOMING_MAX {
        degree_new = toonlink::BOOMERANG_HOMING_MAX;
    }
    else if degree_new < degree_min {
        degree_new = degree_min;
    }

    WorkModule::set_float(weapon.module_accessor, degree_new.to_radians(), *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);

    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_max")); 

    let speed_current = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_SPEED);
    let mut accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("accel")); 
    let speed_new = (speed_current + accel).min(speed_max);
    WorkModule::set_float(weapon.module_accessor, speed_new,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_SPEED);

    //println!("Dir {},{}",direction.x,direction.y);
    let rad = degree_new.to_radians();
    let speed_x = rad.cos()*speed_current*lr;
    let speed_y = rad.sin()*speed_current;

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );  
    0.into()
}

pub fn install() {
    smashline::Agent::new("toonlink_boomerang")
        .status(Main, *WN_LINK_BOOMERANG_STATUS_KIND_FLY, fly_main)

        .status(Init, toonlink::BOOMERANG_STATUS_KIND_HOMING, homing_init)
        .status(Pre, toonlink::BOOMERANG_STATUS_KIND_HOMING, homing_pre)
        .status(Main, toonlink::BOOMERANG_STATUS_KIND_HOMING, homing_main)
        .status(Exec, toonlink::BOOMERANG_STATUS_KIND_HOMING, homing_exec)
        .status(End, toonlink::BOOMERANG_STATUS_KIND_HOMING, empty_status)
    .install();
}