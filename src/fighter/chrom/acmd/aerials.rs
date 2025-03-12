use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn game_landingairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 60, 105, 0, 65, 4.0, 0.0, 5.7, -4.5, Some(0.0), Some(5.7), Some(7.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 0, 0.2);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let mut damage = 1.2;
    let mut angle = 366 as u64;
    let mut kbg = 0;
    let mut bkb = 50;
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    FT_MOTION_RATE(agent,0.75);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), damage, angle, kbg, 10, bkb, 2.9, 0.0, 0.0, 0.7, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), damage, angle, kbg, 10, bkb, 2.9, 0.0, 0.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), damage, angle, kbg, 10, bkb, 6.0, 0.0, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
        VarModule::on_flag(agent.battle_object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        damage = 6.0;
        angle = 50;
        kbg = 125;
        bkb = 45;
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), damage, angle, kbg, 0, bkb, 4.2, 2.5, 0.0, 0.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), damage, angle, kbg, 0, bkb, 4.2, 2.5, 0.0, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), damage, angle, kbg, 0, bkb, 8.0, 0.0, 7.5, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        VarModule::off_flag(agent.battle_object,fighter::instance::flag::AERIAL_ENABLE_LANDING_HITBOX);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 66.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_light"), Hash40::new("sword1"), 0, 0, 11, 0, 0, 0, 0.4, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_sword_spin"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 90, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.75);
    }
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 8, -2, 0, 180, -90, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 8, -1.5, 0, 90, -90, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("chrom_tenku_sword_spin"), false, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 9, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_light"), false, true);
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_chrom_rnd_attack"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_chrom_swing_l"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 0.5, 0);
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_chrom_swing_l"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 0.5, 0);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_chrom_special_h05"));
    }
}

unsafe extern "C" fn expression_attackairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 8);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.25, 270, 90, 0, 20, 3.75, 0.0, -4.625, 0.4, Some(0.0), Some(-6.75), Some(0.4), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.25, 80, 90, 0, 20, 3.75, 0.0, -4.625, 0.4, Some(0.0), Some(-6.75), Some(0.4), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 11.875, 80, 90, 0, 20, 5.4, 0.0, -6.8, 0.4, Some(0.0), Some(-1.0), Some(0.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install(agent: &mut smashline::Agent) {

    agent.acmd("game_landingairn", game_landingairn,Priority::Low);
    agent.acmd("game_attackairn", game_attackairn,Priority::Low);
    agent.acmd("effect_attackairn", effect_attackairn,Priority::Low);
    agent.acmd("sound_attackairn", sound_attackairn,Priority::Low);
    agent.acmd("expression_attackairn", expression_attackairn,Priority::Low);
    
    agent.acmd("game_attackairlw", game_attackairlw,Priority::Low);
}