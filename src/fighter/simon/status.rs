use crate::imports::imports_agent::*;

pub unsafe extern "C" fn attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec();

    if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("attack_air_lw").hash {
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_SIMON_STATUS_ATTACK_FLAG_HIT) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_STATUS_ATTACK_FLAG_HIT);
            let pos_x = PostureModule::pos_x(fighter.module_accessor);
            let pos_y = PostureModule::pos_y(fighter.module_accessor);
            let hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
            let mut max_y = 30.0;
            let mut half_y = max_y/2.0;
            let ray_check = GroundModule::ray_check_hit_pos(
                fighter.module_accessor,
                &Vector2f{x: pos_x, y: pos_y},
                &Vector2f{x: 0.0, y: -max_y+0.0},
                hit_pos,
                false
            );
            if ray_check {
                max_y = pos_y-hit_pos.y;
                EffectModule::req(fighter.module_accessor, Hash40::new("sys_soil_landing"), &Vector3f{x: hit_pos.x, y: hit_pos.y+0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0,-1,false,0) as u32;
            }
            if !ray_check {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 270, 85, 0, 25, 2.5, 0.0, -30.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 270, 85, 0, 15, 2.5, 0.0, 2.0, 2.5, Some(0.0), Some(-30.0), Some(2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
            }
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 58, 75, 0, 60, 5.5, 0.0, 0.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 270, 85, 0, 15, 2.5, 0.0, 2.0, 2.5, Some(0.0), Some(-max_y), Some(2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP); 
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 10.0, 58, 75, 0, 60, 2.5, 0.0, 2.0, 2.5, Some(0.0), Some(-max_y), Some(2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP); 
        }
    }
    0.into()
}

pub unsafe extern "C" fn attack_air_attack(fighter: &mut L2CFighterCommon, param2: &L2CValue, param3: &L2CValue) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3();
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
    
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_WHIP, false, 0);
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_WHIP,*WEAPON_SIMON_WHIP_STATUS_KIND_NORMAL,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_SIMON_GENERATE_ARTICLE_WHIP,Hash40::new("attack_lw3"),true,-1.0);
    return fighter.sub_shift_status_main(L2CValue::Ptr(attack_lw3_main_loop as *const () as _));
}

unsafe extern "C" fn attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackLw3_Main()
}

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);

    let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    macros::SET_SPEED_EX(fighter, speed_x*lr, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_WHIP, false, 0);
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_WHIP,*WEAPON_SIMON_WHIP_STATUS_KIND_NORMAL,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_SIMON_GENERATE_ARTICLE_WHIP,Hash40::new("attack_air_lw"),true,-1.0);

    return original
}
pub fn install(agent: &mut smashline::Agent) {
    //agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_attack);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exec);

    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, attack_lw3_main);
}