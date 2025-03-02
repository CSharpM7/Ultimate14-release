use crate::imports::imports_agent::*;
use super::*;

pub unsafe extern "C" fn captoss_start_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = get_founder_boma(weapon);
    let owner_kind = utility::get_kind(&mut *owner);
    let is_cappy = owner_kind == *FIGHTER_KIND_MARIO;
    println!("Is Cappy: {is_cappy}");

    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("eyeopenleft"), is_cappy);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("eyeopenright"), is_cappy);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("eyepupil"), is_cappy);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("mario_hathead"), is_cappy);
    
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("wpn_cannonball"), !is_cappy);

    if !is_cappy {
        ModelModule::set_joint_scale(weapon.module_accessor, Hash40::new("all"), &Vector3f{x: 0.01, y: 0.01, z: 0.01});
        return 0.into();
    }
    captoss_start_snap(weapon);
    0.into()
}

pub unsafe extern "C" fn captoss_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn captoss_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    LinkModule::remove_model_constraint(weapon.module_accessor,true);
    if LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink_all(weapon.module_accessor);
    }
    LinkModule::link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,owner);
    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("have"),Hash40::new("havel"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
    
    weapon.fastshift(L2CValue::Ptr(captoss_start_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_start_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn captoss_start_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    let owner = get_owner_boma(weapon);
    if StatusModule::status_kind(owner) != *FIGHTER_STATUS_KIND_SPECIAL_S {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    //captoss_start_snap(weapon);
    0.into()
}

pub unsafe extern "C" fn captoss_start_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x:0.0,y:0.0,z:0.0}, 0);
    PostureModule::add_pos(weapon.module_accessor, &Vector3f{x:10.0*lr,y:-3.0,z:0.0});
    0.into()
}
unsafe extern "C" fn captoss_start_snap(weapon: &mut smashline::L2CWeaponCommon) {
    if !captoss_owner_is_mario(weapon) {
        return;
    }
    let owner = get_owner_boma(weapon);
    let mut ownerPos = VECTOR_ZERO;
    let mut capPos = VECTOR_ZERO;
    let lr = PostureModule::lr(owner);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("havel")}, &mut ownerPos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut capPos);       
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + ownerPos.x - capPos.x - (2.0*lr), y: PostureModule::pos_y(owner) + ownerPos.y - (capPos.y/1.5), z: PostureModule::pos_z(owner) + ownerPos.z- capPos.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    
    let mut vec =Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let offset = ModelModule::joint_global_rotation(owner,Hash40::new("havel"),&mut vec,false);
    let rot = Vector3f{x: vec.x, y: 0.0, z: 0.0};
    PostureModule::set_rot(
        weapon.module_accessor,
        &rot,
        0
    );
}

pub fn install(agent: &mut smashline::Agent) {  
    agent.status(Init, mario_cappy::CAPTOSS_STATUS_KIND_START, captoss_start_init);
    agent.status(Pre, mario_cappy::CAPTOSS_STATUS_KIND_START, captoss_start_pre);
    agent.status(Main, mario_cappy::CAPTOSS_STATUS_KIND_START, captoss_start_main);
    agent.status(Exec, mario_cappy::CAPTOSS_STATUS_KIND_START, captoss_start_exec);
    agent.status(End, mario_cappy::CAPTOSS_STATUS_KIND_START, captoss_start_end);
    
    smashline::Agent::new("koopajr_cannonball").status(Init, mario_cappy::CAPTOSS_STATUS_KIND_START, captoss_start_init).install();
}
