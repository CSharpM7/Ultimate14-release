use crate::imports::imports_agent::*;
pub const DISABLE_FRAME : i32 = 300;
pub const HP : f32 = 1.0;

const STATUS_KIND_FLY : i32 = 0x0;
const STATUS_KIND_BREAK: i32 = 0x1;

const INSTANCE_FLOAT_HP: i32 = 0x3;
const INSTANCE_FLOAT_HP_INIT: i32 = 0x4;

unsafe fn set_link(weapon: &mut L2CWeaponCommon, link: bool) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    
    if link {
        LinkModule::remove_model_constraint(owner_boma,true);
        if LinkModule::is_link(owner_boma,*LINK_NO_CONSTRAINT) {
            LinkModule::unlink(owner_boma,*LINK_NO_CONSTRAINT);
        }
        let link_created = LinkModule::link(owner_boma,*LINK_NO_CONSTRAINT,weapon.battle_object_id);
        if link_created & 1 != 0 {
            println!("Linked");
            //LinkModule::set_model_constraint_pos_ort(owner_boma,*LINK_NO_CONSTRAINT,Hash40::new_raw(0x10489b2b69),Hash40::new("rot"),
            LinkModule::set_model_constraint_pos_ort(owner_boma,*LINK_NO_CONSTRAINT,Hash40::new("bitr"),Hash40::new("rot"),
            (*CONSTRAINT_FLAG_ONE_NODE
            | *CONSTRAINT_FLAG_POSITION 
            | *CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE
            ) as u32,false);
    
            let pos_offset = Vector3f{x:0.0,y:0.0,z:0.0};
            let rot_offset = Vector3f{x:0.0,y:0.0,z:0.0};
            LinkModule::set_constraint_translate_offset(owner_boma, &pos_offset);
            LinkModule::set_constraint_rot_offset(owner_boma, &rot_offset);


        }
    }
    else if LinkModule::is_link(owner_boma,*LINK_NO_CONSTRAINT) {
        println!("Unlink");
        LinkModule::unlink(owner_boma, *LINK_NO_CONSTRAINT);
        //ShieldModule::set_status(owner_boma,0,app::ShieldStatus(*SHIELD_STATUS_NONE),*FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        ReflectorModule::set_status(owner_boma, 0, app::ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
    }
}
unsafe fn fix_shield(weapon: &mut L2CWeaponCommon) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);

    if !ReflectorModule::is_shield(owner_boma, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW) {
        println!("Needs shield");
        //ShieldModule::set_status(owner_boma,0,app::ShieldStatus(*SHIELD_STATUS_NORMAL),*FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        ReflectorModule::set_status(owner_boma, 0, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        ReflectorModule::set_life_mul(owner_boma,0.0,*FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
    }
}

pub unsafe extern "C" fn shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_founder_boma(weapon);
    let owner_kind = utility::get_kind(&mut *owner);
    let is_orb = owner_kind == *FIGHTER_KIND_PITB;
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("pit_rbit"), false);

    if !is_orb {
        return 0.into();
    }

    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let move_speed = 0.5;//WorkModule::get_param_float(owner_boma, hash40("param_orbitar"), hash40("move_speed"));
    let lr = PostureModule::lr(owner_boma);
    PostureModule::set_lr(weapon.module_accessor, lr);

    let speed_x = move_speed;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        move_speed*lr,
        0.0
    );
    
    snap_to_owner(weapon,Hash40::new("top"),Hash40::new("top"));
    PostureModule::add_pos(weapon.module_accessor, &Vector3f{x: pitb::ORBITER_SPAWN_X *lr, y: pitb::ORBITER_SPAWN_Y, z: 0.0});

    WorkModule::set_float(weapon.module_accessor, HP, INSTANCE_FLOAT_HP);
    WorkModule::set_float(weapon.module_accessor, HP, INSTANCE_FLOAT_HP_INIT);

    set_link(weapon,true);
    fix_shield(weapon);

    0.into()
}

pub unsafe extern "C" fn shoot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

pub unsafe extern "C" fn shoot_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Life
    let life = 180;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);

    //Set Reflect
    //ReflectorModule::set_status(owner_boma, *WEAPON_PALUTENA_REFLECTIONBOARD_REFLECTOR_KIND_REFLECTOR, ShieldStatus(*SHIELD_STATUS_NORMAL),0);
    //AbsorberModule::set_status_all(weapon.module_accessor, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_GAMEWATCH_ABSORBER_GROUP_SPECIAL_LW);

    let owner = get_owner_boma(weapon);
    WorkModule::set_int(owner, life + DISABLE_FRAME, *FIGHTER_PIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_NO_SHIELD_FRAME);
    
    weapon.fastshift(L2CValue::Ptr(shoot_main_status_loop as *const () as _))
}

