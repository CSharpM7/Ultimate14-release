use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.9);
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor,1,lucina::SPECIAL_S_CHARGE);
    }
}

unsafe extern "C" fn expression_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("handfalchion"),false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("waistfalchion"),true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_drawhold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    for _ in 0..6 {
        if macros::is_excute(agent) {
            if agent.is_grounded() {
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            if agent.is_grounded() {
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_lucina_special_s04h"));
    }
}

unsafe extern "C" fn effect_specialsholdmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
    }
}
unsafe extern "C" fn sound_specialsholdmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_s04h"));
    }
}
unsafe extern "C" fn effect_specialshold(agent: &mut L2CAgentBase) {
    loop {
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            if agent.is_grounded() {
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 15, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            if agent.is_grounded() {
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 15, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

unsafe extern "C" fn sound_specialshold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_cliffcatch"));
    }
}

unsafe extern "C" fn expression_specialshold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialsfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //ArticleModule::shoot_exist(agent.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}
unsafe extern "C" fn effect_specialsfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_specialsfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_s04l"));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //PLAY_VC(agent, Hash40::new("vc_lucina_attack04"),0.5);
        PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_attack"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_swordpullout"));
    }
}

unsafe extern "C" fn expression_specialsfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, lucina::GENERATE_ARTICLE_BOW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("handfalchion"),true);
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("waistfalchion"),false);
        }
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("handfalchion"),true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("waistfalchion"),false);
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

    agent.acmd("effect_specialsholdmax", effect_specialsholdmax,Priority::Low);
    agent.acmd("sound_specialsholdmax", sound_specialsholdmax,Priority::Low);

    agent.acmd("effect_specialshold", effect_specialshold,Priority::Low);
    agent.acmd("effect_specialairshold", effect_specialshold,Priority::Low);
    agent.acmd("sound_specialshold", sound_specialshold,Priority::Low);
    agent.acmd("sound_specialairshold", sound_specialshold,Priority::Low);
    agent.acmd("expression_specialshold", expression_specialshold,Priority::Low);
    agent.acmd("expression_specialairshold", expression_specialshold,Priority::Low);

    agent.acmd("game_specials", game_specialsfire,Priority::Low);
    agent.acmd("game_specialairs", game_specialsfire,Priority::Low);
    agent.acmd("effect_specials", effect_specialsfire,Priority::Low);
    agent.acmd("effect_specialairs", effect_specialsfire,Priority::Low);
    agent.acmd("sound_specials", sound_specialsfire,Priority::Low);
    agent.acmd("sound_specialairs", sound_specialsfire,Priority::Low);
    agent.acmd("expression_specials", expression_specialsfire,Priority::Low);
    agent.acmd("expression_specialairs", expression_specialsfire,Priority::Low);
}