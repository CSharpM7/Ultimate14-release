use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        //ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, false, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_BULLET_DISP);
    }
}
unsafe extern "C" fn game_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_CHARGE);
    }
}

unsafe extern "C" fn effect_specialnhold(agent: &mut L2CAgentBase) {
    let is_kirby = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let bone = if is_kirby {Hash40::new("handl")} else {Hash40::new("havel")};
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_cshot_hold"), bone, 0,0,0, -91.273, -1.797, 176.373, 0.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_cshot_bullet_sub_a"), bone, 0,0,0, -91.273, -1.797, 176.373, 0.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_cshot_bullet_sub_b"), bone, 0,0,0, -91.273, -1.797, 176.373, 0.5, true);

        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), bone, 0,0,0, -91.273, -1.797, 176.373, 0.15, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        let handle = EffectModule::get_last_handle(agent.module_accessor);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 2.0);
    loop {
        if macros::is_excute(agent) {
            if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
                //macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 12, 0, 12, 0, 0, 0, false);
                macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            }
        }
        wait(agent.lua_state_agent, 12.0);
    }
}

unsafe extern "C" fn sound_specialnhold(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_samusd_win03_02"));
    }
}

pub const FRAME_MIN: f32 = 11.0;
pub const FRAME_MAX: f32 = 15.0;

unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,FRAME_MIN,11.0);
    frame(agent.lua_state_agent, FRAME_MIN);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_CHARGE);
        let charge = WorkModule::get_int(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
        let charge_max = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame"));
        let c = charge / charge_max;

        let damage = lerp(7.0,22.0,c);
        let angle = if c < 0.5 {361 as u64} else {52 as u64};
        let kbg = lerp(40.0,60.0,c) as i32; //+0
        let bkb = lerp(60.0,60.0,c) as i32; //40
        let size = 10.0;
        let level = if c < 0.5 {*ATTACK_SOUND_LEVEL_S} else {*ATTACK_SOUND_LEVEL_M};
        let hitlag = if c < 0.5 {1.1} else {1.25};

        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), damage, angle, kbg, 0, bkb, size, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1.0, 0.0, -5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), level, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialnfiremax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,FRAME_MAX,10.0);
    frame(agent.lua_state_agent, FRAME_MAX);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_CHARGE);

        let damage = 25.0;
        let angle = 52 as u64;
        let kbg = 60; //+0
        let bkb = 60; //40
        let size = 10.0;
        let level = *ATTACK_SOUND_LEVEL_L;
        let hitlag = 1.5;

        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), damage, angle, kbg, 0, bkb, size, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1.0, 0.0, -5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), level, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        //AttackModule::set_poison_param(agent.module_accessor, /*ID*/ 1, /*Frames*/ 300, /*Rehit*/ 30, /* Damage*/ 1.0, /*Unk*/ false);
        FT_ADD_DAMAGE(agent, 10.0);

        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot"), damage*0.5, angle, kbg, 0, bkb, size*1.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, -5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            //AttackModule::set_poison_param(agent.module_accessor, /*ID*/ 0, /*Frames*/ 150, /*Rehit*/ 30, /* Damage*/ 1.0, /*Unk*/ false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialnfire(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, FRAME_MIN);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_atk_bomb"), Hash40::new("rot"), -4.0, 0.0,0.0, 0, 0, 0, 3.5, true);
        LAST_EFFECT_SET_ALPHA(agent,0.5);
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
unsafe extern "C" fn effect_specialnfiremax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 0.753, 1, 0.706);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 10, 0.314, 0.314, 0.314, 0);
    }
    frame(agent.lua_state_agent, FRAME_MAX);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_atk_bomb"), Hash40::new("rot"), -4.0, 0.0,0.0, 0, 0, 0, 3.5, true);
        LAST_EFFECT_SET_ALPHA(agent,0.5);
        macros::COL_NORMAL(agent);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("samusd_atk_bomb"),0);
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_specialnfire(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, FRAME_MIN);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samusd_special_n02"));
    }
}
unsafe extern "C" fn sound_specialnfiremax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, FRAME_MAX-2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_samusd_special_n05"));
    }
}

