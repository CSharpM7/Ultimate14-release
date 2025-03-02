use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
}
unsafe extern "C" fn game_specialshold(agent: &mut L2CAgentBase) {
    let mut offset = 0.0;
    let id_g = 3;
    let id_a = 4;
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, id_g, 0, Hash40::new("top"), 1.2, 150, 100, 35, 0, 4.8, 0.0, 1.25, 9.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, id_a, 0, Hash40::new("top"), 1.2, 180, 100, 35, 0, 4.8, 0.0, 1.25, 9.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::disable_tip(agent.module_accessor);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, id_g, 0, Hash40::new("rot"), 1.2, 150, 100, 35, 0, 3.8, 0.0, -0.5, 0.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, id_a, 0, Hash40::new("rot"), 1.2, 180, 100, 35, 0, 3.8, 0.0, -0.5, 0.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::disable_tip(agent.module_accessor);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor,0,false);
        WorkModule::on_flag(agent.module_accessor,sonic::SPECIAL_TORNADO_SPAWN_TORNADO);
    }
    wait(agent.lua_state_agent, 1.0);
    {
        offset = WorkModule::get_float(agent.module_accessor,sonic::SPECIAL_TORNADO_OFFSET_Y);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 90, 100, 35, 0, 6.8, 0.0, offset+4.25, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 367, 100, 35, 0, 6.8, 0.0, offset+4.25, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.2, 367, 100, 35, 0, 6.8, 0.0, offset+14.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 7.0);
    for i in 0..3 {    
        if macros::is_excute(agent) {
            macros::ATTACK(agent, id_g, 0, Hash40::new("rot"), 1.2, 150, 100, 35, 0, 3.8, 0.0, -0.5, 0.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, id_a, 0, Hash40::new("rot"), 1.2, 180, 100, 35, 0, 3.8, 0.0, -0.5, 0.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 5.0);
        if i < 2 {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, id_g, 0, Hash40::new("rot"), 1.2, 150, 100, 35, 0, 3.8, 0.0, -0.5, 0.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
                macros::ATTACK(agent, id_a, 0, Hash40::new("rot"), 1.2, 180, 100, 35, 0, 3.8, 0.0, -0.5, 0.0, None, None, None, 0.25, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            }
            wait(agent.lua_state_agent, 5.0);
            if macros::is_excute(agent) {
                WorkModule::on_flag(agent.module_accessor,sonic::SPECIAL_TORNADO_SPAWN_EFF);
            }
        }
    }
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 4.5, 90, 95, 0, 72, 7.0, 0.0, offset+5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 1, Hash40::new("top"), 4.5, 90, 95, 0, 72, 7.0, 0.0, offset+14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    let mut offset = 0.0;
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}
unsafe extern "C" fn effect_specialshold(agent: &mut L2CAgentBase) {
    let mut offset = 0.0;
    if macros::is_excute(agent) {
        let is_kirby = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
        if !is_kirby {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace"), Hash40::new("rot"), 0, 0.5, 0, 0, 0, 0, 1, true);
        }
        LAST_EFFECT_SET_ALPHA(agent,0.5);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        offset = WorkModule::get_float(agent.module_accessor,sonic::SPECIAL_TORNADO_OFFSET_Y);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado_appear"), Hash40::new("top"), 0, offset, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent,0.1,0.75,2.0);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, offset, 0, 0, 0, 0, 1.5, true);
        //LAST_EFFECT_SET_COLOR(agent,0.1,0.75,2.0);
    } 
}

unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn sound_specialshold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_sonic_rounddash"));
        macros::PLAY_SE(agent, Hash40::new("se_sonic_attackair_n01"));
        let is_kirby = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
        if !is_kirby {
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_sonic_rnd_attack"));
        }
        else {
            macros::PLAY_SE(agent, Hash40::new("vc_kirby_copy_sonic_01"));
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_club_swing"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_item_club_wind"));
    }
}

unsafe extern "C" fn expression_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        crate::fighter::sonic::set_ball_mode(agent,true);
    }
    if macros::is_excute(agent) {
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
    }
}
unsafe extern "C" fn expression_specialshold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 8);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialsholdjump(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_specialsholdjump(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sonic_homing_hit"), Hash40::new("head"), 0, 0, 0, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        let is_kirby = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
        if !is_kirby {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace"), Hash40::new("rot"), 0, 0.5, 0, 0, 0, 0, 1, true);
        }
        LAST_EFFECT_SET_ALPHA(agent,0.5);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spintrace"), true, false);
    }
}
unsafe extern "C" fn sound_specialsholdjump(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
}

unsafe extern "C" fn expression_specialsholdjump(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        crate::fighter::sonic::set_ball_mode(agent,true);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        crate::fighter::sonic::set_ball_mode(agent,false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spintrace"), true, false);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}
pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialsstart", game_specialsstart,Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart,Priority::Low);
    agent.acmd("effect_specialsstart", effect_specialsstart,Priority::Low);
    agent.acmd("effect_specialairsstart", effect_specialsstart,Priority::Low);
    agent.acmd("sound_specialsstart", sound_specialsstart,Priority::Low);
    agent.acmd("sound_specialairsstart", sound_specialsstart,Priority::Low);
    agent.acmd("expression_specialsstart", expression_specialsstart,Priority::Low);
    agent.acmd("expression_specialairsstart", expression_specialsstart,Priority::Low);

    agent.acmd("game_specialshold", game_specialshold,Priority::Low);
    agent.acmd("effect_specialshold", effect_specialshold,Priority::Low);
    agent.acmd("sound_specialshold", sound_specialshold,Priority::Low);
    agent.acmd("expression_specialshold", expression_specialshold,Priority::Low);

    agent.acmd("game_specialsholdjump", game_specialsholdjump,Priority::Low);
    agent.acmd("effect_specialsholdjump", effect_specialsholdjump,Priority::Low);
    agent.acmd("sound_specialsholdjump", sound_specialsholdjump,Priority::Low);
    agent.acmd("expression_specialsholdjump", expression_specialsholdjump,Priority::Low);
    
    if !sonic::SPECIAL_TORNADO_SIDEB {
        Agent::new("kirby")
        .acmd("game_sonicspecialnstart", game_specialsstart,Priority::Low)
        .acmd("game_sonicspecialairnstart", game_specialsstart,Priority::Low)
        .acmd("effect_sonicspecialnstart", effect_specialsstart,Priority::Low)
        .acmd("effect_sonicspecialairnstart", effect_specialsstart,Priority::Low)
        .acmd("sound_sonicspecialnstart", sound_specialsstart,Priority::Low)
        .acmd("sound_sonicspecialairnstart", sound_specialsstart,Priority::Low)
        .acmd("expression_sonicspecialnstart", expression_specialsstart,Priority::Low)
        .acmd("expression_sonicspecialairnstart", expression_specialsstart,Priority::Low)
    
        .acmd("game_sonicspecialnspin", game_specialshold,Priority::Low)
        .acmd("effect_sonicspecialnspin", effect_specialshold,Priority::Low)
        .acmd("sound_sonicspecialnspin", sound_specialshold,Priority::Low)
        .acmd("expression_sonicspecialnspin", expression_specialshold,Priority::Low)
    
        .acmd("game_sonicspecialnhit", game_specialsholdjump,Priority::Low)
        .acmd("effect_sonicspecialnhit", effect_specialsholdjump,Priority::Low)
        .acmd("sound_sonicspecialnhit", sound_specialsholdjump,Priority::Low)
        .acmd("expression_sonicspecialnhit", expression_specialsholdjump,Priority::Low)
        .install();
    }
}