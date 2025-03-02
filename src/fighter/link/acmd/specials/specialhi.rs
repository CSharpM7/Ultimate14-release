use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_squat"));
    }
}


pub unsafe extern "C" fn expression_specialhistart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}


pub unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let is_air = MotionModule::motion_kind(agent.module_accessor) == Hash40::new("game_specialairhi").hash;

    let charge = WorkModule::get_float(agent.module_accessor,*FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let max_charge = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32;
    let ratio = WorkModule::get_float(agent.module_accessor, link::SPECIAL_HI_CHARGE);
    //let ratio = charge/max_charge;
    let bkb = 92;
    let kbg = 47;// + (15.0*ratio) as i32;
    let size = 6.0;//+(ratio*3.5);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if !is_air{
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 83, kbg, 0, bkb, size, 0.0, 6.0, 4.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 83, kbg, 0, bkb, size, 0.0, 6.0, -3.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        else{
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 83, kbg, 0, bkb, 7.5, 0.0, 6.5, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }

        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 76, kbg, 0, bkb+15, 6.0, 0.0, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 76, kbg, 0, bkb+15, 6.0, 0.0, 18.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term_group(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        WorkModule::enable_transition_term_group(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        WorkModule::enable_transition_term_group(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        CancelModule::enable_cancel(agent.module_accessor);
    }
}


pub unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let charge = WorkModule::get_float(agent.module_accessor,*FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
        let max_charge = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32;
        //let ratio = charge/max_charge;
        let ratio = WorkModule::get_float(agent.module_accessor, link::SPECIAL_HI_CHARGE);
        //FOOT_EFFECT(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0+(ratio*0.5), 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("vc_link_special_h02"));
        //macros::PLAY_SE(agent, Hash40::new("se_link_special_h07"));
        macros::PLAY_SE(agent, Hash40::new("vc_link_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_item_club_swing"));
    }
    /*
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_special_h08"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_special_h09"));
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_special_h11"));
        macros::PLAY_SE(agent, Hash40::new("se_link_special_h10"));
    } */
}

pub unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

pub unsafe extern "C" fn sound_specialhifall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_link_appear01"));
    }
}


pub unsafe extern "C" fn expression_specialhifall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}


pub unsafe extern "C" fn game_specialhihold(agent: &mut L2CAgentBase) {

}

pub unsafe extern "C" fn effect_specialhihold(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.6);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        
        //macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ, 0.7);
    }
    for i in 1..i32::MAX{
        wait(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.6);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
            //macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ, 0.7);
        }
    }
}

pub unsafe extern "C" fn sound_specialhihold(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_link_special_h01"));
    }
}

pub unsafe extern "C" fn expression_specialhihold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub unsafe extern "C" fn sound_landingfallspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
            macros::PLAY_SE(agent, Hash40::new("se_link_appear02"));
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_link_landing03"));
    }
}

pub unsafe extern "C" fn expression_landingfallspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent,4.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(0));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhistart", empty_acmd,Priority::Low);
    agent.acmd("game_specialairhistart", empty_acmd,Priority::Low);
    agent.acmd("effect_specialhistart", empty_acmd,Priority::Low);
    agent.acmd("effect_specialairhistart", empty_acmd,Priority::Low);
    agent.acmd("sound_specialhistart", sound_specialhistart,Priority::Low);
    agent.acmd("sound_specialairhistart", sound_specialhistart,Priority::Low);
    agent.acmd("expression_specialhistart", expression_specialhistart,Priority::Low);
    agent.acmd("expression_specialairhistart", expression_specialhistart,Priority::Low);

    agent.acmd("game_specialhihold", game_specialhihold,Priority::Low);
    agent.acmd("game_specialairhihold", game_specialhihold,Priority::Low);
    agent.acmd("effect_specialhihold", effect_specialhihold,Priority::Low);
    agent.acmd("effect_specialairhihold", effect_specialhihold,Priority::Low);
    agent.acmd("sound_specialhihold", sound_specialhihold,Priority::Low);
    agent.acmd("sound_specialairhihold", sound_specialhihold,Priority::Low);
    agent.acmd("expression_specialhihold", expression_specialhihold,Priority::Low);
    agent.acmd("expression_specialairhihold", expression_specialhihold,Priority::Low);
    
    agent.acmd("game_specialhi", game_specialhi,Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi,Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi,Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialhi,Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi,Priority::Low);
    agent.acmd("sound_specialairhi", sound_specialhi,Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi,Priority::Low);
    agent.acmd("expression_specialairhi", expression_specialhi,Priority::Low);

    agent.acmd("game_specialhifall", empty_acmd,Priority::Low);
    agent.acmd("game_specialairhifall", empty_acmd,Priority::Low);
    agent.acmd("effect_specialhifall", empty_acmd,Priority::Low);
    agent.acmd("effect_specialairhifall", empty_acmd,Priority::Low);
    agent.acmd("sound_specialhifall", sound_specialhifall,Priority::Low);
    agent.acmd("sound_specialairhifall", sound_specialhifall,Priority::Low);
    agent.acmd("expression_specialhifall", expression_specialhifall,Priority::Low);
    agent.acmd("expression_specialairhifall", expression_specialhifall,Priority::Low);

    
    agent.acmd("sound_landingfallspecial", sound_landingfallspecial,Priority::Low);
    agent.acmd("expression_landingfallspecial", expression_landingfallspecial,Priority::Low);
    
}