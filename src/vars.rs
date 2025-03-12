pub mod fighter {
    pub mod instance {
        pub mod flag {
            pub const AERIAL_ENABLE_LANDING_HITBOX : i32 = 0x0000;
            pub const AERIAL_LANDING_HITBOX_CHECK : i32 = 0x0001;

            pub const DISABLE_SPECIAL_S: i32 = 0x0002;
            pub const DISABLE_JUMP: i32 = 0x0003;
        }
        pub mod int {
            pub const TARGET_ID : i32 = 0x0000;
        }
        pub mod float {
            pub const FLICK_DOWN : i32 = 0x0000;
            pub const JUMP_SQUAT_SPEED_X : i32 = 0x0001;
            pub const CURRENT_SPEED_X : i32 = 0x0002;
        }
    }
    pub mod status {
        pub mod flag {
            pub const JUMP_CANCEL : i32 = 0x1000;
        }
        pub mod int {
            pub const ENABLED_AERIALS : i32 = 0x1000;
        }
        pub mod float {
            pub const HIT_FRAME : i32 = 0x1000;
        }
    }
}

pub mod mario {
    pub const STATUS_KIND_LAST: i32 = 0x1e3; //0x1e3 0x15e;
    pub const STATUS_KIND_CAPJUMP: i32 = STATUS_KIND_LAST+1;
    pub const STATUS_KIND_CAPDIVE: i32 = STATUS_KIND_LAST+2;
    pub const STATUS_KIND_CAPCATCH: i32 = STATUS_KIND_LAST+4; //+3 causes a crash

    pub static mut GENERATE_ARTICLE_CAPTOSS: i32 = 6;
    pub mod instance {
        pub mod flag {
            pub const HATLESS: i32 = 0x0100;
            pub const CAPJUMP_ENABLED: i32 = 0x0101;
            pub const CAPDIVE_ENABLED: i32 = 0x0102;
            pub const CAPDIVE_ENABLE_ON_RETURN: i32 = 0x0103;
        }
        pub mod int {
            pub const CAP_TIMER: i32 = 0x0100;
        }
    }
}
pub mod mario_cappy {
    pub const CAPTOSS_STATUS_KIND_START: i32 = 0;
    pub const CAPTOSS_STATUS_KIND_HAVED: i32 = 1; //unused
    pub const CAPTOSS_STATUS_KIND_FLY: i32 = 2;
    pub const CAPTOSS_STATUS_KIND_TURN: i32 = 3;
    pub const CAPTOSS_STATUS_KIND_HOP: i32 = 4;
    pub const CAPTOSS_STATUS_KIND_HOLD: i32 = 5;
    pub const CAPTOSS_STATUS_KIND_SWALLOWED: i32 = 6;
    pub const CAPTOSS_STATUS_KIND_JUMP: i32 = 7;
    pub const CAPTOSS_STATUS_KIND_POCKET: i32 = 8; //unused

    pub const CAPDIVE_ENABLE_ON_RETURN: i32 = 0x20000008;

    pub mod instance {
        pub mod flag {
            pub const CAPDIVE_ENABLE_ON_RETURN: i32 = 0x0100;
        }
    }
}
pub mod link {
    pub const STATUS_KIND_SPECIAL_HI_JUMP: i32 = 0x1e8;
    pub const STATUS_KIND_SHIELD_ATTACK: i32 = 0x1e9;
    
    pub const SPECIAL_HI_CHARGE:i32 = 0x58;
    /*
    FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW2_BLANK_TIME: 0x4F,
    FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_DELAY_MUL: 0x57,
    FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FINAL_DX: 0x4D,
    FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FINAL_DY: 0x4E,
    FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_LANDING_PREV_MOTION_FRAME: 0x53, */
    pub mod instance {
        pub mod int {
        }
        pub mod float {
        }
    }
    pub mod status {
        pub mod flag {
            pub const SHIELD_ATTACK_SUCCEEDED: i32 = 0x1000;
        }
        pub mod int {
        }
        pub mod float {
            pub const SPECIAL_HI_CHARGE: i32 = 0x1000;
        }
    }
}

pub mod samusd {
    pub const STATUS_KIND_SPECIAL_HI_RUSH: i32 = 0x1f6;
    pub const STATUS_KIND_SPECIAL_HI_END: i32 = 0x1f7;
    pub const STATUS_KIND_SPECIAL_HI_BOUND: i32 = 0x1f8;
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_LW_CAN_EXIT: i32 = 0x0100;
            pub const SPECIAL_LW_EXIT: i32 = 0x0101;
        }
        pub mod int {
            pub const CHARGE_EFF: i32 = 0x0100;
        }
        pub mod float {
            pub const RUSH_INPUT_X: i32 = 0x0100;
            pub const RUSH_INPUT_Y: i32 = 0x0101;
            pub const RUSH_DEGREE: i32 = 0x0102;
        }
    }
    pub mod status {
        pub mod int {
            pub const RUSH_FRAME: i32 = 0x0101;
        }
    }
}

pub mod kirby {
    pub mod instance {
        pub mod flag {
            pub const RICHTER_BOOK: i32 = 0x0100;
        }
        pub mod float {
            pub const LUCAS_COOLDOWN: i32 = 0x0100;
        }
    }
}
pub mod lucina {
    pub static mut GENERATE_ARTICLE_BOW : i32 = 1;
    pub static mut GENERATE_ARTICLE_BOWARROW : i32 = 2;
    pub const SPECIAL_S_CHARGE : i32 = 0x100000BF;
    pub const SPECIAL_S_CHARGE_MAX : i32 = 60;
    pub const SPECIAL_S_HOLD_MAX : i32 = 120;
    
