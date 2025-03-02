use crate::imports::imports_agent::*;

pub unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)) //&& !is_in_hitlag(boma))
    || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SEARCH)) //&& !is_in_hitlag(boma))
    || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_REFLECTOR)) // && !is_in_hitlag(boma))
    {
        println!("HUH!");
    }
    if (WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0){
        println!("HIT!");
    }
    if ReflectModule::count(fighter.module_accessor) > 0
    {
        println!("REFLECT!");
    }
    AttackModule::set_reflect_attack(fighter.module_accessor,true);
    let shield = ShieldModule::is_shield(fighter.module_accessor,0,0); //WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR);
    let reflect = ReflectorModule::is_shield(fighter.module_accessor,0,0); //WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR);
    //println!("IsShield {shield} IsReflect: {reflect}");
    //macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    return 0.into();
}

pub unsafe extern "C" fn daisy_uniqfloatstart_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        //*FIGHTER_KINETIC_TYPE_PEACH_UNIQ_FLOAT,
        *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_AIR_N_RAISE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        //(*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        0,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn daisy_uniqfloatstart_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*
    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            vl::param_special_lw::limit_speed_y
        );
        fighter.clear_lua_stack();
    }
    */

    KineticModule::resume_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    //let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("uniq_float_start_y"));
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable
    );
    fighter.clear_lua_stack();

    //if MotionModule::frame(fighter.module_accessor) < 10.0 {
	    KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)-0.01;
        macros::SET_SPEED_EX(fighter, 1.75, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    //}
    //else{
    //}
    0.into()
}

pub unsafe extern "C" fn spore_shot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
        let eff_pos = *PostureModule::pos(weapon.module_accessor);
        EffectModule::req(weapon.module_accessor, Hash40::new("daisy_bomber"), &Vector3f{x: eff_pos.x, y: eff_pos.y+0.0, z: eff_pos.z}, &Vector3f{x: 0.0, y: 300.0, z: 0.0}, 0.8, 0,-1,false,0) as u32;
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_daisy_special_s02"), true, false, false, false, app::enSEType(0));
    }

    0.into()
}

pub unsafe extern "C" fn jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_jump_item_rocketbelt();
    fighter.status_Jump_sub(L2CValue::Hash40s("invalid"), L2CValue::F32(0.0));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fuwafuwa_start"), 0.0, 1.0, false, 0.0, false, false);
        return fighter.sub_shift_status_main(L2CValue::Ptr(jump_main_loop as *const () as _));
    }
    return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Jump_Main as *const () as _));
}
pub unsafe extern "C" fn jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toreturn = fighter.status_Jump_Main();

    if MotionModule::motion_kind(fighter.module_accessor) != Hash40::new("fuwafuwa_start").hash {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("fuwafuwa_start"), -1.0, 1.0, 0.0, false, false);
    }
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_y.abs() < 0.5 {
        let speed = Vector3f { x: 0.0, y: -0.1, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }
    toreturn
}
pub unsafe extern "C" fn jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Jump()
}
unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shorthop = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    if fighter.global_table[PREV_STATUS_KIND] != *FIGHTER_STATUS_KIND_JUMP_SQUAT 
    && !shorthop {
        return 0.into();
    }
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_y.abs() < 0.5 {
        let speed = Vector3f { x: 0.0, y: -0.1, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP, jump_main);
    agent.status(End, *FIGHTER_STATUS_KIND_JUMP, jump_end);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, daisy_uniqfloatstart_pre);
    agent.status(Exec, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, daisy_uniqfloatstart_exec); 

    
    Agent::new("daisy_kinopiospore")
    .status(Exit,*WEAPON_PEACH_KINOPIOSPORE_STATUS_KIND_SHOT,spore_shot_end)
    .status(End,*WEAPON_PEACH_KINOPIOSPORE_STATUS_KIND_SHOT,spore_shot_end)
    .install();
}