unsafe extern "C" fn shoot_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    fix_shield(weapon);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        let eff_pos = *PostureModule::pos(weapon.module_accessor);
        EffectModule::req(weapon.module_accessor, Hash40::new("pitb_guardian_shield_end"), &Vector3f{x: eff_pos.x, y: eff_pos.y+0.0, z: eff_pos.z}, &Vector3f{x: 0.0, y: 300.0, z: 0.0}, 1.9, 0,-1,false,0) as u32;
        
        let owner = get_owner_boma(weapon);
        WorkModule::set_int(owner, DISABLE_FRAME, *FIGHTER_PIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_NO_SHIELD_FRAME);
        set_link(weapon,false);
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    //Lose speed on clank, respawn hitbox
    if !AttackModule::is_attack(weapon.module_accessor, 0, false) {
        println!("Clank");
        MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_GAME, Hash40::new("game_respawn"), -1);
        MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_SOUND, Hash40::new("sound_hit"), -1);

        KineticModule::mul_speed(weapon.module_accessor, &Vector3f { x: 0.625, y: 0.0, z: 0.0 }, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        WorkModule::add_int(weapon.module_accessor, -15,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        
        let hit_stop = StopModule::get_hit_stop_real_frame(weapon.module_accessor) as i32;
        ShakeModule::req(weapon.module_accessor, Hash40::new("damage_ground"), hit_stop, false, &Vector2f{x:0.0, y:0.0}, 1.0, 0.0, false, false);

        WorkModule::add_float(weapon.module_accessor, -1.0, INSTANCE_FLOAT_HP);
        if WorkModule::get_float(weapon.module_accessor, INSTANCE_FLOAT_HP) <= 0.0 {
            weapon.change_status(STATUS_KIND_BREAK.into(), false.into());
        }
    } 
    //Lose speed on collision
    let has_hit = AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT);
    if has_hit {
        KineticModule::mul_speed(weapon.module_accessor, &Vector3f { x: 0.75, y: 0.0, z: 0.0 }, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    //Reflect
    let reflected = AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR);
    if reflected {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 1.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::reflect_accel(weapon.module_accessor,  &Vector3f{x: 1.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    }
    //Disappear
    if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        weapon.change_status(STATUS_KIND_BREAK.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn shoot_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life_spent = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE)-life;
    let eff = WorkModule::get_int(weapon.module_accessor,*WEAPON_MIISWORDSMAN_WAVE_INSTANCE_WORK_ID_INT_EFFECT_TYPE) as u32;

    let slow_frame = 60.0;
    if life_spent > slow_frame as i32 {
        let eff = WorkModule::get_int(weapon.module_accessor,*WEAPON_MIISWORDSMAN_WAVE_INSTANCE_WORK_ID_INT_EFFECT_TYPE) as u32;
        EffectModule::set_rate(weapon.module_accessor, eff, 0.2);
    }
    let reflected = AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR);
    if reflected {
        EffectModule::set_frame(weapon.module_accessor, eff, slow_frame);
        EffectModule::set_rate(weapon.module_accessor, eff, 0.2);
    }
    let hp = WorkModule::get_float(weapon.module_accessor, INSTANCE_FLOAT_HP);
    let hp_max = WorkModule::get_float(weapon.module_accessor, INSTANCE_FLOAT_HP_INIT);
    EffectModule::set_alpha(weapon.module_accessor, eff, (hp/hp_max)+0.2);
    0.into()
}


pub unsafe extern "C" fn break_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

pub unsafe extern "C" fn break_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("pitb_guardian_shield"), false,false);
    //Life
    let life = 1;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("break"), 0.0, 1.0, false, 0.0, false, false);

    let owner = get_owner_boma(weapon);
    WorkModule::set_int(owner, DISABLE_FRAME, *FIGHTER_PIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_NO_SHIELD_FRAME);
    
    weapon.fastshift(L2CValue::Ptr(break_main_status_loop as *const () as _))
}

unsafe extern "C" fn break_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        set_link(weapon,false);
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, STATUS_KIND_FLY, shoot_init);
    agent.status(Pre, STATUS_KIND_FLY, shoot_pre);
    agent.status(Main, STATUS_KIND_FLY, shoot_main);
    agent.status(End, STATUS_KIND_FLY, empty_status);
    agent.status(Exec, STATUS_KIND_FLY, shoot_exec);
    
    agent.status(Pre, STATUS_KIND_BREAK, break_pre);
    agent.status(Main, STATUS_KIND_BREAK, break_main);
    agent.status(End, STATUS_KIND_BREAK, empty_status);

    smashline::Agent::new("luigi_fireball").status(Init, 0, shoot_init).install();
}