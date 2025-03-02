use crate::imports::imports_agent::*;

pub unsafe extern "C" fn move_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_NONE as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn move_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_object = util::get_battle_object_from_id(owner_id);
    let is_kirby = WorkModule::get_int(owner, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;

    let cooldown = WorkModule::get_int(owner,lucas::SPECIAL_N_COOLDOWN_STATUS);
    println!("Freeze cooldown: {cooldown}");
    let factor = 1.0-((cooldown as f32)/(lucas::SPECIAL_N_COOLDOWN_MAX as f32));

    let new_val = lerp(cooldown as f32,lucas::SPECIAL_N_COOLDOWN_MAX as f32,0.5) as i32;
    if is_kirby {
        VarModule::set_int(owner_object,kirby::instance::float::LUCAS_COOLDOWN,new_val);
    }
    else {
        WorkModule::set_int(owner,new_val,lucas::SPECIAL_N_COOLDOWN);
    }

    WorkModule::set_float(owner,0.0,lucas::SPECIAL_N_HEAL);
    WorkModule::set_float(owner,0.0,lucas::SPECIAL_N_HEAL_SIZE);
    WorkModule::set_float(weapon.module_accessor,factor,lucas_pkfreeze::INSTANCE_FLOAT_FRESH_FACTOR);
    0.into()
}

pub unsafe extern "C" fn move_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {    
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;

    if LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    }
    let link_created = LinkModule::link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,owner);
    if link_created & 1 != 0 {
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *LINK_NO_CONSTRAINT, 
            Hash40::new("top"), Hash40::new("rot"), 
            (*CONSTRAINT_FLAG_NO_FLIP | *CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32, 
            true
        );
        let rot_offset = Vector3f{x:0.0,y:0.0,z:0.0};
        let trans_offset = Vector3f{x:0.0,y:10.0,z:0.0};
        LinkModule::set_constraint_rot_offset(weapon.module_accessor, &rot_offset);
        LinkModule::set_constraint_translate_offset(weapon.module_accessor, &trans_offset);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
    }

    let no_bomb_frame_on_button = WorkModule::get_param_int(weapon.module_accessor, hash40("param_special_n"), hash40("no_bomb_frame_on_button"));
    WorkModule::set_int(weapon.module_accessor, no_bomb_frame_on_button, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_NO_BOMB_FRAME);

    let handle = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_EFFECT_HANDLE) as u32;
    EffectModule::kill(weapon.module_accessor, handle, false, false);
    EffectModule::kill_all(weapon.module_accessor,0,false, false);

    let handle = EffectModule::req_follow(weapon.module_accessor, Hash40::new("lucas_psi_hold"), Hash40::new("top"), 
    &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 
    1.0, true, 0, 0, 0, 0, 0, false, false) as u32;
    EffectModule::set_rgb_partial_last(weapon.module_accessor, 0.5, 1.0, 0.3);

    WorkModule::set_int(weapon.module_accessor, handle as i32, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _)).into()
}

unsafe extern "C" fn move_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    KineticModule::clear_speed_all(weapon.module_accessor);
    /* Original-ish */
    let mut no_bang = false;
    let owner = get_owner_boma(weapon);
    if !ControlModule::check_button_on(owner,*CONTROL_PAD_BUTTON_SPECIAL) {
        let countdown = WorkModule::count_down_int(weapon.module_accessor, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_NO_BOMB_FRAME, 0);
        let no_bang_frame = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_NO_BOMB_FRAME);
        if no_bang_frame <= 0 {
            no_bang = true;
        }
    }
    if no_bang
    {
        EffectModule::kill_all(weapon.module_accessor, 0, false, false);
        weapon.change_status(WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_NO_BANG.into(),false.into());
    }
    /* New */
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("lucas_pkfr_bullet"), false, false);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("lucas_pkfr_hold"), false, false);


    0.into()
}

