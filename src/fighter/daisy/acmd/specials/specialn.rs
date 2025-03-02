use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
        /* 
        //shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.0, 0.0, 7.0, 2.0, 0.0, 7.0, -1.5, 1.25, 1.75, 50, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 7.0, 0.0, 7.0, 11.0, Some(0.0), Some(7.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);

        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 7.0, 0.0, 7.0, 2.0, 0.0, 9.0, 1.5, 1.25, 1.75, 50, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        */
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(agent.lua_state_agent, 1.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
    }
}

pub unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item_flip") as i64, hash40("smash_item_racket") as i64);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
		VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item_flip") as i64, hash40("smash_item_none") as i64);
		ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub unsafe extern "C" fn game_specialnhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        let spores = WorkModule::get_int(agent.module_accessor,*FIGHTER_PEACH_INSTANCE_WORK_ID_INT_KINOPIOSPORE_SHOOT_NUM);
    }
    frame(agent.lua_state_agent, 1.0);
    let mut isTurn=false;
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.0, 0.0, 7.0, 2.0, 0.0, 7.0, -1.5, 1.25, 1.75, 50, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        

        isTurn = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_n_turn").hash
        || MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_air_n_turn").hash;
        if !isTurn{
            FT_MOTION_RATE(agent,4.0/(8.0-1.0));
            frame(agent.lua_state_agent, 7.0);
        }
        else{
            FT_MOTION_RATE(agent,(4.0+12.0)/(20.0-1.0));
            frame(agent.lua_state_agent, 19.0);
            if macros::is_excute(agent) {
                macros::REVERSE_LR(agent);
            }
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 3.0, 42, 100, 110, 0, 3.4, -0.5, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 3.0, 42, 100, 110, 0, 4.4, 4.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("armr"), 4.4, 4.0, 0.0, 0.0, 4.0, 0.0, 0.0, 1.25, 1.75, 50, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        AttackModule::clear_all(agent.module_accessor);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}


pub unsafe extern "C" fn effect_specialnhit(agent: &mut L2CAgentBase) {
    let mut isTurn=false;
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0);
        isTurn = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_n_turn").hash
        || MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_air_n_turn").hash;
        if !isTurn{
            frame(agent.lua_state_agent, 8.0);
        }
        else{
            frame(agent.lua_state_agent, 20.0);
        }
    }
    /*
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("daisy_kinopio_hit"), Hash40::new("top"), 0, 9.5, -12, 0, 180, 0, 1, true);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("daisy_kinopio_hit"), false, false);
        macros::COL_NORMAL(agent);
    }
    */
}


pub unsafe extern "C" fn sound_specialnhit(agent: &mut L2CAgentBase) {
    let mut isTurn=false;
    if macros::is_excute(agent) {
        isTurn = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_n_turn").hash
        || MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_air_n_turn").hash;
        if !isTurn{
            frame(agent.lua_state_agent, 7.0);
        }
        else{
            frame(agent.lua_state_agent, 19.0);
        }
    }
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_daisy_special_n01"));
        macros::PLAY_SE(agent, Hash40::new("se_daisy_smash_s03"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_daisy_attack04"));
    }
}


pub unsafe extern "C" fn expression_specialnhit(agent: &mut L2CAgentBase) {
    let mut isTurn=false;
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item_flip") as i64, hash40("smash_item_racket") as i64);

        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);

        isTurn = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_n_turn").hash
        || MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_air_n_turn").hash;
        if !isTurn{
            frame(agent.lua_state_agent, 8.0);
        }
        else{
            frame(agent.lua_state_agent, 20.0);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 25, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 29.0);
	if macros::is_excute(agent) {
		VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item_flip") as i64, hash40("smash_item_none") as i64);
		ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
	}
}


//TODO: Make this a flower effect
pub unsafe extern "C" fn effect_shot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_magicball_attack_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.625, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_shield_damage2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, true);
        //LAST_EFFECT_SET_COLOR(agent,0.5,0.5,0.0);

        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_metamon_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.625, true);
        LAST_EFFECT_SET_COLOR(agent,1.0,0.75,0.0);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("daisy_bomber_jamp"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

pub unsafe extern "C" fn game_shot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DAISY_TENNIS, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialn", game_specialn,Priority::Low);
    agent.acmd("game_specialairn", game_specialn,Priority::Low);
    agent.acmd("effect_specialn", effect_specialn,Priority::Low);
    agent.acmd("effect_specialairn", effect_specialn,Priority::Low);
    agent.acmd("expression_specialn", expression_specialn,Priority::Low);
    agent.acmd("expression_specialairn", expression_specialn,Priority::Low);

    agent.acmd("game_specialnhit", game_specialnhit,Priority::Low);
    agent.acmd("game_specialnturn", game_specialnhit,Priority::Low);
    agent.acmd("game_specialairnhit", game_specialnhit,Priority::Low);
    agent.acmd("game_specialairnturn", game_specialnhit,Priority::Low);
    agent.acmd("effect_specialnhit", effect_specialnhit,Priority::Low);
    agent.acmd("effect_specialnturn", effect_specialnhit,Priority::Low);
    agent.acmd("effect_specialairnhit", effect_specialnhit,Priority::Low);
    agent.acmd("effect_specialairnturn", effect_specialnhit,Priority::Low);
    agent.acmd("sound_specialnhit", sound_specialnhit,Priority::Low);
    agent.acmd("sound_specialnturn", sound_specialnhit,Priority::Low);
    agent.acmd("sound_specialairnhit", sound_specialnhit,Priority::Low);
    agent.acmd("sound_specialairnturn", sound_specialnhit,Priority::Low);
    agent.acmd("expression_specialnhit", expression_specialnhit,Priority::Low);
    agent.acmd("expression_specialnturn", expression_specialnhit,Priority::Low);
    agent.acmd("expression_specialairnhit", expression_specialnhit,Priority::Low);
    agent.acmd("expression_specialairnturn", expression_specialnhit,Priority::Low);
    
    Agent::new("daisy_kinopiospore")
    .acmd("game_shot", game_shot,Priority::Low)
    .acmd("effect_shot", effect_shot,Priority::Low)
    .install();
}