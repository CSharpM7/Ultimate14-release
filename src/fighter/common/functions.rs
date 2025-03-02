use crate::imports::imports_agent::*;
//Shield Module Send Shield Attack Collision Event, basically does the same thing as 0x46ae64, but on shield. Also dictates hard shield breaks
#[skyline::hook(offset = 0x4c7080)]
unsafe fn shield_module_send_shield_attack_collision_event(shield_module: *mut u64, opp_attack_module: *mut u64, collision: *mut u8, group_index: i32, raw_power: f32, real_power: f32, pos_x: f32, lr: f32) {
    let defender_boma = *(shield_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let defender_status_kind = StatusModule::status_kind(defender_boma);
    let attacker_id = *(collision.add(0x24) as *const u32);
    let attacker_battle_object = &mut *get_battle_object_from_id(attacker_id);
    let attacker_boma = attacker_battle_object.module_accessor;
    let attacker_status_kind = StatusModule::status_kind(attacker_boma);
    if !attacker_battle_object.is_fighter() && !attacker_battle_object.is_weapon() {
        return;
    }
    if !attacker_battle_object.is_fighter() {
        call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
        return;
    }
    let is_air = StatusModule::situation_kind(attacker_boma) == *SITUATION_KIND_AIR;// || true;

    let bonus_smash = [*FIGHTER_STATUS_KIND_ATTACK_S4,*FIGHTER_STATUS_KIND_ATTACK_HI4,*FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&attacker_status_kind);
    let bonus = if bonus_smash {1.1} else {1.0};

    let max_damage = if is_air {25.0} else {30.0};
    let min_damage = 2.0;
    let power = (real_power*bonus).clamp(min_damage,max_damage);
    let mut power_factor = 1.0-(power-min_damage)/(max_damage-min_damage );

    let max_rate = if is_air {1.75} else {4.125};
    let min_rate = if is_air {0.8} else {2.5};
    let motion_rate = lerp(min_rate,max_rate,power_factor);
    println!("P {real_power}->{power} Rate: {power_factor}->{motion_rate}");

    if defender_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF
    && FighterUtil::is_valid_just_shield(defender_boma) 
    //if [*FIGHTER_STATUS_KIND_GUARD_OFF,*FIGHTER_STATUS_KIND_GUARD_ON,*FIGHTER_STATUS_KIND_GUARD].contains(&defender_status_kind)
    {
        if attacker_battle_object.is_fighter() {

            let lr = PostureModule::lr(attacker_boma);
            let flash_y_offset = WorkModule::get_param_float(attacker_boma, hash40("height"), 0)*1.0;
            let eff_pos = *PostureModule::pos(attacker_boma);
            let eff = EffectModule::req(defender_boma, Hash40::new("sys_piyo"), &Vector3f{x: eff_pos.x-(0.75*lr), y: eff_pos.y+flash_y_offset, z: eff_pos.z}, &Vector3f{x: 0.0, y: -90.0*(lr-1.0), z: 0.0}, 1.1, 0,-1,false,0) as u32;
            EffectModule::set_rate(defender_boma, eff, 0.75);

            WorkModule::set_float(attacker_boma, motion_rate, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
            if is_air {
                StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND_JUMP, false);
            }
            else {
                StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
            }

        }
    }
    call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
}

/*
#[skyline::hook(replace=WorkModule::is_enable_transition_term)]
unsafe fn is_enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if boma.is_fighter() {
        let obj = boma.battle_object();
        if [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL
        ].contains(&flag) && VarModule::is_flag(obj, fighter::instance::flag::DISABLE_JUMP) {
            return false;
        }
    }
    original!()(boma, flag)
}
*/
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(_module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    return true;
}

unsafe fn apply_momentum(fighter: &mut L2CFighterCommon,jump_speed_x: f32) {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut new_speed_x = jump_speed_x;
    if new_speed_x != 0.0 {
        let lr = PostureModule::lr(fighter.module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let factor_rar = if new_speed_x.signum() != lr.signum() {-lr} else {lr};
        let factor = (stick_x+factor_rar)*0.5;
        new_speed_x *= factor*factor_rar;
        let mut stable_speed = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("air_speed_x_stable"), 0);
        let air_brake = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("air_brake_x"), 0);
        if new_speed_x.abs() > stable_speed {
            let extra_speed_lerp = 0.5;
			let lerp_speed = lerp(stable_speed,new_speed_x.abs(),extra_speed_lerp);
            new_speed_x = lerp_speed.min(1.5)*new_speed_x.signum();
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_brake*2.0
            );
        }
        if (new_speed_x.abs() > speed_x.abs()) {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                new_speed_x
            );
        }
        VarModule::set_float(fighter.battle_object,fighter::instance::float::JUMP_SQUAT_SPEED_X,0.0);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_Jump_sub_param)]
