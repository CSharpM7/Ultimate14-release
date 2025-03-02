use crate::imports::imports_agent::*;
use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let is_max = WorkModule::get_float(agent.module_accessor,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE) >= 1.0;
        let angle: u64 = if is_max {50} else {361};
        let kbg = if is_max {85} else {50};
        let bkb = if is_max {48} else {8};

        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, angle, kbg, 0, bkb, 1.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    //frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let is_max = WorkModule::get_float(agent.module_accessor,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE) >= 1.0;
        macros::EFFECT_FOLLOW(agent, Hash40::new("toonlink_arrow_flare"), Hash40::new("root"), 0, 0, 0, 0, 0, 0, 1, true);

        macros::EFFECT_FOLLOW(agent, Hash40::new("toonlink_arrow_trace"), Hash40::new("root"), 0, 0, -9.5, 0, 0, 0, 1, true);
        if is_max {
            LAST_EFFECT_SET_COLOR(agent,1.2,1.0,0.5);

            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_greenshell_trace"), Hash40::new("arrow"), 0, 0, 1.5, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_ALPHA(agent,0.5);
            LAST_EFFECT_SET_COLOR(agent,1.2,1.0,0.5);

            macros::FLASH(agent, 0.097, 0.006, 0.238, 0.15);
            macros::BURN_COLOR(agent, 6, 0, 40, 0.045);
        }
    }
}

unsafe extern "C" fn sound_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let is_max = WorkModule::get_float(agent.module_accessor,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE) >= 1.0;
        let sfx = if is_max {Hash40::new("se_item_stuff_shot")} else {Hash40::new("se_toonlink_special_n03")};
        PLAY_SE(agent, sfx);
    }
}


pub unsafe fn light_arrow_color(weapon: &mut L2CWeaponCommon) {
    let status = StatusModule::status_kind(weapon.module_accessor);
    let blend = if status == *WN_LINK_BOWARROW_STATUS_KIND_HAVED {0} else {0};
    ModelModule::set_color_rgb(weapon.module_accessor, 1.7, 2.0, 0.3, MODEL_COLOR_TYPE{_address: 0});
    ModelModule::set_emmisive_scale(weapon.module_accessor, 5.0);
}

pub unsafe extern "C" fn haved_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let is_max = WorkModule::is_flag(owner_boma, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX);
    if is_max {
        light_arrow_color(weapon);
    }
    0.into()
}
pub unsafe extern "C" fn fly_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let is_max = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE) >= 1.0;
    if is_max {
        light_arrow_color(weapon);
    }
    0.into()
}

pub unsafe extern "C" fn fly_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = smashline::original_status(Init, weapon, *WN_LINK_BOWARROW_STATUS_KIND_FLY)(weapon);
    original
}
pub unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = smashline::original_status(Main, weapon, *WN_LINK_BOWARROW_STATUS_KIND_FLY)(weapon);
    let is_max = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE) >= 1.0;
    if is_max {
        let life = 24;
        WorkModule::set_int(weapon.module_accessor, life as i32, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, life as i32, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

        AttackModule::set_power(weapon.module_accessor,0,14.0,false);
        
        let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("speed_max"));
        let lr = PostureModule::lr(weapon.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed*lr*2.5,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.0
        );
    }
    original
}

pub fn install() {
    smashline::Agent::new("toonlink_bowarrow")
        .acmd("game_fly",game_fly,Priority::Low)
        .acmd("effect_fly",effect_fly,Priority::Low)
        .acmd("sound_fly",sound_fly,Priority::Low)
        .status(Exec, *WN_LINK_BOWARROW_STATUS_KIND_HAVED, haved_exec)
        .status(Init, *WN_LINK_BOWARROW_STATUS_KIND_FLY, fly_init)
        .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_FLY, fly_main)
        .status(Exec, *WN_LINK_BOWARROW_STATUS_KIND_FLY, fly_exec)
    .install();
}