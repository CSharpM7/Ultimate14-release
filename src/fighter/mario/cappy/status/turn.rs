use crate::imports::imports_status::*;
use super::*;

pub unsafe extern "C" fn captoss_turn_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    let speed_current = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    WorkModule::set_float(weapon.module_accessor, speed_current,*WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let is_reflected = WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL);
    if !is_reflected {
        return 0.into();
    }
    else{
        KineticModule::reflect_accel(weapon.module_accessor,  &Vector3f{x: 1.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        sv_kinetic_energy!(
            set_limit_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            99,
            99
        );
        return 0.into();
    }

    0.into()
}

pub unsafe extern "C" fn captoss_turn_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_EFFECT,
    );
    0.into()
}

pub unsafe extern "C" fn captoss_turn_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP);
    if StopModule::is_stop(weapon.module_accessor){
        captoss_ground_check(weapon);
    }
    AttackModule::clear_all(weapon.module_accessor);
    if StatusModule::prev_status_kind(weapon.module_accessor, 0) == mario_cappy::CAPTOSS_STATUS_KIND_JUMP {
        MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("turn"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::set_float(weapon.module_accessor, 2.0,*WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
    }
    else{
        MotionModule::change_motion_inherit_frame_keep_rate(weapon.module_accessor as _, Hash40::new("turn"), -1.0,1.0,0.0);
    }
    weapon.fastshift(L2CValue::Ptr(captoss_turn_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_turn_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    if StopModule::is_stop(weapon.module_accessor){
        return 0.into();
    }
    GroundModule::set_ignore_boss(weapon.module_accessor, true);
    GroundModule::set_passable_check(weapon.module_accessor, false);
    GroundModule::set_collidable(weapon.module_accessor, false);
    JostleModule::set_status(weapon.module_accessor, false);

    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR){
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL);
        //StatusModule::change_status_force(weapon.module_accessor, mario_cappy::CAPTOSS_STATUS_KIND_HOP, false);
        StatusModule::change_status_force(weapon.module_accessor, mario_cappy::CAPTOSS_STATUS_KIND_SWALLOWED, false);
        return 0.into();
    }

    0.into()
}

pub unsafe extern "C" fn captoss_turn_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    if captoss_swallowed_check(weapon) {
        return 0.into();
    }
    if captoss_attacked_check(weapon) {
        KineticModule::mul_speed(weapon.module_accessor, &Vector3f { x: 0.5, y: 0.0, z: 1.0 }, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        StatusModule::change_status_force(weapon.module_accessor, mario_cappy::CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    let is_reflected = WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL);
    let speed_current_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_current_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_current_x.abs() < 0.1 && !is_reflected {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0,
            speed_current_y
        );
    }
    if speed_current_y.abs() < 0.1 && !is_reflected {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_current_x,
            0
        );
    }

    let kinetic_speed = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
    if StatusModule::prev_status_kind(weapon.module_accessor, 0) == mario_cappy::CAPTOSS_STATUS_KIND_JUMP {
        speed_max*=2.0;
    }
    captoss_check_recapture(weapon);

    if (kinetic_speed > 0.1 && !is_reflected)
    || (kinetic_speed >= speed_max-0.1 && is_reflected) 
    && !StatusModule::prev_status_kind(weapon.module_accessor, 0) == mario_cappy::CAPTOSS_STATUS_KIND_JUMP {
        WorkModule::set_float(weapon.module_accessor, kinetic_speed,*WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
        return 0.into();
    }
    KineticModule::clear_speed_all(weapon.module_accessor);

    let turn_speed = 1.5;

    let owner = get_owner_boma(weapon);
    let owner_pos = *PostureModule::pos(owner);
    let mut owner_offset = Vector3f::zero();
    ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("hip")}, &mut owner_offset);
    let owner_offset_y = owner_offset.y;//WorkModule::get_param_float(owner, hash40("height"), 0) / 2.0;

    let pos = *PostureModule::pos(weapon.module_accessor);
    let offset_y = 1.25;

    let mut direction_full = Vector2f{x:owner_pos.x-pos.x, y: (owner_pos.y+owner_offset_y)-(pos.y+offset_y)};
    let direction_len = sv_math::vec2_length(direction_full.x,direction_full.y);
    let direction = Vector2f{x:direction_full.x/direction_len,y:direction_full.y/direction_len};
    //let direction = sv_math::vec2_normalize(direction_full.x,direction_full.y);

    let speed_current = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let mut accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("brake_x"));
    if StatusModule::prev_status_kind(weapon.module_accessor, 0) == mario_cappy::CAPTOSS_STATUS_KIND_JUMP {
        accel*=2.0;
    }
    let speed_new = (speed_current + accel).min(speed_max);
    WorkModule::set_float(weapon.module_accessor, speed_new,*WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);

    let speed_x = direction.x*speed_current;
    let speed_y = direction.y*speed_current;

    PostureModule::set_lr(weapon.module_accessor, direction_full.x.signum());
    let lr = PostureModule::lr(weapon.module_accessor);

    let mut lr_fix = 1.0;
    if owner_pos.x > pos.x && lr > 0.0 {
        lr_fix = -1.0;
    }
    else if owner_pos.x < pos.x && lr < 0.0 {
        lr_fix = -1.0;
    }
    PostureModule::add_pos(weapon.module_accessor, &Vector3f{x:speed_x,y:speed_y,z:0.0});

    if captoss_reflect_check(weapon) {
        StatusModule::change_status_force(weapon.module_accessor, mario_cappy::CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    0.into()
}

pub unsafe extern "C" fn captoss_turn_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {  
    agent.status(Init, mario_cappy::CAPTOSS_STATUS_KIND_TURN, captoss_turn_init);
    agent.status(Pre, mario_cappy::CAPTOSS_STATUS_KIND_TURN, captoss_turn_pre);
    agent.status(Main, mario_cappy::CAPTOSS_STATUS_KIND_TURN, captoss_turn_main);
    agent.status(Exec, mario_cappy::CAPTOSS_STATUS_KIND_TURN, captoss_turn_exec);
    agent.status(End, mario_cappy::CAPTOSS_STATUS_KIND_TURN, captoss_turn_end);
}
