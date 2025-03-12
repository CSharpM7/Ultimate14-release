use crate::imports::imports_agent::*;
use crate::imports::imports_acmd::*;

pub const AXE_SPIN_SPEED : f32 = 2.5;

unsafe fn is_axe(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    return MotionModule::is_anim_resource(module_accessor, Hash40::new("shield"));
}
unsafe extern "C" fn game_shield(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("leafshield4"), 2.7, 361, 30, 0, 35, 3.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 11, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
}
unsafe extern "C" fn sound_shield(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if !SoundModule::is_playing_status(agent.module_accessor,Hash40::new("se_richter_special_s02_smash")) {
            macros::PLAY_STATUS(agent, Hash40::new("se_richter_special_s02_smash"));
        }
    }
}

unsafe extern "C" fn game_end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        //smash_script::notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("leafshield4"), 0, 8, 0, 0, 0, 0, 0.75, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
}

unsafe extern "C" fn sound_end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_richter_special_s02_smash"));
    }
}


pub unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if is_axe(weapon.module_accessor) {
        weapon.change_status(richter::BOOK_STATUS_KIND_SHIELD.into(), false.into());
        return weapon.fastshift(L2CValue::Ptr(empty_status as *const () as _));
    }
    return smashline::original_status(Main, weapon, *WEAPON_SIMON_AXE_STATUS_KIND_HAVED)(weapon);
}
pub unsafe extern "C" fn have_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = smashline::original_status(Main, weapon, *WEAPON_SIMON_AXE_STATUS_KIND_HAVED)(weapon);
    if is_axe(weapon.module_accessor) {
        println!("Book?");
        MotionModule::set_rate(weapon.module_accessor,0.0);
    }
    return original;
}

pub unsafe extern "C" fn hop_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !is_axe(weapon.module_accessor) {
        return smashline::original_status(Main, weapon, *WEAPON_SIMON_AXE_STATUS_KIND_HOP)(weapon);
    }
    
    let mut offset = VECTOR_ZERO;
    ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40::new("leafshield4"), &mut offset);
    if LinkModule::is_model_constraint(weapon.module_accessor) {
        LinkModule::remove_model_constraint(weapon.module_accessor, true);
    }

    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if sv_battle_object::is_active(owner_id) {
        let owner = sv_battle_object::module_accessor(owner_id);
        super::set_book(owner,false);
    }

    let lr = -offset.x.signum();
    let speed_x = 1.0;
    let speed_y = 1.0;
    let accel_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_axe"), hash40("hop_gravity"));
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x*lr,
        speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        -accel_y
    );

    let life = MotionModule::end_frame(weapon.module_accessor) as i32;
    WorkModule::set_int(weapon.module_accessor, life+30, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life+30, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    PostureModule::add_pos(weapon.module_accessor, &offset);
    let rot_y = PostureModule::rot_y(weapon.module_accessor, 0) + 90.0;
    PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(0.0, rot_y, 0.0), 0);

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("end"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(end_main_loop as *const () as _))
}

pub unsafe extern "C" fn shield_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !is_axe(weapon.module_accessor) {
        return smashline::original_status(Pre, weapon, richter::BOOK_STATUS_KIND_SHIELD)(weapon);
    }
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn shield_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if !is_axe(weapon.module_accessor) {
        return smashline::original_status(Main, weapon, richter::BOOK_STATUS_KIND_SHIELD)(weapon);
    }
    let owner = sv_battle_object::module_accessor(owner_id);
    WorkModule::enable_transition_term_forbid_indivi(owner, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    let lr = PostureModule::lr(owner);
    PostureModule::set_lr(weapon.module_accessor, lr);
    let rot_y = if lr > 0.0 {0.0} else {180.0};
    PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(0.0, rot_y, 0.0), 0);
    if LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    }
    let link_created = LinkModule::link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,owner_id);
    if link_created & 1 != 0 {
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *LINK_NO_CONSTRAINT, 
            Hash40::new("have"), Hash40::new("hip"), 
            (*CONSTRAINT_FLAG_ONE_NODE
                | *CONSTRAINT_FLAG_POSITION
                | *CONSTRAINT_FLAG_OFFSET_TRANSLATE ) as u32, 
            true
        );
        let rot_offset = Vector3f{x:0.0,y:0.0,z:90.0};
        let trans_offset = Vector3f{x:0.0,y:10.0,z:0.0};
        //LinkModule::set_constraint_rot_offset(weapon.module_accessor, &rot_offset);
        LinkModule::set_constraint_translate_offset(weapon.module_accessor, &trans_offset);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
        //LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_LR as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_POS as u8}, true);
    } 
    GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
    KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_NONE);
    KineticModule::unable_energy(weapon.module_accessor, *WEAPON_SIMON_AXE_KINETIC_ENERGY_ID_GRAVITY);

    let life = 90; //WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("life"));;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

	MotionModule::change_motion(weapon.module_accessor, Hash40::new("start"), 5.0, AXE_SPIN_SPEED, false, 0.0, false, false);
    let lr = PostureModule::lr(weapon.module_accessor);
    //let start_frame = if lr < 0.0 {0.0} else {MotionModule::end_frame(weapon.module_accessor)*0.5};
    //MotionModule::set_frame_sync_anim_cmd(weapon.module_accessor, start_frame, true, true, false);
    weapon.fastshift(L2CValue::Ptr(shield_main_loop as *const () as _))
}

