use crate::imports::imports_acmd::*;
pub const WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_SPIKE : i32 = 0x2000000A;
pub const WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_BOUND_HITBOX : i32 = 0x20000009;

unsafe extern "C" fn game_throwed(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_SPIKE) {
            macros::ATTACK(agent, 0, 0, Hash40::new("have"), 10.0, 270, 80, 0, 10, 4.7, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(agent.module_accessor);
            wait(agent.lua_state_agent, 4.0);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("have"), 10.0, 70, 60, 0, 65, 4.2, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(agent.module_accessor);
            wait(agent.lua_state_agent, 6.0);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("have"), 6.0, 70, 60, 0, 65, 3.7, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}
unsafe extern "C" fn effect_throwed(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_SPIKE) {
        }
    }
}

unsafe extern "C" fn game_bound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, *WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_ASASE) {
            macros::ATTACK(agent, 0, 0, Hash40::new("have"), 6.0, 70, 50, 0, 35, 3.7, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        else{
            //water?
        }
    }
}
pub unsafe extern "C" fn pot_thrown_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let motion = MotionModule::motion_kind(weapon.module_accessor);
    if !AttackModule::is_attack(weapon.module_accessor, 0, false) 
    && WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_BOUND)
    && !WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_WATER)
    && !WorkModule::is_flag(weapon.module_accessor, WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_BOUND_HITBOX) {
        WorkModule::on_flag(weapon.module_accessor, WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_BOUND_HITBOX);
        MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_GAME, Hash40::new("game_bound"), -1);
    }

    0.into()
}
pub unsafe extern "C" fn pot_thrown_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    let spike = StatusModule::status_kind(owner) == *FIGHTER_STATUS_KIND_ATTACK_AIR;
    WorkModule::set_flag(weapon.module_accessor, spike,WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_SPIKE);

    let original = smashline::original_status(Main, weapon, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED)(weapon);
    if spike {
        PostureModule::set_rot(weapon.module_accessor, &Vector3f{x:0.0,y:0.0,z:0.0}, 0);
        let owner_speed = KineticModule::get_sum_speed_y(owner, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = (owner_speed-0.5).min(-1.5);

        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_TYPE_NORMAL,
            0.0,
            speed_y
        );
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_BOUND);
    }
    return original;
}
pub unsafe extern "C" fn pot_burst_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_SPIKE) {
        PostureModule::set_rot(weapon.module_accessor, &Vector3f{x:0.0,y:0.0,z:0.0}, 0);
    }
    0.into()
}
pub fn install() {
    smashline::Agent::new("shizue_pot")
        .acmd("game_throwed", game_throwed,Priority::Low)
        .acmd("effect_throwed", effect_throwed,Priority::Low)
        .acmd("game_bound", game_bound,Priority::Low)

        .status(Main, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED, pot_thrown_main)
        .status(Exec, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED, pot_thrown_exec)
        .status(Init, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_BURST, pot_burst_init)
    .install();
}