pub unsafe extern "C" fn move_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = 0.into();//smashline::original_status(Exec, weapon, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_MOVE)(weapon);
    let scale_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfreeze"), hash40("size_min"));
    let scale_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfreeze"), hash40("size_max"));
    let count = WorkModule::get_float(weapon.module_accessor,*WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_FLOAT_COUNT);
    let count_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfreeze"), hash40("count"));
    let count_factor = if (count >= count_max) {1.0} else {0.0};//(count/count_max).min(1.0);
    let fresh_factor = WorkModule::get_float(weapon.module_accessor,lucas_pkfreeze::INSTANCE_FLOAT_FRESH_FACTOR);
    let scale_base = lerp(scale_min,scale_max,count_factor);
    let scale = lerp(scale_min,scale_base,fresh_factor);

    ModelModule::set_scale(weapon.module_accessor,scale);

    WorkModule::add_float(weapon.module_accessor,1.0,*WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_FLOAT_COUNT);
    original
}
pub unsafe extern "C" fn move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let handle = WorkModule::get_int(weapon.module_accessor, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_EFFECT_HANDLE) as u32;
    EffectModule::kill(weapon.module_accessor, handle, false, false);
    /*
    let handle = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_flash"), Hash40::new("top"), 
    &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 
    1.0, true, 0, 0, 0, 0, 0, false, false) as u32;
    EffectModule::set_rgb_partial_last(weapon.module_accessor, 0.5, 1.0, 0.3);
    */
    0.into()
}
pub unsafe extern "C" fn bang_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = 0.into();//smashline::original_status(Init, weapon, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_BANG)(weapon);

    let owner = get_owner_boma(weapon);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_object = util::get_battle_object_from_id(owner_id);
    let is_kirby = WorkModule::get_int(owner, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if is_kirby && false {VarModule::set_int(owner_object,kirby::instance::float::LUCAS_COOLDOWN,lucas::SPECIAL_N_COOLDOWN_MAX);}
    else {WorkModule::set_int(owner,lucas::SPECIAL_N_COOLDOWN_MAX,lucas::SPECIAL_N_COOLDOWN);}

    //let eff_pos = *PostureModule::pos(weapon.module_accessor);
    //let eff = EffectModule::req(weapon.module_accessor, Hash40::new("lucas_pkfr_bomb"), &Vector3f{x: eff_pos.x, y: eff_pos.y, z: eff_pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.1, 0,-1,false,0) as u32;

    let power_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfreeze"), hash40("power_min"));
    let power_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfreeze"), hash40("power_mul"));
    let mut count = WorkModule::get_float(weapon.module_accessor,*WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_FLOAT_COUNT);
    let count_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfreeze"), hash40("count"));
    let count_factor = if (count >= count_max) {1.0} else {0.0};
    let fresh_factor = WorkModule::get_float(weapon.module_accessor,lucas_pkfreeze::INSTANCE_FLOAT_FRESH_FACTOR);

    let mut power = power_min + (power_mul*count*fresh_factor);
    if is_kirby {power *= 1.2;}
    
    WorkModule::set_int(weapon.module_accessor,power as i32,*WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_POWER);
    WorkModule::set_float(owner,power,lucas::SPECIAL_N_HEAL);

    let scale_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfreeze"), hash40("size_min"));
    let scale_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfreeze"), hash40("size_max"));
    let scale_base = lerp(scale_min,scale_max,count_factor);
    let scale = lerp(scale_min,scale_base,fresh_factor)*20.0;
    if count_factor >= 1.0 {
        WorkModule::set_float(owner,scale,lucas::SPECIAL_N_HEAL_SIZE);
    }
    let frame = MotionModule::frame(owner);
    WorkModule::set_float(owner,frame+2.0,lucas::SPECIAL_N_HEAL_FRAME);
    println!("Freeze heal: {power} size: {scale}");
    
    let sfx_m = count_factor >= 1.0 && fresh_factor >= 0.25;
    let sfx_l = count_factor >= 1.0 && fresh_factor >= 0.5;
    let sfx_ll = count_factor >= 1.0 && fresh_factor >= 1.0;
    let sfx = if sfx_ll {Hash40::new("se_lucas_special_n04_ll")} 
    else if sfx_l {Hash40::new("se_lucas_special_n04_l")} 
    else if sfx_m {Hash40::new("se_lucas_special_n04_m")} 
    else {Hash40::new("se_lucas_special_n04_s")};
    let sound = SoundModule::play_se(weapon.module_accessor, sfx, true, false, false, false, app::enSEType(0));

    if count_factor >= 1.0 {
        super::super::pk_lifeup(&mut *owner, power);
    }

    //bang_lifeup(weapon);

    original
}

pub unsafe extern "C" fn bang_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //EffectModule::kill_all(weapon.module_accessor, 0, false, false);
    0.into()
}

pub fn install(weapon: &mut smashline::Agent) {
    weapon.status(Init, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_MOVE, move_init);
    weapon.status(Pre, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_MOVE, move_pre);
    weapon.status(Main, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_MOVE, move_main);
    weapon.status(Exec, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_MOVE, move_exec);
    weapon.status(End, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_MOVE, move_end);

    weapon.status(Init, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_BANG, bang_init);
    weapon.status(Exit, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_BANG, bang_end);
}