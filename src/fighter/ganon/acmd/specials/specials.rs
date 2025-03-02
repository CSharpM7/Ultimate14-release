use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 6.0, 7.5, None,None,None,*FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 11.0, 7.5, None,None,None,*FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn effect_specialairsstart(agent: &mut L2CAgentBase) {
    let mut eff: u32 = 0;
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), 0, 0, 0, -0.286, -45, 25, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent,9.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_engokua_flash"),false,false);

        eff = EffectModule::req_follow(
            agent.module_accessor,
            Hash40::new("ganon_appeal_aura"), 
            Hash40::new("havel"),
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            &VECTOR_ZERO,
            1.0,
            true,
        0,0,0,0,0,false,false) as u32;
        LAST_EFFECT_SET_RATE(agent,1.2);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_raijin_hold"), Hash40::new("arml"), 2, 0, 0, 0, 0, 0, 0.75, true);
    }
    for i in 1..10 {
        let a = (i as f32)*0.1;
        if macros::is_excute(agent) {
            //LAST_EFFECT_SET_ALPHA(agent,a);
            EffectModule::set_alpha(agent.module_accessor,eff,a);
        }
        wait(agent.lua_state_agent, 0.5);
    }
    if macros::is_excute(agent) {
        EffectModule::set_alpha(agent.module_accessor,eff,1.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_raijin_hold"), false, true);
    }
}
unsafe extern "C" fn sound_specialairsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_appeal_s02"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ganon_appeal_s02"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_appeal_l02"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
}

unsafe extern "C" fn expression_specialairsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 18, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhithrow(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 50, 88, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.75);
    }
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialairsstart", game_specialairsstart,Priority::Low);
    agent.acmd("effect_specialairsstart", effect_specialairsstart,Priority::Low);
    agent.acmd("sound_specialairsstart", sound_specialairsstart,Priority::Low);
    agent.acmd("expression_specialairsstart", expression_specialairsstart,Priority::Low);

    agent.acmd("game_specialhithrow", game_specialhithrow,Priority::Low);
}