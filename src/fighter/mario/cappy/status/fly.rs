use crate::imports::imports_status::*;
use super::*;
pub const NEXT_STATUS: i32 = mario_cappy::CAPTOSS_STATUS_KIND_HOLD;

pub unsafe extern "C" fn captoss_fly_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    LinkModule::remove_model_constraint(weapon.module_accessor,true);

    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    let owner_boma = get_owner_boma(weapon);
    GroundModule::set_rhombus_offset(weapon.module_accessor, &Vector2f::new(0.0, 2.0));

    let stick_y = ControlModule::get_stick_y(owner_boma);
    let stick_y_mul = if stick_y.abs() < 0.2 {0.0} else {WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("attack_mul_min"))};
    let angle = (stick_y*(std::f32::consts::PI/2.0))*stick_y_mul;

    let lr = PostureModule::lr(owner_boma);
    PostureModule::set_lr(weapon.module_accessor, lr);
    let scale = PostureModule::scale(owner_boma);
    PostureModule::set_scale(weapon.module_accessor, scale*1.375,false);

    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("brake_x"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));

    let speed_x = speed_max*angle.cos()*lr;
    let speed_y = speed_max*angle.sin();
    //println!("SpeedX: {speed_x} SpeedY: {speed_y}");
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    let max_x = speed_max*angle.cos();
    let max_y = speed_max*angle.sin();
    //println!("MaxX: {max_x} MaxY: {max_y}");
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        max_x,
        max_y
    );
    let accel_x = -accel*lr*angle.cos();
    let accel_y = -accel*angle.sin();
    //println!("Accelx: {accel_x} AccelY: {accel_y}");
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        accel_x,
        accel_y
    );

    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

pub unsafe extern "C" fn captoss_fly_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_ROT_Y_LR | *FS_SUCCEEDS_KEEP_EFFECT,
    );
    0.into()
}

pub unsafe extern "C" fn captoss_fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_NORMAL);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
    if StopModule::is_stop(weapon.module_accessor){
        captoss_ground_check(weapon);
    }
    //Prevent Cappy from spawning if he starts too close to a wall
    let owner_boma = get_owner_boma(weapon);
    let current_pos = *PostureModule::pos(weapon.module_accessor);
    let owner_pos = *PostureModule::pos(owner_boma);
    let dist = 9.0;
    if GroundModule::ray_check(owner_boma, &Vector2f{ x: owner_pos.x, y: current_pos.y}, &Vector2f{ x: dist, y: 0.0},false) == 1
    {
        StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos.x, y: current_pos.y, z: current_pos.z});
    }

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(captoss_fly_main_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(captoss_fly_main_status_loop as *const () as _))
}
unsafe extern "C" fn captoss_fly_main_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    0.into()
}

unsafe extern "C" fn captoss_fly_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));
    let speed_min_mul = speed_min*1.0;

    /*if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP)
    {
        StatusModule::change_status_force(weapon.module_accessor, mario_cappy::CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }*/
    if sum_speed_len <= speed_min_mul {
        if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK) {
            StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
            return 0.into();
        }
    }
    else{
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK);
    }
    if captoss_reflect_check(weapon) {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 1.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::reflect_accel(weapon.module_accessor,  &Vector3f{x: 1.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        StatusModule::change_status_force(weapon.module_accessor, mario_cappy::CAPTOSS_STATUS_KIND_SWALLOWED, false);
        return 0.into();
    }
    if captoss_swallowed_check(weapon) {
        return 0.into();
    }
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    if captoss_attacked_check(weapon) {
        KineticModule::mul_speed(weapon.module_accessor, &Vector3f { x: 0.5, y: 0.0, z: 1.0 }, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        StatusModule::change_status_force(weapon.module_accessor, mario_cappy::CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    {
        StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
    }
    else if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_DOWN) as u32)
    {
        LANDING_EFFECT(weapon, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        let bound_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("floor_bound_x_mul"));
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 0.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::reflect_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    }

    0.into()
}
pub unsafe extern "C" fn captoss_fly_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub unsafe extern "C" fn captoss_fly_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {  
    agent.status(Init, mario_cappy::CAPTOSS_STATUS_KIND_FLY, captoss_fly_init);
    agent.status(Pre, mario_cappy::CAPTOSS_STATUS_KIND_FLY, captoss_fly_pre);
    agent.status(Main, mario_cappy::CAPTOSS_STATUS_KIND_FLY, captoss_fly_main);
    agent.status(Exec, mario_cappy::CAPTOSS_STATUS_KIND_FLY, captoss_fly_exec);
    agent.status(End, mario_cappy::CAPTOSS_STATUS_KIND_FLY, captoss_fly_end);
}