unsafe extern "C" fn expression_specialnfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, FRAME_MAX-1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let charge = WorkModule::get_int(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
        let charge_max = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame"));
        let c = charge / charge_max;
        let rumble_type = if c < 0.5 {Hash40::new("rbkind_attacks")} else {Hash40::new("rbkind_attackm")};
        macros::RUMBLE_HIT(agent, rumble_type, 0);
    }
}
unsafe extern "C" fn expression_specialnfiremax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, FRAME_MAX-1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
}



pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialnstart", game_specialnstart,Priority::Low);
    agent.acmd("game_specialairnstart", game_specialnstart,Priority::Low);

    agent.acmd("effect_specialnhold", effect_specialnhold,Priority::Low);
    agent.acmd("effect_specialairnhold", effect_specialnhold,Priority::Low);
    agent.acmd("sound_specialnhold", sound_specialnhold,Priority::Low);
    agent.acmd("sound_specialairnhold", sound_specialnhold,Priority::Low);
    
    agent.acmd("game_specialnfire", game_specialnfire,Priority::Low);
    agent.acmd("game_specialairnfire", game_specialnfire,Priority::Low);
    agent.acmd("effect_specialnfire", effect_specialnfire,Priority::Low);
    agent.acmd("effect_specialairnfire", effect_specialnfire,Priority::Low);
    agent.acmd("sound_specialnfire", sound_specialnfire,Priority::Low);
    agent.acmd("sound_specialairnfire", sound_specialnfire,Priority::Low);
    agent.acmd("expression_specialnfire", expression_specialnfire,Priority::Low);
    agent.acmd("expression_specialairnfire", expression_specialnfire,Priority::Low);

    agent.acmd("game_specialnfiremax", game_specialnfiremax,Priority::Low);
    agent.acmd("game_specialairnfiremax", game_specialnfiremax,Priority::Low);
    agent.acmd("effect_specialnfiremax", effect_specialnfiremax,Priority::Low);
    agent.acmd("effect_specialairnfiremax", effect_specialnfiremax,Priority::Low);
    agent.acmd("sound_specialnfiremax", sound_specialnfiremax,Priority::Low);
    agent.acmd("sound_specialairnfiremax", sound_specialnfiremax,Priority::Low);
    agent.acmd("expression_specialnfiremax", expression_specialnfiremax,Priority::Low);
    agent.acmd("expression_specialairnfiremax", expression_specialnfiremax,Priority::Low);

    
    Agent::new("kirby")
    .acmd("effect_samusdspecialnhold", effect_specialnhold,Priority::Low)
    .acmd("effect_samusdspecialairnhold", effect_specialnhold,Priority::Low)
    .acmd("sound_samusdspecialnhold", sound_specialnhold,Priority::Low)
    .acmd("sound_samusdspecialairnhold", sound_specialnhold,Priority::Low)
    
    .acmd("game_samusdspecialnfire", game_specialnfire,Priority::Low)
    .acmd("game_samusdspecialairnfire", game_specialnfire,Priority::Low)
    .acmd("effect_samusdspecialnfire", effect_specialnfire,Priority::Low)
    .acmd("effect_samusdspecialairnfire", effect_specialnfire,Priority::Low)
    .acmd("sound_samusdspecialnfire", sound_specialnfire,Priority::Low)
    .acmd("sound_samusdspecialairnfire", sound_specialnfire,Priority::Low)
    .acmd("expression_samusdspecialnfire", expression_specialnfire,Priority::Low)
    .acmd("expression_samusdspecialairnfire", expression_specialnfire,Priority::Low)

    .acmd("game_samusdspecialnfiremax", game_specialnfiremax,Priority::Low)
    .acmd("game_samusdspecialairnfiremax", game_specialnfiremax,Priority::Low)
    .acmd("effect_samusdspecialnfiremax", effect_specialnfiremax,Priority::Low)
    .acmd("effect_samusdspecialairnfiremax", effect_specialnfiremax,Priority::Low)
    .acmd("sound_samusdspecialnfiremax", sound_specialnfiremax,Priority::Low)
    .acmd("sound_samusdspecialairnfiremax", sound_specialnfiremax,Priority::Low)
    .acmd("expression_samusdspecialnfiremax", expression_specialnfiremax,Priority::Low)
    .acmd("expression_samusdspecialairnfiremax", expression_specialnfiremax,Priority::Low)
    .install();
}