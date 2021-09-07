// Generated automatically from xkb.xml by rs_client.py version 0.9.0.
// Do not edit!


use ffi::base::*;
use ffi::xproto::*;

use libc::{c_char, c_int, c_uint, c_void};
use std;


pub const XCB_XKB_MAJOR_VERSION: u32 = 1;
pub const XCB_XKB_MINOR_VERSION: u32 = 0;

pub type xcb_xkb_const_t = u32;
pub const XCB_XKB_CONST_MAX_LEGAL_KEY_CODE    : xcb_xkb_const_t = 0xff;
pub const XCB_XKB_CONST_PER_KEY_BIT_ARRAY_SIZE: xcb_xkb_const_t = 0x20;
pub const XCB_XKB_CONST_KEY_NAME_LENGTH       : xcb_xkb_const_t = 0x04;

pub type xcb_xkb_event_type_t = u32;
pub const XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY    : xcb_xkb_event_type_t =  0x01;
pub const XCB_XKB_EVENT_TYPE_MAP_NOTIFY             : xcb_xkb_event_type_t =  0x02;
pub const XCB_XKB_EVENT_TYPE_STATE_NOTIFY           : xcb_xkb_event_type_t =  0x04;
pub const XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY        : xcb_xkb_event_type_t =  0x08;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY : xcb_xkb_event_type_t =  0x10;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY   : xcb_xkb_event_type_t =  0x20;
pub const XCB_XKB_EVENT_TYPE_NAMES_NOTIFY           : xcb_xkb_event_type_t =  0x40;
pub const XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY      : xcb_xkb_event_type_t =  0x80;
pub const XCB_XKB_EVENT_TYPE_BELL_NOTIFY            : xcb_xkb_event_type_t = 0x100;
pub const XCB_XKB_EVENT_TYPE_ACTION_MESSAGE         : xcb_xkb_event_type_t = 0x200;
pub const XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY        : xcb_xkb_event_type_t = 0x400;
pub const XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY: xcb_xkb_event_type_t = 0x800;

pub type xcb_xkb_nkn_detail_t = u32;
pub const XCB_XKB_NKN_DETAIL_KEYCODES : xcb_xkb_nkn_detail_t = 0x01;
pub const XCB_XKB_NKN_DETAIL_GEOMETRY : xcb_xkb_nkn_detail_t = 0x02;
pub const XCB_XKB_NKN_DETAIL_DEVICE_ID: xcb_xkb_nkn_detail_t = 0x04;

pub type xcb_xkb_axn_detail_t = u32;
pub const XCB_XKB_AXN_DETAIL_SK_PRESS   : xcb_xkb_axn_detail_t = 0x01;
pub const XCB_XKB_AXN_DETAIL_SK_ACCEPT  : xcb_xkb_axn_detail_t = 0x02;
pub const XCB_XKB_AXN_DETAIL_SK_REJECT  : xcb_xkb_axn_detail_t = 0x04;
pub const XCB_XKB_AXN_DETAIL_SK_RELEASE : xcb_xkb_axn_detail_t = 0x08;
pub const XCB_XKB_AXN_DETAIL_BK_ACCEPT  : xcb_xkb_axn_detail_t = 0x10;
pub const XCB_XKB_AXN_DETAIL_BK_REJECT  : xcb_xkb_axn_detail_t = 0x20;
pub const XCB_XKB_AXN_DETAIL_AXK_WARNING: xcb_xkb_axn_detail_t = 0x40;

pub type xcb_xkb_map_part_t = u32;
pub const XCB_XKB_MAP_PART_KEY_TYPES          : xcb_xkb_map_part_t = 0x01;
pub const XCB_XKB_MAP_PART_KEY_SYMS           : xcb_xkb_map_part_t = 0x02;
pub const XCB_XKB_MAP_PART_MODIFIER_MAP       : xcb_xkb_map_part_t = 0x04;
pub const XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS: xcb_xkb_map_part_t = 0x08;
pub const XCB_XKB_MAP_PART_KEY_ACTIONS        : xcb_xkb_map_part_t = 0x10;
pub const XCB_XKB_MAP_PART_KEY_BEHAVIORS      : xcb_xkb_map_part_t = 0x20;
pub const XCB_XKB_MAP_PART_VIRTUAL_MODS       : xcb_xkb_map_part_t = 0x40;
pub const XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP    : xcb_xkb_map_part_t = 0x80;

pub type xcb_xkb_set_map_flags_t = u32;
pub const XCB_XKB_SET_MAP_FLAGS_RESIZE_TYPES     : xcb_xkb_set_map_flags_t = 0x01;
pub const XCB_XKB_SET_MAP_FLAGS_RECOMPUTE_ACTIONS: xcb_xkb_set_map_flags_t = 0x02;

pub type xcb_xkb_state_part_t = u32;
pub const XCB_XKB_STATE_PART_MODIFIER_STATE    : xcb_xkb_state_part_t =   0x01;
pub const XCB_XKB_STATE_PART_MODIFIER_BASE     : xcb_xkb_state_part_t =   0x02;
pub const XCB_XKB_STATE_PART_MODIFIER_LATCH    : xcb_xkb_state_part_t =   0x04;
pub const XCB_XKB_STATE_PART_MODIFIER_LOCK     : xcb_xkb_state_part_t =   0x08;
pub const XCB_XKB_STATE_PART_GROUP_STATE       : xcb_xkb_state_part_t =   0x10;
pub const XCB_XKB_STATE_PART_GROUP_BASE        : xcb_xkb_state_part_t =   0x20;
pub const XCB_XKB_STATE_PART_GROUP_LATCH       : xcb_xkb_state_part_t =   0x40;
pub const XCB_XKB_STATE_PART_GROUP_LOCK        : xcb_xkb_state_part_t =   0x80;
pub const XCB_XKB_STATE_PART_COMPAT_STATE      : xcb_xkb_state_part_t =  0x100;
pub const XCB_XKB_STATE_PART_GRAB_MODS         : xcb_xkb_state_part_t =  0x200;
pub const XCB_XKB_STATE_PART_COMPAT_GRAB_MODS  : xcb_xkb_state_part_t =  0x400;
pub const XCB_XKB_STATE_PART_LOOKUP_MODS       : xcb_xkb_state_part_t =  0x800;
pub const XCB_XKB_STATE_PART_COMPAT_LOOKUP_MODS: xcb_xkb_state_part_t = 0x1000;
pub const XCB_XKB_STATE_PART_POINTER_BUTTONS   : xcb_xkb_state_part_t = 0x2000;

pub type xcb_xkb_bool_ctrl_t = u32;
pub const XCB_XKB_BOOL_CTRL_REPEAT_KEYS           : xcb_xkb_bool_ctrl_t =   0x01;
pub const XCB_XKB_BOOL_CTRL_SLOW_KEYS             : xcb_xkb_bool_ctrl_t =   0x02;
pub const XCB_XKB_BOOL_CTRL_BOUNCE_KEYS           : xcb_xkb_bool_ctrl_t =   0x04;
pub const XCB_XKB_BOOL_CTRL_STICKY_KEYS           : xcb_xkb_bool_ctrl_t =   0x08;
pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS            : xcb_xkb_bool_ctrl_t =   0x10;
pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS_ACCEL      : xcb_xkb_bool_ctrl_t =   0x20;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_KEYS         : xcb_xkb_bool_ctrl_t =   0x40;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_TIMEOUT_MASK : xcb_xkb_bool_ctrl_t =   0x80;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_FEEDBACK_MASK: xcb_xkb_bool_ctrl_t =  0x100;
pub const XCB_XKB_BOOL_CTRL_AUDIBLE_BELL_MASK     : xcb_xkb_bool_ctrl_t =  0x200;
pub const XCB_XKB_BOOL_CTRL_OVERLAY_1_MASK        : xcb_xkb_bool_ctrl_t =  0x400;
pub const XCB_XKB_BOOL_CTRL_OVERLAY_2_MASK        : xcb_xkb_bool_ctrl_t =  0x800;
pub const XCB_XKB_BOOL_CTRL_IGNORE_GROUP_LOCK_MASK: xcb_xkb_bool_ctrl_t = 0x1000;

pub type xcb_xkb_control_t = u32;
pub const XCB_XKB_CONTROL_GROUPS_WRAP     : xcb_xkb_control_t =  0x8000000;
pub const XCB_XKB_CONTROL_INTERNAL_MODS   : xcb_xkb_control_t = 0x10000000;
pub const XCB_XKB_CONTROL_IGNORE_LOCK_MODS: xcb_xkb_control_t = 0x20000000;
pub const XCB_XKB_CONTROL_PER_KEY_REPEAT  : xcb_xkb_control_t = 0x40000000;
pub const XCB_XKB_CONTROL_CONTROLS_ENABLED: xcb_xkb_control_t = 0x80000000;

pub type xcb_xkb_ax_option_t = u32;
pub const XCB_XKB_AX_OPTION_SK_PRESS_FB   : xcb_xkb_ax_option_t =  0x01;
pub const XCB_XKB_AX_OPTION_SK_ACCEPT_FB  : xcb_xkb_ax_option_t =  0x02;
pub const XCB_XKB_AX_OPTION_FEATURE_FB    : xcb_xkb_ax_option_t =  0x04;
pub const XCB_XKB_AX_OPTION_SLOW_WARN_FB  : xcb_xkb_ax_option_t =  0x08;
pub const XCB_XKB_AX_OPTION_INDICATOR_FB  : xcb_xkb_ax_option_t =  0x10;
pub const XCB_XKB_AX_OPTION_STICKY_KEYS_FB: xcb_xkb_ax_option_t =  0x20;
pub const XCB_XKB_AX_OPTION_TWO_KEYS      : xcb_xkb_ax_option_t =  0x40;
pub const XCB_XKB_AX_OPTION_LATCH_TO_LOCK : xcb_xkb_ax_option_t =  0x80;
pub const XCB_XKB_AX_OPTION_SK_RELEASE_FB : xcb_xkb_ax_option_t = 0x100;
pub const XCB_XKB_AX_OPTION_SK_REJECT_FB  : xcb_xkb_ax_option_t = 0x200;
pub const XCB_XKB_AX_OPTION_BK_REJECT_FB  : xcb_xkb_ax_option_t = 0x400;
pub const XCB_XKB_AX_OPTION_DUMB_BELL     : xcb_xkb_ax_option_t = 0x800;

