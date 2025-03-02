use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    FT_MOTION_RATE(agent,5.0/(14.0-6.0));

    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(agent.lua_state_agent, 14.0);
    FT_MOTION_RATE(agent,1.0);

    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.0, 0.0, 12.0, 9.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        //shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MTRANS_AIR_UNABLE);
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        //shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
}


pub unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("pitb_gouwan_dash_start"), Hash40::new("top"), -7, 12, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_gouwan_dash"), Hash40::new("havel"), 0, 0, 3, -0.569, -0.001, 0.028, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_gouwan_dash_ring"), Hash40::new("havel"), 0, 0, 2.2, 0, 0, 0, 1, true);
        macros::EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("havel"), 0, 0, 3, -0.569, -0.001, 0.028, 1, true);
    }
}

pub unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pitb_special_s01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pitb_rnd_special_s02"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pitb_special_s02"));
    }
}


pub unsafe extern "C" fn expression_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_final") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 15);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_default_int64(agent.module_accessor, hash40("weapon") as i64);
    }
}



pub unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let mut isAir=false;
    if macros::is_excute(agent) {
        isAir = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_air_s_end").hash;
        //damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);

        let angle = if isAir {60} else {368};
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, angle, 0, 10, 60, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);

        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, angle, 0, 10, 60, 3.0, 0.0, 12.0, 9.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 0.0}, 7, false);
        AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 0.0}, 7, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 5.0, false);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_MIDDLE), false);

	    KineticModule::suspend_energy(agent.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if isAir{
            KineticModule::suspend_energy(agent.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent,7.0/(5.0-1.0));
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        AttackModule::clear_all(agent.module_accessor);
        if isAir{
            KineticModule::clear_speed_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    FT_MOTION_RATE(agent,1.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.5, 361, 58, 0, 90, 7.5, 0.0, 12.0, 15.0, None,None,None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);

        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.5, 361, 58, 0, 90, 6.5, 0.0, 12.0, 22.0, None,None,None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);

        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.5, 361, 58, 0, 90, 5.5, 0.0, 12.0, 30.0, None,None,None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);

        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 7.5, 361, 58, 0, 90, 3.5, 0.0, 12.0, 37.0, None,None,None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);

        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);

        if isAir{
            KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -0.5, y: 0.0, z: 0.0});
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
	    KineticModule::resume_energy(agent.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        if isAir{
            KineticModule::resume_energy(agent.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

pub unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    let mut effect = 0 as u32;
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //sys_hit_sting
        //effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_hit_sting"), Hash40::new("top"), &Vector3f{x: 13.0, y: 12.0, z: 13.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.75, true, 0, 0, 0, 0,0, false,false) as u32;
        EFFECT_FOLLOW(agent,Hash40::new("sys_hit_sting"), Hash40::new("top"), 11.5, 12.0, 11.5, 0.0, 0.0, 0.0, 0.875, true);
        effect = EffectModule::get_last_handle(agent.module_accessor) as u32;
        EffectModule::set_frame(agent.module_accessor, effect, 10.0);
        EffectModule::set_rate(agent.module_accessor, effect, (5.0)/1.0);
        EffectModule::set_visible(agent.module_accessor, effect, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        LAST_EFFECT_SET_RATE(agent, 0.25);
        EffectModule::set_visible(agent.module_accessor, effect,  true);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        LAST_EFFECT_SET_RATE(agent, 0.75);
    }
}

pub unsafe extern "C" fn sound_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pitb_attack06"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_pitb_final04"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 0.5, 0);
        //macros::PLAY_SE(agent, Hash40::new("se_item_stuff_shot"));
    }
}


pub unsafe extern "C" fn expression_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_final") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_default_int64(agent.module_accessor, hash40("weapon") as i64);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialsstart", game_specialsstart,Priority::Low);
    agent.acmd("sound_specialsstart", sound_specialsstart,Priority::Low);
    agent.acmd("effect_specialsstart", effect_specialsstart,Priority::Low);
    agent.acmd("expression_specialsstart", expression_specialsstart,Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart,Priority::Low);
    agent.acmd("sound_specialairsstart", sound_specialsstart,Priority::Low);
    agent.acmd("effect_specialairsstart", effect_specialsstart,Priority::Low);
    agent.acmd("expression_specialairsstart", expression_specialsstart,Priority::Low);
    
    agent.acmd("game_specialsend", game_specialsend,Priority::Low);
    agent.acmd("sound_specialsend", sound_specialsend,Priority::Low);
    agent.acmd("effect_specialsend", effect_specialsend,Priority::Low);
    agent.acmd("expression_specialsend", expression_specialsend,Priority::Low);
    agent.acmd("game_specialairsend", game_specialsend,Priority::Low);
    agent.acmd("sound_specialairsend", sound_specialsend,Priority::Low);
    agent.acmd("effect_specialairsend", effect_specialsend,Priority::Low);
    agent.acmd("expression_specialairsend", expression_specialsend,Priority::Low);
}