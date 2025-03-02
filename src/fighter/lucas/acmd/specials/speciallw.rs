use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), -1.5, 0, -1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), -0.3, 0, 0.1, 0, 0, 0, 0.8, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 12, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk_hi"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, true);
    }
}
unsafe extern "C" fn effect_speciallwhold(agent: &mut L2CAgentBase) {
    for i in 1..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 1, 4.5, 0, -90, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(agent,0.5);
            let alpha = 1.0-(i as f32)*0.25;
            LAST_EFFECT_SET_ALPHA(agent,alpha);
        }
        for j in 0..7 {
            if macros::is_excute(agent) {
                macros::FLASH(agent, 0, 1, 1, 0.2);
            }
            wait(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::COL_NORMAL(agent);
            }
            wait(agent.lua_state_agent, 1.0);
        }
    }
}
unsafe extern "C" fn sound_speciallwhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor,Hash40::new("se_lucas_smash_l04"),
            true,false,false,false,app::enSEType(0),
        );
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 4.0, 0);
    }
}


unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 60, 0, 90, 16.0, 0.0, 6.3, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_speciallwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0.2, 0, 0, 0, 0, 0, 0.8, true);
        macros::FLASH(agent, 0.5, 1, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 4, 0, 1, 1, 0.1);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("arml"), 10, 0, 0, 0, 90, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent,1.0,0.5,0.5);
        macros::FLASH_FRM(agent, 6, 0, 0, 1, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 6, 0, 0, 1, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk"), false, false);
        macros::FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_hold"), false, false);
        macros::COL_NORMAL(agent);
    }
}
unsafe extern "C" fn sound_speciallwhit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor,Hash40::new("se_lucas_throw_b01"),
            true,false,false,false,app::enSEType(0),
        );
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 1.0, 0);
    }
}

unsafe extern "C" fn effect_speciallwend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk_lw"), false,false);
        macros::COL_NORMAL(agent);
    }
    for i in 3..5 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 1, 4.5, 0, -90, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(agent,0.75);
            let alpha = 1.0-(i as f32)*0.25;
            LAST_EFFECT_SET_ALPHA(agent,alpha);
        }
        wait(agent.lua_state_agent, 12.0);
    }
}
unsafe extern "C" fn sound_speciallwend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor,Hash40::new("se_lucas_throw_l01"),
            true,false,false,false,app::enSEType(0),
        );
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 0.85, 0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_speciallwstart", empty_acmd,Priority::Low);
    agent.acmd("game_specialairlwstart", empty_acmd,Priority::Low);
    agent.acmd("effect_speciallwstart", effect_speciallwstart,Priority::Low);
    agent.acmd("effect_specialairlwstart", effect_speciallwstart,Priority::Low);

    agent.acmd("effect_speciallwhold", effect_speciallwhold,Priority::Low);
    agent.acmd("effect_specialairlwhold", effect_speciallwhold,Priority::Low);
    agent.acmd("sound_speciallwhold", sound_speciallwhold,Priority::Low);
    agent.acmd("sound_specialairlwhold", sound_speciallwhold,Priority::Low);

    agent.acmd("game_speciallwhit", game_speciallwhit,Priority::Low);
    agent.acmd("game_specialairlwhit", game_speciallwhit,Priority::Low);
    agent.acmd("effect_speciallwhit", effect_speciallwhit,Priority::Low);
    agent.acmd("effect_specialairlwhit", effect_speciallwhit,Priority::Low);
    agent.acmd("sound_speciallwhit", sound_speciallwhit,Priority::Low);
    agent.acmd("sound_specialairlwhit", sound_speciallwhit,Priority::Low);

    agent.acmd("game_speciallwend", empty_acmd,Priority::Low);
    agent.acmd("game_specialairlwend", empty_acmd,Priority::Low);
    agent.acmd("effect_speciallwend", effect_speciallwend,Priority::Low);
    agent.acmd("effect_specialairlwend", effect_speciallwend,Priority::Low);
    agent.acmd("sound_speciallwend", sound_speciallwend,Priority::Low);
    agent.acmd("sound_specialairlwend", sound_speciallwend,Priority::Low);
}