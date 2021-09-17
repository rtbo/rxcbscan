// Generated automatically from xkb.xml by rs_client.py version 0.9.0.
// Do not edit!

use ffi::base::*;
use ffi::xproto::*;
use libc::{c_char, c_int, c_uint, c_void};
use std;

pub const XCB_XKB_MAJOR_VERSION: u32 = 1;
pub const XCB_XKB_MINOR_VERSION: u32 = 0;

pub type xcb_xkb_const_t = u32;
pub const XCB_XKB_CONST_MAX_LEGAL_KEY_CODE: xcb_xkb_const_t = 0xff;
pub const XCB_XKB_CONST_PER_KEY_BIT_ARRAY_SIZE: xcb_xkb_const_t = 0x20;
pub const XCB_XKB_CONST_KEY_NAME_LENGTH: xcb_xkb_const_t = 0x04;

pub type xcb_xkb_event_type_t = u32;
pub const XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY: xcb_xkb_event_type_t = 0x01;
pub const XCB_XKB_EVENT_TYPE_MAP_NOTIFY: xcb_xkb_event_type_t = 0x02;
pub const XCB_XKB_EVENT_TYPE_STATE_NOTIFY: xcb_xkb_event_type_t = 0x04;
pub const XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY: xcb_xkb_event_type_t = 0x08;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY: xcb_xkb_event_type_t = 0x10;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY: xcb_xkb_event_type_t = 0x20;
pub const XCB_XKB_EVENT_TYPE_NAMES_NOTIFY: xcb_xkb_event_type_t = 0x40;
pub const XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY: xcb_xkb_event_type_t = 0x80;
pub const XCB_XKB_EVENT_TYPE_BELL_NOTIFY: xcb_xkb_event_type_t = 0x100;
pub const XCB_XKB_EVENT_TYPE_ACTION_MESSAGE: xcb_xkb_event_type_t = 0x200;
pub const XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY: xcb_xkb_event_type_t = 0x400;
pub const XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY: xcb_xkb_event_type_t = 0x800;

pub type xcb_xkb_nkn_detail_t = u32;
pub const XCB_XKB_NKN_DETAIL_KEYCODES: xcb_xkb_nkn_detail_t = 0x01;
pub const XCB_XKB_NKN_DETAIL_GEOMETRY: xcb_xkb_nkn_detail_t = 0x02;
pub const XCB_XKB_NKN_DETAIL_DEVICE_ID: xcb_xkb_nkn_detail_t = 0x04;

pub type xcb_xkb_axn_detail_t = u32;
pub const XCB_XKB_AXN_DETAIL_SK_PRESS: xcb_xkb_axn_detail_t = 0x01;
pub const XCB_XKB_AXN_DETAIL_SK_ACCEPT: xcb_xkb_axn_detail_t = 0x02;
pub const XCB_XKB_AXN_DETAIL_SK_REJECT: xcb_xkb_axn_detail_t = 0x04;
pub const XCB_XKB_AXN_DETAIL_SK_RELEASE: xcb_xkb_axn_detail_t = 0x08;
pub const XCB_XKB_AXN_DETAIL_BK_ACCEPT: xcb_xkb_axn_detail_t = 0x10;
pub const XCB_XKB_AXN_DETAIL_BK_REJECT: xcb_xkb_axn_detail_t = 0x20;
pub const XCB_XKB_AXN_DETAIL_AXK_WARNING: xcb_xkb_axn_detail_t = 0x40;

pub type xcb_xkb_map_part_t = u32;
pub const XCB_XKB_MAP_PART_KEY_TYPES: xcb_xkb_map_part_t = 0x01;
pub const XCB_XKB_MAP_PART_KEY_SYMS: xcb_xkb_map_part_t = 0x02;
pub const XCB_XKB_MAP_PART_MODIFIER_MAP: xcb_xkb_map_part_t = 0x04;
pub const XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS: xcb_xkb_map_part_t = 0x08;
pub const XCB_XKB_MAP_PART_KEY_ACTIONS: xcb_xkb_map_part_t = 0x10;
pub const XCB_XKB_MAP_PART_KEY_BEHAVIORS: xcb_xkb_map_part_t = 0x20;
pub const XCB_XKB_MAP_PART_VIRTUAL_MODS: xcb_xkb_map_part_t = 0x40;
pub const XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP: xcb_xkb_map_part_t = 0x80;

pub type xcb_xkb_set_map_flags_t = u32;
pub const XCB_XKB_SET_MAP_FLAGS_RESIZE_TYPES: xcb_xkb_set_map_flags_t = 0x01;
pub const XCB_XKB_SET_MAP_FLAGS_RECOMPUTE_ACTIONS: xcb_xkb_set_map_flags_t = 0x02;

pub type xcb_xkb_state_part_t = u32;
pub const XCB_XKB_STATE_PART_MODIFIER_STATE: xcb_xkb_state_part_t = 0x01;
pub const XCB_XKB_STATE_PART_MODIFIER_BASE: xcb_xkb_state_part_t = 0x02;
pub const XCB_XKB_STATE_PART_MODIFIER_LATCH: xcb_xkb_state_part_t = 0x04;
pub const XCB_XKB_STATE_PART_MODIFIER_LOCK: xcb_xkb_state_part_t = 0x08;
pub const XCB_XKB_STATE_PART_GROUP_STATE: xcb_xkb_state_part_t = 0x10;
pub const XCB_XKB_STATE_PART_GROUP_BASE: xcb_xkb_state_part_t = 0x20;
pub const XCB_XKB_STATE_PART_GROUP_LATCH: xcb_xkb_state_part_t = 0x40;
pub const XCB_XKB_STATE_PART_GROUP_LOCK: xcb_xkb_state_part_t = 0x80;
pub const XCB_XKB_STATE_PART_COMPAT_STATE: xcb_xkb_state_part_t = 0x100;
pub const XCB_XKB_STATE_PART_GRAB_MODS: xcb_xkb_state_part_t = 0x200;
pub const XCB_XKB_STATE_PART_COMPAT_GRAB_MODS: xcb_xkb_state_part_t = 0x400;
pub const XCB_XKB_STATE_PART_LOOKUP_MODS: xcb_xkb_state_part_t = 0x800;
pub const XCB_XKB_STATE_PART_COMPAT_LOOKUP_MODS: xcb_xkb_state_part_t = 0x1000;
pub const XCB_XKB_STATE_PART_POINTER_BUTTONS: xcb_xkb_state_part_t = 0x2000;

pub type xcb_xkb_bool_ctrl_t = u32;
pub const XCB_XKB_BOOL_CTRL_REPEAT_KEYS: xcb_xkb_bool_ctrl_t = 0x01;
pub const XCB_XKB_BOOL_CTRL_SLOW_KEYS: xcb_xkb_bool_ctrl_t = 0x02;
pub const XCB_XKB_BOOL_CTRL_BOUNCE_KEYS: xcb_xkb_bool_ctrl_t = 0x04;
pub const XCB_XKB_BOOL_CTRL_STICKY_KEYS: xcb_xkb_bool_ctrl_t = 0x08;
pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS: xcb_xkb_bool_ctrl_t = 0x10;
pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS_ACCEL: xcb_xkb_bool_ctrl_t = 0x20;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_KEYS: xcb_xkb_bool_ctrl_t = 0x40;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_TIMEOUT_MASK: xcb_xkb_bool_ctrl_t = 0x80;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_FEEDBACK_MASK: xcb_xkb_bool_ctrl_t = 0x100;
pub const XCB_XKB_BOOL_CTRL_AUDIBLE_BELL_MASK: xcb_xkb_bool_ctrl_t = 0x200;
pub const XCB_XKB_BOOL_CTRL_OVERLAY_1_MASK: xcb_xkb_bool_ctrl_t = 0x400;
pub const XCB_XKB_BOOL_CTRL_OVERLAY_2_MASK: xcb_xkb_bool_ctrl_t = 0x800;
pub const XCB_XKB_BOOL_CTRL_IGNORE_GROUP_LOCK_MASK: xcb_xkb_bool_ctrl_t = 0x1000;

pub type xcb_xkb_control_t = u32;
pub const XCB_XKB_CONTROL_GROUPS_WRAP: xcb_xkb_control_t = 0x8000000;
pub const XCB_XKB_CONTROL_INTERNAL_MODS: xcb_xkb_control_t = 0x10000000;
pub const XCB_XKB_CONTROL_IGNORE_LOCK_MODS: xcb_xkb_control_t = 0x20000000;
pub const XCB_XKB_CONTROL_PER_KEY_REPEAT: xcb_xkb_control_t = 0x40000000;
pub const XCB_XKB_CONTROL_CONTROLS_ENABLED: xcb_xkb_control_t = 0x80000000;

pub type xcb_xkb_ax_option_t = u32;
pub const XCB_XKB_AX_OPTION_SK_PRESS_FB: xcb_xkb_ax_option_t = 0x01;
pub const XCB_XKB_AX_OPTION_SK_ACCEPT_FB: xcb_xkb_ax_option_t = 0x02;
pub const XCB_XKB_AX_OPTION_FEATURE_FB: xcb_xkb_ax_option_t = 0x04;
pub const XCB_XKB_AX_OPTION_SLOW_WARN_FB: xcb_xkb_ax_option_t = 0x08;
pub const XCB_XKB_AX_OPTION_INDICATOR_FB: xcb_xkb_ax_option_t = 0x10;
pub const XCB_XKB_AX_OPTION_STICKY_KEYS_FB: xcb_xkb_ax_option_t = 0x20;
pub const XCB_XKB_AX_OPTION_TWO_KEYS: xcb_xkb_ax_option_t = 0x40;
pub const XCB_XKB_AX_OPTION_LATCH_TO_LOCK: xcb_xkb_ax_option_t = 0x80;
pub const XCB_XKB_AX_OPTION_SK_RELEASE_FB: xcb_xkb_ax_option_t = 0x100;
pub const XCB_XKB_AX_OPTION_SK_REJECT_FB: xcb_xkb_ax_option_t = 0x200;
pub const XCB_XKB_AX_OPTION_BK_REJECT_FB: xcb_xkb_ax_option_t = 0x400;
pub const XCB_XKB_AX_OPTION_DUMB_BELL: xcb_xkb_ax_option_t = 0x800;

