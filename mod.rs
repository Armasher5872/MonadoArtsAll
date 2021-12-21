use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smashline::*;
use smash_script::*;
use smash::phx::Vector3f;

static mut _MONADO_STATE: [&str; 8] = ["none"; 8];
static mut _MONADO_STATE_CHANGE: [&str; 8] = ["none"; 8];
static mut _MONADO_TIMER: [i32; 8] = [0; 8];
static mut _CURRENT_TIMER: [i32; 8] = [0; 8];
static mut _JUMP_COOLDOWN: [i32; 8] = [0; 8];
static mut _SPEED_COOLDOWN: [i32; 8] = [0; 8];
static mut _SHIELD_COOLDOWN: [i32; 8] = [0; 8];
static mut _BUSTER_COOLDOWN: [i32; 8] = [0; 8];
static mut _SMASH_COOLDOWN: [i32; 8] = [0; 8];
static mut _IS_CHANGING: [bool; 8] = [false; 8];
static mut _RED: [f32; 8] = [0.0; 8];
static mut _BLUE: [f32; 8] = [0.0; 8];
static mut _GREEN: [f32; 8] = [0.0; 8];
static mut _GFX_COUNTER: [i32; 8] = [0; 8];

#[fighter_frame_callback]
fn all_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        //println!("Current Monado State: {}", _MONADO_STATE[entry_id]);
        //println!("Current Monado Timer: {}", _CURRENT_TIMER[entry_id]);
        //println!("Current Status Kind: {}", StatusModule::status_kind(module_accessor));
        //println!("Current Motion Kind: {}", MotionModule::motion_kind(module_accessor));
        //println!("Current Motion Frame: {}", MotionModule::frame(module_accessor));
        if MotionModule::motion_kind(module_accessor) == hash40("appeal_s_l") 
        && MotionModule::frame(module_accessor) >= 20.0 
        && MotionModule::frame(module_accessor) <= 21.0 
        || MotionModule::motion_kind(module_accessor) == hash40("appeal_s_r") 
        && MotionModule::frame(module_accessor) >= 20.0 
        && MotionModule::frame(module_accessor) <= 21.0 {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) && _JUMP_COOLDOWN[entry_id] <= 0 { 
                if _MONADO_STATE[entry_id] != "jump" {
                    _MONADO_STATE_CHANGE[entry_id] = "jump";
                    _MONADO_TIMER[entry_id] = 360; 
                    _RED[entry_id] = 0.0;
                    _BLUE[entry_id] = 0.0;
                    _GREEN[entry_id] = 5.0;
                    _IS_CHANGING[entry_id] = true;
                }
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) && _SPEED_COOLDOWN[entry_id] <= 0  {
                if _MONADO_STATE[entry_id] != "speed" {
                    _MONADO_STATE_CHANGE[entry_id] = "speed";
                    _MONADO_TIMER[entry_id] = 480; 
                    _RED[entry_id] = 0.0;
                    _BLUE[entry_id] = 4.0;
                    _GREEN[entry_id] = 4.0;
                    _IS_CHANGING[entry_id] = true;
                }
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) && _SHIELD_COOLDOWN[entry_id] <= 0  {
                if _MONADO_STATE[entry_id] != "shield" {
                    _MONADO_STATE_CHANGE[entry_id] = "shield";
                    _MONADO_TIMER[entry_id] = 360; 
                    _RED[entry_id] = 5.0;
                    _BLUE[entry_id] = 0.0;
                    _GREEN[entry_id] = 5.0;
                    _IS_CHANGING[entry_id] = true;
                }
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) && _BUSTER_COOLDOWN[entry_id] <= 0  {
                if _MONADO_STATE[entry_id] != "buster" {
                    _MONADO_STATE_CHANGE[entry_id] = "buster";
                    _MONADO_TIMER[entry_id] = 600; 
                    _RED[entry_id] = 3.0;
                    _BLUE[entry_id] = 3.0;
                    _GREEN[entry_id] = 0.0;
                    _IS_CHANGING[entry_id] = true;
                }
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && _SMASH_COOLDOWN[entry_id] <= 0  {
                if _MONADO_STATE[entry_id] != "smash" {
                    _MONADO_STATE_CHANGE[entry_id] = "smash";
                    _MONADO_TIMER[entry_id] = 480; 
                    _RED[entry_id] = 5.0;
                    _BLUE[entry_id] = 0.0;
                    _GREEN[entry_id] = 0.0;
                    _IS_CHANGING[entry_id] = true;
                }
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if _MONADO_STATE[entry_id] != "none" {
                    _MONADO_STATE_CHANGE[entry_id] = "none";
                    _MONADO_TIMER[entry_id] = 0;
                    _RED[entry_id] = 2.5;
                    _BLUE[entry_id] = 2.5;
                    _GREEN[entry_id] = 2.5;
                    _IS_CHANGING[entry_id] = true;
                }
            }
        }
        //this just sets the next transition to nothing after you either activate a mode, or if you try to transition into a mode that you're already in
        if _MONADO_STATE_CHANGE[entry_id] == _MONADO_STATE[entry_id] {
            _MONADO_STATE_CHANGE[entry_id] = "none";
            _IS_CHANGING[entry_id] = false;
        }
        //If there is a monado active, the timer counts down
        if _CURRENT_TIMER[entry_id] > 0 {
            _CURRENT_TIMER[entry_id] -= 1;
        }
        //If the timer is 0 while there is a monado state active, set all to default
        if _CURRENT_TIMER[entry_id] <= 0 && _MONADO_STATE[entry_id] != "none" {
            _MONADO_STATE_CHANGE[entry_id] = "none";
            _MONADO_TIMER[entry_id] = 0;
            _RED[entry_id] = 2.5;
            _BLUE[entry_id] = 2.5;
            _GREEN[entry_id] = 2.5;
            _IS_CHANGING[entry_id] = true;
        }
        //If you die or are starting a match, reset the state and timer
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD 
        || sv_information::is_ready_go() == false {
            _MONADO_STATE_CHANGE[entry_id] = "none";
            _MONADO_TIMER[entry_id] = 0;
            _RED[entry_id] = 2.5;
            _BLUE[entry_id] = 2.5;
            _GREEN[entry_id] = 2.5;
            _JUMP_COOLDOWN[entry_id] = 0;
            _SPEED_COOLDOWN[entry_id] = 0;
            _SHIELD_COOLDOWN[entry_id] = 0;
            _BUSTER_COOLDOWN[entry_id] = 0;
            _SMASH_COOLDOWN[entry_id] = 0;
            _IS_CHANGING[entry_id] = true;
        }
        //If the monado state is anything but "none" and the timer for monado runs out, or if the monado state is changing
        if _MONADO_STATE[entry_id] != "none" 
        && _CURRENT_TIMER[entry_id] <= 0 
        || _IS_CHANGING[entry_id] == true {
            if _MONADO_STATE[entry_id] == "jump" {
                _JUMP_COOLDOWN[entry_id] = 1080; //Activate the cooldown for the currently active art
            }
            if _MONADO_STATE[entry_id] == "speed" {
                _SPEED_COOLDOWN[entry_id] = 960;
            }
            if _MONADO_STATE[entry_id] == "shield" {
                _SHIELD_COOLDOWN[entry_id] = 1080;
            }
            if _MONADO_STATE[entry_id] == "buster" {
                _BUSTER_COOLDOWN[entry_id] = 840;
            }
            if _MONADO_STATE[entry_id] == "smash" {
                _SMASH_COOLDOWN[entry_id] = 960;
            }
            _CURRENT_TIMER[entry_id] = _MONADO_TIMER[entry_id]; //Set the active timer to the current monado
            _MONADO_STATE[entry_id] = _MONADO_STATE_CHANGE[entry_id]; //Sets the active monado to the changing value
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
            if _MONADO_STATE[entry_id] == "none" {
                macros::PLAY_SE(fighter, Hash40::new("se_system_cancel"));
            }
            if _MONADO_STATE[entry_id] != "none" {
                macros::PLAY_SE(fighter, Hash40::new("se_common_justshield"));
            }
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.9, true, 0.7);
            macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
            _GFX_COUNTER[entry_id] = 15;
            _IS_CHANGING[entry_id] = false; //Makes sure you aren't reactivating monado every single frame lol
        }
        //If any monado cooldowns are active, they count down every frame
        if _JUMP_COOLDOWN[entry_id] > 0 {
            _JUMP_COOLDOWN[entry_id] -= 1;
        }
        if _SPEED_COOLDOWN[entry_id] > 0 {
            _SPEED_COOLDOWN[entry_id] -= 1;
        }
        if _SHIELD_COOLDOWN[entry_id] > 0 {
            _SHIELD_COOLDOWN[entry_id] -= 1;
        }
        if _BUSTER_COOLDOWN[entry_id] > 0 {
            _BUSTER_COOLDOWN[entry_id] -= 1;
        }
        if _SMASH_COOLDOWN[entry_id] > 0 {
            _SMASH_COOLDOWN[entry_id] -= 1;
        }
        //The actual data for the monado states
        if _MONADO_STATE[entry_id] == "jump" { //JUMP MONADO STATE
            _RED[entry_id] = 0.0;
            _BLUE[entry_id] = 0.0;
            _GREEN[entry_id] = 5.0;
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP_AERIAL && MotionModule::frame(module_accessor) < 1.0 {
                KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: 700000.0, z: 0.0} as *const Vector3f);
            }
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                smash::app::lua_bind::FighterKineticEnergyController::mul_x_speed_max(control_energy, 1100000.0);
                let lua_state = fighter.lua_state_agent;
                acmd!(lua_state, {
                    sv_kinetic_energy::set_accel_x_add(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 100000.0)
                    sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1611000.0)
                    sv_kinetic_energy::friction_off()
                });
                if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1300000.0);
                }
            }
            AttackModule::set_power_up(module_accessor, 1.0);
            AttackModule::set_reaction_mul(module_accessor, 1.0);
            DamageModule::set_damage_mul(module_accessor, 1300000.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            ShieldModule::set_hit_stop_mul(module_accessor, 1.0);
        }
        if _MONADO_STATE[entry_id] == "speed" { //SPEED MONADO STATE
            _RED[entry_id] = 0.0;
            _BLUE[entry_id] = 4.0;
            _GREEN[entry_id] = 4.0;
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP_AERIAL && MotionModule::frame(module_accessor) < 1.0 {
                KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: -80000.0, z: 0.0} as *const Vector3f);
            }
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                let lua_state = fighter.lua_state_agent;
                acmd!(lua_state, {
                    sv_kinetic_energy::set_accel_x_add(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 100000.0)
                    sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1262000.0)
                    sv_kinetic_energy::set_accel_y_mul(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1200000.0)
                });
                if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1100000.0);
                }
            }
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1300000.0);
                }
                let lua_state = fighter.lua_state_agent;
                acmd!(lua_state, {
                    sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 2680000.0)
                    sv_kinetic_energy::set_accel_x_mul(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1500000.0)
                    sv_kinetic_energy::set_limit_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 2680000.0)
                    sv_kinetic_energy::set_brake(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 2500000.0)
                });
            }
            AttackModule::set_power_up(module_accessor, 0.0000007);
            AttackModule::set_reaction_mul(module_accessor, 1.0);
            DamageModule::set_damage_mul(module_accessor, 1.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            ShieldModule::set_hit_stop_mul(module_accessor, 1.0);
        }
        if _MONADO_STATE[entry_id] == "shield" { //SHIELD MONADO STATE
            _RED[entry_id] = 5.0;
            _BLUE[entry_id] = 0.0;
            _GREEN[entry_id] = 5.0;
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                let lua_state = fighter.lua_state_agent;
                acmd!(lua_state, {
                    sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.000000581)
                });
                smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.0000006);
            }
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.0000006);
                let lua_state = fighter.lua_state_agent;
                acmd!(lua_state, {
                    sv_kinetic_energy::mul_x_speed_max(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0000006)
                    sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.000000804)
                });
            }
            DamageModule::set_damage_mul(module_accessor, 0.0000005);
            DamageModule::set_reaction_mul(module_accessor, 0.0000006);
            ShieldModule::set_attack_mul(module_accessor, 0.0000005, *FIGHTER_SHIELD_KIND_GUARD);
            ShieldModule::set_hit_stop_mul(module_accessor, 0.0000008);
            AttackModule::set_power_up(module_accessor, 0.0000005);
            AttackModule::set_reaction_mul(module_accessor, 0.0000008);
        }
        if _MONADO_STATE[entry_id] == "buster" { //BUSTER MONADO STATE
            _RED[entry_id] = 3.0;
            _BLUE[entry_id] = 3.0;
            _GREEN[entry_id] = 0.0;
            AttackModule::set_power_up(module_accessor, 1400000.0);
            AttackModule::set_reaction_mul(module_accessor, 0.00000065);
            DamageModule::set_damage_mul(module_accessor, 1300000.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            ShieldModule::set_hit_stop_mul(module_accessor, 1.0);
        }
        if _MONADO_STATE[entry_id] == "smash" { //SMASH MONADO STATE
            _RED[entry_id] = 5.0;
            _BLUE[entry_id] = 0.0;
            _GREEN[entry_id] = 0.0;
            AttackModule::set_power_up(module_accessor, 0.0000003);
            AttackModule::set_reaction_mul(module_accessor, 1250000.0);
            DamageModule::set_reaction_mul(module_accessor, 1200000.0);
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            DamageModule::set_damage_mul(module_accessor, 1.0);
        }
        if _MONADO_STATE[entry_id] == "none" { //NO MONADO STATE
            DamageModule::set_damage_mul(module_accessor, 1.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            ShieldModule::set_hit_stop_mul(module_accessor, 1.0);
            AttackModule::set_power_up(module_accessor, 1.0);
            AttackModule::set_reaction_mul(module_accessor, 1.0);
        }

        if _MONADO_STATE[entry_id] != "none" { //GFX AND SHIT
            _GFX_COUNTER[entry_id] += 1;
            if _GFX_COUNTER[entry_id] == 10 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.5, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.5, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.6, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.6, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
            }
            if _GFX_COUNTER[entry_id] >= 20 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.5, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.5, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.6, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.6, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, _RED[entry_id], _GREEN[entry_id], _BLUE[entry_id]);
                _GFX_COUNTER[entry_id] = 0;
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        all_frame
    );
}
