use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,4.0,5.0);
    frame(agent.lua_state_agent, 4.0);
    FT_MOTION_RATE(agent,1.0);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 12.0, 361, 75, 0, 55, 3.8, 4.6, 0.0, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 12.0, 361, 75, 0, 55, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 12.0, 361, 75, 0, 55, 2.0, 1.1, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let motion = MotionModule::motion_kind(agent.module_accessor);
        let is_hi = motion == Hash40::new("attack_s3_lw").hash;
        let is_lw = motion == Hash40::new("attack_s3_lw").hash;
        let x = if is_lw {-2} else {1};
        let y = if is_lw {3} else if is_hi {7} else {2};
        let z = if is_hi {0.5} else {2.0};

        let rx = if is_lw {5} else if is_hi {30} else {10};
        let ry = if is_lw {-90} else if is_hi {-60} else {-39};
        let rz = if is_lw {170} else if is_hi {135} else {154};
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), x,y,z,rx,ry,rz, 0.95, true, *EF_FLIP_YZ);
        //LAST_EFFECT_SET_COLOR(agent,0.1,0.75,1.0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);

        macros::EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_impact"), Hash40::new("kneer"), 4.0, 0.0, 0.3, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(agent,1.5);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attacks3lw", game_attacks3,Priority::Low);
    agent.acmd("game_attacks3", game_attacks3,Priority::Low);
    agent.acmd("game_attacks3hi", game_attacks3,Priority::Low);
    agent.acmd("effect_attacks3lw", effect_attacks3,Priority::Low);
    agent.acmd("effect_attacks3", effect_attacks3,Priority::Low);
    agent.acmd("effect_attacks3hi", effect_attacks3,Priority::Low);
}