pub type xcb_xkb_device_spec_t = u16;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_device_spec_iterator_t {
    pub data:  *mut xcb_xkb_device_spec_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_led_class_result_t = u32;
pub const XCB_XKB_LED_CLASS_RESULT_KBD_FEEDBACK_CLASS: xcb_xkb_led_class_result_t = 0x00;
pub const XCB_XKB_LED_CLASS_RESULT_LED_FEEDBACK_CLASS: xcb_xkb_led_class_result_t = 0x04;

pub type xcb_xkb_led_class_t = u32;
pub const XCB_XKB_LED_CLASS_KBD_FEEDBACK_CLASS: xcb_xkb_led_class_t =  0x00;
pub const XCB_XKB_LED_CLASS_LED_FEEDBACK_CLASS: xcb_xkb_led_class_t =  0x04;
pub const XCB_XKB_LED_CLASS_DFLT_XI_CLASS     : xcb_xkb_led_class_t = 0x300;
pub const XCB_XKB_LED_CLASS_ALL_XI_CLASSES    : xcb_xkb_led_class_t = 0x500;

pub type xcb_xkb_led_class_spec_t = u16;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_led_class_spec_iterator_t {
    pub data:  *mut xcb_xkb_led_class_spec_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_bell_class_result_t = u32;
pub const XCB_XKB_BELL_CLASS_RESULT_KBD_FEEDBACK_CLASS : xcb_xkb_bell_class_result_t = 0x00;
pub const XCB_XKB_BELL_CLASS_RESULT_BELL_FEEDBACK_CLASS: xcb_xkb_bell_class_result_t = 0x05;

pub type xcb_xkb_bell_class_t = u32;
pub const XCB_XKB_BELL_CLASS_KBD_FEEDBACK_CLASS : xcb_xkb_bell_class_t =  0x00;
pub const XCB_XKB_BELL_CLASS_BELL_FEEDBACK_CLASS: xcb_xkb_bell_class_t =  0x05;
pub const XCB_XKB_BELL_CLASS_DFLT_XI_CLASS      : xcb_xkb_bell_class_t = 0x300;

pub type xcb_xkb_bell_class_spec_t = u16;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_bell_class_spec_iterator_t {
    pub data:  *mut xcb_xkb_bell_class_spec_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_id_t = u32;
pub const XCB_XKB_ID_USE_CORE_KBD : xcb_xkb_id_t =  0x100;
pub const XCB_XKB_ID_USE_CORE_PTR : xcb_xkb_id_t =  0x200;
pub const XCB_XKB_ID_DFLT_XI_CLASS: xcb_xkb_id_t =  0x300;
pub const XCB_XKB_ID_DFLT_XI_ID   : xcb_xkb_id_t =  0x400;
pub const XCB_XKB_ID_ALL_XI_CLASS : xcb_xkb_id_t =  0x500;
pub const XCB_XKB_ID_ALL_XI_ID    : xcb_xkb_id_t =  0x600;
pub const XCB_XKB_ID_XI_NONE      : xcb_xkb_id_t = 0xff00;

pub type xcb_xkb_id_spec_t = u16;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_id_spec_iterator_t {
    pub data:  *mut xcb_xkb_id_spec_t,
    pub rem:   c_int,
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
pub const XCB_XKB_GROUPS_WRAP_WRAP_INTO_RANGE    : xcb_xkb_groups_wrap_t = 0x00;
pub const XCB_XKB_GROUPS_WRAP_CLAMP_INTO_RANGE   : xcb_xkb_groups_wrap_t = 0x40;
pub const XCB_XKB_GROUPS_WRAP_REDIRECT_INTO_RANGE: xcb_xkb_groups_wrap_t = 0x80;

pub type xcb_xkb_v_mods_high_t = u32;
pub const XCB_XKB_V_MODS_HIGH_15: xcb_xkb_v_mods_high_t = 0x80;
pub const XCB_XKB_V_MODS_HIGH_14: xcb_xkb_v_mods_high_t = 0x40;
pub const XCB_XKB_V_MODS_HIGH_13: xcb_xkb_v_mods_high_t = 0x20;
pub const XCB_XKB_V_MODS_HIGH_12: xcb_xkb_v_mods_high_t = 0x10;
pub const XCB_XKB_V_MODS_HIGH_11: xcb_xkb_v_mods_high_t = 0x08;
pub const XCB_XKB_V_MODS_HIGH_10: xcb_xkb_v_mods_high_t = 0x04;
pub const XCB_XKB_V_MODS_HIGH_9 : xcb_xkb_v_mods_high_t = 0x02;
pub const XCB_XKB_V_MODS_HIGH_8 : xcb_xkb_v_mods_high_t = 0x01;

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
pub const XCB_XKB_V_MOD_11: xcb_xkb_v_mod_t =  0x800;
pub const XCB_XKB_V_MOD_10: xcb_xkb_v_mod_t =  0x400;
pub const XCB_XKB_V_MOD_9 : xcb_xkb_v_mod_t =  0x200;
pub const XCB_XKB_V_MOD_8 : xcb_xkb_v_mod_t =  0x100;
pub const XCB_XKB_V_MOD_7 : xcb_xkb_v_mod_t =   0x80;
pub const XCB_XKB_V_MOD_6 : xcb_xkb_v_mod_t =   0x40;
pub const XCB_XKB_V_MOD_5 : xcb_xkb_v_mod_t =   0x20;
pub const XCB_XKB_V_MOD_4 : xcb_xkb_v_mod_t =   0x10;
pub const XCB_XKB_V_MOD_3 : xcb_xkb_v_mod_t =   0x08;
pub const XCB_XKB_V_MOD_2 : xcb_xkb_v_mod_t =   0x04;
pub const XCB_XKB_V_MOD_1 : xcb_xkb_v_mod_t =   0x02;
pub const XCB_XKB_V_MOD_0 : xcb_xkb_v_mod_t =   0x01;

pub type xcb_xkb_explicit_t = u32;
pub const XCB_XKB_EXPLICIT_V_MOD_MAP  : xcb_xkb_explicit_t = 0x80;
pub const XCB_XKB_EXPLICIT_BEHAVIOR   : xcb_xkb_explicit_t = 0x40;
pub const XCB_XKB_EXPLICIT_AUTO_REPEAT: xcb_xkb_explicit_t = 0x20;
pub const XCB_XKB_EXPLICIT_INTERPRET  : xcb_xkb_explicit_t = 0x10;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_4 : xcb_xkb_explicit_t = 0x08;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_3 : xcb_xkb_explicit_t = 0x04;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_2 : xcb_xkb_explicit_t = 0x02;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_1 : xcb_xkb_explicit_t = 0x01;

pub type xcb_xkb_sym_interpret_match_t = u32;
pub const XCB_XKB_SYM_INTERPRET_MATCH_NONE_OF       : xcb_xkb_sym_interpret_match_t = 0x00;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF_OR_NONE: xcb_xkb_sym_interpret_match_t = 0x01;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF        : xcb_xkb_sym_interpret_match_t = 0x02;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ALL_OF        : xcb_xkb_sym_interpret_match_t = 0x03;
pub const XCB_XKB_SYM_INTERPRET_MATCH_EXACTLY       : xcb_xkb_sym_interpret_match_t = 0x04;

pub type xcb_xkb_sym_interp_match_t = u32;
pub const XCB_XKB_SYM_INTERP_MATCH_LEVEL_ONE_ONLY: xcb_xkb_sym_interp_match_t = 0x80;
pub const XCB_XKB_SYM_INTERP_MATCH_OP_MASK       : xcb_xkb_sym_interp_match_t = 0x7f;

pub type xcb_xkb_im_flag_t = u32;
pub const XCB_XKB_IM_FLAG_NO_EXPLICIT  : xcb_xkb_im_flag_t = 0x80;
pub const XCB_XKB_IM_FLAG_NO_AUTOMATIC : xcb_xkb_im_flag_t = 0x40;
pub const XCB_XKB_IM_FLAG_LED_DRIVES_KB: xcb_xkb_im_flag_t = 0x20;

pub type xcb_xkb_im_mods_which_t = u32;
pub const XCB_XKB_IM_MODS_WHICH_USE_COMPAT   : xcb_xkb_im_mods_which_t = 0x10;
pub const XCB_XKB_IM_MODS_WHICH_USE_EFFECTIVE: xcb_xkb_im_mods_which_t = 0x08;
pub const XCB_XKB_IM_MODS_WHICH_USE_LOCKED   : xcb_xkb_im_mods_which_t = 0x04;
pub const XCB_XKB_IM_MODS_WHICH_USE_LATCHED  : xcb_xkb_im_mods_which_t = 0x02;
pub const XCB_XKB_IM_MODS_WHICH_USE_BASE     : xcb_xkb_im_mods_which_t = 0x01;

pub type xcb_xkb_im_groups_which_t = u32;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_COMPAT   : xcb_xkb_im_groups_which_t = 0x10;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_EFFECTIVE: xcb_xkb_im_groups_which_t = 0x08;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_LOCKED   : xcb_xkb_im_groups_which_t = 0x04;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_LATCHED  : xcb_xkb_im_groups_which_t = 0x02;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_BASE     : xcb_xkb_im_groups_which_t = 0x01;

#[repr(C)]
pub struct xcb_xkb_indicator_map_t {
    pub flags:       u8,
    pub whichGroups: u8,
    pub groups:      u8,
    pub whichMods:   u8,
    pub mods:        u8,
    pub realMods:    u8,
    pub vmods:       u16,
    pub ctrls:       u32,
}

impl Copy for xcb_xkb_indicator_map_t {}
impl Clone for xcb_xkb_indicator_map_t {
    fn clone(&self) -> xcb_xkb_indicator_map_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_indicator_map_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_indicator_map_t")
            .field("flags", &self.flags)
            .field("whichGroups", &self.whichGroups)
            .field("groups", &self.groups)
            .field("whichMods", &self.whichMods)
            .field("mods", &self.mods)
            .field("realMods", &self.realMods)
            .field("vmods", &self.vmods)
            .field("ctrls", &self.ctrls)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_indicator_map_iterator_t {
    pub data:  *mut xcb_xkb_indicator_map_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_cm_detail_t = u32;
pub const XCB_XKB_CM_DETAIL_SYM_INTERP  : xcb_xkb_cm_detail_t = 0x01;
pub const XCB_XKB_CM_DETAIL_GROUP_COMPAT: xcb_xkb_cm_detail_t = 0x02;

pub type xcb_xkb_name_detail_t = u32;
pub const XCB_XKB_NAME_DETAIL_KEYCODES         : xcb_xkb_name_detail_t =   0x01;
pub const XCB_XKB_NAME_DETAIL_GEOMETRY         : xcb_xkb_name_detail_t =   0x02;
pub const XCB_XKB_NAME_DETAIL_SYMBOLS          : xcb_xkb_name_detail_t =   0x04;
pub const XCB_XKB_NAME_DETAIL_PHYS_SYMBOLS     : xcb_xkb_name_detail_t =   0x08;
pub const XCB_XKB_NAME_DETAIL_TYPES            : xcb_xkb_name_detail_t =   0x10;
pub const XCB_XKB_NAME_DETAIL_COMPAT           : xcb_xkb_name_detail_t =   0x20;
pub const XCB_XKB_NAME_DETAIL_KEY_TYPE_NAMES   : xcb_xkb_name_detail_t =   0x40;
pub const XCB_XKB_NAME_DETAIL_KT_LEVEL_NAMES   : xcb_xkb_name_detail_t =   0x80;
pub const XCB_XKB_NAME_DETAIL_INDICATOR_NAMES  : xcb_xkb_name_detail_t =  0x100;
pub const XCB_XKB_NAME_DETAIL_KEY_NAMES        : xcb_xkb_name_detail_t =  0x200;
pub const XCB_XKB_NAME_DETAIL_KEY_ALIASES      : xcb_xkb_name_detail_t =  0x400;
pub const XCB_XKB_NAME_DETAIL_VIRTUAL_MOD_NAMES: xcb_xkb_name_detail_t =  0x800;
pub const XCB_XKB_NAME_DETAIL_GROUP_NAMES      : xcb_xkb_name_detail_t = 0x1000;
pub const XCB_XKB_NAME_DETAIL_RG_NAMES         : xcb_xkb_name_detail_t = 0x2000;

pub type xcb_xkb_gbn_detail_t = u32;
pub const XCB_XKB_GBN_DETAIL_TYPES         : xcb_xkb_gbn_detail_t = 0x01;
pub const XCB_XKB_GBN_DETAIL_COMPAT_MAP    : xcb_xkb_gbn_detail_t = 0x02;
pub const XCB_XKB_GBN_DETAIL_CLIENT_SYMBOLS: xcb_xkb_gbn_detail_t = 0x04;
pub const XCB_XKB_GBN_DETAIL_SERVER_SYMBOLS: xcb_xkb_gbn_detail_t = 0x08;
pub const XCB_XKB_GBN_DETAIL_INDICATOR_MAPS: xcb_xkb_gbn_detail_t = 0x10;
pub const XCB_XKB_GBN_DETAIL_KEY_NAMES     : xcb_xkb_gbn_detail_t = 0x20;
pub const XCB_XKB_GBN_DETAIL_GEOMETRY      : xcb_xkb_gbn_detail_t = 0x40;
pub const XCB_XKB_GBN_DETAIL_OTHER_NAMES   : xcb_xkb_gbn_detail_t = 0x80;

pub type xcb_xkb_xi_feature_t = u32;
pub const XCB_XKB_XI_FEATURE_KEYBOARDS      : xcb_xkb_xi_feature_t = 0x01;
pub const XCB_XKB_XI_FEATURE_BUTTON_ACTIONS : xcb_xkb_xi_feature_t = 0x02;
pub const XCB_XKB_XI_FEATURE_INDICATOR_NAMES: xcb_xkb_xi_feature_t = 0x04;
pub const XCB_XKB_XI_FEATURE_INDICATOR_MAPS : xcb_xkb_xi_feature_t = 0x08;
pub const XCB_XKB_XI_FEATURE_INDICATOR_STATE: xcb_xkb_xi_feature_t = 0x10;

pub type xcb_xkb_per_client_flag_t = u32;
pub const XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT   : xcb_xkb_per_client_flag_t = 0x01;
pub const XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE      : xcb_xkb_per_client_flag_t = 0x02;
pub const XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS      : xcb_xkb_per_client_flag_t = 0x04;
pub const XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED: xcb_xkb_per_client_flag_t = 0x08;
pub const XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE: xcb_xkb_per_client_flag_t = 0x10;

#[repr(C)]
pub struct xcb_xkb_mod_def_t {
    pub mask:     u8,
    pub realMods: u8,
    pub vmods:    u16,
}

impl Copy for xcb_xkb_mod_def_t {}
impl Clone for xcb_xkb_mod_def_t {
    fn clone(&self) -> xcb_xkb_mod_def_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_mod_def_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_mod_def_t")
            .field("mask", &self.mask)
            .field("realMods", &self.realMods)
            .field("vmods", &self.vmods)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_mod_def_iterator_t {
    pub data:  *mut xcb_xkb_mod_def_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_key_name_t {
    pub name: [c_char; 4],
}

impl Copy for xcb_xkb_key_name_t {}
impl Clone for xcb_xkb_key_name_t {
    fn clone(&self) -> xcb_xkb_key_name_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_key_name_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_key_name_t")
            .field("name", &&self.name[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_name_iterator_t<'a> {
    pub data:  *mut xcb_xkb_key_name_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_key_name_t>,
}

#[repr(C)]
pub struct xcb_xkb_key_alias_t {
    pub real:  [c_char; 4],
    pub alias: [c_char; 4],
}

impl Copy for xcb_xkb_key_alias_t {}
impl Clone for xcb_xkb_key_alias_t {
    fn clone(&self) -> xcb_xkb_key_alias_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_key_alias_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_key_alias_t")
            .field("real", &&self.real[..])
            .field("alias", &&self.alias[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_alias_iterator_t<'a> {
    pub data:  *mut xcb_xkb_key_alias_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_key_alias_t>,
}

#[repr(C)]
pub struct xcb_xkb_counted_string_16_t {
    pub length:        u16,
}
impl ::std::fmt::Debug for xcb_xkb_counted_string_16_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_counted_string_16_t")
            .field("length", &self.length)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_counted_string_16_iterator_t<'a> {
    pub data:  *mut xcb_xkb_counted_string_16_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_counted_string_16_t>,
}

#[repr(C)]
pub struct xcb_xkb_kt_map_entry_t {
    pub active:     u8,
    pub mods_mask:  u8,
    pub level:      u8,
    pub mods_mods:  u8,
    pub mods_vmods: u16,
    pub pad0:       [u8; 2],
}

impl Copy for xcb_xkb_kt_map_entry_t {}
impl Clone for xcb_xkb_kt_map_entry_t {
    fn clone(&self) -> xcb_xkb_kt_map_entry_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_kt_map_entry_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_kt_map_entry_t")
            .field("active", &self.active)
            .field("mods_mask", &self.mods_mask)
            .field("level", &self.level)
            .field("mods_mods", &self.mods_mods)
            .field("mods_vmods", &self.mods_vmods)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_kt_map_entry_iterator_t {
    pub data:  *mut xcb_xkb_kt_map_entry_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_key_type_t {
    pub mods_mask:   u8,
    pub mods_mods:   u8,
    pub mods_vmods:  u16,
    pub numLevels:   u8,
    pub nMapEntries: u8,
    pub hasPreserve: u8,
    pub pad0:        u8,
}
impl ::std::fmt::Debug for xcb_xkb_key_type_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_key_type_t")
            .field("mods_mask", &self.mods_mask)
            .field("mods_mods", &self.mods_mods)
            .field("mods_vmods", &self.mods_vmods)
            .field("numLevels", &self.numLevels)
            .field("nMapEntries", &self.nMapEntries)
            .field("hasPreserve", &self.hasPreserve)
            .field("pad0", &self.pad0)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_type_iterator_t<'a> {
    pub data:  *mut xcb_xkb_key_type_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_key_type_t>,
}

#[repr(C)]
pub struct xcb_xkb_key_sym_map_t {
    pub kt_index:  [u8; 4],
    pub groupInfo: u8,
    pub width:     u8,
    pub nSyms:     u16,
}
impl ::std::fmt::Debug for xcb_xkb_key_sym_map_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_key_sym_map_t")
            .field("kt_index", &&self.kt_index[..])
            .field("groupInfo", &self.groupInfo)
            .field("width", &self.width)
            .field("nSyms", &self.nSyms)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_sym_map_iterator_t<'a> {
    pub data:  *mut xcb_xkb_key_sym_map_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_key_sym_map_t>,
}

#[repr(C)]
pub struct xcb_xkb_common_behavior_t {
    pub type_: u8,
    pub data:  u8,
}

impl Copy for xcb_xkb_common_behavior_t {}
impl Clone for xcb_xkb_common_behavior_t {
    fn clone(&self) -> xcb_xkb_common_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_common_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_common_behavior_t")
            .field("type_", &self.type_)
            .field("data", &self.data)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_common_behavior_iterator_t {
    pub data:  *mut xcb_xkb_common_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_default_behavior_t {
    pub type_: u8,
    pub pad0:  u8,
}

impl Copy for xcb_xkb_default_behavior_t {}
impl Clone for xcb_xkb_default_behavior_t {
    fn clone(&self) -> xcb_xkb_default_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_default_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_default_behavior_t")
            .field("type_", &self.type_)
            .field("pad0", &self.pad0)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_default_behavior_iterator_t {
    pub data:  *mut xcb_xkb_default_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_lock_behavior_t {
    pub type_: u8,
    pub pad0:  u8,
}

impl Copy for xcb_xkb_lock_behavior_t {}
impl Clone for xcb_xkb_lock_behavior_t {
    fn clone(&self) -> xcb_xkb_lock_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_lock_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_lock_behavior_t")
            .field("type_", &self.type_)
            .field("pad0", &self.pad0)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_lock_behavior_iterator_t {
    pub data:  *mut xcb_xkb_lock_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_radio_group_behavior_t {
    pub type_: u8,
    pub group: u8,
}

impl Copy for xcb_xkb_radio_group_behavior_t {}
impl Clone for xcb_xkb_radio_group_behavior_t {
    fn clone(&self) -> xcb_xkb_radio_group_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_radio_group_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_radio_group_behavior_t")
            .field("type_", &self.type_)
            .field("group", &self.group)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_radio_group_behavior_iterator_t {
    pub data:  *mut xcb_xkb_radio_group_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_overlay_behavior_t {
    pub type_: u8,
    pub key:   xcb_keycode_t,
}

impl Copy for xcb_xkb_overlay_behavior_t {}
impl Clone for xcb_xkb_overlay_behavior_t {
    fn clone(&self) -> xcb_xkb_overlay_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_overlay_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_overlay_behavior_t")
            .field("type_", &self.type_)
            .field("key", &self.key)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_overlay_behavior_iterator_t {
    pub data:  *mut xcb_xkb_overlay_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_permament_lock_behavior_t {
    pub type_: u8,
    pub pad0:  u8,
}

impl Copy for xcb_xkb_permament_lock_behavior_t {}
impl Clone for xcb_xkb_permament_lock_behavior_t {
    fn clone(&self) -> xcb_xkb_permament_lock_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_permament_lock_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_permament_lock_behavior_t")
            .field("type_", &self.type_)
            .field("pad0", &self.pad0)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_permament_lock_behavior_iterator_t {
    pub data:  *mut xcb_xkb_permament_lock_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_permament_radio_group_behavior_t {
    pub type_: u8,
    pub group: u8,
}

impl Copy for xcb_xkb_permament_radio_group_behavior_t {}
impl Clone for xcb_xkb_permament_radio_group_behavior_t {
    fn clone(&self) -> xcb_xkb_permament_radio_group_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_permament_radio_group_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_permament_radio_group_behavior_t")
            .field("type_", &self.type_)
            .field("group", &self.group)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_permament_radio_group_behavior_iterator_t {
    pub data:  *mut xcb_xkb_permament_radio_group_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_permament_overlay_behavior_t {
    pub type_: u8,
    pub key:   xcb_keycode_t,
}

impl Copy for xcb_xkb_permament_overlay_behavior_t {}
impl Clone for xcb_xkb_permament_overlay_behavior_t {
    fn clone(&self) -> xcb_xkb_permament_overlay_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_permament_overlay_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_permament_overlay_behavior_t")
            .field("type_", &self.type_)
            .field("key", &self.key)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_permament_overlay_behavior_iterator_t {
    pub data:  *mut xcb_xkb_permament_overlay_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

// union
#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_behavior_t {
    pub data: [u8; 2]
}

impl Copy for xcb_xkb_behavior_t {}
impl Clone for xcb_xkb_behavior_t {
    fn clone(&self) -> xcb_xkb_behavior_t { *self }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_behavior_iterator_t {
    pub data:  *mut xcb_xkb_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_behavior_type_t = u32;
pub const XCB_XKB_BEHAVIOR_TYPE_DEFAULT              : xcb_xkb_behavior_type_t = 0x00;
pub const XCB_XKB_BEHAVIOR_TYPE_LOCK                 : xcb_xkb_behavior_type_t = 0x01;
pub const XCB_XKB_BEHAVIOR_TYPE_RADIO_GROUP          : xcb_xkb_behavior_type_t = 0x02;
pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_1            : xcb_xkb_behavior_type_t = 0x03;
pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_2            : xcb_xkb_behavior_type_t = 0x04;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_LOCK       : xcb_xkb_behavior_type_t = 0x81;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_RADIO_GROUP: xcb_xkb_behavior_type_t = 0x82;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_1  : xcb_xkb_behavior_type_t = 0x83;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_2  : xcb_xkb_behavior_type_t = 0x84;

#[repr(C)]
pub struct xcb_xkb_set_behavior_t {
    pub keycode:  xcb_keycode_t,
    pub behavior: xcb_xkb_behavior_t,
    pub pad0:     u8,
}

impl Copy for xcb_xkb_set_behavior_t {}
impl Clone for xcb_xkb_set_behavior_t {
    fn clone(&self) -> xcb_xkb_set_behavior_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_set_behavior_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_behavior_t")
            .field("keycode", &self.keycode)
            .field("behavior", &self.behavior)
            .field("pad0", &self.pad0)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_set_behavior_iterator_t<'a> {
    pub data:  *mut xcb_xkb_set_behavior_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_set_behavior_t>,
}

#[repr(C)]
pub struct xcb_xkb_set_explicit_t {
    pub keycode:  xcb_keycode_t,
    pub explicit: u8,
}

impl Copy for xcb_xkb_set_explicit_t {}
impl Clone for xcb_xkb_set_explicit_t {
    fn clone(&self) -> xcb_xkb_set_explicit_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_set_explicit_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_explicit_t")
            .field("keycode", &self.keycode)
            .field("explicit", &self.explicit)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_set_explicit_iterator_t {
    pub data:  *mut xcb_xkb_set_explicit_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_key_mod_map_t {
    pub keycode: xcb_keycode_t,
    pub mods:    u8,
}

impl Copy for xcb_xkb_key_mod_map_t {}
impl Clone for xcb_xkb_key_mod_map_t {
    fn clone(&self) -> xcb_xkb_key_mod_map_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_key_mod_map_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_key_mod_map_t")
            .field("keycode", &self.keycode)
            .field("mods", &self.mods)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_mod_map_iterator_t {
    pub data:  *mut xcb_xkb_key_mod_map_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_key_v_mod_map_t {
    pub keycode: xcb_keycode_t,
    pub pad0:    u8,
    pub vmods:   u16,
}

impl Copy for xcb_xkb_key_v_mod_map_t {}
impl Clone for xcb_xkb_key_v_mod_map_t {
    fn clone(&self) -> xcb_xkb_key_v_mod_map_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_key_v_mod_map_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_key_v_mod_map_t")
            .field("keycode", &self.keycode)
            .field("pad0", &self.pad0)
            .field("vmods", &self.vmods)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_v_mod_map_iterator_t {
    pub data:  *mut xcb_xkb_key_v_mod_map_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_kt_set_map_entry_t {
    pub level:       u8,
    pub realMods:    u8,
    pub virtualMods: u16,
}

impl Copy for xcb_xkb_kt_set_map_entry_t {}
impl Clone for xcb_xkb_kt_set_map_entry_t {
    fn clone(&self) -> xcb_xkb_kt_set_map_entry_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_kt_set_map_entry_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_kt_set_map_entry_t")
            .field("level", &self.level)
            .field("realMods", &self.realMods)
            .field("virtualMods", &self.virtualMods)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_kt_set_map_entry_iterator_t {
    pub data:  *mut xcb_xkb_kt_set_map_entry_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_set_key_type_t {
    pub mask:             u8,
    pub realMods:         u8,
    pub virtualMods:      u16,
    pub numLevels:        u8,
    pub nMapEntries:      u8,
    pub preserve:         u8,
    pub pad0:             u8,
}
impl ::std::fmt::Debug for xcb_xkb_set_key_type_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_key_type_t")
            .field("mask", &self.mask)
            .field("realMods", &self.realMods)
            .field("virtualMods", &self.virtualMods)
            .field("numLevels", &self.numLevels)
            .field("nMapEntries", &self.nMapEntries)
            .field("preserve", &self.preserve)
            .field("pad0", &self.pad0)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_set_key_type_iterator_t<'a> {
    pub data:  *mut xcb_xkb_set_key_type_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_set_key_type_t>,
}

pub type xcb_xkb_string8_t = c_char;

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_string8_iterator_t {
    pub data:  *mut xcb_xkb_string8_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_outline_t {
    pub nPoints:      u8,
    pub cornerRadius: u8,
    pub pad0:         [u8; 2],
}
impl ::std::fmt::Debug for xcb_xkb_outline_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_outline_t")
            .field("nPoints", &self.nPoints)
            .field("cornerRadius", &self.cornerRadius)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_outline_iterator_t<'a> {
    pub data:  *mut xcb_xkb_outline_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_outline_t>,
}

#[repr(C)]
pub struct xcb_xkb_shape_t {
    pub name:       xcb_atom_t,
    pub nOutlines:  u8,
    pub primaryNdx: u8,
    pub approxNdx:  u8,
    pub pad0:       u8,
}
impl ::std::fmt::Debug for xcb_xkb_shape_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_shape_t")
            .field("name", &self.name)
            .field("nOutlines", &self.nOutlines)
            .field("primaryNdx", &self.primaryNdx)
            .field("approxNdx", &self.approxNdx)
            .field("pad0", &self.pad0)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_shape_iterator_t<'a> {
    pub data:  *mut xcb_xkb_shape_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_shape_t>,
}

#[repr(C)]
pub struct xcb_xkb_key_t {
    pub name:     [xcb_xkb_string8_t; 4],
    pub gap:      i16,
    pub shapeNdx: u8,
    pub colorNdx: u8,
}

impl Copy for xcb_xkb_key_t {}
impl Clone for xcb_xkb_key_t {
    fn clone(&self) -> xcb_xkb_key_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_key_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_key_t")
            .field("name", &&self.name[..])
            .field("gap", &self.gap)
            .field("shapeNdx", &self.shapeNdx)
            .field("colorNdx", &self.colorNdx)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_key_iterator_t<'a> {
    pub data:  *mut xcb_xkb_key_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_key_t>,
}

#[repr(C)]
pub struct xcb_xkb_overlay_key_t {
    pub over:  [xcb_xkb_string8_t; 4],
    pub under: [xcb_xkb_string8_t; 4],
}

impl Copy for xcb_xkb_overlay_key_t {}
impl Clone for xcb_xkb_overlay_key_t {
    fn clone(&self) -> xcb_xkb_overlay_key_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_overlay_key_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_overlay_key_t")
            .field("over", &&self.over[..])
            .field("under", &&self.under[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_overlay_key_iterator_t<'a> {
    pub data:  *mut xcb_xkb_overlay_key_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_overlay_key_t>,
}

#[repr(C)]
pub struct xcb_xkb_overlay_row_t {
    pub rowUnder: u8,
    pub nKeys:    u8,
    pub pad0:     [u8; 2],
}
impl ::std::fmt::Debug for xcb_xkb_overlay_row_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_overlay_row_t")
            .field("rowUnder", &self.rowUnder)
            .field("nKeys", &self.nKeys)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_overlay_row_iterator_t<'a> {
    pub data:  *mut xcb_xkb_overlay_row_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_overlay_row_t>,
}

#[repr(C)]
pub struct xcb_xkb_overlay_t {
    pub name:  xcb_atom_t,
    pub nRows: u8,
    pub pad0:  [u8; 3],
}
impl ::std::fmt::Debug for xcb_xkb_overlay_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_overlay_t")
            .field("name", &self.name)
            .field("nRows", &self.nRows)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_overlay_iterator_t<'a> {
    pub data:  *mut xcb_xkb_overlay_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_overlay_t>,
}

#[repr(C)]
pub struct xcb_xkb_row_t {
    pub top:      i16,
    pub left:     i16,
    pub nKeys:    u8,
    pub vertical: u8,
    pub pad0:     [u8; 2],
}
impl ::std::fmt::Debug for xcb_xkb_row_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_row_t")
            .field("top", &self.top)
            .field("left", &self.left)
            .field("nKeys", &self.nKeys)
            .field("vertical", &self.vertical)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_row_iterator_t<'a> {
    pub data:  *mut xcb_xkb_row_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_row_t>,
}

pub type xcb_xkb_doodad_type_t = u32;
pub const XCB_XKB_DOODAD_TYPE_OUTLINE  : xcb_xkb_doodad_type_t = 0x01;
pub const XCB_XKB_DOODAD_TYPE_SOLID    : xcb_xkb_doodad_type_t = 0x02;
pub const XCB_XKB_DOODAD_TYPE_TEXT     : xcb_xkb_doodad_type_t = 0x03;
pub const XCB_XKB_DOODAD_TYPE_INDICATOR: xcb_xkb_doodad_type_t = 0x04;
pub const XCB_XKB_DOODAD_TYPE_LOGO     : xcb_xkb_doodad_type_t = 0x05;

#[repr(C)]
pub struct xcb_xkb_listing_t {
    pub flags:  u16,
    pub length: u16,
}
impl ::std::fmt::Debug for xcb_xkb_listing_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_listing_t")
            .field("flags", &self.flags)
            .field("length", &self.length)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_listing_iterator_t<'a> {
    pub data:  *mut xcb_xkb_listing_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_listing_t>,
}

#[repr(C)]
pub struct xcb_xkb_device_led_info_t {
    pub ledClass:       xcb_xkb_led_class_spec_t,
    pub ledID:          xcb_xkb_id_spec_t,
    pub namesPresent:   u32,
    pub mapsPresent:    u32,
    pub physIndicators: u32,
    pub state:          u32,
}
impl ::std::fmt::Debug for xcb_xkb_device_led_info_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_device_led_info_t")
            .field("ledClass", &self.ledClass)
            .field("ledID", &self.ledID)
            .field("namesPresent", &self.namesPresent)
            .field("mapsPresent", &self.mapsPresent)
            .field("physIndicators", &self.physIndicators)
            .field("state", &self.state)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_device_led_info_iterator_t<'a> {
    pub data:  *mut xcb_xkb_device_led_info_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_device_led_info_t>,
}

pub type xcb_xkb_error_t = u32;
pub const XCB_XKB_ERROR_BAD_DEVICE: xcb_xkb_error_t = 0xff;
pub const XCB_XKB_ERROR_BAD_CLASS : xcb_xkb_error_t = 0xfe;
pub const XCB_XKB_ERROR_BAD_ID    : xcb_xkb_error_t = 0xfd;

pub const XCB_XKB_KEYBOARD: u8 = 0;

#[repr(C)]
pub struct xcb_xkb_keyboard_error_t {
    pub response_type: u8,
    pub error_code:    u8,
    pub sequence:      u16,
    pub value:         u32,
    pub minorOpcode:   u16,
    pub majorOpcode:   u8,
    pub pad0:          [u8; 21],
}

impl Copy for xcb_xkb_keyboard_error_t {}
impl Clone for xcb_xkb_keyboard_error_t {
    fn clone(&self) -> xcb_xkb_keyboard_error_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_keyboard_error_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_keyboard_error_t")
            .field("response_type", &self.response_type)
            .field("error_code", &self.error_code)
            .field("sequence", &self.sequence)
            .field("value", &self.value)
            .field("minorOpcode", &self.minorOpcode)
            .field("majorOpcode", &self.majorOpcode)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub type xcb_xkb_sa_t = u32;
pub const XCB_XKB_SA_CLEAR_LOCKS     : xcb_xkb_sa_t = 0x01;
pub const XCB_XKB_SA_LATCH_TO_LOCK   : xcb_xkb_sa_t = 0x02;
pub const XCB_XKB_SA_USE_MOD_MAP_MODS: xcb_xkb_sa_t = 0x04;
pub const XCB_XKB_SA_GROUP_ABSOLUTE  : xcb_xkb_sa_t = 0x04;

pub type xcb_xkb_sa_type_t = u32;
pub const XCB_XKB_SA_TYPE_NO_ACTION      : xcb_xkb_sa_type_t = 0x00;
pub const XCB_XKB_SA_TYPE_SET_MODS       : xcb_xkb_sa_type_t = 0x01;
pub const XCB_XKB_SA_TYPE_LATCH_MODS     : xcb_xkb_sa_type_t = 0x02;
pub const XCB_XKB_SA_TYPE_LOCK_MODS      : xcb_xkb_sa_type_t = 0x03;
pub const XCB_XKB_SA_TYPE_SET_GROUP      : xcb_xkb_sa_type_t = 0x04;
pub const XCB_XKB_SA_TYPE_LATCH_GROUP    : xcb_xkb_sa_type_t = 0x05;
pub const XCB_XKB_SA_TYPE_LOCK_GROUP     : xcb_xkb_sa_type_t = 0x06;
pub const XCB_XKB_SA_TYPE_MOVE_PTR       : xcb_xkb_sa_type_t = 0x07;
pub const XCB_XKB_SA_TYPE_PTR_BTN        : xcb_xkb_sa_type_t = 0x08;
pub const XCB_XKB_SA_TYPE_LOCK_PTR_BTN   : xcb_xkb_sa_type_t = 0x09;
pub const XCB_XKB_SA_TYPE_SET_PTR_DFLT   : xcb_xkb_sa_type_t = 0x0a;
pub const XCB_XKB_SA_TYPE_ISO_LOCK       : xcb_xkb_sa_type_t = 0x0b;
pub const XCB_XKB_SA_TYPE_TERMINATE      : xcb_xkb_sa_type_t = 0x0c;
pub const XCB_XKB_SA_TYPE_SWITCH_SCREEN  : xcb_xkb_sa_type_t = 0x0d;
pub const XCB_XKB_SA_TYPE_SET_CONTROLS   : xcb_xkb_sa_type_t = 0x0e;
pub const XCB_XKB_SA_TYPE_LOCK_CONTROLS  : xcb_xkb_sa_type_t = 0x0f;
pub const XCB_XKB_SA_TYPE_ACTION_MESSAGE : xcb_xkb_sa_type_t = 0x10;
pub const XCB_XKB_SA_TYPE_REDIRECT_KEY   : xcb_xkb_sa_type_t = 0x11;
pub const XCB_XKB_SA_TYPE_DEVICE_BTN     : xcb_xkb_sa_type_t = 0x12;
pub const XCB_XKB_SA_TYPE_LOCK_DEVICE_BTN: xcb_xkb_sa_type_t = 0x13;
pub const XCB_XKB_SA_TYPE_DEVICE_VALUATOR: xcb_xkb_sa_type_t = 0x14;

#[repr(C)]
pub struct xcb_xkb_sa_no_action_t {
    pub type_: u8,
    pub pad0:  [u8; 7],
}

impl Copy for xcb_xkb_sa_no_action_t {}
impl Clone for xcb_xkb_sa_no_action_t {
    fn clone(&self) -> xcb_xkb_sa_no_action_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_no_action_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_no_action_t")
            .field("type_", &self.type_)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_no_action_iterator_t {
    pub data:  *mut xcb_xkb_sa_no_action_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_set_mods_t {
    pub type_:     u8,
    pub flags:     u8,
    pub mask:      u8,
    pub realMods:  u8,
    pub vmodsHigh: u8,
    pub vmodsLow:  u8,
    pub pad0:      [u8; 2],
}

impl Copy for xcb_xkb_sa_set_mods_t {}
impl Clone for xcb_xkb_sa_set_mods_t {
    fn clone(&self) -> xcb_xkb_sa_set_mods_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_set_mods_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_set_mods_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("mask", &self.mask)
            .field("realMods", &self.realMods)
            .field("vmodsHigh", &self.vmodsHigh)
            .field("vmodsLow", &self.vmodsLow)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_set_mods_iterator_t {
    pub data:  *mut xcb_xkb_sa_set_mods_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_latch_mods_t {
    pub type_:     u8,
    pub flags:     u8,
    pub mask:      u8,
    pub realMods:  u8,
    pub vmodsHigh: u8,
    pub vmodsLow:  u8,
    pub pad0:      [u8; 2],
}

impl Copy for xcb_xkb_sa_latch_mods_t {}
impl Clone for xcb_xkb_sa_latch_mods_t {
    fn clone(&self) -> xcb_xkb_sa_latch_mods_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_latch_mods_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_latch_mods_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("mask", &self.mask)
            .field("realMods", &self.realMods)
            .field("vmodsHigh", &self.vmodsHigh)
            .field("vmodsLow", &self.vmodsLow)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_latch_mods_iterator_t {
    pub data:  *mut xcb_xkb_sa_latch_mods_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_lock_mods_t {
    pub type_:     u8,
    pub flags:     u8,
    pub mask:      u8,
    pub realMods:  u8,
    pub vmodsHigh: u8,
    pub vmodsLow:  u8,
    pub pad0:      [u8; 2],
}

impl Copy for xcb_xkb_sa_lock_mods_t {}
impl Clone for xcb_xkb_sa_lock_mods_t {
    fn clone(&self) -> xcb_xkb_sa_lock_mods_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_lock_mods_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_lock_mods_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("mask", &self.mask)
            .field("realMods", &self.realMods)
            .field("vmodsHigh", &self.vmodsHigh)
            .field("vmodsLow", &self.vmodsLow)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_mods_iterator_t {
    pub data:  *mut xcb_xkb_sa_lock_mods_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_set_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0:  [u8; 5],
}

impl Copy for xcb_xkb_sa_set_group_t {}
impl Clone for xcb_xkb_sa_set_group_t {
    fn clone(&self) -> xcb_xkb_sa_set_group_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_set_group_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_set_group_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("group", &self.group)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_set_group_iterator_t {
    pub data:  *mut xcb_xkb_sa_set_group_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_latch_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0:  [u8; 5],
}

impl Copy for xcb_xkb_sa_latch_group_t {}
impl Clone for xcb_xkb_sa_latch_group_t {
    fn clone(&self) -> xcb_xkb_sa_latch_group_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_latch_group_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_latch_group_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("group", &self.group)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_latch_group_iterator_t {
    pub data:  *mut xcb_xkb_sa_latch_group_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_lock_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0:  [u8; 5],
}

impl Copy for xcb_xkb_sa_lock_group_t {}
impl Clone for xcb_xkb_sa_lock_group_t {
    fn clone(&self) -> xcb_xkb_sa_lock_group_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_lock_group_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_lock_group_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("group", &self.group)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_group_iterator_t {
    pub data:  *mut xcb_xkb_sa_lock_group_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_sa_move_ptr_flag_t = u32;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_NO_ACCELERATION: xcb_xkb_sa_move_ptr_flag_t = 0x01;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_X: xcb_xkb_sa_move_ptr_flag_t = 0x02;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_Y: xcb_xkb_sa_move_ptr_flag_t = 0x04;

#[repr(C)]
pub struct xcb_xkb_sa_move_ptr_t {
    pub type_: u8,
    pub flags: u8,
    pub xHigh: i8,
    pub xLow:  u8,
    pub yHigh: i8,
    pub yLow:  u8,
    pub pad0:  [u8; 2],
}

impl Copy for xcb_xkb_sa_move_ptr_t {}
impl Clone for xcb_xkb_sa_move_ptr_t {
    fn clone(&self) -> xcb_xkb_sa_move_ptr_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_move_ptr_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_move_ptr_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("xHigh", &self.xHigh)
            .field("xLow", &self.xLow)
            .field("yHigh", &self.yHigh)
            .field("yLow", &self.yLow)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_move_ptr_iterator_t {
    pub data:  *mut xcb_xkb_sa_move_ptr_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_ptr_btn_t {
    pub type_:  u8,
    pub flags:  u8,
    pub count:  u8,
    pub button: u8,
    pub pad0:   [u8; 4],
}

impl Copy for xcb_xkb_sa_ptr_btn_t {}
impl Clone for xcb_xkb_sa_ptr_btn_t {
    fn clone(&self) -> xcb_xkb_sa_ptr_btn_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_ptr_btn_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_ptr_btn_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("count", &self.count)
            .field("button", &self.button)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_ptr_btn_iterator_t {
    pub data:  *mut xcb_xkb_sa_ptr_btn_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_lock_ptr_btn_t {
    pub type_:  u8,
    pub flags:  u8,
    pub pad0:   u8,
    pub button: u8,
    pub pad1:   [u8; 4],
}

impl Copy for xcb_xkb_sa_lock_ptr_btn_t {}
impl Clone for xcb_xkb_sa_lock_ptr_btn_t {
    fn clone(&self) -> xcb_xkb_sa_lock_ptr_btn_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_lock_ptr_btn_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_lock_ptr_btn_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("pad0", &self.pad0)
            .field("button", &self.button)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_ptr_btn_iterator_t {
    pub data:  *mut xcb_xkb_sa_lock_ptr_btn_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_sa_set_ptr_dflt_flag_t = u32;
pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_DFLT_BTN_ABSOLUTE : xcb_xkb_sa_set_ptr_dflt_flag_t = 0x04;
pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_AFFECT_DFLT_BUTTON: xcb_xkb_sa_set_ptr_dflt_flag_t = 0x01;

#[repr(C)]
pub struct xcb_xkb_sa_set_ptr_dflt_t {
    pub type_:  u8,
    pub flags:  u8,
    pub affect: u8,
    pub value:  i8,
    pub pad0:   [u8; 4],
}

impl Copy for xcb_xkb_sa_set_ptr_dflt_t {}
impl Clone for xcb_xkb_sa_set_ptr_dflt_t {
    fn clone(&self) -> xcb_xkb_sa_set_ptr_dflt_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_set_ptr_dflt_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_set_ptr_dflt_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("affect", &self.affect)
            .field("value", &self.value)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_set_ptr_dflt_iterator_t {
    pub data:  *mut xcb_xkb_sa_set_ptr_dflt_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_sa_iso_lock_flag_t = u32;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_LOCK          : xcb_xkb_sa_iso_lock_flag_t = 0x01;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_UNLOCK        : xcb_xkb_sa_iso_lock_flag_t = 0x02;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_USE_MOD_MAP_MODS : xcb_xkb_sa_iso_lock_flag_t = 0x04;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_GROUP_ABSOLUTE   : xcb_xkb_sa_iso_lock_flag_t = 0x04;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_ISO_DFLT_IS_GROUP: xcb_xkb_sa_iso_lock_flag_t = 0x08;

pub type xcb_xkb_sa_iso_lock_no_affect_t = u32;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_CTRLS: xcb_xkb_sa_iso_lock_no_affect_t = 0x08;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_PTR  : xcb_xkb_sa_iso_lock_no_affect_t = 0x10;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_GROUP: xcb_xkb_sa_iso_lock_no_affect_t = 0x20;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_MODS : xcb_xkb_sa_iso_lock_no_affect_t = 0x40;

#[repr(C)]
pub struct xcb_xkb_sa_iso_lock_t {
    pub type_:     u8,
    pub flags:     u8,
    pub mask:      u8,
    pub realMods:  u8,
    pub group:     i8,
    pub affect:    u8,
    pub vmodsHigh: u8,
    pub vmodsLow:  u8,
}

impl Copy for xcb_xkb_sa_iso_lock_t {}
impl Clone for xcb_xkb_sa_iso_lock_t {
    fn clone(&self) -> xcb_xkb_sa_iso_lock_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_iso_lock_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_iso_lock_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("mask", &self.mask)
            .field("realMods", &self.realMods)
            .field("group", &self.group)
            .field("affect", &self.affect)
            .field("vmodsHigh", &self.vmodsHigh)
            .field("vmodsLow", &self.vmodsLow)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_iso_lock_iterator_t {
    pub data:  *mut xcb_xkb_sa_iso_lock_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_terminate_t {
    pub type_: u8,
    pub pad0:  [u8; 7],
}

impl Copy for xcb_xkb_sa_terminate_t {}
impl Clone for xcb_xkb_sa_terminate_t {
    fn clone(&self) -> xcb_xkb_sa_terminate_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_terminate_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_terminate_t")
            .field("type_", &self.type_)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_terminate_iterator_t {
    pub data:  *mut xcb_xkb_sa_terminate_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_switch_screen_flag_t = u32;
pub const XCB_XKB_SWITCH_SCREEN_FLAG_APPLICATION: xcb_xkb_switch_screen_flag_t = 0x01;
pub const XCB_XKB_SWITCH_SCREEN_FLAG_ABSOLUTE   : xcb_xkb_switch_screen_flag_t = 0x04;

#[repr(C)]
pub struct xcb_xkb_sa_switch_screen_t {
    pub type_:     u8,
    pub flags:     u8,
    pub newScreen: i8,
    pub pad0:      [u8; 5],
}

impl Copy for xcb_xkb_sa_switch_screen_t {}
impl Clone for xcb_xkb_sa_switch_screen_t {
    fn clone(&self) -> xcb_xkb_sa_switch_screen_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_switch_screen_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_switch_screen_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("newScreen", &self.newScreen)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_switch_screen_iterator_t {
    pub data:  *mut xcb_xkb_sa_switch_screen_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_bool_ctrls_high_t = u32;
pub const XCB_XKB_BOOL_CTRLS_HIGH_ACCESS_X_FEEDBACK: xcb_xkb_bool_ctrls_high_t = 0x01;
pub const XCB_XKB_BOOL_CTRLS_HIGH_AUDIBLE_BELL     : xcb_xkb_bool_ctrls_high_t = 0x02;
pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_1        : xcb_xkb_bool_ctrls_high_t = 0x04;
pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_2        : xcb_xkb_bool_ctrls_high_t = 0x08;
pub const XCB_XKB_BOOL_CTRLS_HIGH_IGNORE_GROUP_LOCK: xcb_xkb_bool_ctrls_high_t = 0x10;

pub type xcb_xkb_bool_ctrls_low_t = u32;
pub const XCB_XKB_BOOL_CTRLS_LOW_REPEAT_KEYS     : xcb_xkb_bool_ctrls_low_t = 0x01;
pub const XCB_XKB_BOOL_CTRLS_LOW_SLOW_KEYS       : xcb_xkb_bool_ctrls_low_t = 0x02;
pub const XCB_XKB_BOOL_CTRLS_LOW_BOUNCE_KEYS     : xcb_xkb_bool_ctrls_low_t = 0x04;
pub const XCB_XKB_BOOL_CTRLS_LOW_STICKY_KEYS     : xcb_xkb_bool_ctrls_low_t = 0x08;
pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS      : xcb_xkb_bool_ctrls_low_t = 0x10;
pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS_ACCEL: xcb_xkb_bool_ctrls_low_t = 0x20;
pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_KEYS   : xcb_xkb_bool_ctrls_low_t = 0x40;
pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_TIMEOUT: xcb_xkb_bool_ctrls_low_t = 0x80;

#[repr(C)]
pub struct xcb_xkb_sa_set_controls_t {
    pub type_:         u8,
    pub pad0:          [u8; 3],
    pub boolCtrlsHigh: u8,
    pub boolCtrlsLow:  u8,
    pub pad1:          [u8; 2],
}

impl Copy for xcb_xkb_sa_set_controls_t {}
impl Clone for xcb_xkb_sa_set_controls_t {
    fn clone(&self) -> xcb_xkb_sa_set_controls_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_set_controls_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_set_controls_t")
            .field("type_", &self.type_)
            .field("pad0", &&self.pad0[..])
            .field("boolCtrlsHigh", &self.boolCtrlsHigh)
            .field("boolCtrlsLow", &self.boolCtrlsLow)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_set_controls_iterator_t {
    pub data:  *mut xcb_xkb_sa_set_controls_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_lock_controls_t {
    pub type_:         u8,
    pub pad0:          [u8; 3],
    pub boolCtrlsHigh: u8,
    pub boolCtrlsLow:  u8,
    pub pad1:          [u8; 2],
}

impl Copy for xcb_xkb_sa_lock_controls_t {}
impl Clone for xcb_xkb_sa_lock_controls_t {
    fn clone(&self) -> xcb_xkb_sa_lock_controls_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_lock_controls_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_lock_controls_t")
            .field("type_", &self.type_)
            .field("pad0", &&self.pad0[..])
            .field("boolCtrlsHigh", &self.boolCtrlsHigh)
            .field("boolCtrlsLow", &self.boolCtrlsLow)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_controls_iterator_t {
    pub data:  *mut xcb_xkb_sa_lock_controls_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_action_message_flag_t = u32;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_PRESS     : xcb_xkb_action_message_flag_t = 0x01;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_RELEASE   : xcb_xkb_action_message_flag_t = 0x02;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_GEN_KEY_EVENT: xcb_xkb_action_message_flag_t = 0x04;

#[repr(C)]
pub struct xcb_xkb_sa_action_message_t {
    pub type_:   u8,
    pub flags:   u8,
    pub message: [u8; 6],
}

impl Copy for xcb_xkb_sa_action_message_t {}
impl Clone for xcb_xkb_sa_action_message_t {
    fn clone(&self) -> xcb_xkb_sa_action_message_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_action_message_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_action_message_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("message", &&self.message[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_action_message_iterator_t<'a> {
    pub data:  *mut xcb_xkb_sa_action_message_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_sa_action_message_t>,
}

#[repr(C)]
pub struct xcb_xkb_sa_redirect_key_t {
    pub type_:         u8,
    pub newkey:        xcb_keycode_t,
    pub mask:          u8,
    pub realModifiers: u8,
    pub vmodsMaskHigh: u8,
    pub vmodsMaskLow:  u8,
    pub vmodsHigh:     u8,
    pub vmodsLow:      u8,
}

impl Copy for xcb_xkb_sa_redirect_key_t {}
impl Clone for xcb_xkb_sa_redirect_key_t {
    fn clone(&self) -> xcb_xkb_sa_redirect_key_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_redirect_key_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_redirect_key_t")
            .field("type_", &self.type_)
            .field("newkey", &self.newkey)
            .field("mask", &self.mask)
            .field("realModifiers", &self.realModifiers)
            .field("vmodsMaskHigh", &self.vmodsMaskHigh)
            .field("vmodsMaskLow", &self.vmodsMaskLow)
            .field("vmodsHigh", &self.vmodsHigh)
            .field("vmodsLow", &self.vmodsLow)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_redirect_key_iterator_t {
    pub data:  *mut xcb_xkb_sa_redirect_key_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_sa_device_btn_t {
    pub type_:  u8,
    pub flags:  u8,
    pub count:  u8,
    pub button: u8,
    pub device: u8,
    pub pad0:   [u8; 3],
}

impl Copy for xcb_xkb_sa_device_btn_t {}
impl Clone for xcb_xkb_sa_device_btn_t {
    fn clone(&self) -> xcb_xkb_sa_device_btn_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_device_btn_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_device_btn_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("count", &self.count)
            .field("button", &self.button)
            .field("device", &self.device)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_device_btn_iterator_t {
    pub data:  *mut xcb_xkb_sa_device_btn_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_lock_device_flags_t = u32;
pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_LOCK  : xcb_xkb_lock_device_flags_t = 0x01;
pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_UNLOCK: xcb_xkb_lock_device_flags_t = 0x02;

#[repr(C)]
pub struct xcb_xkb_sa_lock_device_btn_t {
    pub type_:  u8,
    pub flags:  u8,
    pub pad0:   u8,
    pub button: u8,
    pub device: u8,
    pub pad1:   [u8; 3],
}

impl Copy for xcb_xkb_sa_lock_device_btn_t {}
impl Clone for xcb_xkb_sa_lock_device_btn_t {
    fn clone(&self) -> xcb_xkb_sa_lock_device_btn_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_lock_device_btn_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_lock_device_btn_t")
            .field("type_", &self.type_)
            .field("flags", &self.flags)
            .field("pad0", &self.pad0)
            .field("button", &self.button)
            .field("device", &self.device)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_lock_device_btn_iterator_t {
    pub data:  *mut xcb_xkb_sa_lock_device_btn_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub type xcb_xkb_sa_val_what_t = u32;
pub const XCB_XKB_SA_VAL_WHAT_IGNORE_VAL      : xcb_xkb_sa_val_what_t = 0x00;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MIN     : xcb_xkb_sa_val_what_t = 0x01;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_CENTER  : xcb_xkb_sa_val_what_t = 0x02;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MAX     : xcb_xkb_sa_val_what_t = 0x03;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_RELATIVE: xcb_xkb_sa_val_what_t = 0x04;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_ABSOLUTE: xcb_xkb_sa_val_what_t = 0x05;

#[repr(C)]
pub struct xcb_xkb_sa_device_valuator_t {
    pub type_:     u8,
    pub device:    u8,
    pub val1what:  u8,
    pub val1index: u8,
    pub val1value: u8,
    pub val2what:  u8,
    pub val2index: u8,
    pub val2value: u8,
}

impl Copy for xcb_xkb_sa_device_valuator_t {}
impl Clone for xcb_xkb_sa_device_valuator_t {
    fn clone(&self) -> xcb_xkb_sa_device_valuator_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sa_device_valuator_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sa_device_valuator_t")
            .field("type_", &self.type_)
            .field("device", &self.device)
            .field("val1what", &self.val1what)
            .field("val1index", &self.val1index)
            .field("val1value", &self.val1value)
            .field("val2what", &self.val2what)
            .field("val2index", &self.val2index)
            .field("val2value", &self.val2value)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sa_device_valuator_iterator_t {
    pub data:  *mut xcb_xkb_sa_device_valuator_t,
    pub rem:   c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct xcb_xkb_si_action_t {
    pub type_: u8,
    pub data:  [u8; 7],
}

impl Copy for xcb_xkb_si_action_t {}
impl Clone for xcb_xkb_si_action_t {
    fn clone(&self) -> xcb_xkb_si_action_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_si_action_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_si_action_t")
            .field("type_", &self.type_)
            .field("data", &&self.data[..])
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_si_action_iterator_t<'a> {
    pub data:  *mut xcb_xkb_si_action_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_si_action_t>,
}

#[repr(C)]
pub struct xcb_xkb_sym_interpret_t {
    pub sym:        xcb_keysym_t,
    pub mods:       u8,
    pub match_:     u8,
    pub virtualMod: u8,
    pub flags:      u8,
    pub action:     xcb_xkb_si_action_t,
}

impl Copy for xcb_xkb_sym_interpret_t {}
impl Clone for xcb_xkb_sym_interpret_t {
    fn clone(&self) -> xcb_xkb_sym_interpret_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_sym_interpret_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_sym_interpret_t")
            .field("sym", &self.sym)
            .field("mods", &self.mods)
            .field("match_", &self.match_)
            .field("virtualMod", &self.virtualMod)
            .field("flags", &self.flags)
            .field("action", &self.action)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_sym_interpret_iterator_t<'a> {
    pub data:  *mut xcb_xkb_sym_interpret_t,
    pub rem:   c_int,
    pub index: c_int,
    _phantom:  std::marker::PhantomData<&'a xcb_xkb_sym_interpret_t>,
}

// union
#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_action_t {
    pub data: [u8; 8]
}

impl Copy for xcb_xkb_action_t {}
impl Clone for xcb_xkb_action_t {
    fn clone(&self) -> xcb_xkb_action_t { *self }
}

#[repr(C)]
#[derive(Debug)]
pub struct xcb_xkb_action_iterator_t {
    pub data:  *mut xcb_xkb_action_t,
    pub rem:   c_int,
    pub index: c_int,
}

pub const XCB_XKB_USE_EXTENSION: u8 = 0;

#[repr(C)]
pub struct xcb_xkb_use_extension_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub wantedMajor:  u16,
    pub wantedMinor:  u16,
}

impl Copy for xcb_xkb_use_extension_request_t {}
impl Clone for xcb_xkb_use_extension_request_t {
    fn clone(&self) -> xcb_xkb_use_extension_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_use_extension_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_use_extension_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("wantedMajor", &self.wantedMajor)
            .field("wantedMinor", &self.wantedMinor)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_use_extension_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_use_extension_reply_t {
    pub response_type: u8,
    pub supported:     u8,
    pub sequence:      u16,
    pub length:        u32,
    pub serverMajor:   u16,
    pub serverMinor:   u16,
    pub pad0:          [u8; 20],
}

impl Copy for xcb_xkb_use_extension_reply_t {}
impl Clone for xcb_xkb_use_extension_reply_t {
    fn clone(&self) -> xcb_xkb_use_extension_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_use_extension_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_use_extension_reply_t")
            .field("response_type", &self.response_type)
            .field("supported", &self.supported)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("serverMajor", &self.serverMajor)
            .field("serverMinor", &self.serverMinor)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
pub struct xcb_xkb_select_events_details_t {
    pub affectNewKeyboard:     u16,
    pub newKeyboardDetails:    u16,
    pub affectState:           u16,
    pub stateDetails:          u16,
    pub affectCtrls:           u32,
    pub ctrlDetails:           u32,
    pub affectIndicatorState:  u32,
    pub indicatorStateDetails: u32,
    pub affectIndicatorMap:    u32,
    pub indicatorMapDetails:   u32,
    pub affectNames:           u16,
    pub namesDetails:          u16,
    pub affectCompat:          u8,
    pub compatDetails:         u8,
    pub affectBell:            u8,
    pub bellDetails:           u8,
    pub affectMsgDetails:      u8,
    pub msgDetails:            u8,
    pub affectAccessX:         u16,
    pub accessXDetails:        u16,
    pub affectExtDev:          u16,
    pub extdevDetails:         u16,
}
impl ::std::fmt::Debug for xcb_xkb_select_events_details_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_select_events_details_t")
            .field("affectNewKeyboard", &self.affectNewKeyboard)
            .field("newKeyboardDetails", &self.newKeyboardDetails)
            .field("affectState", &self.affectState)
            .field("stateDetails", &self.stateDetails)
            .field("affectCtrls", &self.affectCtrls)
            .field("ctrlDetails", &self.ctrlDetails)
            .field("affectIndicatorState", &self.affectIndicatorState)
            .field("indicatorStateDetails", &self.indicatorStateDetails)
            .field("affectIndicatorMap", &self.affectIndicatorMap)
            .field("indicatorMapDetails", &self.indicatorMapDetails)
            .field("affectNames", &self.affectNames)
            .field("namesDetails", &self.namesDetails)
            .field("affectCompat", &self.affectCompat)
            .field("compatDetails", &self.compatDetails)
            .field("affectBell", &self.affectBell)
            .field("bellDetails", &self.bellDetails)
            .field("affectMsgDetails", &self.affectMsgDetails)
            .field("msgDetails", &self.msgDetails)
            .field("affectAccessX", &self.affectAccessX)
            .field("accessXDetails", &self.accessXDetails)
            .field("affectExtDev", &self.affectExtDev)
            .field("extdevDetails", &self.extdevDetails)
            .finish()
    }
}

pub const XCB_XKB_SELECT_EVENTS: u8 = 1;

#[repr(C)]
pub struct xcb_xkb_select_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub affectWhich:  u16,
    pub clear:        u16,
    pub selectAll:    u16,
    pub affectMap:    u16,
    pub map:          u16,
}
impl ::std::fmt::Debug for xcb_xkb_select_events_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_select_events_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("affectWhich", &self.affectWhich)
            .field("clear", &self.clear)
            .field("selectAll", &self.selectAll)
            .field("affectMap", &self.affectMap)
            .field("map", &self.map)
            .finish()
    }
}

pub const XCB_XKB_BELL: u8 = 3;

#[repr(C)]
pub struct xcb_xkb_bell_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub bellClass:    xcb_xkb_bell_class_spec_t,
    pub bellID:       xcb_xkb_id_spec_t,
    pub percent:      i8,
    pub forceSound:   u8,
    pub eventOnly:    u8,
    pub pad0:         u8,
    pub pitch:        i16,
    pub duration:     i16,
    pub pad1:         [u8; 2],
    pub name:         xcb_atom_t,
    pub window:       xcb_window_t,
}

impl Copy for xcb_xkb_bell_request_t {}
impl Clone for xcb_xkb_bell_request_t {
    fn clone(&self) -> xcb_xkb_bell_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_bell_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_bell_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("bellClass", &self.bellClass)
            .field("bellID", &self.bellID)
            .field("percent", &self.percent)
            .field("forceSound", &self.forceSound)
            .field("eventOnly", &self.eventOnly)
            .field("pad0", &self.pad0)
            .field("pitch", &self.pitch)
            .field("duration", &self.duration)
            .field("pad1", &&self.pad1[..])
            .field("name", &self.name)
            .field("window", &self.window)
            .finish()
    }
}

pub const XCB_XKB_GET_STATE: u8 = 4;

#[repr(C)]
pub struct xcb_xkb_get_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub pad0:         [u8; 2],
}

impl Copy for xcb_xkb_get_state_request_t {}
impl Clone for xcb_xkb_get_state_request_t {
    fn clone(&self) -> xcb_xkb_get_state_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_state_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_state_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_state_reply_t {
    pub response_type:    u8,
    pub deviceID:         u8,
    pub sequence:         u16,
    pub length:           u32,
    pub mods:             u8,
    pub baseMods:         u8,
    pub latchedMods:      u8,
    pub lockedMods:       u8,
    pub group:            u8,
    pub lockedGroup:      u8,
    pub baseGroup:        i16,
    pub latchedGroup:     i16,
    pub compatState:      u8,
    pub grabMods:         u8,
    pub compatGrabMods:   u8,
    pub lookupMods:       u8,
    pub compatLookupMods: u8,
    pub pad0:             u8,
    pub ptrBtnState:      u16,
    pub pad1:             [u8; 6],
}

impl Copy for xcb_xkb_get_state_reply_t {}
impl Clone for xcb_xkb_get_state_reply_t {
    fn clone(&self) -> xcb_xkb_get_state_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_state_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_state_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("mods", &self.mods)
            .field("baseMods", &self.baseMods)
            .field("latchedMods", &self.latchedMods)
            .field("lockedMods", &self.lockedMods)
            .field("group", &self.group)
            .field("lockedGroup", &self.lockedGroup)
            .field("baseGroup", &self.baseGroup)
            .field("latchedGroup", &self.latchedGroup)
            .field("compatState", &self.compatState)
            .field("grabMods", &self.grabMods)
            .field("compatGrabMods", &self.compatGrabMods)
            .field("lookupMods", &self.lookupMods)
            .field("compatLookupMods", &self.compatLookupMods)
            .field("pad0", &self.pad0)
            .field("ptrBtnState", &self.ptrBtnState)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

pub const XCB_XKB_LATCH_LOCK_STATE: u8 = 5;

#[repr(C)]
pub struct xcb_xkb_latch_lock_state_request_t {
    pub major_opcode:     u8,
    pub minor_opcode:     u8,
    pub length:           u16,
    pub deviceSpec:       xcb_xkb_device_spec_t,
    pub affectModLocks:   u8,
    pub modLocks:         u8,
    pub lockGroup:        u8,
    pub groupLock:        u8,
    pub affectModLatches: u8,
    pub pad0:             u8,
    pub pad1:             u8,
    pub latchGroup:       u8,
    pub groupLatch:       u16,
}

impl Copy for xcb_xkb_latch_lock_state_request_t {}
impl Clone for xcb_xkb_latch_lock_state_request_t {
    fn clone(&self) -> xcb_xkb_latch_lock_state_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_latch_lock_state_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_latch_lock_state_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("affectModLocks", &self.affectModLocks)
            .field("modLocks", &self.modLocks)
            .field("lockGroup", &self.lockGroup)
            .field("groupLock", &self.groupLock)
            .field("affectModLatches", &self.affectModLatches)
            .field("pad0", &self.pad0)
            .field("pad1", &self.pad1)
            .field("latchGroup", &self.latchGroup)
            .field("groupLatch", &self.groupLatch)
            .finish()
    }
}

pub const XCB_XKB_GET_CONTROLS: u8 = 6;

#[repr(C)]
pub struct xcb_xkb_get_controls_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub pad0:         [u8; 2],
}

impl Copy for xcb_xkb_get_controls_request_t {}
impl Clone for xcb_xkb_get_controls_request_t {
    fn clone(&self) -> xcb_xkb_get_controls_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_controls_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_controls_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_controls_reply_t {
    pub response_type:               u8,
    pub deviceID:                    u8,
    pub sequence:                    u16,
    pub length:                      u32,
    pub mouseKeysDfltBtn:            u8,
    pub numGroups:                   u8,
    pub groupsWrap:                  u8,
    pub internalModsMask:            u8,
    pub ignoreLockModsMask:          u8,
    pub internalModsRealMods:        u8,
    pub ignoreLockModsRealMods:      u8,
    pub pad0:                        u8,
    pub internalModsVmods:           u16,
    pub ignoreLockModsVmods:         u16,
    pub repeatDelay:                 u16,
    pub repeatInterval:              u16,
    pub slowKeysDelay:               u16,
    pub debounceDelay:               u16,
    pub mouseKeysDelay:              u16,
    pub mouseKeysInterval:           u16,
    pub mouseKeysTimeToMax:          u16,
    pub mouseKeysMaxSpeed:           u16,
    pub mouseKeysCurve:              i16,
    pub accessXOption:               u16,
    pub accessXTimeout:              u16,
    pub accessXTimeoutOptionsMask:   u16,
    pub accessXTimeoutOptionsValues: u16,
    pub pad1:                        [u8; 2],
    pub accessXTimeoutMask:          u32,
    pub accessXTimeoutValues:        u32,
    pub enabledControls:             u32,
    pub perKeyRepeat:                [u8; 32],
}

impl Copy for xcb_xkb_get_controls_reply_t {}
impl Clone for xcb_xkb_get_controls_reply_t {
    fn clone(&self) -> xcb_xkb_get_controls_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_controls_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_controls_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("mouseKeysDfltBtn", &self.mouseKeysDfltBtn)
            .field("numGroups", &self.numGroups)
            .field("groupsWrap", &self.groupsWrap)
            .field("internalModsMask", &self.internalModsMask)
            .field("ignoreLockModsMask", &self.ignoreLockModsMask)
            .field("internalModsRealMods", &self.internalModsRealMods)
            .field("ignoreLockModsRealMods", &self.ignoreLockModsRealMods)
            .field("pad0", &self.pad0)
            .field("internalModsVmods", &self.internalModsVmods)
            .field("ignoreLockModsVmods", &self.ignoreLockModsVmods)
            .field("repeatDelay", &self.repeatDelay)
            .field("repeatInterval", &self.repeatInterval)
            .field("slowKeysDelay", &self.slowKeysDelay)
            .field("debounceDelay", &self.debounceDelay)
            .field("mouseKeysDelay", &self.mouseKeysDelay)
            .field("mouseKeysInterval", &self.mouseKeysInterval)
            .field("mouseKeysTimeToMax", &self.mouseKeysTimeToMax)
            .field("mouseKeysMaxSpeed", &self.mouseKeysMaxSpeed)
            .field("mouseKeysCurve", &self.mouseKeysCurve)
            .field("accessXOption", &self.accessXOption)
            .field("accessXTimeout", &self.accessXTimeout)
            .field("accessXTimeoutOptionsMask", &self.accessXTimeoutOptionsMask)
            .field("accessXTimeoutOptionsValues", &self.accessXTimeoutOptionsValues)
            .field("pad1", &&self.pad1[..])
            .field("accessXTimeoutMask", &self.accessXTimeoutMask)
            .field("accessXTimeoutValues", &self.accessXTimeoutValues)
            .field("enabledControls", &self.enabledControls)
            .field("perKeyRepeat", &&self.perKeyRepeat[..])
            .finish()
    }
}

pub const XCB_XKB_SET_CONTROLS: u8 = 7;

#[repr(C)]
pub struct xcb_xkb_set_controls_request_t {
    pub major_opcode:                u8,
    pub minor_opcode:                u8,
    pub length:                      u16,
    pub deviceSpec:                  xcb_xkb_device_spec_t,
    pub affectInternalRealMods:      u8,
    pub internalRealMods:            u8,
    pub affectIgnoreLockRealMods:    u8,
    pub ignoreLockRealMods:          u8,
    pub affectInternalVirtualMods:   u16,
    pub internalVirtualMods:         u16,
    pub affectIgnoreLockVirtualMods: u16,
    pub ignoreLockVirtualMods:       u16,
    pub mouseKeysDfltBtn:            u8,
    pub groupsWrap:                  u8,
    pub accessXOptions:              u16,
    pub pad0:                        [u8; 2],
    pub affectEnabledControls:       u32,
    pub enabledControls:             u32,
    pub changeControls:              u32,
    pub repeatDelay:                 u16,
    pub repeatInterval:              u16,
    pub slowKeysDelay:               u16,
    pub debounceDelay:               u16,
    pub mouseKeysDelay:              u16,
    pub mouseKeysInterval:           u16,
    pub mouseKeysTimeToMax:          u16,
    pub mouseKeysMaxSpeed:           u16,
    pub mouseKeysCurve:              i16,
    pub accessXTimeout:              u16,
    pub accessXTimeoutMask:          u32,
    pub accessXTimeoutValues:        u32,
    pub accessXTimeoutOptionsMask:   u16,
    pub accessXTimeoutOptionsValues: u16,
    pub perKeyRepeat:                [u8; 32],
}

impl Copy for xcb_xkb_set_controls_request_t {}
impl Clone for xcb_xkb_set_controls_request_t {
    fn clone(&self) -> xcb_xkb_set_controls_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_set_controls_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_controls_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("affectInternalRealMods", &self.affectInternalRealMods)
            .field("internalRealMods", &self.internalRealMods)
            .field("affectIgnoreLockRealMods", &self.affectIgnoreLockRealMods)
            .field("ignoreLockRealMods", &self.ignoreLockRealMods)
            .field("affectInternalVirtualMods", &self.affectInternalVirtualMods)
            .field("internalVirtualMods", &self.internalVirtualMods)
            .field("affectIgnoreLockVirtualMods", &self.affectIgnoreLockVirtualMods)
            .field("ignoreLockVirtualMods", &self.ignoreLockVirtualMods)
            .field("mouseKeysDfltBtn", &self.mouseKeysDfltBtn)
            .field("groupsWrap", &self.groupsWrap)
            .field("accessXOptions", &self.accessXOptions)
            .field("pad0", &&self.pad0[..])
            .field("affectEnabledControls", &self.affectEnabledControls)
            .field("enabledControls", &self.enabledControls)
            .field("changeControls", &self.changeControls)
            .field("repeatDelay", &self.repeatDelay)
            .field("repeatInterval", &self.repeatInterval)
            .field("slowKeysDelay", &self.slowKeysDelay)
            .field("debounceDelay", &self.debounceDelay)
            .field("mouseKeysDelay", &self.mouseKeysDelay)
            .field("mouseKeysInterval", &self.mouseKeysInterval)
            .field("mouseKeysTimeToMax", &self.mouseKeysTimeToMax)
            .field("mouseKeysMaxSpeed", &self.mouseKeysMaxSpeed)
            .field("mouseKeysCurve", &self.mouseKeysCurve)
            .field("accessXTimeout", &self.accessXTimeout)
            .field("accessXTimeoutMask", &self.accessXTimeoutMask)
            .field("accessXTimeoutValues", &self.accessXTimeoutValues)
            .field("accessXTimeoutOptionsMask", &self.accessXTimeoutOptionsMask)
            .field("accessXTimeoutOptionsValues", &self.accessXTimeoutOptionsValues)
            .field("perKeyRepeat", &&self.perKeyRepeat[..])
            .finish()
    }
}

pub const XCB_XKB_GET_MAP: u8 = 8;

#[repr(C)]
pub struct xcb_xkb_get_map_request_t {
    pub major_opcode:     u8,
    pub minor_opcode:     u8,
    pub length:           u16,
    pub deviceSpec:       xcb_xkb_device_spec_t,
    pub full:             u16,
    pub partial:          u16,
    pub firstType:        u8,
    pub nTypes:           u8,
    pub firstKeySym:      xcb_keycode_t,
    pub nKeySyms:         u8,
    pub firstKeyAction:   xcb_keycode_t,
    pub nKeyActions:      u8,
    pub firstKeyBehavior: xcb_keycode_t,
    pub nKeyBehaviors:    u8,
    pub virtualMods:      u16,
    pub firstKeyExplicit: xcb_keycode_t,
    pub nKeyExplicit:     u8,
    pub firstModMapKey:   xcb_keycode_t,
    pub nModMapKeys:      u8,
    pub firstVModMapKey:  xcb_keycode_t,
    pub nVModMapKeys:     u8,
    pub pad0:             [u8; 2],
}

impl Copy for xcb_xkb_get_map_request_t {}
impl Clone for xcb_xkb_get_map_request_t {
    fn clone(&self) -> xcb_xkb_get_map_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_map_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_map_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("full", &self.full)
            .field("partial", &self.partial)
            .field("firstType", &self.firstType)
            .field("nTypes", &self.nTypes)
            .field("firstKeySym", &self.firstKeySym)
            .field("nKeySyms", &self.nKeySyms)
            .field("firstKeyAction", &self.firstKeyAction)
            .field("nKeyActions", &self.nKeyActions)
            .field("firstKeyBehavior", &self.firstKeyBehavior)
            .field("nKeyBehaviors", &self.nKeyBehaviors)
            .field("virtualMods", &self.virtualMods)
            .field("firstKeyExplicit", &self.firstKeyExplicit)
            .field("nKeyExplicit", &self.nKeyExplicit)
            .field("firstModMapKey", &self.firstModMapKey)
            .field("nModMapKeys", &self.nModMapKeys)
            .field("firstVModMapKey", &self.firstVModMapKey)
            .field("nVModMapKeys", &self.nVModMapKeys)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_map_map_t {
    pub types_rtrn:      *mut xcb_xkb_key_type_t,
    pub syms_rtrn:       *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count: *mut u8,
    pub pad2:            *mut u8,
    pub acts_rtrn_acts:  *mut xcb_xkb_action_t,
    pub behaviors_rtrn:  *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn:      *mut u8,
    pub pad3:            *mut u8,
    pub explicit_rtrn:   *mut xcb_xkb_set_explicit_t,
    pub pad4:            *mut u8,
    pub modmap_rtrn:     *mut xcb_xkb_key_mod_map_t,
    pub pad5:            *mut u8,
    pub vmodmap_rtrn:    *mut xcb_xkb_key_v_mod_map_t,
}
impl ::std::fmt::Debug for xcb_xkb_get_map_map_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_map_map_t")
            .field("types_rtrn", &self.types_rtrn)
            .field("syms_rtrn", &self.syms_rtrn)
            .field("acts_rtrn_count", &self.acts_rtrn_count)
            .field("pad2", &self.pad2)
            .field("acts_rtrn_acts", &self.acts_rtrn_acts)
            .field("behaviors_rtrn", &self.behaviors_rtrn)
            .field("vmods_rtrn", &self.vmods_rtrn)
            .field("pad3", &self.pad3)
            .field("explicit_rtrn", &self.explicit_rtrn)
            .field("pad4", &self.pad4)
            .field("modmap_rtrn", &self.modmap_rtrn)
            .field("pad5", &self.pad5)
            .field("vmodmap_rtrn", &self.vmodmap_rtrn)
            .finish()
    }
}

#[repr(C)]
pub struct xcb_xkb_get_map_reply_t {
    pub response_type:     u8,
    pub deviceID:          u8,
    pub sequence:          u16,
    pub length:            u32,
    pub pad0:              [u8; 2],
    pub minKeyCode:        xcb_keycode_t,
    pub maxKeyCode:        xcb_keycode_t,
    pub present:           u16,
    pub firstType:         u8,
    pub nTypes:            u8,
    pub totalTypes:        u8,
    pub firstKeySym:       xcb_keycode_t,
    pub totalSyms:         u16,
    pub nKeySyms:          u8,
    pub firstKeyAction:    xcb_keycode_t,
    pub totalActions:      u16,
    pub nKeyActions:       u8,
    pub firstKeyBehavior:  xcb_keycode_t,
    pub nKeyBehaviors:     u8,
    pub totalKeyBehaviors: u8,
    pub firstKeyExplicit:  xcb_keycode_t,
    pub nKeyExplicit:      u8,
    pub totalKeyExplicit:  u8,
    pub firstModMapKey:    xcb_keycode_t,
    pub nModMapKeys:       u8,
    pub totalModMapKeys:   u8,
    pub firstVModMapKey:   xcb_keycode_t,
    pub nVModMapKeys:      u8,
    pub totalVModMapKeys:  u8,
    pub pad1:              u8,
    pub virtualMods:       u16,
}
impl ::std::fmt::Debug for xcb_xkb_get_map_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_map_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("pad0", &&self.pad0[..])
            .field("minKeyCode", &self.minKeyCode)
            .field("maxKeyCode", &self.maxKeyCode)
            .field("present", &self.present)
            .field("firstType", &self.firstType)
            .field("nTypes", &self.nTypes)
            .field("totalTypes", &self.totalTypes)
            .field("firstKeySym", &self.firstKeySym)
            .field("totalSyms", &self.totalSyms)
            .field("nKeySyms", &self.nKeySyms)
            .field("firstKeyAction", &self.firstKeyAction)
            .field("totalActions", &self.totalActions)
            .field("nKeyActions", &self.nKeyActions)
            .field("firstKeyBehavior", &self.firstKeyBehavior)
            .field("nKeyBehaviors", &self.nKeyBehaviors)
            .field("totalKeyBehaviors", &self.totalKeyBehaviors)
            .field("firstKeyExplicit", &self.firstKeyExplicit)
            .field("nKeyExplicit", &self.nKeyExplicit)
            .field("totalKeyExplicit", &self.totalKeyExplicit)
            .field("firstModMapKey", &self.firstModMapKey)
            .field("nModMapKeys", &self.nModMapKeys)
            .field("totalModMapKeys", &self.totalModMapKeys)
            .field("firstVModMapKey", &self.firstVModMapKey)
            .field("nVModMapKeys", &self.nVModMapKeys)
            .field("totalVModMapKeys", &self.totalVModMapKeys)
            .field("pad1", &self.pad1)
            .field("virtualMods", &self.virtualMods)
            .finish()
    }
}

#[repr(C)]
pub struct xcb_xkb_set_map_values_t {
    pub types:        *mut xcb_xkb_set_key_type_t,
    pub syms:         *mut xcb_xkb_key_sym_map_t,
    pub actionsCount: *mut u8,
    pub actions:      *mut xcb_xkb_action_t,
    pub behaviors:    *mut xcb_xkb_set_behavior_t,
    pub vmods:        *mut u8,
    pub explicit:     *mut xcb_xkb_set_explicit_t,
    pub modmap:       *mut xcb_xkb_key_mod_map_t,
    pub vmodmap:      *mut xcb_xkb_key_v_mod_map_t,
}
impl ::std::fmt::Debug for xcb_xkb_set_map_values_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_map_values_t")
            .field("types", &self.types)
            .field("syms", &self.syms)
            .field("actionsCount", &self.actionsCount)
            .field("actions", &self.actions)
            .field("behaviors", &self.behaviors)
            .field("vmods", &self.vmods)
            .field("explicit", &self.explicit)
            .field("modmap", &self.modmap)
            .field("vmodmap", &self.vmodmap)
            .finish()
    }
}

pub const XCB_XKB_SET_MAP: u8 = 9;

#[repr(C)]
pub struct xcb_xkb_set_map_request_t {
    pub major_opcode:      u8,
    pub minor_opcode:      u8,
    pub length:            u16,
    pub deviceSpec:        xcb_xkb_device_spec_t,
    pub present:           u16,
    pub flags:             u16,
    pub minKeyCode:        xcb_keycode_t,
    pub maxKeyCode:        xcb_keycode_t,
    pub firstType:         u8,
    pub nTypes:            u8,
    pub firstKeySym:       xcb_keycode_t,
    pub nKeySyms:          u8,
    pub totalSyms:         u16,
    pub firstKeyAction:    xcb_keycode_t,
    pub nKeyActions:       u8,
    pub totalActions:      u16,
    pub firstKeyBehavior:  xcb_keycode_t,
    pub nKeyBehaviors:     u8,
    pub totalKeyBehaviors: u8,
    pub firstKeyExplicit:  xcb_keycode_t,
    pub nKeyExplicit:      u8,
    pub totalKeyExplicit:  u8,
    pub firstModMapKey:    xcb_keycode_t,
    pub nModMapKeys:       u8,
    pub totalModMapKeys:   u8,
    pub firstVModMapKey:   xcb_keycode_t,
    pub nVModMapKeys:      u8,
    pub totalVModMapKeys:  u8,
    pub virtualMods:       u16,
}
impl ::std::fmt::Debug for xcb_xkb_set_map_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_map_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("present", &self.present)
            .field("flags", &self.flags)
            .field("minKeyCode", &self.minKeyCode)
            .field("maxKeyCode", &self.maxKeyCode)
            .field("firstType", &self.firstType)
            .field("nTypes", &self.nTypes)
            .field("firstKeySym", &self.firstKeySym)
            .field("nKeySyms", &self.nKeySyms)
            .field("totalSyms", &self.totalSyms)
            .field("firstKeyAction", &self.firstKeyAction)
            .field("nKeyActions", &self.nKeyActions)
            .field("totalActions", &self.totalActions)
            .field("firstKeyBehavior", &self.firstKeyBehavior)
            .field("nKeyBehaviors", &self.nKeyBehaviors)
            .field("totalKeyBehaviors", &self.totalKeyBehaviors)
            .field("firstKeyExplicit", &self.firstKeyExplicit)
            .field("nKeyExplicit", &self.nKeyExplicit)
            .field("totalKeyExplicit", &self.totalKeyExplicit)
            .field("firstModMapKey", &self.firstModMapKey)
            .field("nModMapKeys", &self.nModMapKeys)
            .field("totalModMapKeys", &self.totalModMapKeys)
            .field("firstVModMapKey", &self.firstVModMapKey)
            .field("nVModMapKeys", &self.nVModMapKeys)
            .field("totalVModMapKeys", &self.totalVModMapKeys)
            .field("virtualMods", &self.virtualMods)
            .finish()
    }
}

pub const XCB_XKB_GET_COMPAT_MAP: u8 = 10;

#[repr(C)]
pub struct xcb_xkb_get_compat_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub groups:       u8,
    pub getAllSI:     u8,
    pub firstSI:      u16,
    pub nSI:          u16,
}

impl Copy for xcb_xkb_get_compat_map_request_t {}
impl Clone for xcb_xkb_get_compat_map_request_t {
    fn clone(&self) -> xcb_xkb_get_compat_map_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_compat_map_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_compat_map_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("groups", &self.groups)
            .field("getAllSI", &self.getAllSI)
            .field("firstSI", &self.firstSI)
            .field("nSI", &self.nSI)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_compat_map_reply_t {
    pub response_type: u8,
    pub deviceID:      u8,
    pub sequence:      u16,
    pub length:        u32,
    pub groupsRtrn:    u8,
    pub pad0:          u8,
    pub firstSIRtrn:   u16,
    pub nSIRtrn:       u16,
    pub nTotalSI:      u16,
    pub pad1:          [u8; 16],
}
impl ::std::fmt::Debug for xcb_xkb_get_compat_map_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_compat_map_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("groupsRtrn", &self.groupsRtrn)
            .field("pad0", &self.pad0)
            .field("firstSIRtrn", &self.firstSIRtrn)
            .field("nSIRtrn", &self.nSIRtrn)
            .field("nTotalSI", &self.nTotalSI)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

pub const XCB_XKB_SET_COMPAT_MAP: u8 = 11;

#[repr(C)]
pub struct xcb_xkb_set_compat_map_request_t {
    pub major_opcode:     u8,
    pub minor_opcode:     u8,
    pub length:           u16,
    pub deviceSpec:       xcb_xkb_device_spec_t,
    pub pad0:             u8,
    pub recomputeActions: u8,
    pub truncateSI:       u8,
    pub groups:           u8,
    pub firstSI:          u16,
    pub nSI:              u16,
    pub pad1:             [u8; 2],
}
impl ::std::fmt::Debug for xcb_xkb_set_compat_map_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_compat_map_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("pad0", &self.pad0)
            .field("recomputeActions", &self.recomputeActions)
            .field("truncateSI", &self.truncateSI)
            .field("groups", &self.groups)
            .field("firstSI", &self.firstSI)
            .field("nSI", &self.nSI)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

pub const XCB_XKB_GET_INDICATOR_STATE: u8 = 12;

#[repr(C)]
pub struct xcb_xkb_get_indicator_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub pad0:         [u8; 2],
}

impl Copy for xcb_xkb_get_indicator_state_request_t {}
impl Clone for xcb_xkb_get_indicator_state_request_t {
    fn clone(&self) -> xcb_xkb_get_indicator_state_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_indicator_state_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_indicator_state_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_indicator_state_reply_t {
    pub response_type: u8,
    pub deviceID:      u8,
    pub sequence:      u16,
    pub length:        u32,
    pub state:         u32,
    pub pad0:          [u8; 20],
}

impl Copy for xcb_xkb_get_indicator_state_reply_t {}
impl Clone for xcb_xkb_get_indicator_state_reply_t {
    fn clone(&self) -> xcb_xkb_get_indicator_state_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_indicator_state_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_indicator_state_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("state", &self.state)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_GET_INDICATOR_MAP: u8 = 13;

#[repr(C)]
pub struct xcb_xkb_get_indicator_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub pad0:         [u8; 2],
    pub which:        u32,
}

impl Copy for xcb_xkb_get_indicator_map_request_t {}
impl Clone for xcb_xkb_get_indicator_map_request_t {
    fn clone(&self) -> xcb_xkb_get_indicator_map_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_indicator_map_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_indicator_map_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("pad0", &&self.pad0[..])
            .field("which", &self.which)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_indicator_map_reply_t {
    pub response_type:  u8,
    pub deviceID:       u8,
    pub sequence:       u16,
    pub length:         u32,
    pub which:          u32,
    pub realIndicators: u32,
    pub nIndicators:    u8,
    pub pad0:           [u8; 15],
}
impl ::std::fmt::Debug for xcb_xkb_get_indicator_map_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_indicator_map_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("which", &self.which)
            .field("realIndicators", &self.realIndicators)
            .field("nIndicators", &self.nIndicators)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_SET_INDICATOR_MAP: u8 = 14;

#[repr(C)]
pub struct xcb_xkb_set_indicator_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub pad0:         [u8; 2],
    pub which:        u32,
}
impl ::std::fmt::Debug for xcb_xkb_set_indicator_map_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_indicator_map_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("pad0", &&self.pad0[..])
            .field("which", &self.which)
            .finish()
    }
}

pub const XCB_XKB_GET_NAMED_INDICATOR: u8 = 15;

#[repr(C)]
pub struct xcb_xkb_get_named_indicator_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub ledClass:     xcb_xkb_led_class_spec_t,
    pub ledID:        xcb_xkb_id_spec_t,
    pub pad0:         [u8; 2],
    pub indicator:    xcb_atom_t,
}

impl Copy for xcb_xkb_get_named_indicator_request_t {}
impl Clone for xcb_xkb_get_named_indicator_request_t {
    fn clone(&self) -> xcb_xkb_get_named_indicator_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_named_indicator_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_named_indicator_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("ledClass", &self.ledClass)
            .field("ledID", &self.ledID)
            .field("pad0", &&self.pad0[..])
            .field("indicator", &self.indicator)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_named_indicator_reply_t {
    pub response_type:   u8,
    pub deviceID:        u8,
    pub sequence:        u16,
    pub length:          u32,
    pub indicator:       xcb_atom_t,
    pub found:           u8,
    pub on:              u8,
    pub realIndicator:   u8,
    pub ndx:             u8,
    pub map_flags:       u8,
    pub map_whichGroups: u8,
    pub map_groups:      u8,
    pub map_whichMods:   u8,
    pub map_mods:        u8,
    pub map_realMods:    u8,
    pub map_vmod:        u16,
    pub map_ctrls:       u32,
    pub supported:       u8,
    pub pad0:            [u8; 3],
}

impl Copy for xcb_xkb_get_named_indicator_reply_t {}
impl Clone for xcb_xkb_get_named_indicator_reply_t {
    fn clone(&self) -> xcb_xkb_get_named_indicator_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_named_indicator_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_named_indicator_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("indicator", &self.indicator)
            .field("found", &self.found)
            .field("on", &self.on)
            .field("realIndicator", &self.realIndicator)
            .field("ndx", &self.ndx)
            .field("map_flags", &self.map_flags)
            .field("map_whichGroups", &self.map_whichGroups)
            .field("map_groups", &self.map_groups)
            .field("map_whichMods", &self.map_whichMods)
            .field("map_mods", &self.map_mods)
            .field("map_realMods", &self.map_realMods)
            .field("map_vmod", &self.map_vmod)
            .field("map_ctrls", &self.map_ctrls)
            .field("supported", &self.supported)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_SET_NAMED_INDICATOR: u8 = 16;

#[repr(C)]
pub struct xcb_xkb_set_named_indicator_request_t {
    pub major_opcode:    u8,
    pub minor_opcode:    u8,
    pub length:          u16,
    pub deviceSpec:      xcb_xkb_device_spec_t,
    pub ledClass:        xcb_xkb_led_class_spec_t,
    pub ledID:           xcb_xkb_id_spec_t,
    pub pad0:            [u8; 2],
    pub indicator:       xcb_atom_t,
    pub setState:        u8,
    pub on:              u8,
    pub setMap:          u8,
    pub createMap:       u8,
    pub pad1:            u8,
    pub map_flags:       u8,
    pub map_whichGroups: u8,
    pub map_groups:      u8,
    pub map_whichMods:   u8,
    pub map_realMods:    u8,
    pub map_vmods:       u16,
    pub map_ctrls:       u32,
}

impl Copy for xcb_xkb_set_named_indicator_request_t {}
impl Clone for xcb_xkb_set_named_indicator_request_t {
    fn clone(&self) -> xcb_xkb_set_named_indicator_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_set_named_indicator_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_named_indicator_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("ledClass", &self.ledClass)
            .field("ledID", &self.ledID)
            .field("pad0", &&self.pad0[..])
            .field("indicator", &self.indicator)
            .field("setState", &self.setState)
            .field("on", &self.on)
            .field("setMap", &self.setMap)
            .field("createMap", &self.createMap)
            .field("pad1", &self.pad1)
            .field("map_flags", &self.map_flags)
            .field("map_whichGroups", &self.map_whichGroups)
            .field("map_groups", &self.map_groups)
            .field("map_whichMods", &self.map_whichMods)
            .field("map_realMods", &self.map_realMods)
            .field("map_vmods", &self.map_vmods)
            .field("map_ctrls", &self.map_ctrls)
            .finish()
    }
}

pub const XCB_XKB_GET_NAMES: u8 = 17;

#[repr(C)]
pub struct xcb_xkb_get_names_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub pad0:         [u8; 2],
    pub which:        u32,
}

impl Copy for xcb_xkb_get_names_request_t {}
impl Clone for xcb_xkb_get_names_request_t {
    fn clone(&self) -> xcb_xkb_get_names_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_names_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_names_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("pad0", &&self.pad0[..])
            .field("which", &self.which)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_names_value_list_t {
    pub keycodesName:    xcb_atom_t,
    pub geometryName:    xcb_atom_t,
    pub symbolsName:     xcb_atom_t,
    pub physSymbolsName: xcb_atom_t,
    pub typesName:       xcb_atom_t,
    pub compatName:      xcb_atom_t,
    pub typeNames:       *mut xcb_atom_t,
    pub nLevelsPerType:  *mut u8,
    pub alignment_pad:   *mut u8,
    pub ktLevelNames:    *mut xcb_atom_t,
    pub indicatorNames:  *mut xcb_atom_t,
    pub virtualModNames: *mut xcb_atom_t,
    pub groups:          *mut xcb_atom_t,
    pub keyNames:        *mut xcb_xkb_key_name_t,
    pub keyAliases:      *mut xcb_xkb_key_alias_t,
    pub radioGroupNames: *mut xcb_atom_t,
}
impl ::std::fmt::Debug for xcb_xkb_get_names_value_list_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_names_value_list_t")
            .field("keycodesName", &self.keycodesName)
            .field("geometryName", &self.geometryName)
            .field("symbolsName", &self.symbolsName)
            .field("physSymbolsName", &self.physSymbolsName)
            .field("typesName", &self.typesName)
            .field("compatName", &self.compatName)
            .field("typeNames", &self.typeNames)
            .field("nLevelsPerType", &self.nLevelsPerType)
            .field("alignment_pad", &self.alignment_pad)
            .field("ktLevelNames", &self.ktLevelNames)
            .field("indicatorNames", &self.indicatorNames)
            .field("virtualModNames", &self.virtualModNames)
            .field("groups", &self.groups)
            .field("keyNames", &self.keyNames)
            .field("keyAliases", &self.keyAliases)
            .field("radioGroupNames", &self.radioGroupNames)
            .finish()
    }
}

#[repr(C)]
pub struct xcb_xkb_get_names_reply_t {
    pub response_type: u8,
    pub deviceID:      u8,
    pub sequence:      u16,
    pub length:        u32,
    pub which:         u32,
    pub minKeyCode:    xcb_keycode_t,
    pub maxKeyCode:    xcb_keycode_t,
    pub nTypes:        u8,
    pub groupNames:    u8,
    pub virtualMods:   u16,
    pub firstKey:      xcb_keycode_t,
    pub nKeys:         u8,
    pub indicators:    u32,
    pub nRadioGroups:  u8,
    pub nKeyAliases:   u8,
    pub nKTLevels:     u16,
    pub pad0:          [u8; 4],
}
impl ::std::fmt::Debug for xcb_xkb_get_names_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_names_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("which", &self.which)
            .field("minKeyCode", &self.minKeyCode)
            .field("maxKeyCode", &self.maxKeyCode)
            .field("nTypes", &self.nTypes)
            .field("groupNames", &self.groupNames)
            .field("virtualMods", &self.virtualMods)
            .field("firstKey", &self.firstKey)
            .field("nKeys", &self.nKeys)
            .field("indicators", &self.indicators)
            .field("nRadioGroups", &self.nRadioGroups)
            .field("nKeyAliases", &self.nKeyAliases)
            .field("nKTLevels", &self.nKTLevels)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

#[repr(C)]
pub struct xcb_xkb_set_names_values_t {
    pub keycodesName:    xcb_atom_t,
    pub geometryName:    xcb_atom_t,
    pub symbolsName:     xcb_atom_t,
    pub physSymbolsName: xcb_atom_t,
    pub typesName:       xcb_atom_t,
    pub compatName:      xcb_atom_t,
    pub typeNames:       *mut xcb_atom_t,
    pub nLevelsPerType:  *mut u8,
    pub ktLevelNames:    *mut xcb_atom_t,
    pub indicatorNames:  *mut xcb_atom_t,
    pub virtualModNames: *mut xcb_atom_t,
    pub groups:          *mut xcb_atom_t,
    pub keyNames:        *mut xcb_xkb_key_name_t,
    pub keyAliases:      *mut xcb_xkb_key_alias_t,
    pub radioGroupNames: *mut xcb_atom_t,
}
impl ::std::fmt::Debug for xcb_xkb_set_names_values_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_names_values_t")
            .field("keycodesName", &self.keycodesName)
            .field("geometryName", &self.geometryName)
            .field("symbolsName", &self.symbolsName)
            .field("physSymbolsName", &self.physSymbolsName)
            .field("typesName", &self.typesName)
            .field("compatName", &self.compatName)
            .field("typeNames", &self.typeNames)
            .field("nLevelsPerType", &self.nLevelsPerType)
            .field("ktLevelNames", &self.ktLevelNames)
            .field("indicatorNames", &self.indicatorNames)
            .field("virtualModNames", &self.virtualModNames)
            .field("groups", &self.groups)
            .field("keyNames", &self.keyNames)
            .field("keyAliases", &self.keyAliases)
            .field("radioGroupNames", &self.radioGroupNames)
            .finish()
    }
}

pub const XCB_XKB_SET_NAMES: u8 = 18;

#[repr(C)]
pub struct xcb_xkb_set_names_request_t {
    pub major_opcode:      u8,
    pub minor_opcode:      u8,
    pub length:            u16,
    pub deviceSpec:        xcb_xkb_device_spec_t,
    pub virtualMods:       u16,
    pub which:             u32,
    pub firstType:         u8,
    pub nTypes:            u8,
    pub firstKTLevelt:     u8,
    pub nKTLevels:         u8,
    pub indicators:        u32,
    pub groupNames:        u8,
    pub nRadioGroups:      u8,
    pub firstKey:          xcb_keycode_t,
    pub nKeys:             u8,
    pub nKeyAliases:       u8,
    pub pad0:              u8,
    pub totalKTLevelNames: u16,
}
impl ::std::fmt::Debug for xcb_xkb_set_names_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_names_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("virtualMods", &self.virtualMods)
            .field("which", &self.which)
            .field("firstType", &self.firstType)
            .field("nTypes", &self.nTypes)
            .field("firstKTLevelt", &self.firstKTLevelt)
            .field("nKTLevels", &self.nKTLevels)
            .field("indicators", &self.indicators)
            .field("groupNames", &self.groupNames)
            .field("nRadioGroups", &self.nRadioGroups)
            .field("firstKey", &self.firstKey)
            .field("nKeys", &self.nKeys)
            .field("nKeyAliases", &self.nKeyAliases)
            .field("pad0", &self.pad0)
            .field("totalKTLevelNames", &self.totalKTLevelNames)
            .finish()
    }
}

pub const XCB_XKB_PER_CLIENT_FLAGS: u8 = 21;

#[repr(C)]
pub struct xcb_xkb_per_client_flags_request_t {
    pub major_opcode:    u8,
    pub minor_opcode:    u8,
    pub length:          u16,
    pub deviceSpec:      xcb_xkb_device_spec_t,
    pub pad0:            [u8; 2],
    pub change:          u32,
    pub value:           u32,
    pub ctrlsToChange:   u32,
    pub autoCtrls:       u32,
    pub autoCtrlsValues: u32,
}

impl Copy for xcb_xkb_per_client_flags_request_t {}
impl Clone for xcb_xkb_per_client_flags_request_t {
    fn clone(&self) -> xcb_xkb_per_client_flags_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_per_client_flags_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_per_client_flags_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("pad0", &&self.pad0[..])
            .field("change", &self.change)
            .field("value", &self.value)
            .field("ctrlsToChange", &self.ctrlsToChange)
            .field("autoCtrls", &self.autoCtrls)
            .field("autoCtrlsValues", &self.autoCtrlsValues)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_per_client_flags_reply_t {
    pub response_type:   u8,
    pub deviceID:        u8,
    pub sequence:        u16,
    pub length:          u32,
    pub supported:       u32,
    pub value:           u32,
    pub autoCtrls:       u32,
    pub autoCtrlsValues: u32,
    pub pad0:            [u8; 8],
}

impl Copy for xcb_xkb_per_client_flags_reply_t {}
impl Clone for xcb_xkb_per_client_flags_reply_t {
    fn clone(&self) -> xcb_xkb_per_client_flags_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_per_client_flags_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_per_client_flags_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("supported", &self.supported)
            .field("value", &self.value)
            .field("autoCtrls", &self.autoCtrls)
            .field("autoCtrlsValues", &self.autoCtrlsValues)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_LIST_COMPONENTS: u8 = 22;

#[repr(C)]
pub struct xcb_xkb_list_components_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub maxNames:     u16,
}

impl Copy for xcb_xkb_list_components_request_t {}
impl Clone for xcb_xkb_list_components_request_t {
    fn clone(&self) -> xcb_xkb_list_components_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_list_components_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_list_components_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("maxNames", &self.maxNames)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_list_components_reply_t {
    pub response_type: u8,
    pub deviceID:      u8,
    pub sequence:      u16,
    pub length:        u32,
    pub nKeymaps:      u16,
    pub nKeycodes:     u16,
    pub nTypes:        u16,
    pub nCompatMaps:   u16,
    pub nSymbols:      u16,
    pub nGeometries:   u16,
    pub extra:         u16,
    pub pad0:          [u8; 10],
}
impl ::std::fmt::Debug for xcb_xkb_list_components_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_list_components_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("nKeymaps", &self.nKeymaps)
            .field("nKeycodes", &self.nKeycodes)
            .field("nTypes", &self.nTypes)
            .field("nCompatMaps", &self.nCompatMaps)
            .field("nSymbols", &self.nSymbols)
            .field("nGeometries", &self.nGeometries)
            .field("extra", &self.extra)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_GET_KBD_BY_NAME: u8 = 23;

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub need:         u16,
    pub want:         u16,
    pub load:         u8,
    pub pad0:         u8,
}

impl Copy for xcb_xkb_get_kbd_by_name_request_t {}
impl Clone for xcb_xkb_get_kbd_by_name_request_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_kbd_by_name_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_kbd_by_name_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("need", &self.need)
            .field("want", &self.want)
            .field("load", &self.load)
            .field("pad0", &self.pad0)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t {
    pub types:          _xcb_xkb_get_kbd_by_name_replies__types,
    pub compat_map:     _xcb_xkb_get_kbd_by_name_replies__compat_map,
    pub indicator_maps: _xcb_xkb_get_kbd_by_name_replies__indicator_maps,
    pub key_names:      _xcb_xkb_get_kbd_by_name_replies__key_names,
    pub geometry:       _xcb_xkb_get_kbd_by_name_replies__geometry,
}
impl ::std::fmt::Debug for xcb_xkb_get_kbd_by_name_replies_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_kbd_by_name_replies_t")
            .field("types", &self.types)
            .field("compat_map", &self.compat_map)
            .field("indicator_maps", &self.indicator_maps)
            .field("key_names", &self.key_names)
            .field("geometry", &self.geometry)
            .finish()
    }
}

#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__types {
    pub getmap_type:       u8,
    pub typeDeviceID:      u8,
    pub getmap_sequence:   u16,
    pub getmap_length:     u32,
    pub pad1:              [u8; 2],
    pub typeMinKeyCode:    xcb_keycode_t,
    pub typeMaxKeyCode:    xcb_keycode_t,
    pub present:           u16,
    pub firstType:         u8,
    pub nTypes:            u8,
    pub totalTypes:        u8,
    pub firstKeySym:       xcb_keycode_t,
    pub totalSyms:         u16,
    pub nKeySyms:          u8,
    pub firstKeyAction:    xcb_keycode_t,
    pub totalActions:      u16,
    pub nKeyActions:       u8,
    pub firstKeyBehavior:  xcb_keycode_t,
    pub nKeyBehaviors:     u8,
    pub totalKeyBehaviors: u8,
    pub firstKeyExplicit:  xcb_keycode_t,
    pub nKeyExplicit:      u8,
    pub totalKeyExplicit:  u8,
    pub firstModMapKey:    xcb_keycode_t,
    pub nModMapKeys:       u8,
    pub totalModMapKeys:   u8,
    pub firstVModMapKey:   xcb_keycode_t,
    pub nVModMapKeys:      u8,
    pub totalVModMapKeys:  u8,
    pub pad2:              u8,
    pub virtualMods:       u16,
    pub map:               xcb_xkb_get_kbd_by_name_replies_types_map_t,
}
impl ::std::fmt::Debug for _xcb_xkb_get_kbd_by_name_replies__types {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_xcb_xkb_get_kbd_by_name_replies__types")
            .field("getmap_type", &self.getmap_type)
            .field("typeDeviceID", &self.typeDeviceID)
            .field("getmap_sequence", &self.getmap_sequence)
            .field("getmap_length", &self.getmap_length)
            .field("pad1", &&self.pad1[..])
            .field("typeMinKeyCode", &self.typeMinKeyCode)
            .field("typeMaxKeyCode", &self.typeMaxKeyCode)
            .field("present", &self.present)
            .field("firstType", &self.firstType)
            .field("nTypes", &self.nTypes)
            .field("totalTypes", &self.totalTypes)
            .field("firstKeySym", &self.firstKeySym)
            .field("totalSyms", &self.totalSyms)
            .field("nKeySyms", &self.nKeySyms)
            .field("firstKeyAction", &self.firstKeyAction)
            .field("totalActions", &self.totalActions)
            .field("nKeyActions", &self.nKeyActions)
            .field("firstKeyBehavior", &self.firstKeyBehavior)
            .field("nKeyBehaviors", &self.nKeyBehaviors)
            .field("totalKeyBehaviors", &self.totalKeyBehaviors)
            .field("firstKeyExplicit", &self.firstKeyExplicit)
            .field("nKeyExplicit", &self.nKeyExplicit)
            .field("totalKeyExplicit", &self.totalKeyExplicit)
            .field("firstModMapKey", &self.firstModMapKey)
            .field("nModMapKeys", &self.nModMapKeys)
            .field("totalModMapKeys", &self.totalModMapKeys)
            .field("firstVModMapKey", &self.firstVModMapKey)
            .field("nVModMapKeys", &self.nVModMapKeys)
            .field("totalVModMapKeys", &self.totalVModMapKeys)
            .field("pad2", &self.pad2)
            .field("virtualMods", &self.virtualMods)
            .field("map", &self.map)
            .finish()
    }
}

#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__compat_map {
    pub compatmap_type:     u8,
    pub compatDeviceID:     u8,
    pub compatmap_sequence: u16,
    pub compatmap_length:   u32,
    pub groupsRtrn:         u8,
    pub pad3:               u8,
    pub firstSIRtrn:        u16,
    pub nSIRtrn:            u16,
    pub nTotalSI:           u16,
    pub pad4:               [u8; 16],
    pub si_rtrn:            *mut xcb_xkb_sym_interpret_t,
    pub group_rtrn:         *mut xcb_xkb_mod_def_t,
}
impl ::std::fmt::Debug for _xcb_xkb_get_kbd_by_name_replies__compat_map {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_xcb_xkb_get_kbd_by_name_replies__compat_map")
            .field("compatmap_type", &self.compatmap_type)
            .field("compatDeviceID", &self.compatDeviceID)
            .field("compatmap_sequence", &self.compatmap_sequence)
            .field("compatmap_length", &self.compatmap_length)
            .field("groupsRtrn", &self.groupsRtrn)
            .field("pad3", &self.pad3)
            .field("firstSIRtrn", &self.firstSIRtrn)
            .field("nSIRtrn", &self.nSIRtrn)
            .field("nTotalSI", &self.nTotalSI)
            .field("pad4", &&self.pad4[..])
            .field("si_rtrn", &self.si_rtrn)
            .field("group_rtrn", &self.group_rtrn)
            .finish()
    }
}

#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__indicator_maps {
    pub indicatormap_type:     u8,
    pub indicatorDeviceID:     u8,
    pub indicatormap_sequence: u16,
    pub indicatormap_length:   u32,
    pub which:                 u32,
    pub realIndicators:        u32,
    pub nIndicators:           u8,
    pub pad5:                  [u8; 15],
    pub maps:                  *mut xcb_xkb_indicator_map_t,
}
impl ::std::fmt::Debug for _xcb_xkb_get_kbd_by_name_replies__indicator_maps {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_xcb_xkb_get_kbd_by_name_replies__indicator_maps")
            .field("indicatormap_type", &self.indicatormap_type)
            .field("indicatorDeviceID", &self.indicatorDeviceID)
            .field("indicatormap_sequence", &self.indicatormap_sequence)
            .field("indicatormap_length", &self.indicatormap_length)
            .field("which", &self.which)
            .field("realIndicators", &self.realIndicators)
            .field("nIndicators", &self.nIndicators)
            .field("pad5", &&self.pad5[..])
            .field("maps", &self.maps)
            .finish()
    }
}

#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__key_names {
    pub keyname_type:     u8,
    pub keyDeviceID:      u8,
    pub keyname_sequence: u16,
    pub keyname_length:   u32,
    pub which:            u32,
    pub keyMinKeyCode:    xcb_keycode_t,
    pub keyMaxKeyCode:    xcb_keycode_t,
    pub nTypes:           u8,
    pub groupNames:       u8,
    pub virtualMods:      u16,
    pub firstKey:         xcb_keycode_t,
    pub nKeys:            u8,
    pub indicators:       u32,
    pub nRadioGroups:     u8,
    pub nKeyAliases:      u8,
    pub nKTLevels:        u16,
    pub pad6:             [u8; 4],
    pub valueList:        xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
}
impl ::std::fmt::Debug for _xcb_xkb_get_kbd_by_name_replies__key_names {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_xcb_xkb_get_kbd_by_name_replies__key_names")
            .field("keyname_type", &self.keyname_type)
            .field("keyDeviceID", &self.keyDeviceID)
            .field("keyname_sequence", &self.keyname_sequence)
            .field("keyname_length", &self.keyname_length)
            .field("which", &self.which)
            .field("keyMinKeyCode", &self.keyMinKeyCode)
            .field("keyMaxKeyCode", &self.keyMaxKeyCode)
            .field("nTypes", &self.nTypes)
            .field("groupNames", &self.groupNames)
            .field("virtualMods", &self.virtualMods)
            .field("firstKey", &self.firstKey)
            .field("nKeys", &self.nKeys)
            .field("indicators", &self.indicators)
            .field("nRadioGroups", &self.nRadioGroups)
            .field("nKeyAliases", &self.nKeyAliases)
            .field("nKTLevels", &self.nKTLevels)
            .field("pad6", &&self.pad6[..])
            .field("valueList", &self.valueList)
            .finish()
    }
}

#[repr(C)]
pub struct _xcb_xkb_get_kbd_by_name_replies__geometry {
    pub geometry_type:     u8,
    pub geometryDeviceID:  u8,
    pub geometry_sequence: u16,
    pub geometry_length:   u32,
    pub name:              xcb_atom_t,
    pub geometryFound:     u8,
    pub pad7:              u8,
    pub widthMM:           u16,
    pub heightMM:          u16,
    pub nProperties:       u16,
    pub nColors:           u16,
    pub nShapes:           u16,
    pub nSections:         u16,
    pub nDoodads:          u16,
    pub nKeyAliases:       u16,
    pub baseColorNdx:      u8,
    pub labelColorNdx:     u8,
    pub labelFont:         *mut xcb_xkb_counted_string_16_t,
}
impl ::std::fmt::Debug for _xcb_xkb_get_kbd_by_name_replies__geometry {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_xcb_xkb_get_kbd_by_name_replies__geometry")
            .field("geometry_type", &self.geometry_type)
            .field("geometryDeviceID", &self.geometryDeviceID)
            .field("geometry_sequence", &self.geometry_sequence)
            .field("geometry_length", &self.geometry_length)
            .field("name", &self.name)
            .field("geometryFound", &self.geometryFound)
            .field("pad7", &self.pad7)
            .field("widthMM", &self.widthMM)
            .field("heightMM", &self.heightMM)
            .field("nProperties", &self.nProperties)
            .field("nColors", &self.nColors)
            .field("nShapes", &self.nShapes)
            .field("nSections", &self.nSections)
            .field("nDoodads", &self.nDoodads)
            .field("nKeyAliases", &self.nKeyAliases)
            .field("baseColorNdx", &self.baseColorNdx)
            .field("labelColorNdx", &self.labelColorNdx)
            .field("labelFont", &self.labelFont)
            .finish()
    }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_types_map_t {
    pub types_rtrn:      *mut xcb_xkb_key_type_t,
    pub syms_rtrn:       *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count: *mut u8,
    pub acts_rtrn_acts:  *mut xcb_xkb_action_t,
    pub behaviors_rtrn:  *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn:      *mut u8,
    pub explicit_rtrn:   *mut xcb_xkb_set_explicit_t,
    pub modmap_rtrn:     *mut xcb_xkb_key_mod_map_t,
    pub vmodmap_rtrn:    *mut xcb_xkb_key_v_mod_map_t,
}
impl ::std::fmt::Debug for xcb_xkb_get_kbd_by_name_replies_types_map_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_kbd_by_name_replies_types_map_t")
            .field("types_rtrn", &self.types_rtrn)
            .field("syms_rtrn", &self.syms_rtrn)
            .field("acts_rtrn_count", &self.acts_rtrn_count)
            .field("acts_rtrn_acts", &self.acts_rtrn_acts)
            .field("behaviors_rtrn", &self.behaviors_rtrn)
            .field("vmods_rtrn", &self.vmods_rtrn)
            .field("explicit_rtrn", &self.explicit_rtrn)
            .field("modmap_rtrn", &self.modmap_rtrn)
            .field("vmodmap_rtrn", &self.vmodmap_rtrn)
            .finish()
    }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {
    pub keycodesName:    xcb_atom_t,
    pub geometryName:    xcb_atom_t,
    pub symbolsName:     xcb_atom_t,
    pub physSymbolsName: xcb_atom_t,
    pub typesName:       xcb_atom_t,
    pub compatName:      xcb_atom_t,
    pub typeNames:       *mut xcb_atom_t,
    pub nLevelsPerType:  *mut u8,
    pub ktLevelNames:    *mut xcb_atom_t,
    pub indicatorNames:  *mut xcb_atom_t,
    pub virtualModNames: *mut xcb_atom_t,
    pub groups:          *mut xcb_atom_t,
    pub keyNames:        *mut xcb_xkb_key_name_t,
    pub keyAliases:      *mut xcb_xkb_key_alias_t,
    pub radioGroupNames: *mut xcb_atom_t,
}
impl ::std::fmt::Debug for xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t")
            .field("keycodesName", &self.keycodesName)
            .field("geometryName", &self.geometryName)
            .field("symbolsName", &self.symbolsName)
            .field("physSymbolsName", &self.physSymbolsName)
            .field("typesName", &self.typesName)
            .field("compatName", &self.compatName)
            .field("typeNames", &self.typeNames)
            .field("nLevelsPerType", &self.nLevelsPerType)
            .field("ktLevelNames", &self.ktLevelNames)
            .field("indicatorNames", &self.indicatorNames)
            .field("virtualModNames", &self.virtualModNames)
            .field("groups", &self.groups)
            .field("keyNames", &self.keyNames)
            .field("keyAliases", &self.keyAliases)
            .field("radioGroupNames", &self.radioGroupNames)
            .finish()
    }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_reply_t {
    pub response_type: u8,
    pub deviceID:      u8,
    pub sequence:      u16,
    pub length:        u32,
    pub minKeyCode:    xcb_keycode_t,
    pub maxKeyCode:    xcb_keycode_t,
    pub loaded:        u8,
    pub newKeyboard:   u8,
    pub found:         u16,
    pub reported:      u16,
    pub pad0:          [u8; 16],
}
impl ::std::fmt::Debug for xcb_xkb_get_kbd_by_name_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_kbd_by_name_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("minKeyCode", &self.minKeyCode)
            .field("maxKeyCode", &self.maxKeyCode)
            .field("loaded", &self.loaded)
            .field("newKeyboard", &self.newKeyboard)
            .field("found", &self.found)
            .field("reported", &self.reported)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_GET_DEVICE_INFO: u8 = 24;

#[repr(C)]
pub struct xcb_xkb_get_device_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub deviceSpec:   xcb_xkb_device_spec_t,
    pub wanted:       u16,
    pub allButtons:   u8,
    pub firstButton:  u8,
    pub nButtons:     u8,
    pub pad0:         u8,
    pub ledClass:     xcb_xkb_led_class_spec_t,
    pub ledID:        xcb_xkb_id_spec_t,
}

impl Copy for xcb_xkb_get_device_info_request_t {}
impl Clone for xcb_xkb_get_device_info_request_t {
    fn clone(&self) -> xcb_xkb_get_device_info_request_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_get_device_info_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_device_info_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("wanted", &self.wanted)
            .field("allButtons", &self.allButtons)
            .field("firstButton", &self.firstButton)
            .field("nButtons", &self.nButtons)
            .field("pad0", &self.pad0)
            .field("ledClass", &self.ledClass)
            .field("ledID", &self.ledID)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_get_device_info_reply_t {
    pub response_type:  u8,
    pub deviceID:       u8,
    pub sequence:       u16,
    pub length:         u32,
    pub present:        u16,
    pub supported:      u16,
    pub unsupported:    u16,
    pub nDeviceLedFBs:  u16,
    pub firstBtnWanted: u8,
    pub nBtnsWanted:    u8,
    pub firstBtnRtrn:   u8,
    pub nBtnsRtrn:      u8,
    pub totalBtns:      u8,
    pub hasOwnState:    u8,
    pub dfltKbdFB:      u16,
    pub dfltLedFB:      u16,
    pub pad0:           [u8; 2],
    pub devType:        xcb_atom_t,
    pub nameLen:        u16,
}
impl ::std::fmt::Debug for xcb_xkb_get_device_info_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_get_device_info_reply_t")
            .field("response_type", &self.response_type)
            .field("deviceID", &self.deviceID)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("present", &self.present)
            .field("supported", &self.supported)
            .field("unsupported", &self.unsupported)
            .field("nDeviceLedFBs", &self.nDeviceLedFBs)
            .field("firstBtnWanted", &self.firstBtnWanted)
            .field("nBtnsWanted", &self.nBtnsWanted)
            .field("firstBtnRtrn", &self.firstBtnRtrn)
            .field("nBtnsRtrn", &self.nBtnsRtrn)
            .field("totalBtns", &self.totalBtns)
            .field("hasOwnState", &self.hasOwnState)
            .field("dfltKbdFB", &self.dfltKbdFB)
            .field("dfltLedFB", &self.dfltLedFB)
            .field("pad0", &&self.pad0[..])
            .field("devType", &self.devType)
            .field("nameLen", &self.nameLen)
            .finish()
    }
}

pub const XCB_XKB_SET_DEVICE_INFO: u8 = 25;

#[repr(C)]
pub struct xcb_xkb_set_device_info_request_t {
    pub major_opcode:  u8,
    pub minor_opcode:  u8,
    pub length:        u16,
    pub deviceSpec:    xcb_xkb_device_spec_t,
    pub firstBtn:      u8,
    pub nBtns:         u8,
    pub change:        u16,
    pub nDeviceLedFBs: u16,
}
impl ::std::fmt::Debug for xcb_xkb_set_device_info_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_device_info_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("deviceSpec", &self.deviceSpec)
            .field("firstBtn", &self.firstBtn)
            .field("nBtns", &self.nBtns)
            .field("change", &self.change)
            .field("nDeviceLedFBs", &self.nDeviceLedFBs)
            .finish()
    }
}

pub const XCB_XKB_SET_DEBUGGING_FLAGS: u8 = 101;

#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub msgLength:    u16,
    pub pad0:         [u8; 2],
    pub affectFlags:  u32,
    pub flags:        u32,
    pub affectCtrls:  u32,
    pub ctrls:        u32,
}
impl ::std::fmt::Debug for xcb_xkb_set_debugging_flags_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_debugging_flags_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("msgLength", &self.msgLength)
            .field("pad0", &&self.pad0[..])
            .field("affectFlags", &self.affectFlags)
            .field("flags", &self.flags)
            .field("affectCtrls", &self.affectCtrls)
            .field("ctrls", &self.ctrls)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_reply_t {
    pub response_type:  u8,
    pub pad0:           u8,
    pub sequence:       u16,
    pub length:         u32,
    pub currentFlags:   u32,
    pub currentCtrls:   u32,
    pub supportedFlags: u32,
    pub supportedCtrls: u32,
    pub pad1:           [u8; 8],
}

impl Copy for xcb_xkb_set_debugging_flags_reply_t {}
impl Clone for xcb_xkb_set_debugging_flags_reply_t {
    fn clone(&self) -> xcb_xkb_set_debugging_flags_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_set_debugging_flags_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_set_debugging_flags_reply_t")
            .field("response_type", &self.response_type)
            .field("pad0", &self.pad0)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("currentFlags", &self.currentFlags)
            .field("currentCtrls", &self.currentCtrls)
            .field("supportedFlags", &self.supportedFlags)
            .field("supportedCtrls", &self.supportedCtrls)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

pub const XCB_XKB_NEW_KEYBOARD_NOTIFY: u8 = 0;

#[repr(C)]
pub struct xcb_xkb_new_keyboard_notify_event_t {
    pub response_type: u8,
    pub xkbType:       u8,
    pub sequence:      u16,
    pub time:          xcb_timestamp_t,
    pub deviceID:      u8,
    pub oldDeviceID:   u8,
    pub minKeyCode:    xcb_keycode_t,
    pub maxKeyCode:    xcb_keycode_t,
    pub oldMinKeyCode: xcb_keycode_t,
    pub oldMaxKeyCode: xcb_keycode_t,
    pub requestMajor:  u8,
    pub requestMinor:  u8,
    pub changed:       u16,
    pub pad0:          [u8; 14],
}

impl Copy for xcb_xkb_new_keyboard_notify_event_t {}
impl Clone for xcb_xkb_new_keyboard_notify_event_t {
    fn clone(&self) -> xcb_xkb_new_keyboard_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_new_keyboard_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_new_keyboard_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("oldDeviceID", &self.oldDeviceID)
            .field("minKeyCode", &self.minKeyCode)
            .field("maxKeyCode", &self.maxKeyCode)
            .field("oldMinKeyCode", &self.oldMinKeyCode)
            .field("oldMaxKeyCode", &self.oldMaxKeyCode)
            .field("requestMajor", &self.requestMajor)
            .field("requestMinor", &self.requestMinor)
            .field("changed", &self.changed)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_MAP_NOTIFY: u8 = 1;

#[repr(C)]
pub struct xcb_xkb_map_notify_event_t {
    pub response_type:    u8,
    pub xkbType:          u8,
    pub sequence:         u16,
    pub time:             xcb_timestamp_t,
    pub deviceID:         u8,
    pub ptrBtnActions:    u8,
    pub changed:          u16,
    pub minKeyCode:       xcb_keycode_t,
    pub maxKeyCode:       xcb_keycode_t,
    pub firstType:        u8,
    pub nTypes:           u8,
    pub firstKeySym:      xcb_keycode_t,
    pub nKeySyms:         u8,
    pub firstKeyAct:      xcb_keycode_t,
    pub nKeyActs:         u8,
    pub firstKeyBehavior: xcb_keycode_t,
    pub nKeyBehavior:     u8,
    pub firstKeyExplicit: xcb_keycode_t,
    pub nKeyExplicit:     u8,
    pub firstModMapKey:   xcb_keycode_t,
    pub nModMapKeys:      u8,
    pub firstVModMapKey:  xcb_keycode_t,
    pub nVModMapKeys:     u8,
    pub virtualMods:      u16,
    pub pad0:             [u8; 2],
}

impl Copy for xcb_xkb_map_notify_event_t {}
impl Clone for xcb_xkb_map_notify_event_t {
    fn clone(&self) -> xcb_xkb_map_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_map_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_map_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("ptrBtnActions", &self.ptrBtnActions)
            .field("changed", &self.changed)
            .field("minKeyCode", &self.minKeyCode)
            .field("maxKeyCode", &self.maxKeyCode)
            .field("firstType", &self.firstType)
            .field("nTypes", &self.nTypes)
            .field("firstKeySym", &self.firstKeySym)
            .field("nKeySyms", &self.nKeySyms)
            .field("firstKeyAct", &self.firstKeyAct)
            .field("nKeyActs", &self.nKeyActs)
            .field("firstKeyBehavior", &self.firstKeyBehavior)
            .field("nKeyBehavior", &self.nKeyBehavior)
            .field("firstKeyExplicit", &self.firstKeyExplicit)
            .field("nKeyExplicit", &self.nKeyExplicit)
            .field("firstModMapKey", &self.firstModMapKey)
            .field("nModMapKeys", &self.nModMapKeys)
            .field("firstVModMapKey", &self.firstVModMapKey)
            .field("nVModMapKeys", &self.nVModMapKeys)
            .field("virtualMods", &self.virtualMods)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_STATE_NOTIFY: u8 = 2;

#[repr(C)]
pub struct xcb_xkb_state_notify_event_t {
    pub response_type:     u8,
    pub xkbType:           u8,
    pub sequence:          u16,
    pub time:              xcb_timestamp_t,
    pub deviceID:          u8,
    pub mods:              u8,
    pub baseMods:          u8,
    pub latchedMods:       u8,
    pub lockedMods:        u8,
    pub group:             u8,
    pub baseGroup:         i16,
    pub latchedGroup:      i16,
    pub lockedGroup:       u8,
    pub compatState:       u8,
    pub grabMods:          u8,
    pub compatGrabMods:    u8,
    pub lookupMods:        u8,
    pub compatLoockupMods: u8,
    pub ptrBtnState:       u16,
    pub changed:           u16,
    pub keycode:           xcb_keycode_t,
    pub eventType:         u8,
    pub requestMajor:      u8,
    pub requestMinor:      u8,
}

impl Copy for xcb_xkb_state_notify_event_t {}
impl Clone for xcb_xkb_state_notify_event_t {
    fn clone(&self) -> xcb_xkb_state_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_state_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_state_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("mods", &self.mods)
            .field("baseMods", &self.baseMods)
            .field("latchedMods", &self.latchedMods)
            .field("lockedMods", &self.lockedMods)
            .field("group", &self.group)
            .field("baseGroup", &self.baseGroup)
            .field("latchedGroup", &self.latchedGroup)
            .field("lockedGroup", &self.lockedGroup)
            .field("compatState", &self.compatState)
            .field("grabMods", &self.grabMods)
            .field("compatGrabMods", &self.compatGrabMods)
            .field("lookupMods", &self.lookupMods)
            .field("compatLoockupMods", &self.compatLoockupMods)
            .field("ptrBtnState", &self.ptrBtnState)
            .field("changed", &self.changed)
            .field("keycode", &self.keycode)
            .field("eventType", &self.eventType)
            .field("requestMajor", &self.requestMajor)
            .field("requestMinor", &self.requestMinor)
            .finish()
    }
}

pub const XCB_XKB_CONTROLS_NOTIFY: u8 = 3;

#[repr(C)]
pub struct xcb_xkb_controls_notify_event_t {
    pub response_type:         u8,
    pub xkbType:               u8,
    pub sequence:              u16,
    pub time:                  xcb_timestamp_t,
    pub deviceID:              u8,
    pub numGroups:             u8,
    pub pad0:                  [u8; 2],
    pub changedControls:       u32,
    pub enabledControls:       u32,
    pub enabledControlChanges: u32,
    pub keycode:               xcb_keycode_t,
    pub eventType:             u8,
    pub requestMajor:          u8,
    pub requestMinor:          u8,
    pub pad1:                  [u8; 4],
}

impl Copy for xcb_xkb_controls_notify_event_t {}
impl Clone for xcb_xkb_controls_notify_event_t {
    fn clone(&self) -> xcb_xkb_controls_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_controls_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_controls_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("numGroups", &self.numGroups)
            .field("pad0", &&self.pad0[..])
            .field("changedControls", &self.changedControls)
            .field("enabledControls", &self.enabledControls)
            .field("enabledControlChanges", &self.enabledControlChanges)
            .field("keycode", &self.keycode)
            .field("eventType", &self.eventType)
            .field("requestMajor", &self.requestMajor)
            .field("requestMinor", &self.requestMinor)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

pub const XCB_XKB_INDICATOR_STATE_NOTIFY: u8 = 4;

#[repr(C)]
pub struct xcb_xkb_indicator_state_notify_event_t {
    pub response_type: u8,
    pub xkbType:       u8,
    pub sequence:      u16,
    pub time:          xcb_timestamp_t,
    pub deviceID:      u8,
    pub pad0:          [u8; 3],
    pub state:         u32,
    pub stateChanged:  u32,
    pub pad1:          [u8; 12],
}

impl Copy for xcb_xkb_indicator_state_notify_event_t {}
impl Clone for xcb_xkb_indicator_state_notify_event_t {
    fn clone(&self) -> xcb_xkb_indicator_state_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_indicator_state_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_indicator_state_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("pad0", &&self.pad0[..])
            .field("state", &self.state)
            .field("stateChanged", &self.stateChanged)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

pub const XCB_XKB_INDICATOR_MAP_NOTIFY: u8 = 5;

#[repr(C)]
pub struct xcb_xkb_indicator_map_notify_event_t {
    pub response_type: u8,
    pub xkbType:       u8,
    pub sequence:      u16,
    pub time:          xcb_timestamp_t,
    pub deviceID:      u8,
    pub pad0:          [u8; 3],
    pub state:         u32,
    pub mapChanged:    u32,
    pub pad1:          [u8; 12],
}

impl Copy for xcb_xkb_indicator_map_notify_event_t {}
impl Clone for xcb_xkb_indicator_map_notify_event_t {
    fn clone(&self) -> xcb_xkb_indicator_map_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_indicator_map_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_indicator_map_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("pad0", &&self.pad0[..])
            .field("state", &self.state)
            .field("mapChanged", &self.mapChanged)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}

pub const XCB_XKB_NAMES_NOTIFY: u8 = 6;

#[repr(C)]
pub struct xcb_xkb_names_notify_event_t {
    pub response_type:      u8,
    pub xkbType:            u8,
    pub sequence:           u16,
    pub time:               xcb_timestamp_t,
    pub deviceID:           u8,
    pub pad0:               u8,
    pub changed:            u16,
    pub firstType:          u8,
    pub nTypes:             u8,
    pub firstLevelName:     u8,
    pub nLevelNames:        u8,
    pub pad1:               u8,
    pub nRadioGroups:       u8,
    pub nKeyAliases:        u8,
    pub changedGroupNames:  u8,
    pub changedVirtualMods: u16,
    pub firstKey:           xcb_keycode_t,
    pub nKeys:              u8,
    pub changedIndicators:  u32,
    pub pad2:               [u8; 4],
}

impl Copy for xcb_xkb_names_notify_event_t {}
impl Clone for xcb_xkb_names_notify_event_t {
    fn clone(&self) -> xcb_xkb_names_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_names_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_names_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("pad0", &self.pad0)
            .field("changed", &self.changed)
            .field("firstType", &self.firstType)
            .field("nTypes", &self.nTypes)
            .field("firstLevelName", &self.firstLevelName)
            .field("nLevelNames", &self.nLevelNames)
            .field("pad1", &self.pad1)
            .field("nRadioGroups", &self.nRadioGroups)
            .field("nKeyAliases", &self.nKeyAliases)
            .field("changedGroupNames", &self.changedGroupNames)
            .field("changedVirtualMods", &self.changedVirtualMods)
            .field("firstKey", &self.firstKey)
            .field("nKeys", &self.nKeys)
            .field("changedIndicators", &self.changedIndicators)
            .field("pad2", &&self.pad2[..])
            .finish()
    }
}

pub const XCB_XKB_COMPAT_MAP_NOTIFY: u8 = 7;

#[repr(C)]
pub struct xcb_xkb_compat_map_notify_event_t {
    pub response_type: u8,
    pub xkbType:       u8,
    pub sequence:      u16,
    pub time:          xcb_timestamp_t,
    pub deviceID:      u8,
    pub changedGroups: u8,
    pub firstSI:       u16,
    pub nSI:           u16,
    pub nTotalSI:      u16,
    pub pad0:          [u8; 16],
}

impl Copy for xcb_xkb_compat_map_notify_event_t {}
impl Clone for xcb_xkb_compat_map_notify_event_t {
    fn clone(&self) -> xcb_xkb_compat_map_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_compat_map_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_compat_map_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("changedGroups", &self.changedGroups)
            .field("firstSI", &self.firstSI)
            .field("nSI", &self.nSI)
            .field("nTotalSI", &self.nTotalSI)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_BELL_NOTIFY: u8 = 8;

#[repr(C)]
pub struct xcb_xkb_bell_notify_event_t {
    pub response_type: u8,
    pub xkbType:       u8,
    pub sequence:      u16,
    pub time:          xcb_timestamp_t,
    pub deviceID:      u8,
    pub bellClass:     u8,
    pub bellID:        u8,
    pub percent:       u8,
    pub pitch:         u16,
    pub duration:      u16,
    pub name:          xcb_atom_t,
    pub window:        xcb_window_t,
    pub eventOnly:     u8,
    pub pad0:          [u8; 7],
}

impl Copy for xcb_xkb_bell_notify_event_t {}
impl Clone for xcb_xkb_bell_notify_event_t {
    fn clone(&self) -> xcb_xkb_bell_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_bell_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_bell_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("bellClass", &self.bellClass)
            .field("bellID", &self.bellID)
            .field("percent", &self.percent)
            .field("pitch", &self.pitch)
            .field("duration", &self.duration)
            .field("name", &self.name)
            .field("window", &self.window)
            .field("eventOnly", &self.eventOnly)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_ACTION_MESSAGE: u8 = 9;

#[repr(C)]
pub struct xcb_xkb_action_message_event_t {
    pub response_type:   u8,
    pub xkbType:         u8,
    pub sequence:        u16,
    pub time:            xcb_timestamp_t,
    pub deviceID:        u8,
    pub keycode:         xcb_keycode_t,
    pub press:           u8,
    pub keyEventFollows: u8,
    pub mods:            u8,
    pub group:           u8,
    pub message:         [xcb_xkb_string8_t; 8],
    pub pad0:            [u8; 10],
}

impl Copy for xcb_xkb_action_message_event_t {}
impl Clone for xcb_xkb_action_message_event_t {
    fn clone(&self) -> xcb_xkb_action_message_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_action_message_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_action_message_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("keycode", &self.keycode)
            .field("press", &self.press)
            .field("keyEventFollows", &self.keyEventFollows)
            .field("mods", &self.mods)
            .field("group", &self.group)
            .field("message", &&self.message[..])
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_ACCESS_X_NOTIFY: u8 = 10;

#[repr(C)]
pub struct xcb_xkb_access_x_notify_event_t {
    pub response_type: u8,
    pub xkbType:       u8,
    pub sequence:      u16,
    pub time:          xcb_timestamp_t,
    pub deviceID:      u8,
    pub keycode:       xcb_keycode_t,
    pub detailt:       u16,
    pub slowKeysDelay: u16,
    pub debounceDelay: u16,
    pub pad0:          [u8; 16],
}

impl Copy for xcb_xkb_access_x_notify_event_t {}
impl Clone for xcb_xkb_access_x_notify_event_t {
    fn clone(&self) -> xcb_xkb_access_x_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_access_x_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_access_x_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("keycode", &self.keycode)
            .field("detailt", &self.detailt)
            .field("slowKeysDelay", &self.slowKeysDelay)
            .field("debounceDelay", &self.debounceDelay)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}

pub const XCB_XKB_EXTENSION_DEVICE_NOTIFY: u8 = 11;

#[repr(C)]
pub struct xcb_xkb_extension_device_notify_event_t {
    pub response_type: u8,
    pub xkbType:       u8,
    pub sequence:      u16,
    pub time:          xcb_timestamp_t,
    pub deviceID:      u8,
    pub pad0:          u8,
    pub reason:        u16,
    pub ledClass:      u16,
    pub ledID:         u16,
    pub ledsDefined:   u32,
    pub ledState:      u32,
    pub firstButton:   u8,
    pub nButtons:      u8,
    pub supported:     u16,
    pub unsupported:   u16,
    pub pad1:          [u8; 2],
}

impl Copy for xcb_xkb_extension_device_notify_event_t {}
impl Clone for xcb_xkb_extension_device_notify_event_t {
    fn clone(&self) -> xcb_xkb_extension_device_notify_event_t { *self }
}
impl ::std::fmt::Debug for xcb_xkb_extension_device_notify_event_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_xkb_extension_device_notify_event_t")
            .field("response_type", &self.response_type)
            .field("xkbType", &self.xkbType)
            .field("sequence", &self.sequence)
            .field("time", &self.time)
            .field("deviceID", &self.deviceID)
            .field("pad0", &self.pad0)
            .field("reason", &self.reason)
            .field("ledClass", &self.ledClass)
            .field("ledID", &self.ledID)
            .field("ledsDefined", &self.ledsDefined)
            .field("ledState", &self.ledState)
            .field("firstButton", &self.firstButton)
            .field("nButtons", &self.nButtons)
            .field("supported", &self.supported)
            .field("unsupported", &self.unsupported)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}


#[link(name="xcb-xkb")]
extern {

    pub static mut xcb_xkb_id: xcb_extension_t;

    pub fn xcb_xkb_device_spec_next (i: *mut xcb_xkb_device_spec_iterator_t);

    pub fn xcb_xkb_device_spec_end (i: *mut xcb_xkb_device_spec_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_led_class_spec_next (i: *mut xcb_xkb_led_class_spec_iterator_t);

    pub fn xcb_xkb_led_class_spec_end (i: *mut xcb_xkb_led_class_spec_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_bell_class_spec_next (i: *mut xcb_xkb_bell_class_spec_iterator_t);

    pub fn xcb_xkb_bell_class_spec_end (i: *mut xcb_xkb_bell_class_spec_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_id_spec_next (i: *mut xcb_xkb_id_spec_iterator_t);

    pub fn xcb_xkb_id_spec_end (i: *mut xcb_xkb_id_spec_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_indicator_map_next (i: *mut xcb_xkb_indicator_map_iterator_t);

    pub fn xcb_xkb_indicator_map_end (i: *mut xcb_xkb_indicator_map_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_mod_def_next (i: *mut xcb_xkb_mod_def_iterator_t);

    pub fn xcb_xkb_mod_def_end (i: *mut xcb_xkb_mod_def_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_name_next (i: *mut xcb_xkb_key_name_iterator_t);

    pub fn xcb_xkb_key_name_end (i: *mut xcb_xkb_key_name_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_alias_next (i: *mut xcb_xkb_key_alias_iterator_t);

    pub fn xcb_xkb_key_alias_end (i: *mut xcb_xkb_key_alias_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_counted_string_16_string (R: *const xcb_xkb_counted_string_16_t)
            -> *mut c_char;

    pub fn xcb_xkb_counted_string_16_string_length (R: *const xcb_xkb_counted_string_16_t)
            -> c_int;

    pub fn xcb_xkb_counted_string_16_string_end (R: *const xcb_xkb_counted_string_16_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_counted_string_16_alignment_pad (R: *const xcb_xkb_counted_string_16_t)
            -> *mut c_void;

    pub fn xcb_xkb_counted_string_16_alignment_pad_length (R: *const xcb_xkb_counted_string_16_t)
            -> c_int;

    pub fn xcb_xkb_counted_string_16_alignment_pad_end (R: *const xcb_xkb_counted_string_16_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_counted_string_16_next (i: *mut xcb_xkb_counted_string_16_iterator_t);

    pub fn xcb_xkb_counted_string_16_end (i: *mut xcb_xkb_counted_string_16_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_kt_map_entry_next (i: *mut xcb_xkb_kt_map_entry_iterator_t);

    pub fn xcb_xkb_kt_map_entry_end (i: *mut xcb_xkb_kt_map_entry_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_type_map (R: *const xcb_xkb_key_type_t)
            -> *mut xcb_xkb_kt_map_entry_t;

    pub fn xcb_xkb_key_type_map_length (R: *const xcb_xkb_key_type_t)
            -> c_int;

    pub fn xcb_xkb_key_type_map_iterator (R: *const xcb_xkb_key_type_t)
            -> xcb_xkb_kt_map_entry_iterator_t;

    pub fn xcb_xkb_key_type_preserve (R: *const xcb_xkb_key_type_t)
            -> *mut xcb_xkb_mod_def_t;

    pub fn xcb_xkb_key_type_preserve_length (R: *const xcb_xkb_key_type_t)
            -> c_int;

    pub fn xcb_xkb_key_type_preserve_iterator (R: *const xcb_xkb_key_type_t)
            -> xcb_xkb_mod_def_iterator_t;

    pub fn xcb_xkb_key_type_next (i: *mut xcb_xkb_key_type_iterator_t);

    pub fn xcb_xkb_key_type_end (i: *mut xcb_xkb_key_type_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_sym_map_syms (R: *const xcb_xkb_key_sym_map_t)
            -> *mut xcb_keysym_t;

    pub fn xcb_xkb_key_sym_map_syms_length (R: *const xcb_xkb_key_sym_map_t)
            -> c_int;

    pub fn xcb_xkb_key_sym_map_syms_end (R: *const xcb_xkb_key_sym_map_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_sym_map_next (i: *mut xcb_xkb_key_sym_map_iterator_t);

    pub fn xcb_xkb_key_sym_map_end (i: *mut xcb_xkb_key_sym_map_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_common_behavior_next (i: *mut xcb_xkb_common_behavior_iterator_t);

    pub fn xcb_xkb_common_behavior_end (i: *mut xcb_xkb_common_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_default_behavior_next (i: *mut xcb_xkb_default_behavior_iterator_t);

    pub fn xcb_xkb_default_behavior_end (i: *mut xcb_xkb_default_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_lock_behavior_next (i: *mut xcb_xkb_lock_behavior_iterator_t);

    pub fn xcb_xkb_lock_behavior_end (i: *mut xcb_xkb_lock_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_radio_group_behavior_next (i: *mut xcb_xkb_radio_group_behavior_iterator_t);

    pub fn xcb_xkb_radio_group_behavior_end (i: *mut xcb_xkb_radio_group_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_overlay_behavior_next (i: *mut xcb_xkb_overlay_behavior_iterator_t);

    pub fn xcb_xkb_overlay_behavior_end (i: *mut xcb_xkb_overlay_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_permament_lock_behavior_next (i: *mut xcb_xkb_permament_lock_behavior_iterator_t);

    pub fn xcb_xkb_permament_lock_behavior_end (i: *mut xcb_xkb_permament_lock_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_permament_radio_group_behavior_next (i: *mut xcb_xkb_permament_radio_group_behavior_iterator_t);

    pub fn xcb_xkb_permament_radio_group_behavior_end (i: *mut xcb_xkb_permament_radio_group_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_permament_overlay_behavior_next (i: *mut xcb_xkb_permament_overlay_behavior_iterator_t);

    pub fn xcb_xkb_permament_overlay_behavior_end (i: *mut xcb_xkb_permament_overlay_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_behavior_next (i: *mut xcb_xkb_behavior_iterator_t);

    pub fn xcb_xkb_behavior_end (i: *mut xcb_xkb_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_behavior_next (i: *mut xcb_xkb_set_behavior_iterator_t);

    pub fn xcb_xkb_set_behavior_end (i: *mut xcb_xkb_set_behavior_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_explicit_next (i: *mut xcb_xkb_set_explicit_iterator_t);

    pub fn xcb_xkb_set_explicit_end (i: *mut xcb_xkb_set_explicit_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_mod_map_next (i: *mut xcb_xkb_key_mod_map_iterator_t);

    pub fn xcb_xkb_key_mod_map_end (i: *mut xcb_xkb_key_mod_map_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_v_mod_map_next (i: *mut xcb_xkb_key_v_mod_map_iterator_t);

    pub fn xcb_xkb_key_v_mod_map_end (i: *mut xcb_xkb_key_v_mod_map_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_kt_set_map_entry_next (i: *mut xcb_xkb_kt_set_map_entry_iterator_t);

    pub fn xcb_xkb_kt_set_map_entry_end (i: *mut xcb_xkb_kt_set_map_entry_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_key_type_entries (R: *const xcb_xkb_set_key_type_t)
            -> *mut xcb_xkb_kt_set_map_entry_t;

    pub fn xcb_xkb_set_key_type_entries_length (R: *const xcb_xkb_set_key_type_t)
            -> c_int;

    pub fn xcb_xkb_set_key_type_entries_iterator (R: *const xcb_xkb_set_key_type_t)
            -> xcb_xkb_kt_set_map_entry_iterator_t;

    pub fn xcb_xkb_set_key_type_preserve_entries (R: *const xcb_xkb_set_key_type_t)
            -> *mut xcb_xkb_kt_set_map_entry_t;

    pub fn xcb_xkb_set_key_type_preserve_entries_length (R: *const xcb_xkb_set_key_type_t)
            -> c_int;

    pub fn xcb_xkb_set_key_type_preserve_entries_iterator (R: *const xcb_xkb_set_key_type_t)
            -> xcb_xkb_kt_set_map_entry_iterator_t;

    pub fn xcb_xkb_set_key_type_next (i: *mut xcb_xkb_set_key_type_iterator_t);

    pub fn xcb_xkb_set_key_type_end (i: *mut xcb_xkb_set_key_type_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_string8_next (i: *mut xcb_xkb_string8_iterator_t);

    pub fn xcb_xkb_string8_end (i: *mut xcb_xkb_string8_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_outline_points (R: *const xcb_xkb_outline_t)
            -> *mut xcb_point_t;

    pub fn xcb_xkb_outline_points_length (R: *const xcb_xkb_outline_t)
            -> c_int;

    pub fn xcb_xkb_outline_points_iterator (R: *const xcb_xkb_outline_t)
            -> xcb_point_iterator_t;

    pub fn xcb_xkb_outline_next (i: *mut xcb_xkb_outline_iterator_t);

    pub fn xcb_xkb_outline_end (i: *mut xcb_xkb_outline_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_shape_outlines_length (R: *const xcb_xkb_shape_t)
            -> c_int;

    pub fn xcb_xkb_shape_outlines_iterator<'a> (R: *const xcb_xkb_shape_t)
            -> xcb_xkb_outline_iterator_t<'a>;

    pub fn xcb_xkb_shape_next (i: *mut xcb_xkb_shape_iterator_t);

    pub fn xcb_xkb_shape_end (i: *mut xcb_xkb_shape_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_key_next (i: *mut xcb_xkb_key_iterator_t);

    pub fn xcb_xkb_key_end (i: *mut xcb_xkb_key_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_overlay_key_next (i: *mut xcb_xkb_overlay_key_iterator_t);

    pub fn xcb_xkb_overlay_key_end (i: *mut xcb_xkb_overlay_key_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_overlay_row_keys (R: *const xcb_xkb_overlay_row_t)
            -> *mut xcb_xkb_overlay_key_t;

    pub fn xcb_xkb_overlay_row_keys_length (R: *const xcb_xkb_overlay_row_t)
            -> c_int;

    pub fn xcb_xkb_overlay_row_keys_iterator<'a> (R: *const xcb_xkb_overlay_row_t)
            -> xcb_xkb_overlay_key_iterator_t<'a>;

    pub fn xcb_xkb_overlay_row_next (i: *mut xcb_xkb_overlay_row_iterator_t);

    pub fn xcb_xkb_overlay_row_end (i: *mut xcb_xkb_overlay_row_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_overlay_rows_length (R: *const xcb_xkb_overlay_t)
            -> c_int;

    pub fn xcb_xkb_overlay_rows_iterator<'a> (R: *const xcb_xkb_overlay_t)
            -> xcb_xkb_overlay_row_iterator_t<'a>;

    pub fn xcb_xkb_overlay_next (i: *mut xcb_xkb_overlay_iterator_t);

    pub fn xcb_xkb_overlay_end (i: *mut xcb_xkb_overlay_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_row_keys (R: *const xcb_xkb_row_t)
            -> *mut xcb_xkb_key_t;

    pub fn xcb_xkb_row_keys_length (R: *const xcb_xkb_row_t)
            -> c_int;

    pub fn xcb_xkb_row_keys_iterator<'a> (R: *const xcb_xkb_row_t)
            -> xcb_xkb_key_iterator_t<'a>;

    pub fn xcb_xkb_row_next (i: *mut xcb_xkb_row_iterator_t);

    pub fn xcb_xkb_row_end (i: *mut xcb_xkb_row_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_listing_string (R: *const xcb_xkb_listing_t)
            -> *mut xcb_xkb_string8_t;

    pub fn xcb_xkb_listing_string_length (R: *const xcb_xkb_listing_t)
            -> c_int;

    pub fn xcb_xkb_listing_string_end (R: *const xcb_xkb_listing_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_listing_next (i: *mut xcb_xkb_listing_iterator_t);

    pub fn xcb_xkb_listing_end (i: *mut xcb_xkb_listing_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_device_led_info_names (R: *const xcb_xkb_device_led_info_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_device_led_info_names_length (R: *const xcb_xkb_device_led_info_t)
            -> c_int;

    pub fn xcb_xkb_device_led_info_names_end (R: *const xcb_xkb_device_led_info_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_device_led_info_maps (R: *const xcb_xkb_device_led_info_t)
            -> *mut xcb_xkb_indicator_map_t;

    pub fn xcb_xkb_device_led_info_maps_length (R: *const xcb_xkb_device_led_info_t)
            -> c_int;

    pub fn xcb_xkb_device_led_info_maps_iterator (R: *const xcb_xkb_device_led_info_t)
            -> xcb_xkb_indicator_map_iterator_t;

    pub fn xcb_xkb_device_led_info_next (i: *mut xcb_xkb_device_led_info_iterator_t);

    pub fn xcb_xkb_device_led_info_end (i: *mut xcb_xkb_device_led_info_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_no_action_next (i: *mut xcb_xkb_sa_no_action_iterator_t);

    pub fn xcb_xkb_sa_no_action_end (i: *mut xcb_xkb_sa_no_action_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_set_mods_next (i: *mut xcb_xkb_sa_set_mods_iterator_t);

    pub fn xcb_xkb_sa_set_mods_end (i: *mut xcb_xkb_sa_set_mods_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_latch_mods_next (i: *mut xcb_xkb_sa_latch_mods_iterator_t);

    pub fn xcb_xkb_sa_latch_mods_end (i: *mut xcb_xkb_sa_latch_mods_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_mods_next (i: *mut xcb_xkb_sa_lock_mods_iterator_t);

    pub fn xcb_xkb_sa_lock_mods_end (i: *mut xcb_xkb_sa_lock_mods_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_set_group_next (i: *mut xcb_xkb_sa_set_group_iterator_t);

    pub fn xcb_xkb_sa_set_group_end (i: *mut xcb_xkb_sa_set_group_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_latch_group_next (i: *mut xcb_xkb_sa_latch_group_iterator_t);

    pub fn xcb_xkb_sa_latch_group_end (i: *mut xcb_xkb_sa_latch_group_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_group_next (i: *mut xcb_xkb_sa_lock_group_iterator_t);

    pub fn xcb_xkb_sa_lock_group_end (i: *mut xcb_xkb_sa_lock_group_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_move_ptr_next (i: *mut xcb_xkb_sa_move_ptr_iterator_t);

    pub fn xcb_xkb_sa_move_ptr_end (i: *mut xcb_xkb_sa_move_ptr_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_ptr_btn_next (i: *mut xcb_xkb_sa_ptr_btn_iterator_t);

    pub fn xcb_xkb_sa_ptr_btn_end (i: *mut xcb_xkb_sa_ptr_btn_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_ptr_btn_next (i: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t);

    pub fn xcb_xkb_sa_lock_ptr_btn_end (i: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_set_ptr_dflt_next (i: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t);

    pub fn xcb_xkb_sa_set_ptr_dflt_end (i: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_iso_lock_next (i: *mut xcb_xkb_sa_iso_lock_iterator_t);

    pub fn xcb_xkb_sa_iso_lock_end (i: *mut xcb_xkb_sa_iso_lock_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_terminate_next (i: *mut xcb_xkb_sa_terminate_iterator_t);

    pub fn xcb_xkb_sa_terminate_end (i: *mut xcb_xkb_sa_terminate_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_switch_screen_next (i: *mut xcb_xkb_sa_switch_screen_iterator_t);

    pub fn xcb_xkb_sa_switch_screen_end (i: *mut xcb_xkb_sa_switch_screen_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_set_controls_next (i: *mut xcb_xkb_sa_set_controls_iterator_t);

    pub fn xcb_xkb_sa_set_controls_end (i: *mut xcb_xkb_sa_set_controls_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_controls_next (i: *mut xcb_xkb_sa_lock_controls_iterator_t);

    pub fn xcb_xkb_sa_lock_controls_end (i: *mut xcb_xkb_sa_lock_controls_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_action_message_next (i: *mut xcb_xkb_sa_action_message_iterator_t);

    pub fn xcb_xkb_sa_action_message_end (i: *mut xcb_xkb_sa_action_message_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_redirect_key_next (i: *mut xcb_xkb_sa_redirect_key_iterator_t);

    pub fn xcb_xkb_sa_redirect_key_end (i: *mut xcb_xkb_sa_redirect_key_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_device_btn_next (i: *mut xcb_xkb_sa_device_btn_iterator_t);

    pub fn xcb_xkb_sa_device_btn_end (i: *mut xcb_xkb_sa_device_btn_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_lock_device_btn_next (i: *mut xcb_xkb_sa_lock_device_btn_iterator_t);

    pub fn xcb_xkb_sa_lock_device_btn_end (i: *mut xcb_xkb_sa_lock_device_btn_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sa_device_valuator_next (i: *mut xcb_xkb_sa_device_valuator_iterator_t);

    pub fn xcb_xkb_sa_device_valuator_end (i: *mut xcb_xkb_sa_device_valuator_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_si_action_next (i: *mut xcb_xkb_si_action_iterator_t);

    pub fn xcb_xkb_si_action_end (i: *mut xcb_xkb_si_action_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_sym_interpret_next (i: *mut xcb_xkb_sym_interpret_iterator_t);

    pub fn xcb_xkb_sym_interpret_end (i: *mut xcb_xkb_sym_interpret_iterator_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_action_next (i: *mut xcb_xkb_action_iterator_t);

    pub fn xcb_xkb_action_end (i: *mut xcb_xkb_action_iterator_t)
            -> xcb_generic_iterator_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_use_extension_reply (c:      *mut xcb_connection_t,
                                        cookie: xcb_xkb_use_extension_cookie_t,
                                        error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_use_extension_reply_t;

    pub fn xcb_xkb_use_extension (c:           *mut xcb_connection_t,
                                  wantedMajor: u16,
                                  wantedMinor: u16)
            -> xcb_xkb_use_extension_cookie_t;

    pub fn xcb_xkb_use_extension_unchecked (c:           *mut xcb_connection_t,
                                            wantedMajor: u16,
                                            wantedMinor: u16)
            -> xcb_xkb_use_extension_cookie_t;

    pub fn xcb_xkb_select_events (c:           *mut xcb_connection_t,
                                  deviceSpec:  xcb_xkb_device_spec_t,
                                  affectWhich: u16,
                                  clear:       u16,
                                  selectAll:   u16,
                                  affectMap:   u16,
                                  map:         u16,
                                  details:     *const xcb_xkb_select_events_details_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_select_events_checked (c:           *mut xcb_connection_t,
                                          deviceSpec:  xcb_xkb_device_spec_t,
                                          affectWhich: u16,
                                          clear:       u16,
                                          selectAll:   u16,
                                          affectMap:   u16,
                                          map:         u16,
                                          details:     *const xcb_xkb_select_events_details_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_bell (c:          *mut xcb_connection_t,
                         deviceSpec: xcb_xkb_device_spec_t,
                         bellClass:  xcb_xkb_bell_class_spec_t,
                         bellID:     xcb_xkb_id_spec_t,
                         percent:    i8,
                         forceSound: u8,
                         eventOnly:  u8,
                         pitch:      i16,
                         duration:   i16,
                         name:       xcb_atom_t,
                         window:     xcb_window_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_bell_checked (c:          *mut xcb_connection_t,
                                 deviceSpec: xcb_xkb_device_spec_t,
                                 bellClass:  xcb_xkb_bell_class_spec_t,
                                 bellID:     xcb_xkb_id_spec_t,
                                 percent:    i8,
                                 forceSound: u8,
                                 eventOnly:  u8,
                                 pitch:      i16,
                                 duration:   i16,
                                 name:       xcb_atom_t,
                                 window:     xcb_window_t)
            -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_state_reply (c:      *mut xcb_connection_t,
                                    cookie: xcb_xkb_get_state_cookie_t,
                                    error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_state_reply_t;

    pub fn xcb_xkb_get_state (c:          *mut xcb_connection_t,
                              deviceSpec: xcb_xkb_device_spec_t)
            -> xcb_xkb_get_state_cookie_t;

    pub fn xcb_xkb_get_state_unchecked (c:          *mut xcb_connection_t,
                                        deviceSpec: xcb_xkb_device_spec_t)
            -> xcb_xkb_get_state_cookie_t;

    pub fn xcb_xkb_latch_lock_state (c:                *mut xcb_connection_t,
                                     deviceSpec:       xcb_xkb_device_spec_t,
                                     affectModLocks:   u8,
                                     modLocks:         u8,
                                     lockGroup:        u8,
                                     groupLock:        u8,
                                     affectModLatches: u8,
                                     latchGroup:       u8,
                                     groupLatch:       u16)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_latch_lock_state_checked (c:                *mut xcb_connection_t,
                                             deviceSpec:       xcb_xkb_device_spec_t,
                                             affectModLocks:   u8,
                                             modLocks:         u8,
                                             lockGroup:        u8,
                                             groupLock:        u8,
                                             affectModLatches: u8,
                                             latchGroup:       u8,
                                             groupLatch:       u16)
            -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_controls_reply (c:      *mut xcb_connection_t,
                                       cookie: xcb_xkb_get_controls_cookie_t,
                                       error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_controls_reply_t;

    pub fn xcb_xkb_get_controls (c:          *mut xcb_connection_t,
                                 deviceSpec: xcb_xkb_device_spec_t)
            -> xcb_xkb_get_controls_cookie_t;

    pub fn xcb_xkb_get_controls_unchecked (c:          *mut xcb_connection_t,
                                           deviceSpec: xcb_xkb_device_spec_t)
            -> xcb_xkb_get_controls_cookie_t;

    pub fn xcb_xkb_set_controls (c:                           *mut xcb_connection_t,
                                 deviceSpec:                  xcb_xkb_device_spec_t,
                                 affectInternalRealMods:      u8,
                                 internalRealMods:            u8,
                                 affectIgnoreLockRealMods:    u8,
                                 ignoreLockRealMods:          u8,
                                 affectInternalVirtualMods:   u16,
                                 internalVirtualMods:         u16,
                                 affectIgnoreLockVirtualMods: u16,
                                 ignoreLockVirtualMods:       u16,
                                 mouseKeysDfltBtn:            u8,
                                 groupsWrap:                  u8,
                                 accessXOptions:              u16,
                                 affectEnabledControls:       u32,
                                 enabledControls:             u32,
                                 changeControls:              u32,
                                 repeatDelay:                 u16,
                                 repeatInterval:              u16,
                                 slowKeysDelay:               u16,
                                 debounceDelay:               u16,
                                 mouseKeysDelay:              u16,
                                 mouseKeysInterval:           u16,
                                 mouseKeysTimeToMax:          u16,
                                 mouseKeysMaxSpeed:           u16,
                                 mouseKeysCurve:              i16,
                                 accessXTimeout:              u16,
                                 accessXTimeoutMask:          u32,
                                 accessXTimeoutValues:        u32,
                                 accessXTimeoutOptionsMask:   u16,
                                 accessXTimeoutOptionsValues: u16,
                                 perKeyRepeat:                *const u8)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_controls_checked (c:                           *mut xcb_connection_t,
                                         deviceSpec:                  xcb_xkb_device_spec_t,
                                         affectInternalRealMods:      u8,
                                         internalRealMods:            u8,
                                         affectIgnoreLockRealMods:    u8,
                                         ignoreLockRealMods:          u8,
                                         affectInternalVirtualMods:   u16,
                                         internalVirtualMods:         u16,
                                         affectIgnoreLockVirtualMods: u16,
                                         ignoreLockVirtualMods:       u16,
                                         mouseKeysDfltBtn:            u8,
                                         groupsWrap:                  u8,
                                         accessXOptions:              u16,
                                         affectEnabledControls:       u32,
                                         enabledControls:             u32,
                                         changeControls:              u32,
                                         repeatDelay:                 u16,
                                         repeatInterval:              u16,
                                         slowKeysDelay:               u16,
                                         debounceDelay:               u16,
                                         mouseKeysDelay:              u16,
                                         mouseKeysInterval:           u16,
                                         mouseKeysTimeToMax:          u16,
                                         mouseKeysMaxSpeed:           u16,
                                         mouseKeysCurve:              i16,
                                         accessXTimeout:              u16,
                                         accessXTimeoutMask:          u32,
                                         accessXTimeoutValues:        u32,
                                         accessXTimeoutOptionsMask:   u16,
                                         accessXTimeoutOptionsValues: u16,
                                         perKeyRepeat:                *const u8)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_get_map_map_types_rtrn_length (R: *const xcb_xkb_get_map_reply_t,
                                                  S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_types_rtrn_iterator<'a> (R: *const xcb_xkb_get_map_reply_t,
                                                        S: *const xcb_xkb_get_map_map_t)
            -> xcb_xkb_key_type_iterator_t<'a>;

    pub fn xcb_xkb_get_map_map_syms_rtrn_length (R: *const xcb_xkb_get_map_reply_t,
                                                 S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_syms_rtrn_iterator<'a> (R: *const xcb_xkb_get_map_reply_t,
                                                       S: *const xcb_xkb_get_map_map_t)
            -> xcb_xkb_key_sym_map_iterator_t<'a>;

    pub fn xcb_xkb_get_map_map_acts_rtrn_count (S: *const xcb_xkb_get_map_map_t)
            -> *mut u8;

    pub fn xcb_xkb_get_map_map_acts_rtrn_count_length (R: *const xcb_xkb_get_map_reply_t,
                                                       S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_acts_rtrn_count_end (R: *const xcb_xkb_get_map_reply_t,
                                                    S: *const xcb_xkb_get_map_map_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_map_map_acts_rtrn_acts (S: *const xcb_xkb_get_map_map_t)
            -> *mut xcb_xkb_action_t;

    pub fn xcb_xkb_get_map_map_acts_rtrn_acts_length (R: *const xcb_xkb_get_map_reply_t,
                                                      S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_acts_rtrn_acts_iterator (R: *const xcb_xkb_get_map_reply_t,
                                                        S: *const xcb_xkb_get_map_map_t)
            -> xcb_xkb_action_iterator_t;

    pub fn xcb_xkb_get_map_map_behaviors_rtrn (S: *const xcb_xkb_get_map_map_t)
            -> *mut xcb_xkb_set_behavior_t;

    pub fn xcb_xkb_get_map_map_behaviors_rtrn_length (R: *const xcb_xkb_get_map_reply_t,
                                                      S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_behaviors_rtrn_iterator<'a> (R: *const xcb_xkb_get_map_reply_t,
                                                            S: *const xcb_xkb_get_map_map_t)
            -> xcb_xkb_set_behavior_iterator_t<'a>;

    pub fn xcb_xkb_get_map_map_vmods_rtrn (S: *const xcb_xkb_get_map_map_t)
            -> *mut u8;

    pub fn xcb_xkb_get_map_map_vmods_rtrn_length (R: *const xcb_xkb_get_map_reply_t,
                                                  S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_vmods_rtrn_end (R: *const xcb_xkb_get_map_reply_t,
                                               S: *const xcb_xkb_get_map_map_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_map_map_explicit_rtrn (S: *const xcb_xkb_get_map_map_t)
            -> *mut xcb_xkb_set_explicit_t;

    pub fn xcb_xkb_get_map_map_explicit_rtrn_length (R: *const xcb_xkb_get_map_reply_t,
                                                     S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_explicit_rtrn_iterator (R: *const xcb_xkb_get_map_reply_t,
                                                       S: *const xcb_xkb_get_map_map_t)
            -> xcb_xkb_set_explicit_iterator_t;

    pub fn xcb_xkb_get_map_map_modmap_rtrn (S: *const xcb_xkb_get_map_map_t)
            -> *mut xcb_xkb_key_mod_map_t;

    pub fn xcb_xkb_get_map_map_modmap_rtrn_length (R: *const xcb_xkb_get_map_reply_t,
                                                   S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_modmap_rtrn_iterator (R: *const xcb_xkb_get_map_reply_t,
                                                     S: *const xcb_xkb_get_map_map_t)
            -> xcb_xkb_key_mod_map_iterator_t;

    pub fn xcb_xkb_get_map_map_vmodmap_rtrn (S: *const xcb_xkb_get_map_map_t)
            -> *mut xcb_xkb_key_v_mod_map_t;

    pub fn xcb_xkb_get_map_map_vmodmap_rtrn_length (R: *const xcb_xkb_get_map_reply_t,
                                                    S: *const xcb_xkb_get_map_map_t)
            -> c_int;

    pub fn xcb_xkb_get_map_map_vmodmap_rtrn_iterator (R: *const xcb_xkb_get_map_reply_t,
                                                      S: *const xcb_xkb_get_map_map_t)
            -> xcb_xkb_key_v_mod_map_iterator_t;

    pub fn xcb_xkb_get_map_map (R: *const xcb_xkb_get_map_reply_t)
            -> *mut c_void;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_map_reply (c:      *mut xcb_connection_t,
                                  cookie: xcb_xkb_get_map_cookie_t,
                                  error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_map_reply_t;

    pub fn xcb_xkb_get_map (c:                *mut xcb_connection_t,
                            deviceSpec:       xcb_xkb_device_spec_t,
                            full:             u16,
                            partial:          u16,
                            firstType:        u8,
                            nTypes:           u8,
                            firstKeySym:      xcb_keycode_t,
                            nKeySyms:         u8,
                            firstKeyAction:   xcb_keycode_t,
                            nKeyActions:      u8,
                            firstKeyBehavior: xcb_keycode_t,
                            nKeyBehaviors:    u8,
                            virtualMods:      u16,
                            firstKeyExplicit: xcb_keycode_t,
                            nKeyExplicit:     u8,
                            firstModMapKey:   xcb_keycode_t,
                            nModMapKeys:      u8,
                            firstVModMapKey:  xcb_keycode_t,
                            nVModMapKeys:     u8)
            -> xcb_xkb_get_map_cookie_t;

    pub fn xcb_xkb_get_map_unchecked (c:                *mut xcb_connection_t,
                                      deviceSpec:       xcb_xkb_device_spec_t,
                                      full:             u16,
                                      partial:          u16,
                                      firstType:        u8,
                                      nTypes:           u8,
                                      firstKeySym:      xcb_keycode_t,
                                      nKeySyms:         u8,
                                      firstKeyAction:   xcb_keycode_t,
                                      nKeyActions:      u8,
                                      firstKeyBehavior: xcb_keycode_t,
                                      nKeyBehaviors:    u8,
                                      virtualMods:      u16,
                                      firstKeyExplicit: xcb_keycode_t,
                                      nKeyExplicit:     u8,
                                      firstModMapKey:   xcb_keycode_t,
                                      nModMapKeys:      u8,
                                      firstVModMapKey:  xcb_keycode_t,
                                      nVModMapKeys:     u8)
            -> xcb_xkb_get_map_cookie_t;

    pub fn xcb_xkb_set_map_values_types_length (R: *const xcb_xkb_set_map_request_t,
                                                S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_types_iterator<'a> (R: *const xcb_xkb_set_map_request_t,
                                                      S: *const xcb_xkb_set_map_values_t)
            -> xcb_xkb_set_key_type_iterator_t<'a>;

    pub fn xcb_xkb_set_map_values_syms_length (R: *const xcb_xkb_set_map_request_t,
                                               S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_syms_iterator<'a> (R: *const xcb_xkb_set_map_request_t,
                                                     S: *const xcb_xkb_set_map_values_t)
            -> xcb_xkb_key_sym_map_iterator_t<'a>;

    pub fn xcb_xkb_set_map_values_actions_count (S: *const xcb_xkb_set_map_values_t)
            -> *mut u8;

    pub fn xcb_xkb_set_map_values_actions_count_length (R: *const xcb_xkb_set_map_request_t,
                                                        S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_actions_count_end (R: *const xcb_xkb_set_map_request_t,
                                                     S: *const xcb_xkb_set_map_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_map_values_actions (S: *const xcb_xkb_set_map_values_t)
            -> *mut xcb_xkb_action_t;

    pub fn xcb_xkb_set_map_values_actions_length (R: *const xcb_xkb_set_map_request_t,
                                                  S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_actions_iterator (R: *const xcb_xkb_set_map_request_t,
                                                    S: *const xcb_xkb_set_map_values_t)
            -> xcb_xkb_action_iterator_t;

    pub fn xcb_xkb_set_map_values_behaviors (S: *const xcb_xkb_set_map_values_t)
            -> *mut xcb_xkb_set_behavior_t;

    pub fn xcb_xkb_set_map_values_behaviors_length (R: *const xcb_xkb_set_map_request_t,
                                                    S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_behaviors_iterator<'a> (R: *const xcb_xkb_set_map_request_t,
                                                          S: *const xcb_xkb_set_map_values_t)
            -> xcb_xkb_set_behavior_iterator_t<'a>;

    pub fn xcb_xkb_set_map_values_vmods (S: *const xcb_xkb_set_map_values_t)
            -> *mut u8;

    pub fn xcb_xkb_set_map_values_vmods_length (R: *const xcb_xkb_set_map_request_t,
                                                S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_vmods_end (R: *const xcb_xkb_set_map_request_t,
                                             S: *const xcb_xkb_set_map_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_map_values_explicit (S: *const xcb_xkb_set_map_values_t)
            -> *mut xcb_xkb_set_explicit_t;

    pub fn xcb_xkb_set_map_values_explicit_length (R: *const xcb_xkb_set_map_request_t,
                                                   S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_explicit_iterator (R: *const xcb_xkb_set_map_request_t,
                                                     S: *const xcb_xkb_set_map_values_t)
            -> xcb_xkb_set_explicit_iterator_t;

    pub fn xcb_xkb_set_map_values_modmap (S: *const xcb_xkb_set_map_values_t)
            -> *mut xcb_xkb_key_mod_map_t;

    pub fn xcb_xkb_set_map_values_modmap_length (R: *const xcb_xkb_set_map_request_t,
                                                 S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_modmap_iterator (R: *const xcb_xkb_set_map_request_t,
                                                   S: *const xcb_xkb_set_map_values_t)
            -> xcb_xkb_key_mod_map_iterator_t;

    pub fn xcb_xkb_set_map_values_vmodmap (S: *const xcb_xkb_set_map_values_t)
            -> *mut xcb_xkb_key_v_mod_map_t;

    pub fn xcb_xkb_set_map_values_vmodmap_length (R: *const xcb_xkb_set_map_request_t,
                                                  S: *const xcb_xkb_set_map_values_t)
            -> c_int;

    pub fn xcb_xkb_set_map_values_vmodmap_iterator (R: *const xcb_xkb_set_map_request_t,
                                                    S: *const xcb_xkb_set_map_values_t)
            -> xcb_xkb_key_v_mod_map_iterator_t;

    pub fn xcb_xkb_set_map (c:                 *mut xcb_connection_t,
                            deviceSpec:        xcb_xkb_device_spec_t,
                            present:           u16,
                            flags:             u16,
                            minKeyCode:        xcb_keycode_t,
                            maxKeyCode:        xcb_keycode_t,
                            firstType:         u8,
                            nTypes:            u8,
                            firstKeySym:       xcb_keycode_t,
                            nKeySyms:          u8,
                            totalSyms:         u16,
                            firstKeyAction:    xcb_keycode_t,
                            nKeyActions:       u8,
                            totalActions:      u16,
                            firstKeyBehavior:  xcb_keycode_t,
                            nKeyBehaviors:     u8,
                            totalKeyBehaviors: u8,
                            firstKeyExplicit:  xcb_keycode_t,
                            nKeyExplicit:      u8,
                            totalKeyExplicit:  u8,
                            firstModMapKey:    xcb_keycode_t,
                            nModMapKeys:       u8,
                            totalModMapKeys:   u8,
                            firstVModMapKey:   xcb_keycode_t,
                            nVModMapKeys:      u8,
                            totalVModMapKeys:  u8,
                            virtualMods:       u16,
                            values:            *const xcb_xkb_set_map_values_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_map_checked (c:                 *mut xcb_connection_t,
                                    deviceSpec:        xcb_xkb_device_spec_t,
                                    present:           u16,
                                    flags:             u16,
                                    minKeyCode:        xcb_keycode_t,
                                    maxKeyCode:        xcb_keycode_t,
                                    firstType:         u8,
                                    nTypes:            u8,
                                    firstKeySym:       xcb_keycode_t,
                                    nKeySyms:          u8,
                                    totalSyms:         u16,
                                    firstKeyAction:    xcb_keycode_t,
                                    nKeyActions:       u8,
                                    totalActions:      u16,
                                    firstKeyBehavior:  xcb_keycode_t,
                                    nKeyBehaviors:     u8,
                                    totalKeyBehaviors: u8,
                                    firstKeyExplicit:  xcb_keycode_t,
                                    nKeyExplicit:      u8,
                                    totalKeyExplicit:  u8,
                                    firstModMapKey:    xcb_keycode_t,
                                    nModMapKeys:       u8,
                                    totalModMapKeys:   u8,
                                    firstVModMapKey:   xcb_keycode_t,
                                    nVModMapKeys:      u8,
                                    totalVModMapKeys:  u8,
                                    virtualMods:       u16,
                                    values:            *const xcb_xkb_set_map_values_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_get_compat_map_si_rtrn (R: *const xcb_xkb_get_compat_map_reply_t)
            -> *mut xcb_xkb_sym_interpret_t;

    pub fn xcb_xkb_get_compat_map_si_rtrn_length (R: *const xcb_xkb_get_compat_map_reply_t)
            -> c_int;

    pub fn xcb_xkb_get_compat_map_si_rtrn_iterator<'a> (R: *const xcb_xkb_get_compat_map_reply_t)
            -> xcb_xkb_sym_interpret_iterator_t<'a>;

    pub fn xcb_xkb_get_compat_map_group_rtrn (R: *const xcb_xkb_get_compat_map_reply_t)
            -> *mut xcb_xkb_mod_def_t;

    pub fn xcb_xkb_get_compat_map_group_rtrn_length (R: *const xcb_xkb_get_compat_map_reply_t)
            -> c_int;

    pub fn xcb_xkb_get_compat_map_group_rtrn_iterator (R: *const xcb_xkb_get_compat_map_reply_t)
            -> xcb_xkb_mod_def_iterator_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_compat_map_reply (c:      *mut xcb_connection_t,
                                         cookie: xcb_xkb_get_compat_map_cookie_t,
                                         error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_compat_map_reply_t;

    pub fn xcb_xkb_get_compat_map (c:          *mut xcb_connection_t,
                                   deviceSpec: xcb_xkb_device_spec_t,
                                   groups:     u8,
                                   getAllSI:   u8,
                                   firstSI:    u16,
                                   nSI:        u16)
            -> xcb_xkb_get_compat_map_cookie_t;

    pub fn xcb_xkb_get_compat_map_unchecked (c:          *mut xcb_connection_t,
                                             deviceSpec: xcb_xkb_device_spec_t,
                                             groups:     u8,
                                             getAllSI:   u8,
                                             firstSI:    u16,
                                             nSI:        u16)
            -> xcb_xkb_get_compat_map_cookie_t;

    pub fn xcb_xkb_set_compat_map (c:                *mut xcb_connection_t,
                                   deviceSpec:       xcb_xkb_device_spec_t,
                                   recomputeActions: u8,
                                   truncateSI:       u8,
                                   groups:           u8,
                                   firstSI:          u16,
                                   nSI:              u16,
                                   si:               *const xcb_xkb_sym_interpret_t,
                                   groupMaps:        *const xcb_xkb_mod_def_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_compat_map_checked (c:                *mut xcb_connection_t,
                                           deviceSpec:       xcb_xkb_device_spec_t,
                                           recomputeActions: u8,
                                           truncateSI:       u8,
                                           groups:           u8,
                                           firstSI:          u16,
                                           nSI:              u16,
                                           si:               *const xcb_xkb_sym_interpret_t,
                                           groupMaps:        *const xcb_xkb_mod_def_t)
            -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_indicator_state_reply (c:      *mut xcb_connection_t,
                                              cookie: xcb_xkb_get_indicator_state_cookie_t,
                                              error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_indicator_state_reply_t;

    pub fn xcb_xkb_get_indicator_state (c:          *mut xcb_connection_t,
                                        deviceSpec: xcb_xkb_device_spec_t)
            -> xcb_xkb_get_indicator_state_cookie_t;

    pub fn xcb_xkb_get_indicator_state_unchecked (c:          *mut xcb_connection_t,
                                                  deviceSpec: xcb_xkb_device_spec_t)
            -> xcb_xkb_get_indicator_state_cookie_t;

    pub fn xcb_xkb_get_indicator_map_maps (R: *const xcb_xkb_get_indicator_map_reply_t)
            -> *mut xcb_xkb_indicator_map_t;

    pub fn xcb_xkb_get_indicator_map_maps_length (R: *const xcb_xkb_get_indicator_map_reply_t)
            -> c_int;

    pub fn xcb_xkb_get_indicator_map_maps_iterator (R: *const xcb_xkb_get_indicator_map_reply_t)
            -> xcb_xkb_indicator_map_iterator_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_indicator_map_reply (c:      *mut xcb_connection_t,
                                            cookie: xcb_xkb_get_indicator_map_cookie_t,
                                            error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_indicator_map_reply_t;

    pub fn xcb_xkb_get_indicator_map (c:          *mut xcb_connection_t,
                                      deviceSpec: xcb_xkb_device_spec_t,
                                      which:      u32)
            -> xcb_xkb_get_indicator_map_cookie_t;

    pub fn xcb_xkb_get_indicator_map_unchecked (c:          *mut xcb_connection_t,
                                                deviceSpec: xcb_xkb_device_spec_t,
                                                which:      u32)
            -> xcb_xkb_get_indicator_map_cookie_t;

    pub fn xcb_xkb_set_indicator_map (c:          *mut xcb_connection_t,
                                      deviceSpec: xcb_xkb_device_spec_t,
                                      which:      u32,
                                      maps:       *const xcb_xkb_indicator_map_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_indicator_map_checked (c:          *mut xcb_connection_t,
                                              deviceSpec: xcb_xkb_device_spec_t,
                                              which:      u32,
                                              maps:       *const xcb_xkb_indicator_map_t)
            -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_named_indicator_reply (c:      *mut xcb_connection_t,
                                              cookie: xcb_xkb_get_named_indicator_cookie_t,
                                              error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_named_indicator_reply_t;

    pub fn xcb_xkb_get_named_indicator (c:          *mut xcb_connection_t,
                                        deviceSpec: xcb_xkb_device_spec_t,
                                        ledClass:   xcb_xkb_led_class_spec_t,
                                        ledID:      xcb_xkb_id_spec_t,
                                        indicator:  xcb_atom_t)
            -> xcb_xkb_get_named_indicator_cookie_t;

    pub fn xcb_xkb_get_named_indicator_unchecked (c:          *mut xcb_connection_t,
                                                  deviceSpec: xcb_xkb_device_spec_t,
                                                  ledClass:   xcb_xkb_led_class_spec_t,
                                                  ledID:      xcb_xkb_id_spec_t,
                                                  indicator:  xcb_atom_t)
            -> xcb_xkb_get_named_indicator_cookie_t;

    pub fn xcb_xkb_set_named_indicator (c:               *mut xcb_connection_t,
                                        deviceSpec:      xcb_xkb_device_spec_t,
                                        ledClass:        xcb_xkb_led_class_spec_t,
                                        ledID:           xcb_xkb_id_spec_t,
                                        indicator:       xcb_atom_t,
                                        setState:        u8,
                                        on:              u8,
                                        setMap:          u8,
                                        createMap:       u8,
                                        map_flags:       u8,
                                        map_whichGroups: u8,
                                        map_groups:      u8,
                                        map_whichMods:   u8,
                                        map_realMods:    u8,
                                        map_vmods:       u16,
                                        map_ctrls:       u32)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_named_indicator_checked (c:               *mut xcb_connection_t,
                                                deviceSpec:      xcb_xkb_device_spec_t,
                                                ledClass:        xcb_xkb_led_class_spec_t,
                                                ledID:           xcb_xkb_id_spec_t,
                                                indicator:       xcb_atom_t,
                                                setState:        u8,
                                                on:              u8,
                                                setMap:          u8,
                                                createMap:       u8,
                                                map_flags:       u8,
                                                map_whichGroups: u8,
                                                map_groups:      u8,
                                                map_whichMods:   u8,
                                                map_realMods:    u8,
                                                map_vmods:       u16,
                                                map_ctrls:       u32)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_get_names_value_list_type_names (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_type_names_length (R: *const xcb_xkb_get_names_reply_t,
                                                           S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_type_names_end (R: *const xcb_xkb_get_names_reply_t,
                                                        S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_n_levels_per_type (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut u8;

    pub fn xcb_xkb_get_names_value_list_n_levels_per_type_length (R: *const xcb_xkb_get_names_reply_t,
                                                                  S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_n_levels_per_type_end (R: *const xcb_xkb_get_names_reply_t,
                                                               S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_alignment_pad (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut u8;

    pub fn xcb_xkb_get_names_value_list_alignment_pad_length (R: *const xcb_xkb_get_names_reply_t,
                                                              S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_alignment_pad_end (R: *const xcb_xkb_get_names_reply_t,
                                                           S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_kt_level_names (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_kt_level_names_length (R: *const xcb_xkb_get_names_reply_t,
                                                               S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_kt_level_names_end (R: *const xcb_xkb_get_names_reply_t,
                                                            S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_indicator_names (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_indicator_names_length (R: *const xcb_xkb_get_names_reply_t,
                                                                S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_indicator_names_end (R: *const xcb_xkb_get_names_reply_t,
                                                             S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_virtual_mod_names (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_virtual_mod_names_length (R: *const xcb_xkb_get_names_reply_t,
                                                                  S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_virtual_mod_names_end (R: *const xcb_xkb_get_names_reply_t,
                                                               S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_groups (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_groups_length (R: *const xcb_xkb_get_names_reply_t,
                                                       S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_groups_end (R: *const xcb_xkb_get_names_reply_t,
                                                    S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list_key_names (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut xcb_xkb_key_name_t;

    pub fn xcb_xkb_get_names_value_list_key_names_length (R: *const xcb_xkb_get_names_reply_t,
                                                          S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_key_names_iterator<'a> (R: *const xcb_xkb_get_names_reply_t,
                                                                S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_xkb_key_name_iterator_t<'a>;

    pub fn xcb_xkb_get_names_value_list_key_aliases (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut xcb_xkb_key_alias_t;

    pub fn xcb_xkb_get_names_value_list_key_aliases_length (R: *const xcb_xkb_get_names_reply_t,
                                                            S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_key_aliases_iterator<'a> (R: *const xcb_xkb_get_names_reply_t,
                                                                  S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_xkb_key_alias_iterator_t<'a>;

    pub fn xcb_xkb_get_names_value_list_radio_group_names (S: *const xcb_xkb_get_names_value_list_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_names_value_list_radio_group_names_length (R: *const xcb_xkb_get_names_reply_t,
                                                                  S: *const xcb_xkb_get_names_value_list_t)
            -> c_int;

    pub fn xcb_xkb_get_names_value_list_radio_group_names_end (R: *const xcb_xkb_get_names_reply_t,
                                                               S: *const xcb_xkb_get_names_value_list_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_names_value_list (R: *const xcb_xkb_get_names_reply_t)
            -> *mut c_void;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_names_reply (c:      *mut xcb_connection_t,
                                    cookie: xcb_xkb_get_names_cookie_t,
                                    error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_names_reply_t;

    pub fn xcb_xkb_get_names (c:          *mut xcb_connection_t,
                              deviceSpec: xcb_xkb_device_spec_t,
                              which:      u32)
            -> xcb_xkb_get_names_cookie_t;

    pub fn xcb_xkb_get_names_unchecked (c:          *mut xcb_connection_t,
                                        deviceSpec: xcb_xkb_device_spec_t,
                                        which:      u32)
            -> xcb_xkb_get_names_cookie_t;

    pub fn xcb_xkb_set_names_values_type_names (S: *const xcb_xkb_set_names_values_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_type_names_length (R: *const xcb_xkb_set_names_request_t,
                                                       S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_type_names_end (R: *const xcb_xkb_set_names_request_t,
                                                    S: *const xcb_xkb_set_names_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_n_levels_per_type (S: *const xcb_xkb_set_names_values_t)
            -> *mut u8;

    pub fn xcb_xkb_set_names_values_n_levels_per_type_length (R: *const xcb_xkb_set_names_request_t,
                                                              S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_n_levels_per_type_end (R: *const xcb_xkb_set_names_request_t,
                                                           S: *const xcb_xkb_set_names_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_kt_level_names (S: *const xcb_xkb_set_names_values_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_kt_level_names_length (R: *const xcb_xkb_set_names_request_t,
                                                           S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_kt_level_names_end (R: *const xcb_xkb_set_names_request_t,
                                                        S: *const xcb_xkb_set_names_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_indicator_names (S: *const xcb_xkb_set_names_values_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_indicator_names_length (R: *const xcb_xkb_set_names_request_t,
                                                            S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_indicator_names_end (R: *const xcb_xkb_set_names_request_t,
                                                         S: *const xcb_xkb_set_names_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_virtual_mod_names (S: *const xcb_xkb_set_names_values_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_virtual_mod_names_length (R: *const xcb_xkb_set_names_request_t,
                                                              S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_virtual_mod_names_end (R: *const xcb_xkb_set_names_request_t,
                                                           S: *const xcb_xkb_set_names_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_groups (S: *const xcb_xkb_set_names_values_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_groups_length (R: *const xcb_xkb_set_names_request_t,
                                                   S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_groups_end (R: *const xcb_xkb_set_names_request_t,
                                                S: *const xcb_xkb_set_names_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names_values_key_names (S: *const xcb_xkb_set_names_values_t)
            -> *mut xcb_xkb_key_name_t;

    pub fn xcb_xkb_set_names_values_key_names_length (R: *const xcb_xkb_set_names_request_t,
                                                      S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_key_names_iterator<'a> (R: *const xcb_xkb_set_names_request_t,
                                                            S: *const xcb_xkb_set_names_values_t)
            -> xcb_xkb_key_name_iterator_t<'a>;

    pub fn xcb_xkb_set_names_values_key_aliases (S: *const xcb_xkb_set_names_values_t)
            -> *mut xcb_xkb_key_alias_t;

    pub fn xcb_xkb_set_names_values_key_aliases_length (R: *const xcb_xkb_set_names_request_t,
                                                        S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_key_aliases_iterator<'a> (R: *const xcb_xkb_set_names_request_t,
                                                              S: *const xcb_xkb_set_names_values_t)
            -> xcb_xkb_key_alias_iterator_t<'a>;

    pub fn xcb_xkb_set_names_values_radio_group_names (S: *const xcb_xkb_set_names_values_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_set_names_values_radio_group_names_length (R: *const xcb_xkb_set_names_request_t,
                                                              S: *const xcb_xkb_set_names_values_t)
            -> c_int;

    pub fn xcb_xkb_set_names_values_radio_group_names_end (R: *const xcb_xkb_set_names_request_t,
                                                           S: *const xcb_xkb_set_names_values_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_set_names (c:                 *mut xcb_connection_t,
                              deviceSpec:        xcb_xkb_device_spec_t,
                              virtualMods:       u16,
                              which:             u32,
                              firstType:         u8,
                              nTypes:            u8,
                              firstKTLevelt:     u8,
                              nKTLevels:         u8,
                              indicators:        u32,
                              groupNames:        u8,
                              nRadioGroups:      u8,
                              firstKey:          xcb_keycode_t,
                              nKeys:             u8,
                              nKeyAliases:       u8,
                              totalKTLevelNames: u16,
                              values:            *const xcb_xkb_set_names_values_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_names_checked (c:                 *mut xcb_connection_t,
                                      deviceSpec:        xcb_xkb_device_spec_t,
                                      virtualMods:       u16,
                                      which:             u32,
                                      firstType:         u8,
                                      nTypes:            u8,
                                      firstKTLevelt:     u8,
                                      nKTLevels:         u8,
                                      indicators:        u32,
                                      groupNames:        u8,
                                      nRadioGroups:      u8,
                                      firstKey:          xcb_keycode_t,
                                      nKeys:             u8,
                                      nKeyAliases:       u8,
                                      totalKTLevelNames: u16,
                                      values:            *const xcb_xkb_set_names_values_t)
            -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_per_client_flags_reply (c:      *mut xcb_connection_t,
                                           cookie: xcb_xkb_per_client_flags_cookie_t,
                                           error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_per_client_flags_reply_t;

    pub fn xcb_xkb_per_client_flags (c:               *mut xcb_connection_t,
                                     deviceSpec:      xcb_xkb_device_spec_t,
                                     change:          u32,
                                     value:           u32,
                                     ctrlsToChange:   u32,
                                     autoCtrls:       u32,
                                     autoCtrlsValues: u32)
            -> xcb_xkb_per_client_flags_cookie_t;

    pub fn xcb_xkb_per_client_flags_unchecked (c:               *mut xcb_connection_t,
                                               deviceSpec:      xcb_xkb_device_spec_t,
                                               change:          u32,
                                               value:           u32,
                                               ctrlsToChange:   u32,
                                               autoCtrls:       u32,
                                               autoCtrlsValues: u32)
            -> xcb_xkb_per_client_flags_cookie_t;

    pub fn xcb_xkb_list_components_keymaps_length (R: *const xcb_xkb_list_components_reply_t)
            -> c_int;

    pub fn xcb_xkb_list_components_keymaps_iterator<'a> (R: *const xcb_xkb_list_components_reply_t)
            -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_keycodes_length (R: *const xcb_xkb_list_components_reply_t)
            -> c_int;

    pub fn xcb_xkb_list_components_keycodes_iterator<'a> (R: *const xcb_xkb_list_components_reply_t)
            -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_types_length (R: *const xcb_xkb_list_components_reply_t)
            -> c_int;

    pub fn xcb_xkb_list_components_types_iterator<'a> (R: *const xcb_xkb_list_components_reply_t)
            -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_compat_maps_length (R: *const xcb_xkb_list_components_reply_t)
            -> c_int;

    pub fn xcb_xkb_list_components_compat_maps_iterator<'a> (R: *const xcb_xkb_list_components_reply_t)
            -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_symbols_length (R: *const xcb_xkb_list_components_reply_t)
            -> c_int;

    pub fn xcb_xkb_list_components_symbols_iterator<'a> (R: *const xcb_xkb_list_components_reply_t)
            -> xcb_xkb_listing_iterator_t<'a>;

    pub fn xcb_xkb_list_components_geometries_length (R: *const xcb_xkb_list_components_reply_t)
            -> c_int;

    pub fn xcb_xkb_list_components_geometries_iterator<'a> (R: *const xcb_xkb_list_components_reply_t)
            -> xcb_xkb_listing_iterator_t<'a>;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_list_components_reply (c:      *mut xcb_connection_t,
                                          cookie: xcb_xkb_list_components_cookie_t,
                                          error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_list_components_reply_t;

    pub fn xcb_xkb_list_components (c:          *mut xcb_connection_t,
                                    deviceSpec: xcb_xkb_device_spec_t,
                                    maxNames:   u16)
            -> xcb_xkb_list_components_cookie_t;

    pub fn xcb_xkb_list_components_unchecked (c:          *mut xcb_connection_t,
                                              deviceSpec: xcb_xkb_device_spec_t,
                                              maxNames:   u16)
            -> xcb_xkb_list_components_cookie_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map (R: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_get_kbd_by_name_replies_types_map_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_sym_interpret_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                      S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator<'a> (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                            S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_sym_interpret_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_mod_def_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                         S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                           S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_mod_def_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_indicator_map_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                       S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                         S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_indicator_map_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list (R: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_geometry_label_font (R: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_counted_string_16_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                        S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator<'a> (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                              S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_key_type_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                       S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator<'a> (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                             S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_key_sym_map_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut u8;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                             S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                          S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_action_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                            S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                              S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_action_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_set_behavior_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                            S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator<'a> (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                  S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_set_behavior_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut u8;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                        S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                     S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_set_explicit_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                           S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                             S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_set_explicit_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_key_mod_map_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                         S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                           S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_key_mod_map_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_key_v_mod_map_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                          S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                            S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_key_v_mod_map_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                   S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut u8;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                          S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                       S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                       S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                    S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                        S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                     S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                          S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                       S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                               S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                            S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_key_name_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                  S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator<'a> (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                        S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_key_name_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_xkb_key_alias_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                    S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator<'a> (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                          S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_xkb_key_alias_iterator_t<'a>;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names (S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> *mut xcb_atom_t;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                          S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> c_int;

    pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end (R: *const xcb_xkb_get_kbd_by_name_reply_t,
                                                                                       S: *const xcb_xkb_get_kbd_by_name_replies_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_kbd_by_name_replies (R: *const xcb_xkb_get_kbd_by_name_reply_t)
            -> *mut c_void;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_kbd_by_name_reply (c:      *mut xcb_connection_t,
                                          cookie: xcb_xkb_get_kbd_by_name_cookie_t,
                                          error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_kbd_by_name_reply_t;

    pub fn xcb_xkb_get_kbd_by_name (c:          *mut xcb_connection_t,
                                    deviceSpec: xcb_xkb_device_spec_t,
                                    need:       u16,
                                    want:       u16,
                                    load:       u8)
            -> xcb_xkb_get_kbd_by_name_cookie_t;

    pub fn xcb_xkb_get_kbd_by_name_unchecked (c:          *mut xcb_connection_t,
                                              deviceSpec: xcb_xkb_device_spec_t,
                                              need:       u16,
                                              want:       u16,
                                              load:       u8)
            -> xcb_xkb_get_kbd_by_name_cookie_t;

    pub fn xcb_xkb_get_device_info_name (R: *const xcb_xkb_get_device_info_reply_t)
            -> *mut xcb_xkb_string8_t;

    pub fn xcb_xkb_get_device_info_name_length (R: *const xcb_xkb_get_device_info_reply_t)
            -> c_int;

    pub fn xcb_xkb_get_device_info_name_end (R: *const xcb_xkb_get_device_info_reply_t)
            -> xcb_generic_iterator_t;

    pub fn xcb_xkb_get_device_info_btn_actions (R: *const xcb_xkb_get_device_info_reply_t)
            -> *mut xcb_xkb_action_t;

    pub fn xcb_xkb_get_device_info_btn_actions_length (R: *const xcb_xkb_get_device_info_reply_t)
            -> c_int;

    pub fn xcb_xkb_get_device_info_btn_actions_iterator (R: *const xcb_xkb_get_device_info_reply_t)
            -> xcb_xkb_action_iterator_t;

    pub fn xcb_xkb_get_device_info_leds_length (R: *const xcb_xkb_get_device_info_reply_t)
            -> c_int;

    pub fn xcb_xkb_get_device_info_leds_iterator<'a> (R: *const xcb_xkb_get_device_info_reply_t)
            -> xcb_xkb_device_led_info_iterator_t<'a>;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_get_device_info_reply (c:      *mut xcb_connection_t,
                                          cookie: xcb_xkb_get_device_info_cookie_t,
                                          error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_get_device_info_reply_t;

    pub fn xcb_xkb_get_device_info (c:           *mut xcb_connection_t,
                                    deviceSpec:  xcb_xkb_device_spec_t,
                                    wanted:      u16,
                                    allButtons:  u8,
                                    firstButton: u8,
                                    nButtons:    u8,
                                    ledClass:    xcb_xkb_led_class_spec_t,
                                    ledID:       xcb_xkb_id_spec_t)
            -> xcb_xkb_get_device_info_cookie_t;

    pub fn xcb_xkb_get_device_info_unchecked (c:           *mut xcb_connection_t,
                                              deviceSpec:  xcb_xkb_device_spec_t,
                                              wanted:      u16,
                                              allButtons:  u8,
                                              firstButton: u8,
                                              nButtons:    u8,
                                              ledClass:    xcb_xkb_led_class_spec_t,
                                              ledID:       xcb_xkb_id_spec_t)
            -> xcb_xkb_get_device_info_cookie_t;

    pub fn xcb_xkb_set_device_info (c:             *mut xcb_connection_t,
                                    deviceSpec:    xcb_xkb_device_spec_t,
                                    firstBtn:      u8,
                                    nBtns:         u8,
                                    change:        u16,
                                    nDeviceLedFBs: u16,
                                    btnActions:    *const xcb_xkb_action_t,
                                    leds:          *const xcb_xkb_device_led_info_t)
            -> xcb_void_cookie_t;

    pub fn xcb_xkb_set_device_info_checked (c:             *mut xcb_connection_t,
                                            deviceSpec:    xcb_xkb_device_spec_t,
                                            firstBtn:      u8,
                                            nBtns:         u8,
                                            change:        u16,
                                            nDeviceLedFBs: u16,
                                            btnActions:    *const xcb_xkb_action_t,
                                            leds:          *const xcb_xkb_device_led_info_t)
            -> xcb_void_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_xkb_set_debugging_flags_reply (c:      *mut xcb_connection_t,
                                              cookie: xcb_xkb_set_debugging_flags_cookie_t,
                                              error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_xkb_set_debugging_flags_reply_t;

    pub fn xcb_xkb_set_debugging_flags (c:           *mut xcb_connection_t,
                                        msgLength:   u16,
                                        affectFlags: u32,
                                        flags:       u32,
                                        affectCtrls: u32,
                                        ctrls:       u32,
                                        message:     *const xcb_xkb_string8_t)
            -> xcb_xkb_set_debugging_flags_cookie_t;

    pub fn xcb_xkb_set_debugging_flags_unchecked (c:           *mut xcb_connection_t,
                                                  msgLength:   u16,
                                                  affectFlags: u32,
                                                  flags:       u32,
                                                  affectCtrls: u32,
                                                  ctrls:       u32,
                                                  message:     *const xcb_xkb_string8_t)
            -> xcb_xkb_set_debugging_flags_cookie_t;

} // extern