pub type xcb_xkb_device_spec_t = u16;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_device_spec_iterator_t {
    pub data: *mut xcb_xkb_device_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_led_class_result_t = u32;
pub const XCB_XKB_LED_CLASS_RESULT_KBD_FEEDBACK_CLASS: xcb_xkb_led_class_result_t = 0x00;
pub const XCB_XKB_LED_CLASS_RESULT_LED_FEEDBACK_CLASS: xcb_xkb_led_class_result_t = 0x04;

pub type xcb_xkb_led_class_t = u32;
pub const XCB_XKB_LED_CLASS_KBD_FEEDBACK_CLASS: xcb_xkb_led_class_t = 0x00;
pub const XCB_XKB_LED_CLASS_LED_FEEDBACK_CLASS: xcb_xkb_led_class_t = 0x04;
pub const XCB_XKB_LED_CLASS_DFLT_XI_CLASS: xcb_xkb_led_class_t = 0x300;
pub const XCB_XKB_LED_CLASS_ALL_XI_CLASSES: xcb_xkb_led_class_t = 0x500;

pub type xcb_xkb_led_class_spec_t = u16;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_led_class_spec_iterator_t {
    pub data: *mut xcb_xkb_led_class_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_bell_class_result_t = u32;
pub const XCB_XKB_BELL_CLASS_RESULT_KBD_FEEDBACK_CLASS: xcb_xkb_bell_class_result_t = 0x00;
pub const XCB_XKB_BELL_CLASS_RESULT_BELL_FEEDBACK_CLASS: xcb_xkb_bell_class_result_t = 0x05;

pub type xcb_xkb_bell_class_t = u32;
pub const XCB_XKB_BELL_CLASS_KBD_FEEDBACK_CLASS: xcb_xkb_bell_class_t = 0x00;
pub const XCB_XKB_BELL_CLASS_BELL_FEEDBACK_CLASS: xcb_xkb_bell_class_t = 0x05;
pub const XCB_XKB_BELL_CLASS_DFLT_XI_CLASS: xcb_xkb_bell_class_t = 0x300;

pub type xcb_xkb_bell_class_spec_t = u16;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_bell_class_spec_iterator_t {
    pub data: *mut xcb_xkb_bell_class_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_id_t = u32;
pub const XCB_XKB_ID_USE_CORE_KBD: xcb_xkb_id_t = 0x100;
pub const XCB_XKB_ID_USE_CORE_PTR: xcb_xkb_id_t = 0x200;
pub const XCB_XKB_ID_DFLT_XI_CLASS: xcb_xkb_id_t = 0x300;
pub const XCB_XKB_ID_DFLT_XI_ID: xcb_xkb_id_t = 0x400;
pub const XCB_XKB_ID_ALL_XI_CLASS: xcb_xkb_id_t = 0x500;
pub const XCB_XKB_ID_ALL_XI_ID: xcb_xkb_id_t = 0x600;
pub const XCB_XKB_ID_XI_NONE: xcb_xkb_id_t = 0xff00;

pub type xcb_xkb_id_spec_t = u16;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_id_spec_iterator_t {
    pub data: *mut xcb_xkb_id_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_group_t = u32;
pub const XCB_XKB_GROUP_1: xcb_xkb_group_t = 0x00;
pub const XCB_XKB_GROUP_2: xcb_xkb_group_t = 0x01;
pub const XCB_XKB_GROUP_3: xcb_xkb_group_t = 0x02;
pub const XCB_XKB_GROUP_4: xcb_xkb_group_t = 0x03;

pub type xcb_xkb_groups_t = u32;
pub const XCB_XKB_GROUPS_ANY: xcb_xkb_groups_t = 0xfe;
pub const XCB_XKB_GROUPS_ALL: xcb_xkb_groups_t = 0xff;

pub type xcb_xkb_set_of_group_t = u32;
pub const XCB_XKB_SET_OF_GROUP_GROUP_1: xcb_xkb_set_of_group_t = 0x01;
pub const XCB_XKB_SET_OF_GROUP_GROUP_2: xcb_xkb_set_of_group_t = 0x02;
pub const XCB_XKB_SET_OF_GROUP_GROUP_3: xcb_xkb_set_of_group_t = 0x04;
pub const XCB_XKB_SET_OF_GROUP_GROUP_4: xcb_xkb_set_of_group_t = 0x08;

pub type xcb_xkb_set_of_groups_t = u32;
pub const XCB_XKB_SET_OF_GROUPS_ANY: xcb_xkb_set_of_groups_t = 0x80;

pub type xcb_xkb_groups_wrap_t = u32;
pub const XCB_XKB_GROUPS_WRAP_WRAP_INTO_RANGE: xcb_xkb_groups_wrap_t = 0x00;
pub const XCB_XKB_GROUPS_WRAP_CLAMP_INTO_RANGE: xcb_xkb_groups_wrap_t = 0x40;
pub const XCB_XKB_GROUPS_WRAP_REDIRECT_INTO_RANGE: xcb_xkb_groups_wrap_t = 0x80;

pub type xcb_xkb_v_mods_high_t = u32;
pub const XCB_XKB_V_MODS_HIGH_15: xcb_xkb_v_mods_high_t = 0x80;
pub const XCB_XKB_V_MODS_HIGH_14: xcb_xkb_v_mods_high_t = 0x40;
pub const XCB_XKB_V_MODS_HIGH_13: xcb_xkb_v_mods_high_t = 0x20;
pub const XCB_XKB_V_MODS_HIGH_12: xcb_xkb_v_mods_high_t = 0x10;
pub const XCB_XKB_V_MODS_HIGH_11: xcb_xkb_v_mods_high_t = 0x08;
pub const XCB_XKB_V_MODS_HIGH_10: xcb_xkb_v_mods_high_t = 0x04;
pub const XCB_XKB_V_MODS_HIGH_9: xcb_xkb_v_mods_high_t = 0x02;
pub const XCB_XKB_V_MODS_HIGH_8: xcb_xkb_v_mods_high_t = 0x01;

pub type xcb_xkb_v_mods_low_t = u32;
pub const XCB_XKB_V_MODS_LOW_7: xcb_xkb_v_mods_low_t = 0x80;
pub const XCB_XKB_V_MODS_LOW_6: xcb_xkb_v_mods_low_t = 0x40;
pub const XCB_XKB_V_MODS_LOW_5: xcb_xkb_v_mods_low_t = 0x20;
pub const XCB_XKB_V_MODS_LOW_4: xcb_xkb_v_mods_low_t = 0x10;
pub const XCB_XKB_V_MODS_LOW_3: xcb_xkb_v_mods_low_t = 0x08;
pub const XCB_XKB_V_MODS_LOW_2: xcb_xkb_v_mods_low_t = 0x04;
pub const XCB_XKB_V_MODS_LOW_1: xcb_xkb_v_mods_low_t = 0x02;
pub const XCB_XKB_V_MODS_LOW_0: xcb_xkb_v_mods_low_t = 0x01;

pub type xcb_xkb_v_mod_t = u32;
pub const XCB_XKB_V_MOD_15: xcb_xkb_v_mod_t = 0x8000;
pub const XCB_XKB_V_MOD_14: xcb_xkb_v_mod_t = 0x4000;
pub const XCB_XKB_V_MOD_13: xcb_xkb_v_mod_t = 0x2000;
pub const XCB_XKB_V_MOD_12: xcb_xkb_v_mod_t = 0x1000;
pub const XCB_XKB_V_MOD_11: xcb_xkb_v_mod_t = 0x800;
pub const XCB_XKB_V_MOD_10: xcb_xkb_v_mod_t = 0x400;
pub const XCB_XKB_V_MOD_9: xcb_xkb_v_mod_t = 0x200;
pub const XCB_XKB_V_MOD_8: xcb_xkb_v_mod_t = 0x100;
pub const XCB_XKB_V_MOD_7: xcb_xkb_v_mod_t = 0x80;
pub const XCB_XKB_V_MOD_6: xcb_xkb_v_mod_t = 0x40;
pub const XCB_XKB_V_MOD_5: xcb_xkb_v_mod_t = 0x20;
pub const XCB_XKB_V_MOD_4: xcb_xkb_v_mod_t = 0x10;
pub const XCB_XKB_V_MOD_3: xcb_xkb_v_mod_t = 0x08;
pub const XCB_XKB_V_MOD_2: xcb_xkb_v_mod_t = 0x04;
pub const XCB_XKB_V_MOD_1: xcb_xkb_v_mod_t = 0x02;
pub const XCB_XKB_V_MOD_0: xcb_xkb_v_mod_t = 0x01;

pub type xcb_xkb_explicit_t = u32;
pub const XCB_XKB_EXPLICIT_V_MOD_MAP: xcb_xkb_explicit_t = 0x80;
pub const XCB_XKB_EXPLICIT_BEHAVIOR: xcb_xkb_explicit_t = 0x40;
pub const XCB_XKB_EXPLICIT_AUTO_REPEAT: xcb_xkb_explicit_t = 0x20;
pub const XCB_XKB_EXPLICIT_INTERPRET: xcb_xkb_explicit_t = 0x10;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_4: xcb_xkb_explicit_t = 0x08;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_3: xcb_xkb_explicit_t = 0x04;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_2: xcb_xkb_explicit_t = 0x02;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_1: xcb_xkb_explicit_t = 0x01;

pub type xcb_xkb_sym_interpret_match_t = u32;
pub const XCB_XKB_SYM_INTERPRET_MATCH_NONE_OF: xcb_xkb_sym_interpret_match_t = 0x00;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF_OR_NONE: xcb_xkb_sym_interpret_match_t = 0x01;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF: xcb_xkb_sym_interpret_match_t = 0x02;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ALL_OF: xcb_xkb_sym_interpret_match_t = 0x03;
pub const XCB_XKB_SYM_INTERPRET_MATCH_EXACTLY: xcb_xkb_sym_interpret_match_t = 0x04;

pub type xcb_xkb_sym_interp_match_t = u32;
pub const XCB_XKB_SYM_INTERP_MATCH_LEVEL_ONE_ONLY: xcb_xkb_sym_interp_match_t = 0x80;
pub const XCB_XKB_SYM_INTERP_MATCH_OP_MASK: xcb_xkb_sym_interp_match_t = 0x7f;

pub type xcb_xkb_im_flag_t = u32;
pub const XCB_XKB_IM_FLAG_NO_EXPLICIT: xcb_xkb_im_flag_t = 0x80;
pub const XCB_XKB_IM_FLAG_NO_AUTOMATIC: xcb_xkb_im_flag_t = 0x40;
pub const XCB_XKB_IM_FLAG_LED_DRIVES_KB: xcb_xkb_im_flag_t = 0x20;

pub type xcb_xkb_im_mods_which_t = u32;
pub const XCB_XKB_IM_MODS_WHICH_USE_COMPAT: xcb_xkb_im_mods_which_t = 0x10;
pub const XCB_XKB_IM_MODS_WHICH_USE_EFFECTIVE: xcb_xkb_im_mods_which_t = 0x08;
pub const XCB_XKB_IM_MODS_WHICH_USE_LOCKED: xcb_xkb_im_mods_which_t = 0x04;
pub const XCB_XKB_IM_MODS_WHICH_USE_LATCHED: xcb_xkb_im_mods_which_t = 0x02;
pub const XCB_XKB_IM_MODS_WHICH_USE_BASE: xcb_xkb_im_mods_which_t = 0x01;

pub type xcb_xkb_im_groups_which_t = u32;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_COMPAT: xcb_xkb_im_groups_which_t = 0x10;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_EFFECTIVE: xcb_xkb_im_groups_which_t = 0x08;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_LOCKED: xcb_xkb_im_groups_which_t = 0x04;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_LATCHED: xcb_xkb_im_groups_which_t = 0x02;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_BASE: xcb_xkb_im_groups_which_t = 0x01;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_map_t {
    pub flags: u8,
    pub which_groups: u8,
    pub groups: u8,
    pub which_mods: u8,
    pub mods: u8,
    pub real_mods: u8,
    pub vmods: u16,
    pub ctrls: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_indicator_map_iterator_t {
    pub data: *mut xcb_xkb_indicator_map_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_cm_detail_t = u32;
pub const XCB_XKB_CM_DETAIL_SYM_INTERP: xcb_xkb_cm_detail_t = 0x01;
pub const XCB_XKB_CM_DETAIL_GROUP_COMPAT: xcb_xkb_cm_detail_t = 0x02;

pub type xcb_xkb_name_detail_t = u32;
pub const XCB_XKB_NAME_DETAIL_KEYCODES: xcb_xkb_name_detail_t = 0x01;
pub const XCB_XKB_NAME_DETAIL_GEOMETRY: xcb_xkb_name_detail_t = 0x02;
pub const XCB_XKB_NAME_DETAIL_SYMBOLS: xcb_xkb_name_detail_t = 0x04;
pub const XCB_XKB_NAME_DETAIL_PHYS_SYMBOLS: xcb_xkb_name_detail_t = 0x08;
pub const XCB_XKB_NAME_DETAIL_TYPES: xcb_xkb_name_detail_t = 0x10;
pub const XCB_XKB_NAME_DETAIL_COMPAT: xcb_xkb_name_detail_t = 0x20;
pub const XCB_XKB_NAME_DETAIL_KEY_TYPE_NAMES: xcb_xkb_name_detail_t = 0x40;
pub const XCB_XKB_NAME_DETAIL_KT_LEVEL_NAMES: xcb_xkb_name_detail_t = 0x80;
pub const XCB_XKB_NAME_DETAIL_INDICATOR_NAMES: xcb_xkb_name_detail_t = 0x100;
pub const XCB_XKB_NAME_DETAIL_KEY_NAMES: xcb_xkb_name_detail_t = 0x200;
pub const XCB_XKB_NAME_DETAIL_KEY_ALIASES: xcb_xkb_name_detail_t = 0x400;
pub const XCB_XKB_NAME_DETAIL_VIRTUAL_MOD_NAMES: xcb_xkb_name_detail_t = 0x800;
pub const XCB_XKB_NAME_DETAIL_GROUP_NAMES: xcb_xkb_name_detail_t = 0x1000;
pub const XCB_XKB_NAME_DETAIL_RG_NAMES: xcb_xkb_name_detail_t = 0x2000;

pub type xcb_xkb_gbn_detail_t = u32;
pub const XCB_XKB_GBN_DETAIL_TYPES: xcb_xkb_gbn_detail_t = 0x01;
pub const XCB_XKB_GBN_DETAIL_COMPAT_MAP: xcb_xkb_gbn_detail_t = 0x02;
pub const XCB_XKB_GBN_DETAIL_CLIENT_SYMBOLS: xcb_xkb_gbn_detail_t = 0x04;
pub const XCB_XKB_GBN_DETAIL_SERVER_SYMBOLS: xcb_xkb_gbn_detail_t = 0x08;
pub const XCB_XKB_GBN_DETAIL_INDICATOR_MAPS: xcb_xkb_gbn_detail_t = 0x10;
pub const XCB_XKB_GBN_DETAIL_KEY_NAMES: xcb_xkb_gbn_detail_t = 0x20;
pub const XCB_XKB_GBN_DETAIL_GEOMETRY: xcb_xkb_gbn_detail_t = 0x40;
pub const XCB_XKB_GBN_DETAIL_OTHER_NAMES: xcb_xkb_gbn_detail_t = 0x80;

pub type xcb_xkb_xi_feature_t = u32;
pub const XCB_XKB_XI_FEATURE_KEYBOARDS: xcb_xkb_xi_feature_t = 0x01;
pub const XCB_XKB_XI_FEATURE_BUTTON_ACTIONS: xcb_xkb_xi_feature_t = 0x02;
pub const XCB_XKB_XI_FEATURE_INDICATOR_NAMES: xcb_xkb_xi_feature_t = 0x04;
pub const XCB_XKB_XI_FEATURE_INDICATOR_MAPS: xcb_xkb_xi_feature_t = 0x08;
pub const XCB_XKB_XI_FEATURE_INDICATOR_STATE: xcb_xkb_xi_feature_t = 0x10;

pub type xcb_xkb_per_client_flag_t = u32;
pub const XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT: xcb_xkb_per_client_flag_t = 0x01;
pub const XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE: xcb_xkb_per_client_flag_t = 0x02;
pub const XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS: xcb_xkb_per_client_flag_t = 0x04;
pub const XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED: xcb_xkb_per_client_flag_t = 0x08;
pub const XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE: xcb_xkb_per_client_flag_t = 0x10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_mod_def_t {
    pub mask: u8,
    pub real_mods: u8,
    pub vmods: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_mod_def_iterator_t {
    pub data: *mut xcb_xkb_mod_def_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_name_t {
    pub name: [c_char; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_name_iterator_t<'a> {
    pub data: *mut xcb_xkb_key_name_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_key_name_t>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_alias_t {
    pub real: [c_char; 4],
    pub alias: [c_char; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_alias_iterator_t<'a> {
    pub data: *mut xcb_xkb_key_alias_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_key_alias_t>,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_counted_string_16_t {
    pub length: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_counted_string_16_iterator_t<'a> {
    pub data: *mut xcb_xkb_counted_string_16_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_counted_string_16_t>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_kt_map_entry_t {
    pub active: u8,
    pub mods_mask: u8,
    pub level: u8,
    pub mods_mods: u8,
    pub mods_vmods: u16,
    pub pad0: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_kt_map_entry_iterator_t {
    pub data: *mut xcb_xkb_kt_map_entry_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_key_type_t {
    pub mods_mask: u8,
    pub mods_mods: u8,
    pub mods_vmods: u16,
    pub num_levels: u8,
    pub n_map_entries: u8,
    pub has_preserve: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_type_iterator_t<'a> {
    pub data: *mut xcb_xkb_key_type_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_key_type_t>,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_key_sym_map_t {
    pub kt_index: [u8; 4],
    pub group_info: u8,
    pub width: u8,
    pub n_syms: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_sym_map_iterator_t<'a> {
    pub data: *mut xcb_xkb_key_sym_map_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_key_sym_map_t>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_common_behavior_t {
    pub type_: u8,
    pub data: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_common_behavior_iterator_t {
    pub data: *mut xcb_xkb_common_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_default_behavior_t {
    pub type_: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_default_behavior_iterator_t {
    pub data: *mut xcb_xkb_default_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_lock_behavior_t {
    pub type_: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_lock_behavior_iterator_t {
    pub data: *mut xcb_xkb_lock_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_radio_group_behavior_t {
    pub type_: u8,
    pub group: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_radio_group_behavior_iterator_t {
    pub data: *mut xcb_xkb_radio_group_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_behavior_t {
    pub type_: u8,
    pub key: xcb_keycode_t,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_overlay_behavior_iterator_t {
    pub data: *mut xcb_xkb_overlay_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_lock_behavior_t {
    pub type_: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_permament_lock_behavior_iterator_t {
    pub data: *mut xcb_xkb_permament_lock_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_radio_group_behavior_t {
    pub type_: u8,
    pub group: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_permament_radio_group_behavior_iterator_t {
    pub data: *mut xcb_xkb_permament_radio_group_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_overlay_behavior_t {
    pub type_: u8,
    pub key: xcb_keycode_t,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_permament_overlay_behavior_iterator_t {
    pub data: *mut xcb_xkb_permament_overlay_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

// union
#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_behavior_t {
    pub data: [u8; 2],
}

impl Copy for xcb_xkb_behavior_t {}
impl Clone for xcb_xkb_behavior_t {
    fn clone(&self) -> xcb_xkb_behavior_t {
        *self
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_behavior_iterator_t {
    pub data: *mut xcb_xkb_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_behavior_type_t = u32;
pub const XCB_XKB_BEHAVIOR_TYPE_DEFAULT: xcb_xkb_behavior_type_t = 0x00;
pub const XCB_XKB_BEHAVIOR_TYPE_LOCK: xcb_xkb_behavior_type_t = 0x01;
pub const XCB_XKB_BEHAVIOR_TYPE_RADIO_GROUP: xcb_xkb_behavior_type_t = 0x02;
pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_1: xcb_xkb_behavior_type_t = 0x03;
pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_2: xcb_xkb_behavior_type_t = 0x04;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_LOCK: xcb_xkb_behavior_type_t = 0x81;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_RADIO_GROUP: xcb_xkb_behavior_type_t = 0x82;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_1: xcb_xkb_behavior_type_t = 0x83;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_2: xcb_xkb_behavior_type_t = 0x84;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_behavior_t {
    pub keycode: xcb_keycode_t,
    pub behavior: xcb_xkb_behavior_t,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_set_behavior_iterator_t<'a> {
    pub data: *mut xcb_xkb_set_behavior_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_set_behavior_t>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_explicit_t {
    pub keycode: xcb_keycode_t,
    pub explicit: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_set_explicit_iterator_t {
    pub data: *mut xcb_xkb_set_explicit_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_mod_map_t {
    pub keycode: xcb_keycode_t,
    pub mods: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_mod_map_iterator_t {
    pub data: *mut xcb_xkb_key_mod_map_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_v_mod_map_t {
    pub keycode: xcb_keycode_t,
    pub pad0: u8,
    pub vmods: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_v_mod_map_iterator_t {
    pub data: *mut xcb_xkb_key_v_mod_map_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_kt_set_map_entry_t {
    pub level: u8,
    pub real_mods: u8,
    pub virtual_mods: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_kt_set_map_entry_iterator_t {
    pub data: *mut xcb_xkb_kt_set_map_entry_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_key_type_t {
    pub mask: u8,
    pub real_mods: u8,
    pub virtual_mods: u16,
    pub num_levels: u8,
    pub n_map_entries: u8,
    pub preserve: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_set_key_type_iterator_t<'a> {
    pub data: *mut xcb_xkb_set_key_type_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_set_key_type_t>,
}

pub type xcb_xkb_string8_t = c_char;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_string8_iterator_t {
    pub data: *mut xcb_xkb_string8_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_outline_t {
    pub n_points: u8,
    pub corner_radius: u8,
    pub pad0: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_outline_iterator_t<'a> {
    pub data: *mut xcb_xkb_outline_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_outline_t>,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_shape_t {
    pub name: xcb_atom_t,
    pub n_outlines: u8,
    pub primary_ndx: u8,
    pub approx_ndx: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_shape_iterator_t<'a> {
    pub data: *mut xcb_xkb_shape_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_shape_t>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_t {
    pub name: [xcb_xkb_string8_t; 4],
    pub gap: i16,
    pub shape_ndx: u8,
    pub color_ndx: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_iterator_t<'a> {
    pub data: *mut xcb_xkb_key_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_key_t>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_key_t {
    pub over: [xcb_xkb_string8_t; 4],
    pub under: [xcb_xkb_string8_t; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_overlay_key_iterator_t<'a> {
    pub data: *mut xcb_xkb_overlay_key_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_overlay_key_t>,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_row_t {
    pub row_under: u8,
    pub n_keys: u8,
    pub pad0: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_overlay_row_iterator_t<'a> {
    pub data: *mut xcb_xkb_overlay_row_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_overlay_row_t>,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_t {
    pub name: xcb_atom_t,
    pub n_rows: u8,
    pub pad0: [u8; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_overlay_iterator_t<'a> {
    pub data: *mut xcb_xkb_overlay_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_overlay_t>,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_row_t {
    pub top: i16,
    pub left: i16,
    pub n_keys: u8,
    pub vertical: u8,
    pub pad0: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_row_iterator_t<'a> {
    pub data: *mut xcb_xkb_row_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_row_t>,
}

pub type xcb_xkb_doodad_type_t = u32;
pub const XCB_XKB_DOODAD_TYPE_OUTLINE: xcb_xkb_doodad_type_t = 0x01;
pub const XCB_XKB_DOODAD_TYPE_SOLID: xcb_xkb_doodad_type_t = 0x02;
pub const XCB_XKB_DOODAD_TYPE_TEXT: xcb_xkb_doodad_type_t = 0x03;
pub const XCB_XKB_DOODAD_TYPE_INDICATOR: xcb_xkb_doodad_type_t = 0x04;
pub const XCB_XKB_DOODAD_TYPE_LOGO: xcb_xkb_doodad_type_t = 0x05;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_listing_t {
    pub flags: u16,
    pub length: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_listing_iterator_t<'a> {
    pub data: *mut xcb_xkb_listing_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_listing_t>,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_device_led_info_t {
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_id: xcb_xkb_id_spec_t,
    pub names_present: u32,
    pub maps_present: u32,
    pub phys_indicators: u32,
    pub state: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_device_led_info_iterator_t<'a> {
    pub data: *mut xcb_xkb_device_led_info_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_device_led_info_t>,
}

pub type xcb_xkb_error_t = u32;
pub const XCB_XKB_ERROR_BAD_DEVICE: xcb_xkb_error_t = 0xff;
pub const XCB_XKB_ERROR_BAD_CLASS: xcb_xkb_error_t = 0xfe;
pub const XCB_XKB_ERROR_BAD_ID: xcb_xkb_error_t = 0xfd;

pub const XCB_XKB_KEYBOARD: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_keyboard_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: [u8; 21],
}

pub type xcb_xkb_sa_t = u32;
pub const XCB_XKB_SA_CLEAR_LOCKS: xcb_xkb_sa_t = 0x01;
pub const XCB_XKB_SA_LATCH_TO_LOCK: xcb_xkb_sa_t = 0x02;
pub const XCB_XKB_SA_USE_MOD_MAP_MODS: xcb_xkb_sa_t = 0x04;
pub const XCB_XKB_SA_GROUP_ABSOLUTE: xcb_xkb_sa_t = 0x04;

pub type xcb_xkb_sa_type_t = u32;
pub const XCB_XKB_SA_TYPE_NO_ACTION: xcb_xkb_sa_type_t = 0x00;
pub const XCB_XKB_SA_TYPE_SET_MODS: xcb_xkb_sa_type_t = 0x01;
pub const XCB_XKB_SA_TYPE_LATCH_MODS: xcb_xkb_sa_type_t = 0x02;
pub const XCB_XKB_SA_TYPE_LOCK_MODS: xcb_xkb_sa_type_t = 0x03;
pub const XCB_XKB_SA_TYPE_SET_GROUP: xcb_xkb_sa_type_t = 0x04;
pub const XCB_XKB_SA_TYPE_LATCH_GROUP: xcb_xkb_sa_type_t = 0x05;
pub const XCB_XKB_SA_TYPE_LOCK_GROUP: xcb_xkb_sa_type_t = 0x06;
pub const XCB_XKB_SA_TYPE_MOVE_PTR: xcb_xkb_sa_type_t = 0x07;
pub const XCB_XKB_SA_TYPE_PTR_BTN: xcb_xkb_sa_type_t = 0x08;
pub const XCB_XKB_SA_TYPE_LOCK_PTR_BTN: xcb_xkb_sa_type_t = 0x09;
pub const XCB_XKB_SA_TYPE_SET_PTR_DFLT: xcb_xkb_sa_type_t = 0x0a;
pub const XCB_XKB_SA_TYPE_ISO_LOCK: xcb_xkb_sa_type_t = 0x0b;
pub const XCB_XKB_SA_TYPE_TERMINATE: xcb_xkb_sa_type_t = 0x0c;
pub const XCB_XKB_SA_TYPE_SWITCH_SCREEN: xcb_xkb_sa_type_t = 0x0d;
pub const XCB_XKB_SA_TYPE_SET_CONTROLS: xcb_xkb_sa_type_t = 0x0e;
pub const XCB_XKB_SA_TYPE_LOCK_CONTROLS: xcb_xkb_sa_type_t = 0x0f;
pub const XCB_XKB_SA_TYPE_ACTION_MESSAGE: xcb_xkb_sa_type_t = 0x10;
pub const XCB_XKB_SA_TYPE_REDIRECT_KEY: xcb_xkb_sa_type_t = 0x11;
pub const XCB_XKB_SA_TYPE_DEVICE_BTN: xcb_xkb_sa_type_t = 0x12;
pub const XCB_XKB_SA_TYPE_LOCK_DEVICE_BTN: xcb_xkb_sa_type_t = 0x13;
pub const XCB_XKB_SA_TYPE_DEVICE_VALUATOR: xcb_xkb_sa_type_t = 0x14;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_no_action_t {
    pub type_: u8,
    pub pad0: [u8; 7],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_no_action_iterator_t {
    pub data: *mut xcb_xkb_sa_no_action_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_mods_t {
    pub type_: u8,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
    pub pad0: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_set_mods_iterator_t {
    pub data: *mut xcb_xkb_sa_set_mods_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_latch_mods_t {
    pub type_: u8,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
    pub pad0: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_latch_mods_iterator_t {
    pub data: *mut xcb_xkb_sa_latch_mods_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_mods_t {
    pub type_: u8,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
    pub pad0: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_mods_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_mods_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0: [u8; 5],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_set_group_iterator_t {
    pub data: *mut xcb_xkb_sa_set_group_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_latch_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0: [u8; 5],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_latch_group_iterator_t {
    pub data: *mut xcb_xkb_sa_latch_group_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0: [u8; 5],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_group_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_group_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_sa_move_ptr_flag_t = u32;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_NO_ACCELERATION: xcb_xkb_sa_move_ptr_flag_t = 0x01;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_X: xcb_xkb_sa_move_ptr_flag_t = 0x02;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_Y: xcb_xkb_sa_move_ptr_flag_t = 0x04;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_move_ptr_t {
    pub type_: u8,
    pub flags: u8,
    pub x_high: i8,
    pub x_low: u8,
    pub y_high: i8,
    pub y_low: u8,
    pub pad0: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_move_ptr_iterator_t {
    pub data: *mut xcb_xkb_sa_move_ptr_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_ptr_btn_t {
    pub type_: u8,
    pub flags: u8,
    pub count: u8,
    pub button: u8,
    pub pad0: [u8; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_ptr_btn_iterator_t {
    pub data: *mut xcb_xkb_sa_ptr_btn_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_ptr_btn_t {
    pub type_: u8,
    pub flags: u8,
    pub pad0: u8,
    pub button: u8,
    pub pad1: [u8; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_ptr_btn_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_ptr_btn_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_sa_set_ptr_dflt_flag_t = u32;
pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_DFLT_BTN_ABSOLUTE: xcb_xkb_sa_set_ptr_dflt_flag_t = 0x04;
pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_AFFECT_DFLT_BUTTON: xcb_xkb_sa_set_ptr_dflt_flag_t = 0x01;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_ptr_dflt_t {
    pub type_: u8,
    pub flags: u8,
    pub affect: u8,
    pub value: i8,
    pub pad0: [u8; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_set_ptr_dflt_iterator_t {
    pub data: *mut xcb_xkb_sa_set_ptr_dflt_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_sa_iso_lock_flag_t = u32;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_LOCK: xcb_xkb_sa_iso_lock_flag_t = 0x01;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_UNLOCK: xcb_xkb_sa_iso_lock_flag_t = 0x02;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_USE_MOD_MAP_MODS: xcb_xkb_sa_iso_lock_flag_t = 0x04;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_GROUP_ABSOLUTE: xcb_xkb_sa_iso_lock_flag_t = 0x04;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_ISO_DFLT_IS_GROUP: xcb_xkb_sa_iso_lock_flag_t = 0x08;

pub type xcb_xkb_sa_iso_lock_no_affect_t = u32;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_CTRLS: xcb_xkb_sa_iso_lock_no_affect_t = 0x08;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_PTR: xcb_xkb_sa_iso_lock_no_affect_t = 0x10;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_GROUP: xcb_xkb_sa_iso_lock_no_affect_t = 0x20;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_MODS: xcb_xkb_sa_iso_lock_no_affect_t = 0x40;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_iso_lock_t {
    pub type_: u8,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub group: i8,
    pub affect: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_iso_lock_iterator_t {
    pub data: *mut xcb_xkb_sa_iso_lock_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_terminate_t {
    pub type_: u8,
    pub pad0: [u8; 7],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_terminate_iterator_t {
    pub data: *mut xcb_xkb_sa_terminate_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_switch_screen_flag_t = u32;
pub const XCB_XKB_SWITCH_SCREEN_FLAG_APPLICATION: xcb_xkb_switch_screen_flag_t = 0x01;
pub const XCB_XKB_SWITCH_SCREEN_FLAG_ABSOLUTE: xcb_xkb_switch_screen_flag_t = 0x04;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_switch_screen_t {
    pub type_: u8,
    pub flags: u8,
    pub new_screen: i8,
    pub pad0: [u8; 5],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_switch_screen_iterator_t {
    pub data: *mut xcb_xkb_sa_switch_screen_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_bool_ctrls_high_t = u32;
pub const XCB_XKB_BOOL_CTRLS_HIGH_ACCESS_X_FEEDBACK: xcb_xkb_bool_ctrls_high_t = 0x01;
pub const XCB_XKB_BOOL_CTRLS_HIGH_AUDIBLE_BELL: xcb_xkb_bool_ctrls_high_t = 0x02;
pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_1: xcb_xkb_bool_ctrls_high_t = 0x04;
pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_2: xcb_xkb_bool_ctrls_high_t = 0x08;
pub const XCB_XKB_BOOL_CTRLS_HIGH_IGNORE_GROUP_LOCK: xcb_xkb_bool_ctrls_high_t = 0x10;

pub type xcb_xkb_bool_ctrls_low_t = u32;
pub const XCB_XKB_BOOL_CTRLS_LOW_REPEAT_KEYS: xcb_xkb_bool_ctrls_low_t = 0x01;
pub const XCB_XKB_BOOL_CTRLS_LOW_SLOW_KEYS: xcb_xkb_bool_ctrls_low_t = 0x02;
pub const XCB_XKB_BOOL_CTRLS_LOW_BOUNCE_KEYS: xcb_xkb_bool_ctrls_low_t = 0x04;
pub const XCB_XKB_BOOL_CTRLS_LOW_STICKY_KEYS: xcb_xkb_bool_ctrls_low_t = 0x08;
pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS: xcb_xkb_bool_ctrls_low_t = 0x10;
pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS_ACCEL: xcb_xkb_bool_ctrls_low_t = 0x20;
pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_KEYS: xcb_xkb_bool_ctrls_low_t = 0x40;
pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_TIMEOUT: xcb_xkb_bool_ctrls_low_t = 0x80;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_controls_t {
    pub type_: u8,
    pub pad0: [u8; 3],
    pub bool_ctrls_high: u8,
    pub bool_ctrls_low: u8,
    pub pad1: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_set_controls_iterator_t {
    pub data: *mut xcb_xkb_sa_set_controls_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_controls_t {
    pub type_: u8,
    pub pad0: [u8; 3],
    pub bool_ctrls_high: u8,
    pub bool_ctrls_low: u8,
    pub pad1: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_controls_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_controls_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_action_message_flag_t = u32;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_PRESS: xcb_xkb_action_message_flag_t = 0x01;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_RELEASE: xcb_xkb_action_message_flag_t = 0x02;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_GEN_KEY_EVENT: xcb_xkb_action_message_flag_t = 0x04;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_action_message_t {
    pub type_: u8,
    pub flags: u8,
    pub message: [u8; 6],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_action_message_iterator_t<'a> {
    pub data: *mut xcb_xkb_sa_action_message_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_sa_action_message_t>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_redirect_key_t {
    pub type_: u8,
    pub newkey: xcb_keycode_t,
    pub mask: u8,
    pub real_modifiers: u8,
    pub vmods_mask_high: u8,
    pub vmods_mask_low: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_redirect_key_iterator_t {
    pub data: *mut xcb_xkb_sa_redirect_key_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_device_btn_t {
    pub type_: u8,
    pub flags: u8,
    pub count: u8,
    pub button: u8,
    pub device: u8,
    pub pad0: [u8; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_device_btn_iterator_t {
    pub data: *mut xcb_xkb_sa_device_btn_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_lock_device_flags_t = u32;
pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_LOCK: xcb_xkb_lock_device_flags_t = 0x01;
pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_UNLOCK: xcb_xkb_lock_device_flags_t = 0x02;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_device_btn_t {
    pub type_: u8,
    pub flags: u8,
    pub pad0: u8,
    pub button: u8,
    pub device: u8,
    pub pad1: [u8; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_device_btn_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_device_btn_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xkb_sa_val_what_t = u32;
pub const XCB_XKB_SA_VAL_WHAT_IGNORE_VAL: xcb_xkb_sa_val_what_t = 0x00;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MIN: xcb_xkb_sa_val_what_t = 0x01;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_CENTER: xcb_xkb_sa_val_what_t = 0x02;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MAX: xcb_xkb_sa_val_what_t = 0x03;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_RELATIVE: xcb_xkb_sa_val_what_t = 0x04;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_ABSOLUTE: xcb_xkb_sa_val_what_t = 0x05;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_device_valuator_t {
    pub type_: u8,
    pub device: u8,
    pub val1what: u8,
    pub val1index: u8,
    pub val1value: u8,
    pub val2what: u8,
    pub val2index: u8,
    pub val2value: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_device_valuator_iterator_t {
    pub data: *mut xcb_xkb_sa_device_valuator_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_si_action_t {
    pub type_: u8,
    pub data: [u8; 7],
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_si_action_iterator_t<'a> {
    pub data: *mut xcb_xkb_si_action_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_si_action_t>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sym_interpret_t {
    pub sym: xcb_keysym_t,
    pub mods: u8,
    pub match_: u8,
    pub virtual_mod: u8,
    pub flags: u8,
    pub action: xcb_xkb_si_action_t,
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sym_interpret_iterator_t<'a> {
    pub data: *mut xcb_xkb_sym_interpret_t,
    pub rem: c_int,
    pub index: c_int,
    _phantom: std::marker::PhantomData<&'a xcb_xkb_sym_interpret_t>,
}

// union
#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_action_t {
    pub data: [u8; 8],
}

impl Copy for xcb_xkb_action_t {}
impl Clone for xcb_xkb_action_t {
    fn clone(&self) -> xcb_xkb_action_t {
        *self
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_action_iterator_t {
    pub data: *mut xcb_xkb_action_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_XKB_USE_EXTENSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_use_extension_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub wanted_major: u16,
    pub wanted_minor: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_use_extension_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_use_extension_reply_t {
    pub response_type: u8,
    pub supported: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
    pub pad0: [u8; 20],
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_select_events_details_t {
    pub affect_new_keyboard: u16,
    pub new_keyboard_details: u16,
    pub affect_state: u16,
    pub state_details: u16,
    pub affect_ctrls: u32,
    pub ctrl_details: u32,
    pub affect_indicator_state: u32,
    pub indicator_state_details: u32,
    pub affect_indicator_map: u32,
    pub indicator_map_details: u32,
    pub affect_names: u16,
    pub names_details: u16,
    pub affect_compat: u8,
    pub compat_details: u8,
    pub affect_bell: u8,
    pub bell_details: u8,
    pub affect_msg_details: u8,
    pub msg_details: u8,
    pub affect_access_x: u16,
    pub access_x_details: u16,
    pub affect_ext_dev: u16,
    pub extdev_details: u16,
}

pub const XCB_XKB_SELECT_EVENTS: u8 = 1;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_select_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub affect_which: u16,
    pub clear: u16,
    pub select_all: u16,
    pub affect_map: u16,
    pub map: u16,
}

pub const XCB_XKB_BELL: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_bell_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub bell_class: xcb_xkb_bell_class_spec_t,
    pub bell_id: xcb_xkb_id_spec_t,
    pub percent: i8,
    pub force_sound: u8,
    pub event_only: u8,
    pub pad0: u8,
    pub pitch: i16,
    pub duration: i16,
    pub pad1: [u8; 2],
    pub name: xcb_atom_t,
    pub window: xcb_window_t,
}

pub const XCB_XKB_GET_STATE: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub mods: u8,
    pub base_mods: u8,
    pub latched_mods: u8,
    pub locked_mods: u8,
    pub group: u8,
    pub locked_group: u8,
    pub base_group: i16,
    pub latched_group: i16,
    pub compat_state: u8,
    pub grab_mods: u8,
    pub compat_grab_mods: u8,
    pub lookup_mods: u8,
    pub compat_lookup_mods: u8,
    pub pad0: u8,
    pub ptr_btn_state: u16,
    pub pad1: [u8; 6],
}

pub const XCB_XKB_LATCH_LOCK_STATE: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_latch_lock_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub affect_mod_locks: u8,
    pub mod_locks: u8,
    pub lock_group: u8,
    pub group_lock: u8,
    pub affect_mod_latches: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub latch_group: u8,
    pub group_latch: u16,
}

pub const XCB_XKB_GET_CONTROLS: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub mouse_keys_dflt_btn: u8,
    pub num_groups: u8,
    pub groups_wrap: u8,
    pub internal_mods_mask: u8,
    pub ignore_lock_mods_mask: u8,
    pub internal_mods_real_mods: u8,
    pub ignore_lock_mods_real_mods: u8,
    pub pad0: u8,
    pub internal_mods_vmods: u16,
    pub ignore_lock_mods_vmods: u16,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys_delay: u16,
    pub mouse_keys_interval: u16,
    pub mouse_keys_time_to_max: u16,
    pub mouse_keys_max_speed: u16,
    pub mouse_keys_curve: i16,
    pub access_x_option: u16,
    pub access_x_timeout: u16,
    pub access_x_timeout_options_mask: u16,
    pub access_x_timeout_options_values: u16,
    pub pad1: [u8; 2],
    pub access_x_timeout_mask: u32,
    pub access_x_timeout_values: u32,
    pub enabled_controls: u32,
    pub per_key_repeat: [u8; 32],
}

pub const XCB_XKB_SET_CONTROLS: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_controls_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub affect_internal_real_mods: u8,
    pub internal_real_mods: u8,
    pub affect_ignore_lock_real_mods: u8,
    pub ignore_lock_real_mods: u8,
    pub affect_internal_virtual_mods: u16,
    pub internal_virtual_mods: u16,
    pub affect_ignore_lock_virtual_mods: u16,
    pub ignore_lock_virtual_mods: u16,
    pub mouse_keys_dflt_btn: u8,
    pub groups_wrap: u8,
    pub access_x_options: u16,
    pub pad0: [u8; 2],
    pub affect_enabled_controls: u32,
    pub enabled_controls: u32,
    pub change_controls: u32,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys_delay: u16,
    pub mouse_keys_interval: u16,
    pub mouse_keys_time_to_max: u16,
    pub mouse_keys_max_speed: u16,
    pub mouse_keys_curve: i16,
    pub access_x_timeout: u16,
    pub access_x_timeout_mask: u32,
    pub access_x_timeout_values: u32,
    pub access_x_timeout_options_mask: u16,
    pub access_x_timeout_options_values: u16,
    pub per_key_repeat: [u8; 32],
}

pub const XCB_XKB_GET_MAP: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub full: u16,
    pub partial: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub n_key_syms: u8,
    pub first_key_action: xcb_keycode_t,
    pub n_key_actions: u8,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behaviors: u8,
    pub virtual_mods: u16,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_map_t {
    pub types_rtrn: *mut xcb_xkb_key_type_t,
    pub syms_rtrn: *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count: *mut u8,
    pub pad2: *mut u8,
    pub acts_rtrn_acts: *mut xcb_xkb_action_t,
    pub behaviors_rtrn: *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn: *mut u8,
    pub pad3: *mut u8,
    pub explicit_rtrn: *mut xcb_xkb_set_explicit_t,
    pub pad4: *mut u8,
    pub modmap_rtrn: *mut xcb_xkb_key_mod_map_t,
    pub pad5: *mut u8,
    pub vmodmap_rtrn: *mut xcb_xkb_key_v_mod_map_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 2],
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub present: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: xcb_keycode_t,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub pad1: u8,
    pub virtual_mods: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_map_values_t {
    pub types: *mut xcb_xkb_set_key_type_t,
    pub syms: *mut xcb_xkb_key_sym_map_t,
    pub actions_count: *mut u8,
    pub actions: *mut xcb_xkb_action_t,
    pub behaviors: *mut xcb_xkb_set_behavior_t,
    pub vmods: *mut u8,
    pub explicit: *mut xcb_xkb_set_explicit_t,
    pub modmap: *mut xcb_xkb_key_mod_map_t,
    pub vmodmap: *mut xcb_xkb_key_v_mod_map_t,
}

pub const XCB_XKB_SET_MAP: u8 = 9;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub present: u16,
    pub flags: u16,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub n_key_syms: u8,
    pub total_syms: u16,
    pub first_key_action: xcb_keycode_t,
    pub n_key_actions: u8,
    pub total_actions: u16,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub virtual_mods: u16,
}

pub const XCB_XKB_GET_COMPAT_MAP: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub groups: u8,
    pub get_all_si: u8,
    pub first_si: u16,
    pub n_si: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub groups_rtrn: u8,
    pub pad0: u8,
    pub first_si_rtrn: u16,
    pub n_si_rtrn: u16,
    pub n_total_si: u16,
    pub pad1: [u8; 16],
}

pub const XCB_XKB_SET_COMPAT_MAP: u8 = 11;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_compat_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: u8,
    pub recompute_actions: u8,
    pub truncate_si: u8,
    pub groups: u8,
    pub first_si: u16,
    pub n_si: u16,
    pub pad1: [u8; 2],
}

pub const XCB_XKB_GET_INDICATOR_STATE: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
    pub pad0: [u8; 20],
}

pub const XCB_XKB_GET_INDICATOR_MAP: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
    pub which: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub which: u32,
    pub real_indicators: u32,
    pub n_indicators: u8,
    pub pad0: [u8; 15],
}

pub const XCB_XKB_SET_INDICATOR_MAP: u8 = 14;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_indicator_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
    pub which: u32,
}

pub const XCB_XKB_GET_NAMED_INDICATOR: u8 = 15;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_id: xcb_xkb_id_spec_t,
    pub pad0: [u8; 2],
    pub indicator: xcb_atom_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub indicator: xcb_atom_t,
    pub found: u8,
    pub on: u8,
    pub real_indicator: u8,
    pub ndx: u8,
    pub map_flags: u8,
    pub map_which_groups: u8,
    pub map_groups: u8,
    pub map_which_mods: u8,
    pub map_mods: u8,
    pub map_real_mods: u8,
    pub map_vmod: u16,
    pub map_ctrls: u32,
    pub supported: u8,
    pub pad0: [u8; 3],
}

pub const XCB_XKB_SET_NAMED_INDICATOR: u8 = 16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_named_indicator_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_id: xcb_xkb_id_spec_t,
    pub pad0: [u8; 2],
    pub indicator: xcb_atom_t,
    pub set_state: u8,
    pub on: u8,
    pub set_map: u8,
    pub create_map: u8,
    pub pad1: u8,
    pub map_flags: u8,
    pub map_which_groups: u8,
    pub map_groups: u8,
    pub map_which_mods: u8,
    pub map_real_mods: u8,
    pub map_vmods: u16,
    pub map_ctrls: u32,
}

pub const XCB_XKB_GET_NAMES: u8 = 17;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
    pub which: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_value_list_t {
    pub keycodes_name: xcb_atom_t,
    pub geometry_name: xcb_atom_t,
    pub symbols_name: xcb_atom_t,
    pub phys_symbols_name: xcb_atom_t,
    pub types_name: xcb_atom_t,
    pub compat_name: xcb_atom_t,
    pub type_names: *mut xcb_atom_t,
    pub n_levels_per_type: *mut u8,
    pub alignment_pad: *mut u8,
    pub kt_level_names: *mut xcb_atom_t,
    pub indicator_names: *mut xcb_atom_t,
    pub virtual_mod_names: *mut xcb_atom_t,
    pub groups: *mut xcb_atom_t,
    pub key_names: *mut xcb_xkb_key_name_t,
    pub key_aliases: *mut xcb_xkb_key_alias_t,
    pub radio_group_names: *mut xcb_atom_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub which: u32,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub n_types: u8,
    pub group_names: u8,
    pub virtual_mods: u16,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_kt_levels: u16,
    pub pad0: [u8; 4],
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_names_values_t {
    pub keycodes_name: xcb_atom_t,
    pub geometry_name: xcb_atom_t,
    pub symbols_name: xcb_atom_t,
    pub phys_symbols_name: xcb_atom_t,
    pub types_name: xcb_atom_t,
    pub compat_name: xcb_atom_t,
    pub type_names: *mut xcb_atom_t,
    pub n_levels_per_type: *mut u8,
    pub kt_level_names: *mut xcb_atom_t,
    pub indicator_names: *mut xcb_atom_t,
    pub virtual_mod_names: *mut xcb_atom_t,
    pub groups: *mut xcb_atom_t,
    pub key_names: *mut xcb_xkb_key_name_t,
    pub key_aliases: *mut xcb_xkb_key_alias_t,
    pub radio_group_names: *mut xcb_atom_t,
}

pub const XCB_XKB_SET_NAMES: u8 = 18;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_names_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub virtual_mods: u16,
    pub which: u32,
    pub first_type: u8,
    pub n_types: u8,
    pub first_kt_levelt: u8,
    pub n_kt_levels: u8,
    pub indicators: u32,
    pub group_names: u8,
    pub n_radio_groups: u8,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub n_key_aliases: u8,
    pub pad0: u8,
    pub total_kt_level_names: u16,
}

pub const XCB_XKB_PER_CLIENT_FLAGS: u8 = 21;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
    pub change: u32,
    pub value: u32,
    pub ctrls_to_change: u32,
    pub auto_ctrls: u32,
    pub auto_ctrls_values: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub supported: u32,
    pub value: u32,
    pub auto_ctrls: u32,
    pub auto_ctrls_values: u32,
    pub pad0: [u8; 8],
}

pub const XCB_XKB_LIST_COMPONENTS: u8 = 22;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub max_names: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub n_keymaps: u16,
    pub n_keycodes: u16,
    pub n_types: u16,
    pub n_compat_maps: u16,
    pub n_symbols: u16,
    pub n_geometries: u16,
    pub extra: u16,
    pub pad0: [u8; 10],
}

pub const XCB_XKB_GET_KBD_BY_NAME: u8 = 23;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub need: u16,
    pub want: u16,
    pub load: u8,
    pub pad0: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t {
    pub types: _xcb_xkb_get_kbd_by_name_replies__types,
    pub compat_map: _xcb_xkb_get_kbd_by_name_replies__compat_map,
    pub indicator_maps: _xcb_xkb_get_kbd_by_name_replies__indicator_maps,
    pub key_names: _xcb_xkb_get_kbd_by_name_replies__key_names,
    pub geometry: _xcb_xkb_get_kbd_by_name_replies__geometry,
}

#[derive(Debug)]
#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__types {
    pub getmap_type: u8,
    pub type_device_id: u8,
    pub getmap_sequence: u16,
    pub getmap_length: u32,
    pub pad1: [u8; 2],
    pub type_min_key_code: xcb_keycode_t,
    pub type_max_key_code: xcb_keycode_t,
    pub present: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: xcb_keycode_t,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub pad2: u8,
    pub virtual_mods: u16,
    pub map: xcb_xkb_get_kbd_by_name_replies_types_map_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__compat_map {
    pub compatmap_type: u8,
    pub compat_device_id: u8,
    pub compatmap_sequence: u16,
    pub compatmap_length: u32,
    pub groups_rtrn: u8,
    pub pad3: u8,
    pub first_si_rtrn: u16,
    pub n_si_rtrn: u16,
    pub n_total_si: u16,
    pub pad4: [u8; 16],
    pub si_rtrn: *mut xcb_xkb_sym_interpret_t,
    pub group_rtrn: *mut xcb_xkb_mod_def_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__indicator_maps {
    pub indicatormap_type: u8,
    pub indicator_device_id: u8,
    pub indicatormap_sequence: u16,
    pub indicatormap_length: u32,
    pub which: u32,
    pub real_indicators: u32,
    pub n_indicators: u8,
    pub pad5: [u8; 15],
    pub maps: *mut xcb_xkb_indicator_map_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__key_names {
    pub keyname_type: u8,
    pub key_device_id: u8,
    pub keyname_sequence: u16,
    pub keyname_length: u32,
    pub which: u32,
    pub key_min_key_code: xcb_keycode_t,
    pub key_max_key_code: xcb_keycode_t,
    pub n_types: u8,
    pub group_names: u8,
    pub virtual_mods: u16,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_kt_levels: u16,
    pub pad6: [u8; 4],
    pub value_list: xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__geometry {
    pub geometry_type: u8,
    pub geometry_device_id: u8,
    pub geometry_sequence: u16,
    pub geometry_length: u32,
    pub name: xcb_atom_t,
    pub geometry_found: u8,
    pub pad7: u8,
    pub width_mm: u16,
    pub height_mm: u16,
    pub n_properties: u16,
    pub n_colors: u16,
    pub n_shapes: u16,
    pub n_sections: u16,
    pub n_doodads: u16,
    pub n_key_aliases: u16,
    pub base_color_ndx: u8,
    pub label_color_ndx: u8,
    pub label_font: *mut xcb_xkb_counted_string_16_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_types_map_t {
    pub types_rtrn: *mut xcb_xkb_key_type_t,
    pub syms_rtrn: *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count: *mut u8,
    pub acts_rtrn_acts: *mut xcb_xkb_action_t,
    pub behaviors_rtrn: *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn: *mut u8,
    pub explicit_rtrn: *mut xcb_xkb_set_explicit_t,
    pub modmap_rtrn: *mut xcb_xkb_key_mod_map_t,
    pub vmodmap_rtrn: *mut xcb_xkb_key_v_mod_map_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {
    pub keycodes_name: xcb_atom_t,
    pub geometry_name: xcb_atom_t,
    pub symbols_name: xcb_atom_t,
    pub phys_symbols_name: xcb_atom_t,
    pub types_name: xcb_atom_t,
    pub compat_name: xcb_atom_t,
    pub type_names: *mut xcb_atom_t,
    pub n_levels_per_type: *mut u8,
    pub kt_level_names: *mut xcb_atom_t,
    pub indicator_names: *mut xcb_atom_t,
    pub virtual_mod_names: *mut xcb_atom_t,
    pub groups: *mut xcb_atom_t,
    pub key_names: *mut xcb_xkb_key_name_t,
    pub key_aliases: *mut xcb_xkb_key_alias_t,
    pub radio_group_names: *mut xcb_atom_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub loaded: u8,
    pub new_keyboard: u8,
    pub found: u16,
    pub reported: u16,
    pub pad0: [u8; 16],
}

pub const XCB_XKB_GET_DEVICE_INFO: u8 = 24;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub wanted: u16,
    pub all_buttons: u8,
    pub first_button: u8,
    pub n_buttons: u8,
    pub pad0: u8,
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_id: xcb_xkb_id_spec_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u16,
    pub supported: u16,
    pub unsupported: u16,
    pub n_device_led_f_bs: u16,
    pub first_btn_wanted: u8,
    pub n_btns_wanted: u8,
    pub first_btn_rtrn: u8,
    pub n_btns_rtrn: u8,
    pub total_btns: u8,
    pub has_own_state: u8,
    pub dflt_kbd_fb: u16,
    pub dflt_led_fb: u16,
    pub pad0: [u8; 2],
    pub dev_type: xcb_atom_t,
    pub name_len: u16,
}

pub const XCB_XKB_SET_DEVICE_INFO: u8 = 25;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_device_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub first_btn: u8,
    pub n_btns: u8,
    pub change: u16,
    pub n_device_led_f_bs: u16,
}

pub const XCB_XKB_SET_DEBUGGING_FLAGS: u8 = 101;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub msg_length: u16,
    pub pad0: [u8; 2],
    pub affect_flags: u32,
    pub flags: u32,
    pub affect_ctrls: u32,
    pub ctrls: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_cookie_t {
    pub(crate) sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub current_flags: u32,
    pub current_ctrls: u32,
    pub supported_flags: u32,
    pub supported_ctrls: u32,
    pub pad1: [u8; 8],
}

pub const XCB_XKB_NEW_KEYBOARD_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_new_keyboard_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub old_device_id: u8,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub old_min_key_code: xcb_keycode_t,
    pub old_max_key_code: xcb_keycode_t,
    pub request_major: u8,
    pub request_minor: u8,
    pub changed: u16,
    pub pad0: [u8; 14],
}

pub const XCB_XKB_MAP_NOTIFY: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub ptr_btn_actions: u8,
    pub changed: u16,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub n_key_syms: u8,
    pub first_key_act: xcb_keycode_t,
    pub n_key_acts: u8,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behavior: u8,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub virtual_mods: u16,
    pub pad0: [u8; 2],
}

pub const XCB_XKB_STATE_NOTIFY: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_state_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub mods: u8,
    pub base_mods: u8,
    pub latched_mods: u8,
    pub locked_mods: u8,
    pub group: u8,
    pub base_group: i16,
    pub latched_group: i16,
    pub locked_group: u8,
    pub compat_state: u8,
    pub grab_mods: u8,
    pub compat_grab_mods: u8,
    pub lookup_mods: u8,
    pub compat_loockup_mods: u8,
    pub ptr_btn_state: u16,
    pub changed: u16,
    pub keycode: xcb_keycode_t,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
}

pub const XCB_XKB_CONTROLS_NOTIFY: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_controls_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub num_groups: u8,
    pub pad0: [u8; 2],
    pub changed_controls: u32,
    pub enabled_controls: u32,
    pub enabled_control_changes: u32,
    pub keycode: xcb_keycode_t,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
    pub pad1: [u8; 4],
}

pub const XCB_XKB_INDICATOR_STATE_NOTIFY: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_state_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
    pub state: u32,
    pub state_changed: u32,
    pub pad1: [u8; 12],
}

pub const XCB_XKB_INDICATOR_MAP_NOTIFY: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
    pub state: u32,
    pub map_changed: u32,
    pub pad1: [u8; 12],
}

pub const XCB_XKB_NAMES_NOTIFY: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_names_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: u8,
    pub changed: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub first_level_name: u8,
    pub n_level_names: u8,
    pub pad1: u8,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub changed_group_names: u8,
    pub changed_virtual_mods: u16,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub changed_indicators: u32,
    pub pad2: [u8; 4],
}

pub const XCB_XKB_COMPAT_MAP_NOTIFY: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_compat_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub changed_groups: u8,
    pub first_si: u16,
    pub n_si: u16,
    pub n_total_si: u16,
    pub pad0: [u8; 16],
}

pub const XCB_XKB_BELL_NOTIFY: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_bell_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub bell_class: u8,
    pub bell_id: u8,
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
    pub name: xcb_atom_t,
    pub window: xcb_window_t,
    pub event_only: u8,
    pub pad0: [u8; 7],
}

pub const XCB_XKB_ACTION_MESSAGE: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_action_message_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub keycode: xcb_keycode_t,
    pub press: u8,
    pub key_event_follows: u8,
    pub mods: u8,
    pub group: u8,
    pub message: [xcb_xkb_string8_t; 8],
    pub pad0: [u8; 10],
}

pub const XCB_XKB_ACCESS_X_NOTIFY: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_access_x_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub keycode: xcb_keycode_t,
    pub detailt: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub pad0: [u8; 16],
}

pub const XCB_XKB_EXTENSION_DEVICE_NOTIFY: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_extension_device_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: u8,
    pub reason: u16,
    pub led_class: u16,
    pub led_id: u16,
    pub leds_defined: u32,
    pub led_state: u32,
    pub first_button: u8,
    pub n_buttons: u8,
    pub supported: u16,
    pub unsupported: u16,
    pub pad1: [u8; 2],
}

#[link(name = "xcb-xkb")]
extern "C" {

    pub static mut xcb_xkb_id: xcb_extension_t;

    pub fn xcb_xkb_device_spec_next(i: *mut xcb_xkb_device_spec_iterator_t);

    pub fn xcb_xkb_device_spec_end(
        i: *mut xcb_xkb_device_spec_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_led_class_spec_next(i: *mut xcb_xkb_led_class_spec_iterator_t);

    pub fn xcb_xkb_led_class_spec_end(
        i: *mut xcb_xkb_led_class_spec_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_bell_class_spec_next(i: *mut xcb_xkb_bell_class_spec_iterator_t);

    pub fn xcb_xkb_bell_class_spec_end(
        i: *mut xcb_xkb_bell_class_spec_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_id_spec_next(i: *mut xcb_xkb_id_spec_iterator_t);

    pub fn xcb_xkb_id_spec_end(i: *mut xcb_xkb_id_spec_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_indicator_map_next(i: *mut xcb_xkb_indicator_map_iterator_t);

    pub fn xcb_xkb_indicator_map_end(
        i: *mut xcb_xkb_indicator_map_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_mod_def_next(i: *mut xcb_xkb_mod_def_iterator_t);

    pub fn xcb_xkb_mod_def_end(i: *mut xcb_xkb_mod_def_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_name_next(i: *mut xcb_xkb_key_name_iterator_t);

    pub fn xcb_xkb_key_name_end(i: *mut xcb_xkb_key_name_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_alias_next(i: *mut xcb_xkb_key_alias_iterator_t);

    pub fn xcb_xkb_key_alias_end(i: *mut xcb_xkb_key_alias_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_counted_string_16_string(R: *const xcb_xkb_counted_string_16_t) -> *mut c_char;

    pub fn xcb_xkb_counted_string_16_string_length(R: *const xcb_xkb_counted_string_16_t) -> c_int;

    pub fn xcb_xkb_counted_string_16_string_end(
        R: *const xcb_xkb_counted_string_16_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_counted_string_16_alignment_pad(
        R: *const xcb_xkb_counted_string_16_t,
    ) -> *mut c_void;

    pub fn xcb_xkb_counted_string_16_alignment_pad_length(
        R: *const xcb_xkb_counted_string_16_t,
    ) -> c_int;

    pub fn xcb_xkb_counted_string_16_alignment_pad_end(
        R: *const xcb_xkb_counted_string_16_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_counted_string_16_next(i: *mut xcb_xkb_counted_string_16_iterator_t);

    pub fn xcb_xkb_counted_string_16_end(
        i: *mut xcb_xkb_counted_string_16_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_kt_map_entry_next(i: *mut xcb_xkb_kt_map_entry_iterator_t);

    pub fn xcb_xkb_kt_map_entry_end(
        i: *mut xcb_xkb_kt_map_entry_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_type_map(R: *const xcb_xkb_key_type_t) -> *mut xcb_xkb_kt_map_entry_t;

    pub fn xcb_xkb_key_type_map_length(R: *const xcb_xkb_key_type_t) -> c_int;

    pub fn xcb_xkb_key_type_map_iterator(
        R: *const xcb_xkb_key_type_t,
    ) -> xcb_xkb_kt_map_entry_iterator_t;

    pub fn xcb_xkb_key_type_preserve(R: *const xcb_xkb_key_type_t) -> *mut xcb_xkb_mod_def_t;

    pub fn xcb_xkb_key_type_preserve_length(R: *const xcb_xkb_key_type_t) -> c_int;

    pub fn xcb_xkb_key_type_preserve_iterator(
        R: *const xcb_xkb_key_type_t,
    ) -> xcb_xkb_mod_def_iterator_t;

    pub fn xcb_xkb_key_type_next(i: *mut xcb_xkb_key_type_iterator_t);

    pub fn xcb_xkb_key_type_end(i: *mut xcb_xkb_key_type_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_sym_map_syms(R: *const xcb_xkb_key_sym_map_t) -> *mut xcb_keysym_t;

    pub fn xcb_xkb_key_sym_map_syms_length(R: *const xcb_xkb_key_sym_map_t) -> c_int;

    pub fn xcb_xkb_key_sym_map_syms_end(R: *const xcb_xkb_key_sym_map_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_sym_map_next(i: *mut xcb_xkb_key_sym_map_iterator_t);

    pub fn xcb_xkb_key_sym_map_end(
        i: *mut xcb_xkb_key_sym_map_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_common_behavior_next(i: *mut xcb_xkb_common_behavior_iterator_t);

    pub fn xcb_xkb_common_behavior_end(
        i: *mut xcb_xkb_common_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_default_behavior_next(i: *mut xcb_xkb_default_behavior_iterator_t);

    pub fn xcb_xkb_default_behavior_end(
        i: *mut xcb_xkb_default_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_lock_behavior_next(i: *mut xcb_xkb_lock_behavior_iterator_t);

    pub fn xcb_xkb_lock_behavior_end(
        i: *mut xcb_xkb_lock_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_radio_group_behavior_next(i: *mut xcb_xkb_radio_group_behavior_iterator_t);

    pub fn xcb_xkb_radio_group_behavior_end(
        i: *mut xcb_xkb_radio_group_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_overlay_behavior_next(i: *mut xcb_xkb_overlay_behavior_iterator_t);

    pub fn xcb_xkb_overlay_behavior_end(
        i: *mut xcb_xkb_overlay_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_permament_lock_behavior_next(i: *mut xcb_xkb_permament_lock_behavior_iterator_t);

    pub fn xcb_xkb_permament_lock_behavior_end(
        i: *mut xcb_xkb_permament_lock_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_permament_radio_group_behavior_next(
        i: *mut xcb_xkb_permament_radio_group_behavior_iterator_t,
    );

    pub fn xcb_xkb_permament_radio_group_behavior_end(
        i: *mut xcb_xkb_permament_radio_group_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_permament_overlay_behavior_next(
        i: *mut xcb_xkb_permament_overlay_behavior_iterator_t,
    );

    pub fn xcb_xkb_permament_overlay_behavior_end(
        i: *mut xcb_xkb_permament_overlay_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_behavior_next(i: *mut xcb_xkb_behavior_iterator_t);

    pub fn xcb_xkb_behavior_end(i: *mut xcb_xkb_behavior_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_behavior_next(i: *mut xcb_xkb_set_behavior_iterator_t);

    pub fn xcb_xkb_set_behavior_end(
        i: *mut xcb_xkb_set_behavior_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_explicit_next(i: *mut xcb_xkb_set_explicit_iterator_t);

    pub fn xcb_xkb_set_explicit_end(
        i: *mut xcb_xkb_set_explicit_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_mod_map_next(i: *mut xcb_xkb_key_mod_map_iterator_t);

    pub fn xcb_xkb_key_mod_map_end(
        i: *mut xcb_xkb_key_mod_map_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_v_mod_map_next(i: *mut xcb_xkb_key_v_mod_map_iterator_t);

    pub fn xcb_xkb_key_v_mod_map_end(
        i: *mut xcb_xkb_key_v_mod_map_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_kt_set_map_entry_next(i: *mut xcb_xkb_kt_set_map_entry_iterator_t);

    pub fn xcb_xkb_kt_set_map_entry_end(
        i: *mut xcb_xkb_kt_set_map_entry_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_key_type_entries(
        R: *const xcb_xkb_set_key_type_t,
    ) -> *mut xcb_xkb_kt_set_map_entry_t;

    pub fn xcb_xkb_set_key_type_entries_length(R: *const xcb_xkb_set_key_type_t) -> c_int;

    pub fn xcb_xkb_set_key_type_entries_iterator(
        R: *const xcb_xkb_set_key_type_t,
    ) -> xcb_xkb_kt_set_map_entry_iterator_t;

    pub fn xcb_xkb_set_key_type_preserve_entries(
        R: *const xcb_xkb_set_key_type_t,
    ) -> *mut xcb_xkb_kt_set_map_entry_t;

    pub fn xcb_xkb_set_key_type_preserve_entries_length(R: *const xcb_xkb_set_key_type_t) -> c_int;

    pub fn xcb_xkb_set_key_type_preserve_entries_iterator(
        R: *const xcb_xkb_set_key_type_t,
    ) -> xcb_xkb_kt_set_map_entry_iterator_t;

    pub fn xcb_xkb_set_key_type_next(i: *mut xcb_xkb_set_key_type_iterator_t);

    pub fn xcb_xkb_set_key_type_end(
        i: *mut xcb_xkb_set_key_type_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_string8_next(i: *mut xcb_xkb_string8_iterator_t);

    pub fn xcb_xkb_string8_end(i: *mut xcb_xkb_string8_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_outline_points(R: *const xcb_xkb_outline_t) -> *mut xcb_point_t;

    pub fn xcb_xkb_outline_points_length(R: *const xcb_xkb_outline_t) -> c_int;

    pub fn xcb_xkb_outline_points_iterator(R: *const xcb_xkb_outline_t) -> xcb_point_iterator_t;

    pub fn xcb_xkb_outline_next(i: *mut xcb_xkb_outline_iterator_t);

    pub fn xcb_xkb_outline_end(i: *mut xcb_xkb_outline_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_shape_outlines_length(R: *const xcb_xkb_shape_t) -> c_int;

    pub fn xcb_xkb_shape_outlines_iterator<'a>(
        R: *const xcb_xkb_shape_t,
    ) -> xcb_xkb_outline_iterator_t<'a>;

    pub fn xcb_xkb_shape_next(i: *mut xcb_xkb_shape_iterator_t);

    pub fn xcb_xkb_shape_end(i: *mut xcb_xkb_shape_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_next(i: *mut xcb_xkb_key_iterator_t);

    pub fn xcb_xkb_key_end(i: *mut xcb_xkb_key_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_overlay_key_next(i: *mut xcb_xkb_overlay_key_iterator_t);

    pub fn xcb_xkb_overlay_key_end(
        i: *mut xcb_xkb_overlay_key_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_overlay_row_keys(R: *const xcb_xkb_overlay_row_t) -> *mut xcb_xkb_overlay_key_t;

    pub fn xcb_xkb_overlay_row_keys_length(R: *const xcb_xkb_overlay_row_t) -> c_int;

    pub fn xcb_xkb_overlay_row_keys_iterator<'a>(
        R: *const xcb_xkb_overlay_row_t,
    ) -> xcb_xkb_overlay_key_iterator_t<'a>;

    pub fn xcb_xkb_overlay_row_next(i: *mut xcb_xkb_overlay_row_iterator_t);

    pub fn xcb_xkb_overlay_row_end(
        i: *mut xcb_xkb_overlay_row_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_overlay_rows_length(R: *const xcb_xkb_overlay_t) -> c_int;

    pub fn xcb_xkb_overlay_rows_iterator<'a>(
        R: *const xcb_xkb_overlay_t,
    ) -> xcb_xkb_overlay_row_iterator_t<'a>;

    pub fn xcb_xkb_overlay_next(i: *mut xcb_xkb_overlay_iterator_t);

    pub fn xcb_xkb_overlay_end(i: *mut xcb_xkb_overlay_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_row_keys(R: *const xcb_xkb_row_t) -> *mut xcb_xkb_key_t;

    pub fn xcb_xkb_row_keys_length(R: *const xcb_xkb_row_t) -> c_int;

    pub fn xcb_xkb_row_keys_iterator<'a>(R: *const xcb_xkb_row_t) -> xcb_xkb_key_iterator_t<'a>;

    pub fn xcb_xkb_row_next(i: *mut xcb_xkb_row_iterator_t);

    pub fn xcb_xkb_row_end(i: *mut xcb_xkb_row_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_listing_string(R: *const xcb_xkb_listing_t) -> *mut xcb_xkb_string8_t;

    pub fn xcb_xkb_listing_string_length(R: *const xcb_xkb_listing_t) -> c_int;

    pub fn xcb_xkb_listing_string_end(R: *const xcb_xkb_listing_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_listing_next(i: *mut xcb_xkb_listing_iterator_t);

    pub fn xcb_xkb_listing_end(i: *mut xcb_xkb_listing_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_device_led_info_names(R: *const xcb_xkb_device_led_info_t) -> *mut xcb_atom_t;

    pub fn xcb_xkb_device_led_info_names_length(R: *const xcb_xkb_device_led_info_t) -> c_int;

    pub fn xcb_xkb_device_led_info_names_end(
        R: *const xcb_xkb_device_led_info_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_device_led_info_maps(
        R: *const xcb_xkb_device_led_info_t,
    ) -> *mut xcb_xkb_indicator_map_t;

    pub fn xcb_xkb_device_led_info_maps_length(R: *const xcb_xkb_device_led_info_t) -> c_int;

    pub fn xcb_xkb_device_led_info_maps_iterator(
        R: *const xcb_xkb_device_led_info_t,
    ) -> xcb_xkb_indicator_map_iterator_t;

    pub fn xcb_xkb_device_led_info_next(i: *mut xcb_xkb_device_led_info_iterator_t);

    pub fn xcb_xkb_device_led_info_end(
        i: *mut xcb_xkb_device_led_info_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_no_action_next(i: *mut xcb_xkb_sa_no_action_iterator_t);

    pub fn xcb_xkb_sa_no_action_end(
        i: *mut xcb_xkb_sa_no_action_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_set_mods_next(i: *mut xcb_xkb_sa_set_mods_iterator_t);

    pub fn xcb_xkb_sa_set_mods_end(
        i: *mut xcb_xkb_sa_set_mods_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_latch_mods_next(i: *mut xcb_xkb_sa_latch_mods_iterator_t);

    pub fn xcb_xkb_sa_latch_mods_end(
        i: *mut xcb_xkb_sa_latch_mods_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_mods_next(i: *mut xcb_xkb_sa_lock_mods_iterator_t);

    pub fn xcb_xkb_sa_lock_mods_end(
        i: *mut xcb_xkb_sa_lock_mods_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_set_group_next(i: *mut xcb_xkb_sa_set_group_iterator_t);

    pub fn xcb_xkb_sa_set_group_end(
        i: *mut xcb_xkb_sa_set_group_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_latch_group_next(i: *mut xcb_xkb_sa_latch_group_iterator_t);

    pub fn xcb_xkb_sa_latch_group_end(
        i: *mut xcb_xkb_sa_latch_group_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_group_next(i: *mut xcb_xkb_sa_lock_group_iterator_t);

    pub fn xcb_xkb_sa_lock_group_end(
        i: *mut xcb_xkb_sa_lock_group_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_move_ptr_next(i: *mut xcb_xkb_sa_move_ptr_iterator_t);

    pub fn xcb_xkb_sa_move_ptr_end(
        i: *mut xcb_xkb_sa_move_ptr_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_ptr_btn_next(i: *mut xcb_xkb_sa_ptr_btn_iterator_t);

    pub fn xcb_xkb_sa_ptr_btn_end(i: *mut xcb_xkb_sa_ptr_btn_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_ptr_btn_next(i: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t);

    pub fn xcb_xkb_sa_lock_ptr_btn_end(
        i: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_set_ptr_dflt_next(i: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t);

    pub fn xcb_xkb_sa_set_ptr_dflt_end(
        i: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_iso_lock_next(i: *mut xcb_xkb_sa_iso_lock_iterator_t);

    pub fn xcb_xkb_sa_iso_lock_end(
        i: *mut xcb_xkb_sa_iso_lock_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_terminate_next(i: *mut xcb_xkb_sa_terminate_iterator_t);

    pub fn xcb_xkb_sa_terminate_end(
        i: *mut xcb_xkb_sa_terminate_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_switch_screen_next(i: *mut xcb_xkb_sa_switch_screen_iterator_t);

    pub fn xcb_xkb_sa_switch_screen_end(
        i: *mut xcb_xkb_sa_switch_screen_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_set_controls_next(i: *mut xcb_xkb_sa_set_controls_iterator_t);

    pub fn xcb_xkb_sa_set_controls_end(
        i: *mut xcb_xkb_sa_set_controls_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_controls_next(i: *mut xcb_xkb_sa_lock_controls_iterator_t);

    pub fn xcb_xkb_sa_lock_controls_end(
        i: *mut xcb_xkb_sa_lock_controls_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_action_message_next(i: *mut xcb_xkb_sa_action_message_iterator_t);

    pub fn xcb_xkb_sa_action_message_end(
        i: *mut xcb_xkb_sa_action_message_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_redirect_key_next(i: *mut xcb_xkb_sa_redirect_key_iterator_t);

    pub fn xcb_xkb_sa_redirect_key_end(
        i: *mut xcb_xkb_sa_redirect_key_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_device_btn_next(i: *mut xcb_xkb_sa_device_btn_iterator_t);

    pub fn xcb_xkb_sa_device_btn_end(
        i: *mut xcb_xkb_sa_device_btn_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_device_btn_next(i: *mut xcb_xkb_sa_lock_device_btn_iterator_t);

    pub fn xcb_xkb_sa_lock_device_btn_end(
        i: *mut xcb_xkb_sa_lock_device_btn_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_device_valuator_next(i: *mut xcb_xkb_sa_device_valuator_iterator_t);

    pub fn xcb_xkb_sa_device_valuator_end(
        i: *mut xcb_xkb_sa_device_valuator_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_si_action_next(i: *mut xcb_xkb_si_action_iterator_t);

    pub fn xcb_xkb_si_action_end(i: *mut xcb_xkb_si_action_iterator_t) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sym_interpret_next(i: *mut xcb_xkb_sym_interpret_iterator_t);

    pub fn xcb_xkb_sym_interpret_end(
        i: *mut xcb_xkb_sym_interpret_iterator_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_action_next(i: *mut xcb_xkb_action_iterator_t);

    pub fn xcb_xkb_action_end(i: *mut xcb_xkb_action_iterator_t) -> xcb_generic_iterator_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_use_extension_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_use_extension_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_use_extension_reply_t;

    pub fn xcb_xkb_use_extension(
        c: *mut xcb_connection_t,
        wanted_major: u16,
        wanted_minor: u16,
    ) -> xcb_xkb_use_extension_cookie_t;

    pub fn xcb_xkb_use_extension_unchecked(
        c: *mut xcb_connection_t,
        wanted_major: u16,
        wanted_minor: u16,
    ) -> xcb_xkb_use_extension_cookie_t;

    pub fn xcb_xkb_select_events(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_which: u16,
        clear: u16,
        select_all: u16,
        affect_map: u16,
        map: u16,
        details: *const xcb_xkb_select_events_details_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_select_events_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_which: u16,
        clear: u16,
        select_all: u16,
        affect_map: u16,
        map: u16,
        details: *const xcb_xkb_select_events_details_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_bell(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        bell_class: xcb_xkb_bell_class_spec_t,
        bell_id: xcb_xkb_id_spec_t,
        percent: i8,
        force_sound: u8,
        event_only: u8,
        pitch: i16,
        duration: i16,
        name: xcb_atom_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_bell_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        bell_class: xcb_xkb_bell_class_spec_t,
        bell_id: xcb_xkb_id_spec_t,
        percent: i8,
        force_sound: u8,
        event_only: u8,
        pitch: i16,
        duration: i16,
        name: xcb_atom_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_state_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_state_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_state_reply_t;

    pub fn xcb_xkb_get_state(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_state_cookie_t;

    pub fn xcb_xkb_get_state_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_state_cookie_t;

    pub fn xcb_xkb_latch_lock_state(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_mod_locks: u8,
        mod_locks: u8,
        lock_group: u8,
        group_lock: u8,
        affect_mod_latches: u8,
        latch_group: u8,
        group_latch: u16,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_latch_lock_state_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_mod_locks: u8,
        mod_locks: u8,
        lock_group: u8,
        group_lock: u8,
        affect_mod_latches: u8,
        latch_group: u8,
        group_latch: u16,
    ) -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_controls_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_controls_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_controls_reply_t;

    pub fn xcb_xkb_get_controls(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_controls_cookie_t;

    pub fn xcb_xkb_get_controls_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_controls_cookie_t;

    pub fn xcb_xkb_set_controls(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_internal_real_mods: u8,
        internal_real_mods: u8,
        affect_ignore_lock_real_mods: u8,
        ignore_lock_real_mods: u8,
        affect_internal_virtual_mods: u16,
        internal_virtual_mods: u16,
        affect_ignore_lock_virtual_mods: u16,
        ignore_lock_virtual_mods: u16,
        mouse_keys_dflt_btn: u8,
        groups_wrap: u8,
        access_x_options: u16,
        affect_enabled_controls: u32,
        enabled_controls: u32,
        change_controls: u32,
        repeat_delay: u16,
        repeat_interval: u16,
        slow_keys_delay: u16,
        debounce_delay: u16,
        mouse_keys_delay: u16,
        mouse_keys_interval: u16,
        mouse_keys_time_to_max: u16,
        mouse_keys_max_speed: u16,
        mouse_keys_curve: i16,
        access_x_timeout: u16,
        access_x_timeout_mask: u32,
        access_x_timeout_values: u32,
        access_x_timeout_options_mask: u16,
        access_x_timeout_options_values: u16,
        per_key_repeat: *const u8,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_controls_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_internal_real_mods: u8,
        internal_real_mods: u8,
        affect_ignore_lock_real_mods: u8,
        ignore_lock_real_mods: u8,
        affect_internal_virtual_mods: u16,
        internal_virtual_mods: u16,
        affect_ignore_lock_virtual_mods: u16,
        ignore_lock_virtual_mods: u16,
        mouse_keys_dflt_btn: u8,
        groups_wrap: u8,
        access_x_options: u16,
        affect_enabled_controls: u32,
        enabled_controls: u32,
        change_controls: u32,
        repeat_delay: u16,
        repeat_interval: u16,
        slow_keys_delay: u16,
        debounce_delay: u16,
        mouse_keys_delay: u16,
        mouse_keys_interval: u16,
        mouse_keys_time_to_max: u16,
        mouse_keys_max_speed: u16,
        mouse_keys_curve: i16,
        access_x_timeout: u16,
        access_x_timeout_mask: u32,
        access_x_timeout_values: u32,
        access_x_timeout_options_mask: u16,
        access_x_timeout_options_values: u16,
        per_key_repeat: *const u8,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_get_map_map_types_rtrn_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_types_rtrn_iterator<'a>(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_key_type_iterator_t<'a>;

    pub fn xcb_xkb_get_map_map_syms_rtrn_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_syms_rtrn_iterator<'a>(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_key_sym_map_iterator_t<'a>;

    pub fn xcb_xkb_get_map_map_acts_rtrn_count(S: *const xcb_xkb_get_map_map_t) -> *mut u8;

    pub fn xcb_xkb_get_map_map_acts_rtrn_count_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_acts_rtrn_count_end(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_map_map_acts_rtrn_acts(
        S: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_action_t;

    pub fn xcb_xkb_get_map_map_acts_rtrn_acts_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_acts_rtrn_acts_iterator(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_action_iterator_t;

    pub fn xcb_xkb_get_map_map_behaviors_rtrn(
        S: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_set_behavior_t;

    pub fn xcb_xkb_get_map_map_behaviors_rtrn_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_behaviors_rtrn_iterator<'a>(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_set_behavior_iterator_t<'a>;

    pub fn xcb_xkb_get_map_map_vmods_rtrn(S: *const xcb_xkb_get_map_map_t) -> *mut u8;

    pub fn xcb_xkb_get_map_map_vmods_rtrn_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_vmods_rtrn_end(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_map_map_explicit_rtrn(
        S: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_set_explicit_t;

    pub fn xcb_xkb_get_map_map_explicit_rtrn_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_explicit_rtrn_iterator(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_set_explicit_iterator_t;

    pub fn xcb_xkb_get_map_map_modmap_rtrn(
        S: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_key_mod_map_t;

    pub fn xcb_xkb_get_map_map_modmap_rtrn_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_modmap_rtrn_iterator(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_key_mod_map_iterator_t;

    pub fn xcb_xkb_get_map_map_vmodmap_rtrn(
        S: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_key_v_mod_map_t;

    pub fn xcb_xkb_get_map_map_vmodmap_rtrn_length(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> c_int;

    pub fn xcb_xkb_get_map_map_vmodmap_rtrn_iterator(
        R: *const xcb_xkb_get_map_reply_t,
        S: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_key_v_mod_map_iterator_t;
    pub fn xcb_xkb_get_map_map(R: *const xcb_xkb_get_map_reply_t) -> *mut c_void;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_map_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_map_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_map_reply_t;

    pub fn xcb_xkb_get_map(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        full: u16,
        partial: u16,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        virtual_mods: u16,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
    ) -> xcb_xkb_get_map_cookie_t;

    pub fn xcb_xkb_get_map_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        full: u16,
        partial: u16,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        virtual_mods: u16,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
    ) -> xcb_xkb_get_map_cookie_t;

    pub fn xcb_xkb_set_map_values_types_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_types_iterator<'a>(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_set_key_type_iterator_t<'a>;

    pub fn xcb_xkb_set_map_values_syms_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_syms_iterator<'a>(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_key_sym_map_iterator_t<'a>;

    pub fn xcb_xkb_set_map_values_actions_count(S: *const xcb_xkb_set_map_values_t) -> *mut u8;

    pub fn xcb_xkb_set_map_values_actions_count_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_actions_count_end(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_map_values_actions(
        S: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_action_t;

    pub fn xcb_xkb_set_map_values_actions_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_actions_iterator(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_action_iterator_t;

    pub fn xcb_xkb_set_map_values_behaviors(
        S: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_set_behavior_t;

    pub fn xcb_xkb_set_map_values_behaviors_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_behaviors_iterator<'a>(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_set_behavior_iterator_t<'a>;

    pub fn xcb_xkb_set_map_values_vmods(S: *const xcb_xkb_set_map_values_t) -> *mut u8;

    pub fn xcb_xkb_set_map_values_vmods_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_vmods_end(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_map_values_explicit(
        S: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_set_explicit_t;

    pub fn xcb_xkb_set_map_values_explicit_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_explicit_iterator(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_set_explicit_iterator_t;

    pub fn xcb_xkb_set_map_values_modmap(
        S: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_key_mod_map_t;

    pub fn xcb_xkb_set_map_values_modmap_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_modmap_iterator(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_key_mod_map_iterator_t;

    pub fn xcb_xkb_set_map_values_vmodmap(
        S: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_key_v_mod_map_t;

    pub fn xcb_xkb_set_map_values_vmodmap_length(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_map_values_vmodmap_iterator(
        R: *const xcb_xkb_set_map_request_t,
        S: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_key_v_mod_map_iterator_t;

    pub fn xcb_xkb_set_map(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        present: u16,
        flags: u16,
        min_key_code: xcb_keycode_t,
        max_key_code: xcb_keycode_t,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        total_syms: u16,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        total_actions: u16,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        total_key_behaviors: u8,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        total_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        total_mod_map_keys: u8,
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
        virtual_mods: u16,
        values: *const xcb_xkb_set_map_values_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_map_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        present: u16,
        flags: u16,
        min_key_code: xcb_keycode_t,
        max_key_code: xcb_keycode_t,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        total_syms: u16,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        total_actions: u16,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        total_key_behaviors: u8,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        total_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        total_mod_map_keys: u8,
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
        virtual_mods: u16,
        values: *const xcb_xkb_set_map_values_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_get_compat_map_si_rtrn(
        R: *const xcb_xkb_get_compat_map_reply_t,
    ) -> *mut xcb_xkb_sym_interpret_t;

    pub fn xcb_xkb_get_compat_map_si_rtrn_length(R: *const xcb_xkb_get_compat_map_reply_t)
        -> c_int;

    pub fn xcb_xkb_get_compat_map_si_rtrn_iterator<'a>(
        R: *const xcb_xkb_get_compat_map_reply_t,
    ) -> xcb_xkb_sym_interpret_iterator_t<'a>;

    pub fn xcb_xkb_get_compat_map_group_rtrn(
        R: *const xcb_xkb_get_compat_map_reply_t,
    ) -> *mut xcb_xkb_mod_def_t;

    pub fn xcb_xkb_get_compat_map_group_rtrn_length(
        R: *const xcb_xkb_get_compat_map_reply_t,
    ) -> c_int;

    pub fn xcb_xkb_get_compat_map_group_rtrn_iterator(
        R: *const xcb_xkb_get_compat_map_reply_t,
    ) -> xcb_xkb_mod_def_iterator_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_compat_map_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_compat_map_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_compat_map_reply_t;

    pub fn xcb_xkb_get_compat_map(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        groups: u8,
        get_all_si: u8,
        first_si: u16,
        n_si: u16,
    ) -> xcb_xkb_get_compat_map_cookie_t;

    pub fn xcb_xkb_get_compat_map_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        groups: u8,
        get_all_si: u8,
        first_si: u16,
        n_si: u16,
    ) -> xcb_xkb_get_compat_map_cookie_t;

    pub fn xcb_xkb_set_compat_map(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        recompute_actions: u8,
        truncate_si: u8,
        groups: u8,
        first_si: u16,
        n_si: u16,
        si: *const xcb_xkb_sym_interpret_t,
        group_maps: *const xcb_xkb_mod_def_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_compat_map_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        recompute_actions: u8,
        truncate_si: u8,
        groups: u8,
        first_si: u16,
        n_si: u16,
        si: *const xcb_xkb_sym_interpret_t,
        group_maps: *const xcb_xkb_mod_def_t,
    ) -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_indicator_state_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_indicator_state_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_indicator_state_reply_t;

    pub fn xcb_xkb_get_indicator_state(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_indicator_state_cookie_t;

    pub fn xcb_xkb_get_indicator_state_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_indicator_state_cookie_t;

    pub fn xcb_xkb_get_indicator_map_maps(
        R: *const xcb_xkb_get_indicator_map_reply_t,
    ) -> *mut xcb_xkb_indicator_map_t;

    pub fn xcb_xkb_get_indicator_map_maps_length(
        R: *const xcb_xkb_get_indicator_map_reply_t,
    ) -> c_int;

    pub fn xcb_xkb_get_indicator_map_maps_iterator(
        R: *const xcb_xkb_get_indicator_map_reply_t,
    ) -> xcb_xkb_indicator_map_iterator_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_indicator_map_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_indicator_map_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_indicator_map_reply_t;

    pub fn xcb_xkb_get_indicator_map(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
    ) -> xcb_xkb_get_indicator_map_cookie_t;

    pub fn xcb_xkb_get_indicator_map_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
    ) -> xcb_xkb_get_indicator_map_cookie_t;

    pub fn xcb_xkb_set_indicator_map(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
        maps: *const xcb_xkb_indicator_map_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_indicator_map_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
        maps: *const xcb_xkb_indicator_map_t,
    ) -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_named_indicator_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_named_indicator_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_named_indicator_reply_t;

    pub fn xcb_xkb_get_named_indicator(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
    ) -> xcb_xkb_get_named_indicator_cookie_t;

    pub fn xcb_xkb_get_named_indicator_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
    ) -> xcb_xkb_get_named_indicator_cookie_t;

    pub fn xcb_xkb_set_named_indicator(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
        set_state: u8,
        on: u8,
        set_map: u8,
        create_map: u8,
        map_flags: u8,
        map_which_groups: u8,
        map_groups: u8,
        map_which_mods: u8,
        map_real_mods: u8,
        map_vmods: u16,
        map_ctrls: u32,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_named_indicator_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
        set_state: u8,
        on: u8,
        set_map: u8,
        create_map: u8,
        map_flags: u8,
        map_which_groups: u8,
        map_groups: u8,
        map_which_mods: u8,
        map_real_mods: u8,
        map_vmods: u16,
        map_ctrls: u32,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_get_names_value_list_type_names(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_type_names_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_type_names_end(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_n_levels_per_type(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut u8;

    pub fn xcb_xkb_get_names_value_list_n_levels_per_type_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_n_levels_per_type_end(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_alignment_pad(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut u8;

    pub fn xcb_xkb_get_names_value_list_alignment_pad_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_alignment_pad_end(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_kt_level_names(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_kt_level_names_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_kt_level_names_end(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_indicator_names(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_indicator_names_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_indicator_names_end(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_virtual_mod_names(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_virtual_mod_names_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_virtual_mod_names_end(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_groups(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_groups_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_groups_end(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_key_names(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_xkb_key_name_t;

    pub fn xcb_xkb_get_names_value_list_key_names_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_key_names_iterator<'a>(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_xkb_key_name_iterator_t<'a>;

    pub fn xcb_xkb_get_names_value_list_key_aliases(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_xkb_key_alias_t;

    pub fn xcb_xkb_get_names_value_list_key_aliases_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_key_aliases_iterator<'a>(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_xkb_key_alias_iterator_t<'a>;

    pub fn xcb_xkb_get_names_value_list_radio_group_names(
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_radio_group_names_length(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int;

    pub fn xcb_xkb_get_names_value_list_radio_group_names_end(
        R: *const xcb_xkb_get_names_reply_t,
        S: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_xkb_get_names_value_list(R: *const xcb_xkb_get_names_reply_t) -> *mut c_void;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_names_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_names_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_names_reply_t;

    pub fn xcb_xkb_get_names(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
    ) -> xcb_xkb_get_names_cookie_t;

    pub fn xcb_xkb_get_names_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
    ) -> xcb_xkb_get_names_cookie_t;

    pub fn xcb_xkb_set_names_values_type_names(
        S: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_type_names_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_type_names_end(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_n_levels_per_type(
        S: *const xcb_xkb_set_names_values_t,
    ) -> *mut u8;

    pub fn xcb_xkb_set_names_values_n_levels_per_type_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_n_levels_per_type_end(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_kt_level_names(
        S: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_kt_level_names_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_kt_level_names_end(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_indicator_names(
        S: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_indicator_names_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_indicator_names_end(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_virtual_mod_names(
        S: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_virtual_mod_names_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_virtual_mod_names_end(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_groups(S: *const xcb_xkb_set_names_values_t)
        -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_groups_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_groups_end(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_key_names(
        S: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_xkb_key_name_t;

    pub fn xcb_xkb_set_names_values_key_names_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_key_names_iterator<'a>(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_xkb_key_name_iterator_t<'a>;

    pub fn xcb_xkb_set_names_values_key_aliases(
        S: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_xkb_key_alias_t;

    pub fn xcb_xkb_set_names_values_key_aliases_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_key_aliases_iterator<'a>(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_xkb_key_alias_iterator_t<'a>;

    pub fn xcb_xkb_set_names_values_radio_group_names(
        S: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_radio_group_names_length(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> c_int;

    pub fn xcb_xkb_set_names_values_radio_group_names_end(
        R: *const xcb_xkb_set_names_request_t,
        S: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_kt_levelt: u8,
        n_kt_levels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_kt_level_names: u16,
        values: *const xcb_xkb_set_names_values_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_names_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_kt_levelt: u8,
        n_kt_levels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_kt_level_names: u16,
        values: *const xcb_xkb_set_names_values_t,
    ) -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_per_client_flags_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_per_client_flags_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_per_client_flags_reply_t;

    pub fn xcb_xkb_per_client_flags(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        change: u32,
        value: u32,
        ctrls_to_change: u32,
        auto_ctrls: u32,
        auto_ctrls_values: u32,
    ) -> xcb_xkb_per_client_flags_cookie_t;

    pub fn xcb_xkb_per_client_flags_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        change: u32,
        value: u32,
        ctrls_to_change: u32,
        auto_ctrls: u32,
        auto_ctrls_values: u32,
    ) -> xcb_xkb_per_client_flags_cookie_t;

    pub fn xcb_xkb_list_components_keymaps_length(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> c_int;

    pub fn xcb_xkb_list_components_keymaps_iterator<'a>(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_keycodes_length(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> c_int;

    pub fn xcb_xkb_list_components_keycodes_iterator<'a>(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_types_length(R: *const xcb_xkb_list_components_reply_t)
        -> c_int;

    pub fn xcb_xkb_list_components_types_iterator<'a>(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_compat_maps_length(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> c_int;

    pub fn xcb_xkb_list_components_compat_maps_iterator<'a>(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_symbols_length(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> c_int;

    pub fn xcb_xkb_list_components_symbols_iterator<'a>(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_geometries_length(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> c_int;

    pub fn xcb_xkb_list_components_geometries_iterator<'a>(
        R: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t<'a>;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_list_components_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_list_components_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_list_components_reply_t;

    pub fn xcb_xkb_list_components(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        max_names: u16,
    ) -> xcb_xkb_list_components_cookie_t;

    pub fn xcb_xkb_list_components_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        max_names: u16,
    ) -> xcb_xkb_list_components_cookie_t;
    pub fn xcb_xkb_get_kbd_by_name_replies_types_map(
        R: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_get_kbd_by_name_replies_types_map_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_sym_interpret_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator<'a>(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_sym_interpret_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_mod_def_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_mod_def_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_indicator_map_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_indicator_map_iterator_t;
    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list(
        R: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t;
    pub fn xcb_xkb_get_kbd_by_name_replies_geometry_label_font(
        R: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_counted_string_16_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator<'a>(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_type_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator<'a>(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_sym_map_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut u8;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_action_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_action_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_set_behavior_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator<'a>(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_set_behavior_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut u8;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_set_explicit_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_set_explicit_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_key_mod_map_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_mod_map_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_key_v_mod_map_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_v_mod_map_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut u8;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_key_name_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator<'a>(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_name_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_key_alias_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator<'a>(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_alias_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names(
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
        S: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_xkb_get_kbd_by_name_replies(
        R: *const xcb_xkb_get_kbd_by_name_reply_t,
    ) -> *mut c_void;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_kbd_by_name_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_kbd_by_name_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_kbd_by_name_reply_t;

    pub fn xcb_xkb_get_kbd_by_name(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        need: u16,
        want: u16,
        load: u8,
    ) -> xcb_xkb_get_kbd_by_name_cookie_t;

    pub fn xcb_xkb_get_kbd_by_name_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        need: u16,
        want: u16,
        load: u8,
    ) -> xcb_xkb_get_kbd_by_name_cookie_t;

    pub fn xcb_xkb_get_device_info_name(
        R: *const xcb_xkb_get_device_info_reply_t,
    ) -> *mut xcb_xkb_string8_t;

    pub fn xcb_xkb_get_device_info_name_length(R: *const xcb_xkb_get_device_info_reply_t) -> c_int;

    pub fn xcb_xkb_get_device_info_name_end(
        R: *const xcb_xkb_get_device_info_reply_t,
    ) -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_device_info_btn_actions(
        R: *const xcb_xkb_get_device_info_reply_t,
    ) -> *mut xcb_xkb_action_t;

    pub fn xcb_xkb_get_device_info_btn_actions_length(
        R: *const xcb_xkb_get_device_info_reply_t,
    ) -> c_int;

    pub fn xcb_xkb_get_device_info_btn_actions_iterator(
        R: *const xcb_xkb_get_device_info_reply_t,
    ) -> xcb_xkb_action_iterator_t;

    pub fn xcb_xkb_get_device_info_leds_length(R: *const xcb_xkb_get_device_info_reply_t) -> c_int;

    pub fn xcb_xkb_get_device_info_leds_iterator<'a>(
        R: *const xcb_xkb_get_device_info_reply_t,
    ) -> xcb_xkb_device_led_info_iterator_t<'a>;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_device_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_device_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_device_info_reply_t;

    pub fn xcb_xkb_get_device_info(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        wanted: u16,
        all_buttons: u8,
        first_button: u8,
        n_buttons: u8,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
    ) -> xcb_xkb_get_device_info_cookie_t;

    pub fn xcb_xkb_get_device_info_unchecked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        wanted: u16,
        all_buttons: u8,
        first_button: u8,
        n_buttons: u8,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
    ) -> xcb_xkb_get_device_info_cookie_t;

    pub fn xcb_xkb_set_device_info(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        first_btn: u8,
        n_btns: u8,
        change: u16,
        n_device_led_f_bs: u16,
        btn_actions: *const xcb_xkb_action_t,
        leds: *const xcb_xkb_device_led_info_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_device_info_checked(
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        first_btn: u8,
        n_btns: u8,
        change: u16,
        n_device_led_f_bs: u16,
        btn_actions: *const xcb_xkb_action_t,
        leds: *const xcb_xkb_device_led_info_t,
    ) -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_set_debugging_flags_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_set_debugging_flags_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_set_debugging_flags_reply_t;

    pub fn xcb_xkb_set_debugging_flags(
        c: *mut xcb_connection_t,
        msg_length: u16,
        affect_flags: u32,
        flags: u32,
        affect_ctrls: u32,
        ctrls: u32,
        message: *const xcb_xkb_string8_t,
    ) -> xcb_xkb_set_debugging_flags_cookie_t;

    pub fn xcb_xkb_set_debugging_flags_unchecked(
        c: *mut xcb_connection_t,
        msg_length: u16,
        affect_flags: u32,
        flags: u32,
        affect_ctrls: u32,
        ctrls: u32,
        message: *const xcb_xkb_string8_t,
    ) -> xcb_xkb_set_debugging_flags_cookie_t;

} // extern