unsafe extern "C" fn shield_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if !sv_battle_object::is_active(owner_id) {
        println!("Doesnt exist?");
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }
    let lr = PostureModule::lr(weapon.module_accessor);
    let rot_y = if lr > 0.0 {0.0} else {180.0};
    let mut rotation = Vector3f{x: 0.0, y: rot_y, z: 0.0};
    //ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("have"), &rotation, 
    //MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_BEFORE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    if MotionModule::is_end(weapon.module_accessor) {
        let rate = MotionModule::rate(weapon.module_accessor);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("shield"), 0.0, rate, false, 0.0, false, false);
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if !AttackModule::is_attack(weapon.module_accessor, 0, false) || life <= 0 {
        println!("Hop?");
        weapon.change_status(WEAPON_SIMON_AXE_STATUS_KIND_HOP.into(), false.into());
        return 1.into();
    }
    if life < 0 {
        end_main(weapon);
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn end_main(weapon: &mut smashline::L2CWeaponCommon) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if sv_battle_object::is_active(owner_id) {
        let owner = sv_battle_object::module_accessor(owner_id);
        super::set_book(owner,false);
    }

    let life = 10;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    let mut book_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    ModelModule::joint_global_position(weapon.module_accessor, Hash40::new("leafshield4"), &mut book_pos,false);
    EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_erace_smoke"),
        &Vector3f{x:book_pos.x,y:book_pos.y+0.0,z:book_pos.z},
        &Vector3f{x:0.0,y:0.0,z:0.0},
        0.75,
        0,
        -1,
        false,
        0
    );
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("end"), 0.0, 1.0, false, 0.0, false, false);
    VisibilityModule::set_whole(weapon.module_accessor, false);

    weapon.fastshift(L2CValue::Ptr(end_main_loop as *const () as _));
}
unsafe extern "C" fn end_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if MotionModule::is_end(weapon.module_accessor) {
        VisibilityModule::set_whole(weapon.module_accessor, false);
    }
    if life < 0 {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(owner_id) {
            let owner = sv_battle_object::module_accessor(owner_id);
            super::set_book(owner,false);
        }
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}
pub fn install() {
    smashline::Agent::new("richter_axe")
        .acmd("game_shield",game_shield,Priority::Low)
        //.acmd("game_shield",game_shield,Priority::Low)
        .acmd("sound_shield",sound_shield,Priority::Low)

        .acmd("game_end",game_end,Priority::Low)
        //.acmd("effect_end",effect_end,Priority::Low)
        .acmd("sound_end",sound_end,Priority::Low)

        .status(Main, *WEAPON_SIMON_AXE_STATUS_KIND_HAVED, have_main)
        .status(Main, *WEAPON_SIMON_AXE_STATUS_KIND_HOP, hop_main)
        .status(Main, *WEAPON_SIMON_AXE_STATUS_KIND_FLY, fly_main)

        .status(Pre, richter::BOOK_STATUS_KIND_SHIELD, shield_pre)
        .status(Main, richter::BOOK_STATUS_KIND_SHIELD, shield_main)
        .status(End, richter::BOOK_STATUS_KIND_SHIELD, empty_status)
    .install();
}