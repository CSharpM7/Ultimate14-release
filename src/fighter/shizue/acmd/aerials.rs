use crate::imports::imports_acmd::*;
unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG, false, -1);

        constraint_article_with_flags(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG,Hash40::new("trans"),Hash40::new("haver"),(*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32);
        set_article_constraint_offsets(agent.module_accessor,*FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG, &Vector3f{x:0.0, y:0.0, z:180.0},&Vector3f{x:0.0, y:0.0, z:0.5});

        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG, true, ArticleOperationTarget(0));

        let moneybag = get_article_boma(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG);
        PostureModule::set_link_scale(moneybag, 1.75, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 361, 86, 0, 41, 5.0, 0.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 361, 86, 0, 41, 3.0, 0.0, 1.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 361, 76, 0, 41, 5.0, 0.0, 5.5, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 361, 76, 0, 41, 3.0, 0.0, 5.5, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_MONEYBAG,ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("shizue_attack_arc2"), Hash40::new("shizue_attack_arc2"), Hash40::new("top"), 0, 6, -0.2, 6, -27, -05, 1.6, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn sound_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_sword_swing_m"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_shizue_bell08"));
    }
}

unsafe extern "C" fn expression_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, Hash40::new("attack_hi_3"), false, -1.0);//down_attack_d
        remove_article_constraint(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM);
        constraint_article(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,Hash40::new("have"),Hash40::new("haver"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 68, 101, 0, 24, 3.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 68, 101, 0, 24, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,ArticleOperationTarget(0));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("shizue_attack_arc2"), Hash40::new("shizue_attack_arc2"), Hash40::new("top"), 0, 3.5, -1.6, 180, -37, 0, 1.5, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn sound_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_shizue_attackhard_h01"));
    }
}

unsafe extern "C" fn expression_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA, Hash40::new("s"), false, -1.0);
        ArticleModule::set_frame(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,1.0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        
        let speed_y = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        
        let air_accel_y = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_y"), 0);
        let air_speed_y_stable = WorkModule::get_param_float(agent.module_accessor, hash40("air_speed_y_stable"), 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);

        if WorkModule::is_flag(agent.module_accessor, shizue::FLAG_ATTACK_AIR_HOP) {
            WorkModule::off_flag(agent.module_accessor, shizue::FLAG_ATTACK_AIR_HOP);
            let new_speed = 1.0;
            //println!("S: {speed_y}/{air_speed_y_stable}");
            sv_kinetic_energy!(
                reset_energy,
                agent,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                new_speed,
                0.0,
                0.0,
                0.0
            ); 
        }
        sv_kinetic_energy!(
            set_accel,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y*0.75
        );
        sv_kinetic_energy!(
            set_limit_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_speed_y_stable*0.75
        );
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 85, 100, 0, 40, 6.5, 0.0, 21.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 85, 100, 0, 40, 5.5, 0.0, 23.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        
        let air_accel_y = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_y"), 0);
        let air_speed_y_stable = WorkModule::get_param_float(agent.module_accessor, hash40("air_speed_y_stable"), 0);
        sv_kinetic_energy!(
            set_accel,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true, 0.6);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 6.0, 0, 0, 0, 0, 0, 1.8, true,0.6);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}
unsafe extern "C" fn sound_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_shizue_attackhard_s01"));
    }
}


unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        /*
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA, Hash40::new("s"), false, -1.0);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,0.0); */
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POT, false, -1);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA) {
            FT_MOTION_RATE(agent, 0.769);
            ArticleModule::set_rate(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,0.5/0.769);
        }
        else {
            FT_MOTION_RATE_RANGE(agent,1.0,14.0,13.0);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA) {
            ArticleModule::set_rate(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,0.5);
            
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 270, 80, 0, 10, 3.5, 0.0, -9.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 65, 100, 0, 30, 6.0, 0.0, -7.75, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        else {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);

            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 367, 80, 0, 10, 4.2, 0.0, -5.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }

    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if !ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 65, 100, 0, 30, 5.5, 0.0, -9.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::clear(agent.module_accessor, 0, false);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA) {
            //macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true, 0.6);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 8.0, 0, 0, 0, 0, 0, 1.8, true,0.6);
            macros::LAST_EFFECT_SET_RATE(agent, 1.5);
        }
        else {
            macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        }
    }
}
unsafe extern "C" fn sound_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA) {
            macros::PLAY_SE(agent, Hash40::new("se_shizue_attackhard_s01"));
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        if !ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA) {
            macros::PLAY_SE(agent, Hash40::new("se_shizue_attackdash_01"));
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attackairf", game_attackairf,Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf,Priority::Low);
    agent.acmd("sound_attackairf", sound_attackairf,Priority::Low);
    agent.acmd("expression_attackairf", expression_attackairf,Priority::Low);

    agent.acmd("game_attackairb", game_attackairb,Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb,Priority::Low);
    agent.acmd("sound_attackairb", sound_attackairb,Priority::Low);
    agent.acmd("expression_attackairb", expression_attackairb,Priority::Low);

    agent.acmd("game_attackairhi", game_attackairhi,Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi,Priority::Low);
    agent.acmd("sound_attackairhi", sound_attackairhi,Priority::Low);

    agent.acmd("game_attackairlw", game_attackairlw,Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw,Priority::Low);
    agent.acmd("sound_attackairlw", sound_attackairlw,Priority::Low);
}