use smash_rs::app::work_ids::fighter::status::dedede_final_target::ROT_Z;

use crate::imports::imports_agent::*;


pub unsafe extern "C" fn fly_init(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    
    let charge = WorkModule::get_int(owner_boma,lucina::SPECIAL_S_CHARGE) as f32;
    let charge_max = lucina::SPECIAL_S_CHARGE_MAX as f32;//WorkModule::get_int(owner_boma, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT_MAX) as f32;
    let ratio = charge/charge_max;
    WorkModule::set_float(weapon.module_accessor,ratio,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let angle: f32 = if StatusModule::situation_kind(owner_boma) == *SITUATION_KIND_GROUND {0.0} else {-25.0};

    let move_speed = lerp(2.6,10.0,ratio); //TODO: figure out how fast based on how long the charge was. 2.6-10.0
    let lr = PostureModule::lr(owner_boma);
    PostureModule::set_lr(weapon.module_accessor, lr);

    let speed_x = move_speed*angle.to_radians().cos();
    let speed_y = move_speed*angle.to_radians().sin();
    let accel_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("accel_y"));
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x*lr,
        speed_y
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0, //air friction?
        -0.054 //Grav
    );
    0.into()
}
pub unsafe extern "C" fn fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    //Life
    let life = 180; //WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("life"));;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);

    //Set Power
    let charge = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let power = lerp(6.0,12.0,charge);
    AttackModule::set_power(weapon.module_accessor,0,power,false);
    if StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    
    weapon.fastshift(L2CValue::Ptr(fly_main_status_loop as *const () as _))
}

unsafe extern "C" fn fly_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let rot_x = PostureModule::rot_x(weapon.module_accessor,0);
    WorkModule::set_float(weapon.module_accessor,rot_x,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_SHOT_ANGLE);

    //Lose speed on collision
    let has_hit_opp = AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT);
    let has_hit_ground = GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_ALL) as u32);
    if has_hit_opp {
        weapon.change_status(WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK.into(),false.into());
        return 1.into();
    }
    else if has_hit_ground {
        if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_DOWN) as u32) {
            StatusModule::set_situation_kind(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            GroundModule::attach(weapon.module_accessor, GroundTouchFlag(*GROUND_TOUCH_FLAG_ALL));
        }
        weapon.change_status(WN_LINK_BOWARROW_STATUS_KIND_STICK.into(),false.into());
        return 1.into();
    }

    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub unsafe extern "C" fn stuck_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let rand = sv_math::randf(0x66933a7e6,1.0);
    //4.0,2.0,0.0
    WorkModule::set_float(weapon.module_accessor, rand, *WN_LINK_BOWARROW_STATUS_STICK_WORK_FLOAT_RAND); 

    let rot_x = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_SHOT_ANGLE);
    let rot_y = PostureModule::rot_y(weapon.module_accessor,0);
    let rot_z = PostureModule::rot_z(weapon.module_accessor,0);
    let lr = PostureModule::lr(weapon.module_accessor);
    let rot_x_add = if StatusModule::situation_kind(weapon.module_accessor) == *SITUATION_KIND_GROUND {1.0} else {0.0};
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: rot_x+(rot_x_add*lr), y: 90.0*lr, z: 0.0},0);
    let life = 80; //WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("first_stick_life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }

    AttackModule::clear_all(weapon.module_accessor);
    GroundModule::attach(weapon.module_accessor, GroundTouchFlag(*GROUND_TOUCH_FLAG_ALL));
    GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stick"), 0.0, 1.0, false, 0.0, false, false);

    if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_DOWN) as u32) {
        let add_y = -1.0*rot_x.to_radians().cos();
        PostureModule::add_pos(weapon.module_accessor, &Vector3f{x: lr*rot_x.to_radians().sin(), y: add_y, z: 0.0});
    }

    
    weapon.fastshift(L2CValue::Ptr(stuck_main_status_loop as *const () as _)) 
}
unsafe extern "C" fn stuck_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    //if !GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_DOWN) as u32) {
        let rand = sv_math::randf(0x66933a7e6,1.0);
        WorkModule::set_float(weapon.module_accessor, rand, *WN_LINK_BOWARROW_STATUS_STICK_WORK_FLOAT_RAND);
    //}

    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {/*
        let pos = *PostureModule::pos(weapon.module_accessor);
        EffectModule::req(
            weapon.module_accessor,
            Hash40::new("sys_erace_smoke"),
            &Vector3f{x:pos.x,y:pos.y+0.0,z:pos.z},
            &Vector3f{x:0.0,y:0.0,z:0.0},
            0.75,
            0,
            -1,
            false,
            0
        ); */
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *WN_LINK_BOWARROW_STATUS_KIND_FLY, fly_init);
    agent.status(Main, *WN_LINK_BOWARROW_STATUS_KIND_FLY, fly_main);
    agent.status(Main, *WN_LINK_BOWARROW_STATUS_KIND_STICK, stuck_main);
}