    pub mod instance {
        pub mod flag {
            pub static mut IS_MASKED : [bool; 8] = [false; 8];
        }
    }
}
pub mod ganon {
    pub const SPECIAL_HI_FUEL_MAX : i32 = 120;
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_HI_START_RECHARGE: i32 = 0x0100;
            pub const SPECIAL_HI_COOLDOWN: i32 = 0x0101;
            pub const SPECIAL_S_DISABLE: i32 = 0x0102;
        }
        pub mod float {
            pub const SPECIAL_HI_START_FRAME: i32 = 0x0100;
        }
    }
}
pub mod pitb {
    pub static mut GENERATE_ARTICLE_ORBITER : i32 = 2;
    pub const ORBITER_SPAWN_X : f32 = 9.0;
    pub const ORBITER_SPAWN_Y : f32 = 12.0;
    pub const SPECIAL_HI_FUEL_MAX : i32 = 180;
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_HI_START_RECHARGE: i32 = 0x0100;
            pub const SPECIAL_HI_COOLDOWN: i32 = 0x0101;
        }
        pub mod float {
            pub const SPECIAL_HI_START_FRAME: i32 = 0x0100;
        }
    }
}

pub mod lucas {
    pub const SPECIAL_N_COOLDOWN : i32 = 0x100000C1;
    pub const SPECIAL_N_COOLDOWN_STATUS : i32 = 0x1100000B;
    pub const SPECIAL_N_COOLDOWN_MAX : i32 = 1800;

    pub const SPECIAL_N_HEAL : i32 = 0x1000009;
    pub const SPECIAL_N_HEAL_FRAME : i32 = 0x100000A;
    pub const SPECIAL_N_HEAL_SIZE : i32 = 0x100000B;
}
pub mod lucas_pkfreeze {
    pub const INSTANCE_FLOAT_FRESH_FACTOR: i32 = 0x2;
    pub const INSTANCE_FLOAT_HEAL: i32 = 0x4;
}

pub mod richter {
    pub const BOOK_STATUS_KIND_SHIELD : i32 = 3;
    pub const STATUS_KIND_SPECIAL_S_DASH : i32 = 0x1EC;
    pub mod instance {
        pub mod flag {
            pub const HAS_BOOK: i32 = 0x0100;
        }
    }
}

pub mod sonic {
    pub const SPECIAL_TORNADO_SIDEB : bool = false;

    pub const INSTANCE_TORNADO_DISABLE : i32 = 0x200000E4; //FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_TERM
    pub const INSTANCE_HOMING_DISABLE : i32 = 0x200000E5; 
    pub const SPECIAL_TORNADO_TORNADO_ID: i32 = 0x1000000C; //FIGHTER_SONIC_INSTANCE_WORK_ID_INT_TERM
    
    pub const SPECIAL_TORNADO_BALL : i32 = 0x2100000C; //FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_FLAG_SPECIAL_LW_HOLD
    pub const SPECIAL_TORNADO_SPAWN_TORNADO : i32 = 0x2100000D;
    pub const SPECIAL_TORNADO_SPAWN_EFF : i32 = 0x2100000E;
    pub const SPECIAL_TORNADO_EFF: i32 = 0x11000007; //FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_JUMP_WORK_INT_LIMIT_COUNTER
    pub const SPECIAL_TORNADO_OFFSET_Y : i32 = 0x1000007; //FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_JUMP_WORK_FLOAT_ADVANCE_COUNTER
}

pub mod toonlink {    
    pub const SPECIAL_S_FLAG_CAN_SEARCH: i32 = 0x2100000E;
    pub const SPECIAL_S_FLAG_SEARCH: i32 = 0x2100000F;
    pub const SPECIAL_S_TARGET_ID : i32 = 0x11000007;
    pub const SPECIAL_S_TARGET_ID_PREV : i32 = 0x11000008;
    pub const SPECIAL_S_HOMING_EFF_ID : i32 = 0x11000009;
    
    pub const BOOMERANG_STATUS_KIND_HOMING : i32 = 6;
    pub const BOOMERANG_HOMING_FACTOR : f32 = 5.0; //37??
    pub const BOOMERANG_HOMING_LERP : f32 = 0.65; 
    pub const BOOMERANG_HOMING_MAX : f32 = 60.0;
    pub const BOOMERANG_TARGET_ID : i32 = 0x1000000B; //life
    pub const BOOMERANG_HOMING_EFF_ID : i32 = 0x11000001;
    pub const BOOMERANG_FLAG_SEARCH : i32 = 0x2000000C;
}

pub mod shizue {
    pub const FLAG_ATTACK_AIR_HOP : i32 = 0x200000F2;
}
pub mod packun {
    pub const SPECIAL_S_BREATH : i32 = 0x2100000f;
    pub const BREATH_SHOOT_BOOST : i32 = 0x1000000b;
    pub const BREATH_SHOOT_BOOST_LR : i32 = 0x4;
}
pub mod element {
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_CANCEL: i32 = 0x0100;
            pub const SPECIAL_CANCEL_DISABLED: i32 = 0x0101;
            pub const SWAP_DISABLE: i32 = 0x0102;
        }
    }
}