unsafe extern "C" fn status_pre_Jump_sub_param(fighter: &mut L2CFighterCommon, flag_keep: L2CValue, int_keep: L2CValue, float_keep: L2CValue, kinetic_type: L2CValue, arg: L2CValue) {
    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
        let mut speed_x = VarModule::get_float(fighter.battle_object,fighter::instance::float::JUMP_SQUAT_SPEED_X);
        apply_momentum(fighter,speed_x);
    }
    call_original!(fighter,flag_keep,int_keep,float_keep,kinetic_type,arg)
}
#[skyline::hook(replace = L2CFighterCommon_uniq_process_JumpSquat_exec_status_param)]
unsafe fn uniq_process_JumpSquat_exec_status_param(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    let end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut set_speed=true;
    let status_frame = fighter.global_table[STATUS_FRAME].get_f32();
    let minus = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    if [*FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
        let lr = PostureModule::lr(fighter.module_accessor);
        speed_x = speed_x.abs()*-lr;
        if status_frame > 1.0 {
            set_speed=false;
        }
    }
    else if [*FIGHTER_STATUS_KIND_DASH,*FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
        speed_x = VarModule::get_float(fighter.battle_object,fighter::instance::float::CURRENT_SPEED_X);
        if status_frame > 1.0 {
            set_speed=false;
        }
    }
    
    if set_speed {
        VarModule::set_float(fighter.battle_object,fighter::instance::float::JUMP_SQUAT_SPEED_X,speed_x);
        //println!("Squat Speed: {speed_x}");
    }
    call_original!(fighter,arg)
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_AttackAir)]
pub unsafe fn status_pre_AttackAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);

    VarModule::off_flag(fighter.battle_object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX);
    VarModule::on_flag(fighter.battle_object,fighter::instance::flag::AERIAL_LANDING_HITBOX_CHECK);

    original!()(fighter)
}


#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe extern "C" fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_check = VarModule::is_flag(fighter.battle_object,fighter::instance::flag::AERIAL_LANDING_HITBOX_CHECK);
    let landing_enabled = VarModule::is_flag(fighter.battle_object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX);
    let has_attack = AttackModule::is_attack(fighter.module_accessor, 0, false);

    //println!("A: {has_attack} C: {landing_check} E: {landing_enabled}");

    if has_attack
    && landing_check {
        if !landing_enabled {
            VarModule::on_flag(fighter.battle_object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX);
        }
        let attack_data = AttackModule::attack_data(fighter.module_accessor, 0, false);
        let attack_power = (*attack_data).power;
        if attack_power > 2.5 {
            VarModule::off_flag(fighter.battle_object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX);
            VarModule::off_flag(fighter.battle_object,fighter::instance::flag::AERIAL_LANDING_HITBOX_CHECK);
        }
    }
    else if !has_attack
    && landing_enabled
    && !landing_check {
        VarModule::off_flag(fighter.battle_object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX);
        VarModule::off_flag(fighter.battle_object,fighter::instance::flag::AERIAL_LANDING_HITBOX_CHECK);
    }
    original!()(fighter)
}

/*
#[skyline::hook(replace = L2CFighterCommon_status_LandingAttackAir_Main)]
unsafe extern "C" fn status_landingattackair_main(fighter: &mut L2CFighterCommon) {
    let enable = VarModule::is_flag(fighter.battle_object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX);
    let check = VarModule::is_flag(fighter.battle_object,fighter::instance::flag::AERIAL_LANDING_HITBOX_CHECK);
    println!("C {check} E {enable}");
    if !enable {
        println!("Perish");
        AttackModule::set_power_mul_status(fighter.module_accessor, 0.0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    original!()(fighter)
}
 */

 #[skyline::hook(replace=smash::app::sv_animcmd::ATTACK)]
 unsafe fn ATTACK_hook(lua_state: u64) {
     let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
     let object = smash::app::sv_system::battle_object(lua_state);
     if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        if !VarModule::is_flag(object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX) {
            return;
        }
     }
 
     original!()(lua_state);
 }
 #[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_FallSub)]
 pub unsafe fn status_FallSub(fighter: &mut L2CFighterCommon, arg2: L2CValue) {
     call_original!(fighter, arg2);
     if MotionModule::motion_kind(fighter.module_accessor) != hash40("fall") {
        let speed_x = VarModule::get_float(fighter.battle_object,fighter::instance::float::CURRENT_SPEED_X);
        //println!("Fall X: {speed_x}");
        apply_momentum(fighter,speed_x);
     }
 }
 #[skyline::hook(replace = L2CFighterCommon_sub_attack_air_common)]
 unsafe extern "C" fn sub_attack_air_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut stable_speed = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("air_speed_x_stable"), 0);
    let air_brake = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("air_brake_x"), 0);
    if speed_x.abs() > stable_speed {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_brake*3.0
        );
    }
    call_original!(fighter, param_1);
 }
 
fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_AttackAir,
            status_pre_Jump_sub_param,
            uniq_process_JumpSquat_exec_status_param,
            status_attackair_main_common,
            status_FallSub,
            sub_attack_air_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        shield_module_send_shield_attack_collision_event,
        //is_enable_transition_term_hook,
        is_valid_just_shield_reflector,
        ATTACK_hook
    );
}