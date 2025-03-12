use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn game_specialhi_1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 0.3);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);

    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 91, 100, 150, 0, 4.8, 0.0, 5.0, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 89, 100, 150, 0, 4.8, 0.0, 5.0, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 90, 100, 135, 0, 4.8, 0.0, 12.5, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 6.0, 91, 100, 135, 0, 4.8, 0.0, 12.5, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTROL);
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
        //damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_FREE_FALL_CHK);
    }

}


pub unsafe extern "C" fn effect_specialhi_1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 0, 8, 0, 0, 0, 0, 1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, false, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 6);
        macros::EFFECT(agent, Hash40::new("chrom_tenku_jump"), Hash40::new("top"), 0, -0.2, 0, 0, 0, 0, 1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_line"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.95);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword2"), 0, 8, 0, 0, 0, 0, 0.75, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
}

pub unsafe extern "C" fn sound_specialhi_1(agent: &mut L2CAgentBase) {
    
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        PLAY_VC(agent, Hash40::new("se_chrom_attack06"),0.3);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_chrom_special_h01"));
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_chrom_attack02"));
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_chrom_rnd_special_h"));
    }
}

pub unsafe extern "C" fn expression_specialhi_1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}




pub unsafe extern "C" fn game_specialhi_2(agent: &mut L2CAgentBase) {
    
}

pub unsafe extern "C" fn effect_specialhi_2(agent: &mut L2CAgentBase) {
    
}

pub unsafe extern "C" fn sound_specialhi_2(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn expression_specialhi_2(agent: &mut L2CAgentBase) {

}

pub unsafe extern "C" fn game_specialhi_3(agent: &mut L2CAgentBase) {
    let mut damage = 4.0;
    let mut angle = 280 as u64;
    let mut kbg = 100;
    let mut bkb = 30;
    frame(agent.lua_state_agent, 1.0); 
    if is_excute(agent) {
        let init_size = 12.0;
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), damage, angle, kbg, 0, bkb, 3.5, 0.0, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), damage, angle, kbg, 0, bkb, 3.8, 0.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0); 
    if is_excute(agent) {
        damage = 3.0;
        angle = 60 as u64;
        kbg = 115;
        bkb = 20;
        macros::ATTACK(agent, 2, 0, Hash40::new("rot"), damage, angle, kbg, 0, bkb, 8.0, 0.0, 1.0, 0.0, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0); 
    if is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), damage, angle, kbg, 0, bkb, 4.2, 2.5, 0.0, 0.7, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), damage, angle, kbg, 0, bkb, 4.2, 2.5, 0.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
}

pub unsafe extern "C" fn effect_specialhi_3(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_sword_down"), Hash40::new("sword1"), 0, 0, 0, 90, 0, -90, 1, true);
        //EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_line"), Hash40::new("top"), 0, 0, 8.5, -180, 0, 0, 1.8, true);
        //macros::LAST_EFFECT_SET_ALPHA(agent, 0.85);
        //EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    
    loop {
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
        wait(agent.lua_state_agent, 6.0);
    }
}

pub unsafe extern "C" fn sound_specialhi_3(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            //macros::PLAY_SE(agent, Hash40::new("se_chrom_swing_l"));
            let sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_chrom_swing_l"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 0.5, 0);
        }
        wait(agent.lua_state_agent, 12.0);
    }
}

pub unsafe extern "C" fn expression_specialhi_3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashsl"), 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_27_spinslash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub unsafe extern "C" fn game_specialhi_3_attack(agent: &mut L2CAgentBase) {
    /* 
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 1.0, 367, 50, 75, 50, 13.0, 0.0, 2.0, 2.25, Some(0.0),Some(0.0),Some(-0.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 1.0, 20, 50, 125, 50, 13.0, 0.0, 2.0, 2.25, Some(0.0),Some(0.0),Some(-0.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        
        //macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 1.1, 365, 50, 55, 50, 3.8, 0.0, 0.0, 1.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 1.1, 365, 50, 55, 50, 3.2, 0.0, 0.0, 7.4, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        

        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
        for i in 0..3 {
            AttackModule::set_add_reaction_frame(agent.module_accessor, i, 1.0, false);
        }
    }*/
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent){
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent){
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 85, 0, 70, 9.0, 0.0, 10.0, 10.2, None,None,None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);

        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 6.0, 60, 85, 0, 70, 3.8, 0.0, 0.0, 1.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 6.0, 60, 85, 0, 70, 3.2, 0.0, 0.0, 7.4, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent){
        AttackModule::clear(agent.module_accessor,0,false);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent){
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_specialhi_3_attack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind_s"), false,false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind_s"), false,false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 4, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, false, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }

}

pub unsafe extern "C" fn sound_specialhi_3_attack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_chrom_special_h05"));
    }
}

pub unsafe extern "C" fn expression_specialhi_3_attack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 8);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 6.0);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhi1", game_specialhi_1,Priority::Low);
    agent.acmd("sound_specialhi1", sound_specialhi_1,Priority::Low);
    agent.acmd("effect_specialhi1", effect_specialhi_1,Priority::Low);
    agent.acmd("expression_specialhi1", expression_specialhi_1,Priority::Low);
    agent.acmd("game_specialairhi1", game_specialhi_1,Priority::Low);
    agent.acmd("sound_specialairhi1", sound_specialhi_1,Priority::Low);
    agent.acmd("effect_specialairhi1", effect_specialhi_1,Priority::Low);
    agent.acmd("expression_specialairhi1", expression_specialhi_1,Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi_2,Priority::Low);
    agent.acmd("sound_specialhi2", sound_specialhi_2,Priority::Low);
    agent.acmd("effect_specialhi2", effect_specialhi_2,Priority::Low);
    agent.acmd("expression_specialhi2", expression_specialhi_2,Priority::Low);

    agent.acmd("game_specialhiadd", game_specialhi_3_attack,Priority::Low);
    agent.acmd("sound_specialhiadd", sound_specialhi_3_attack,Priority::Low);
    agent.acmd("effect_specialhiadd", effect_specialhi_3_attack,Priority::Low);
    agent.acmd("expression_specialhiadd", expression_specialhi_3_attack,Priority::Low);

    agent.acmd("game_specialhi3", game_specialhi_3,Priority::Low);
    agent.acmd("sound_specialhi3", sound_specialhi_3,Priority::Low);
    agent.acmd("effect_specialhi3", effect_specialhi_3,Priority::Low);
    agent.acmd("expression_specialhi3", expression_specialhi_3,Priority::Low);
}