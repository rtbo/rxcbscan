// Generated automatically from xkb.xml by rs_client.py version 0.9.0.
// Do not edit!

use base;
use xproto;
use ffi::base::*;
use ffi::xkb::*;
use ffi::xproto::*;
use libc::{self, c_char, c_int, c_uint, c_void};
use std;
use std::iter::Iterator;


pub fn id() -> &'static mut base::Extension {
    unsafe {
        &mut xcb_xkb_id
    }
}

pub const MAJOR_VERSION: u32 = 1;
pub const MINOR_VERSION: u32 = 0;

pub type Const = u32;
pub const CONST_MAX_LEGAL_KEY_CODE    : Const = 0xff;
pub const CONST_PER_KEY_BIT_ARRAY_SIZE: Const = 0x20;
pub const CONST_KEY_NAME_LENGTH       : Const = 0x04;

pub type EventType = u32;
pub const EVENT_TYPE_NEW_KEYBOARD_NOTIFY    : EventType =  0x01;
pub const EVENT_TYPE_MAP_NOTIFY             : EventType =  0x02;
pub const EVENT_TYPE_STATE_NOTIFY           : EventType =  0x04;
pub const EVENT_TYPE_CONTROLS_NOTIFY        : EventType =  0x08;
pub const EVENT_TYPE_INDICATOR_STATE_NOTIFY : EventType =  0x10;
pub const EVENT_TYPE_INDICATOR_MAP_NOTIFY   : EventType =  0x20;
pub const EVENT_TYPE_NAMES_NOTIFY           : EventType =  0x40;
pub const EVENT_TYPE_COMPAT_MAP_NOTIFY      : EventType =  0x80;
pub const EVENT_TYPE_BELL_NOTIFY            : EventType = 0x100;
pub const EVENT_TYPE_ACTION_MESSAGE         : EventType = 0x200;
pub const EVENT_TYPE_ACCESS_X_NOTIFY        : EventType = 0x400;
pub const EVENT_TYPE_EXTENSION_DEVICE_NOTIFY: EventType = 0x800;

pub type NknDetail = u32;
pub const NKN_DETAIL_KEYCODES : NknDetail = 0x01;
pub const NKN_DETAIL_GEOMETRY : NknDetail = 0x02;
pub const NKN_DETAIL_DEVICE_ID: NknDetail = 0x04;

pub type AxnDetail = u32;
pub const AXN_DETAIL_SK_PRESS   : AxnDetail = 0x01;
pub const AXN_DETAIL_SK_ACCEPT  : AxnDetail = 0x02;
pub const AXN_DETAIL_SK_REJECT  : AxnDetail = 0x04;
pub const AXN_DETAIL_SK_RELEASE : AxnDetail = 0x08;
pub const AXN_DETAIL_BK_ACCEPT  : AxnDetail = 0x10;
pub const AXN_DETAIL_BK_REJECT  : AxnDetail = 0x20;
pub const AXN_DETAIL_AXK_WARNING: AxnDetail = 0x40;

pub type MapPart = u32;
pub const MAP_PART_KEY_TYPES          : MapPart = 0x01;
pub const MAP_PART_KEY_SYMS           : MapPart = 0x02;
pub const MAP_PART_MODIFIER_MAP       : MapPart = 0x04;
pub const MAP_PART_EXPLICIT_COMPONENTS: MapPart = 0x08;
pub const MAP_PART_KEY_ACTIONS        : MapPart = 0x10;
pub const MAP_PART_KEY_BEHAVIORS      : MapPart = 0x20;
pub const MAP_PART_VIRTUAL_MODS       : MapPart = 0x40;
pub const MAP_PART_VIRTUAL_MOD_MAP    : MapPart = 0x80;

pub type SetMapFlags = u32;
pub const SET_MAP_FLAGS_RESIZE_TYPES     : SetMapFlags = 0x01;
pub const SET_MAP_FLAGS_RECOMPUTE_ACTIONS: SetMapFlags = 0x02;

pub type StatePart = u32;
pub const STATE_PART_MODIFIER_STATE    : StatePart =   0x01;
pub const STATE_PART_MODIFIER_BASE     : StatePart =   0x02;
pub const STATE_PART_MODIFIER_LATCH    : StatePart =   0x04;
pub const STATE_PART_MODIFIER_LOCK     : StatePart =   0x08;
pub const STATE_PART_GROUP_STATE       : StatePart =   0x10;
pub const STATE_PART_GROUP_BASE        : StatePart =   0x20;
pub const STATE_PART_GROUP_LATCH       : StatePart =   0x40;
pub const STATE_PART_GROUP_LOCK        : StatePart =   0x80;
pub const STATE_PART_COMPAT_STATE      : StatePart =  0x100;
pub const STATE_PART_GRAB_MODS         : StatePart =  0x200;
pub const STATE_PART_COMPAT_GRAB_MODS  : StatePart =  0x400;
pub const STATE_PART_LOOKUP_MODS       : StatePart =  0x800;
pub const STATE_PART_COMPAT_LOOKUP_MODS: StatePart = 0x1000;
pub const STATE_PART_POINTER_BUTTONS   : StatePart = 0x2000;

pub type BoolCtrl = u32;
pub const BOOL_CTRL_REPEAT_KEYS           : BoolCtrl =   0x01;
pub const BOOL_CTRL_SLOW_KEYS             : BoolCtrl =   0x02;
pub const BOOL_CTRL_BOUNCE_KEYS           : BoolCtrl =   0x04;
pub const BOOL_CTRL_STICKY_KEYS           : BoolCtrl =   0x08;
pub const BOOL_CTRL_MOUSE_KEYS            : BoolCtrl =   0x10;
pub const BOOL_CTRL_MOUSE_KEYS_ACCEL      : BoolCtrl =   0x20;
pub const BOOL_CTRL_ACCESS_X_KEYS         : BoolCtrl =   0x40;
pub const BOOL_CTRL_ACCESS_X_TIMEOUT_MASK : BoolCtrl =   0x80;
pub const BOOL_CTRL_ACCESS_X_FEEDBACK_MASK: BoolCtrl =  0x100;
pub const BOOL_CTRL_AUDIBLE_BELL_MASK     : BoolCtrl =  0x200;
pub const BOOL_CTRL_OVERLAY_1_MASK        : BoolCtrl =  0x400;
pub const BOOL_CTRL_OVERLAY_2_MASK        : BoolCtrl =  0x800;
pub const BOOL_CTRL_IGNORE_GROUP_LOCK_MASK: BoolCtrl = 0x1000;

pub type Control = u32;
pub const CONTROL_GROUPS_WRAP     : Control =  0x8000000;
pub const CONTROL_INTERNAL_MODS   : Control = 0x10000000;
pub const CONTROL_IGNORE_LOCK_MODS: Control = 0x20000000;
pub const CONTROL_PER_KEY_REPEAT  : Control = 0x40000000;
pub const CONTROL_CONTROLS_ENABLED: Control = 0x80000000;

pub type AxOption = u32;
pub const AX_OPTION_SK_PRESS_FB   : AxOption =  0x01;
pub const AX_OPTION_SK_ACCEPT_FB  : AxOption =  0x02;
pub const AX_OPTION_FEATURE_FB    : AxOption =  0x04;
pub const AX_OPTION_SLOW_WARN_FB  : AxOption =  0x08;
pub const AX_OPTION_INDICATOR_FB  : AxOption =  0x10;
pub const AX_OPTION_STICKY_KEYS_FB: AxOption =  0x20;
pub const AX_OPTION_TWO_KEYS      : AxOption =  0x40;
pub const AX_OPTION_LATCH_TO_LOCK : AxOption =  0x80;
pub const AX_OPTION_SK_RELEASE_FB : AxOption = 0x100;
pub const AX_OPTION_SK_REJECT_FB  : AxOption = 0x200;
pub const AX_OPTION_BK_REJECT_FB  : AxOption = 0x400;
pub const AX_OPTION_DUMB_BELL     : AxOption = 0x800;

pub type DeviceSpec = xcb_xkb_device_spec_t;

pub type LedClassResult = u32;
pub const LED_CLASS_RESULT_KBD_FEEDBACK_CLASS: LedClassResult = 0x00;
pub const LED_CLASS_RESULT_LED_FEEDBACK_CLASS: LedClassResult = 0x04;

pub type LedClass = u32;
pub const LED_CLASS_KBD_FEEDBACK_CLASS: LedClass =  0x00;
pub const LED_CLASS_LED_FEEDBACK_CLASS: LedClass =  0x04;
pub const LED_CLASS_DFLT_XI_CLASS     : LedClass = 0x300;
pub const LED_CLASS_ALL_XI_CLASSES    : LedClass = 0x500;

pub type LedClassSpec = xcb_xkb_led_class_spec_t;

pub type BellClassResult = u32;
pub const BELL_CLASS_RESULT_KBD_FEEDBACK_CLASS : BellClassResult = 0x00;
pub const BELL_CLASS_RESULT_BELL_FEEDBACK_CLASS: BellClassResult = 0x05;

pub type BellClass = u32;
pub const BELL_CLASS_KBD_FEEDBACK_CLASS : BellClass =  0x00;
pub const BELL_CLASS_BELL_FEEDBACK_CLASS: BellClass =  0x05;
pub const BELL_CLASS_DFLT_XI_CLASS      : BellClass = 0x300;

pub type BellClassSpec = xcb_xkb_bell_class_spec_t;

pub type Id = u32;
pub const ID_USE_CORE_KBD : Id =  0x100;
pub const ID_USE_CORE_PTR : Id =  0x200;
pub const ID_DFLT_XI_CLASS: Id =  0x300;
pub const ID_DFLT_XI_ID   : Id =  0x400;
pub const ID_ALL_XI_CLASS : Id =  0x500;
pub const ID_ALL_XI_ID    : Id =  0x600;
pub const ID_XI_NONE      : Id = 0xff00;

pub type IdSpec = xcb_xkb_id_spec_t;

pub type Group = u32;
pub const GROUP_1: Group = 0x00;
pub const GROUP_2: Group = 0x01;
pub const GROUP_3: Group = 0x02;
pub const GROUP_4: Group = 0x03;

pub type Groups = u32;
pub const GROUPS_ANY: Groups = 0xfe;
pub const GROUPS_ALL: Groups = 0xff;

pub type SetOfGroup = u32;
pub const SET_OF_GROUP_GROUP_1: SetOfGroup = 0x01;
pub const SET_OF_GROUP_GROUP_2: SetOfGroup = 0x02;
pub const SET_OF_GROUP_GROUP_3: SetOfGroup = 0x04;
pub const SET_OF_GROUP_GROUP_4: SetOfGroup = 0x08;

pub type SetOfGroups = u32;
pub const SET_OF_GROUPS_ANY: SetOfGroups = 0x80;

pub type GroupsWrap = u32;
pub const GROUPS_WRAP_WRAP_INTO_RANGE    : GroupsWrap = 0x00;
pub const GROUPS_WRAP_CLAMP_INTO_RANGE   : GroupsWrap = 0x40;
pub const GROUPS_WRAP_REDIRECT_INTO_RANGE: GroupsWrap = 0x80;

pub type VModsHigh = u32;
pub const V_MODS_HIGH_15: VModsHigh = 0x80;
pub const V_MODS_HIGH_14: VModsHigh = 0x40;
pub const V_MODS_HIGH_13: VModsHigh = 0x20;
pub const V_MODS_HIGH_12: VModsHigh = 0x10;
pub const V_MODS_HIGH_11: VModsHigh = 0x08;
pub const V_MODS_HIGH_10: VModsHigh = 0x04;
pub const V_MODS_HIGH_9 : VModsHigh = 0x02;
pub const V_MODS_HIGH_8 : VModsHigh = 0x01;

pub type VModsLow = u32;
pub const V_MODS_LOW_7: VModsLow = 0x80;
pub const V_MODS_LOW_6: VModsLow = 0x40;
pub const V_MODS_LOW_5: VModsLow = 0x20;
pub const V_MODS_LOW_4: VModsLow = 0x10;
pub const V_MODS_LOW_3: VModsLow = 0x08;
pub const V_MODS_LOW_2: VModsLow = 0x04;
pub const V_MODS_LOW_1: VModsLow = 0x02;
pub const V_MODS_LOW_0: VModsLow = 0x01;

pub type VMod = u32;
pub const V_MOD_15: VMod = 0x8000;
pub const V_MOD_14: VMod = 0x4000;
pub const V_MOD_13: VMod = 0x2000;
pub const V_MOD_12: VMod = 0x1000;
pub const V_MOD_11: VMod =  0x800;
pub const V_MOD_10: VMod =  0x400;
pub const V_MOD_9 : VMod =  0x200;
pub const V_MOD_8 : VMod =  0x100;
pub const V_MOD_7 : VMod =   0x80;
pub const V_MOD_6 : VMod =   0x40;
pub const V_MOD_5 : VMod =   0x20;
pub const V_MOD_4 : VMod =   0x10;
pub const V_MOD_3 : VMod =   0x08;
pub const V_MOD_2 : VMod =   0x04;
pub const V_MOD_1 : VMod =   0x02;
pub const V_MOD_0 : VMod =   0x01;

pub type Explicit = u32;
pub const EXPLICIT_V_MOD_MAP  : Explicit = 0x80;
pub const EXPLICIT_BEHAVIOR   : Explicit = 0x40;
pub const EXPLICIT_AUTO_REPEAT: Explicit = 0x20;
pub const EXPLICIT_INTERPRET  : Explicit = 0x10;
pub const EXPLICIT_KEY_TYPE_4 : Explicit = 0x08;
pub const EXPLICIT_KEY_TYPE_3 : Explicit = 0x04;
pub const EXPLICIT_KEY_TYPE_2 : Explicit = 0x02;
pub const EXPLICIT_KEY_TYPE_1 : Explicit = 0x01;

pub type SymInterpretMatch = u32;
pub const SYM_INTERPRET_MATCH_NONE_OF       : SymInterpretMatch = 0x00;
pub const SYM_INTERPRET_MATCH_ANY_OF_OR_NONE: SymInterpretMatch = 0x01;
pub const SYM_INTERPRET_MATCH_ANY_OF        : SymInterpretMatch = 0x02;
pub const SYM_INTERPRET_MATCH_ALL_OF        : SymInterpretMatch = 0x03;
pub const SYM_INTERPRET_MATCH_EXACTLY       : SymInterpretMatch = 0x04;

pub type SymInterpMatch = u32;
pub const SYM_INTERP_MATCH_LEVEL_ONE_ONLY: SymInterpMatch = 0x80;
pub const SYM_INTERP_MATCH_OP_MASK       : SymInterpMatch = 0x7f;

pub type ImFlag = u32;
pub const IM_FLAG_NO_EXPLICIT  : ImFlag = 0x80;
pub const IM_FLAG_NO_AUTOMATIC : ImFlag = 0x40;
pub const IM_FLAG_LED_DRIVES_KB: ImFlag = 0x20;

pub type ImModsWhich = u32;
pub const IM_MODS_WHICH_USE_COMPAT   : ImModsWhich = 0x10;
pub const IM_MODS_WHICH_USE_EFFECTIVE: ImModsWhich = 0x08;
pub const IM_MODS_WHICH_USE_LOCKED   : ImModsWhich = 0x04;
pub const IM_MODS_WHICH_USE_LATCHED  : ImModsWhich = 0x02;
pub const IM_MODS_WHICH_USE_BASE     : ImModsWhich = 0x01;

pub type ImGroupsWhich = u32;
pub const IM_GROUPS_WHICH_USE_COMPAT   : ImGroupsWhich = 0x10;
pub const IM_GROUPS_WHICH_USE_EFFECTIVE: ImGroupsWhich = 0x08;
pub const IM_GROUPS_WHICH_USE_LOCKED   : ImGroupsWhich = 0x04;
pub const IM_GROUPS_WHICH_USE_LATCHED  : ImGroupsWhich = 0x02;
pub const IM_GROUPS_WHICH_USE_BASE     : ImGroupsWhich = 0x01;

pub type CmDetail = u32;
pub const CM_DETAIL_SYM_INTERP  : CmDetail = 0x01;
pub const CM_DETAIL_GROUP_COMPAT: CmDetail = 0x02;

pub type NameDetail = u32;
pub const NAME_DETAIL_KEYCODES         : NameDetail =   0x01;
pub const NAME_DETAIL_GEOMETRY         : NameDetail =   0x02;
pub const NAME_DETAIL_SYMBOLS          : NameDetail =   0x04;
pub const NAME_DETAIL_PHYS_SYMBOLS     : NameDetail =   0x08;
pub const NAME_DETAIL_TYPES            : NameDetail =   0x10;
pub const NAME_DETAIL_COMPAT           : NameDetail =   0x20;
pub const NAME_DETAIL_KEY_TYPE_NAMES   : NameDetail =   0x40;
pub const NAME_DETAIL_KT_LEVEL_NAMES   : NameDetail =   0x80;
pub const NAME_DETAIL_INDICATOR_NAMES  : NameDetail =  0x100;
pub const NAME_DETAIL_KEY_NAMES        : NameDetail =  0x200;
pub const NAME_DETAIL_KEY_ALIASES      : NameDetail =  0x400;
pub const NAME_DETAIL_VIRTUAL_MOD_NAMES: NameDetail =  0x800;
pub const NAME_DETAIL_GROUP_NAMES      : NameDetail = 0x1000;
pub const NAME_DETAIL_RG_NAMES         : NameDetail = 0x2000;

pub type GbnDetail = u32;
pub const GBN_DETAIL_TYPES         : GbnDetail = 0x01;
pub const GBN_DETAIL_COMPAT_MAP    : GbnDetail = 0x02;
pub const GBN_DETAIL_CLIENT_SYMBOLS: GbnDetail = 0x04;
pub const GBN_DETAIL_SERVER_SYMBOLS: GbnDetail = 0x08;
pub const GBN_DETAIL_INDICATOR_MAPS: GbnDetail = 0x10;
pub const GBN_DETAIL_KEY_NAMES     : GbnDetail = 0x20;
pub const GBN_DETAIL_GEOMETRY      : GbnDetail = 0x40;
pub const GBN_DETAIL_OTHER_NAMES   : GbnDetail = 0x80;

pub type XiFeature = u32;
pub const XI_FEATURE_KEYBOARDS      : XiFeature = 0x01;
pub const XI_FEATURE_BUTTON_ACTIONS : XiFeature = 0x02;
pub const XI_FEATURE_INDICATOR_NAMES: XiFeature = 0x04;
pub const XI_FEATURE_INDICATOR_MAPS : XiFeature = 0x08;
pub const XI_FEATURE_INDICATOR_STATE: XiFeature = 0x10;

pub type PerClientFlag = u32;
pub const PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT   : PerClientFlag = 0x01;
pub const PER_CLIENT_FLAG_GRABS_USE_XKB_STATE      : PerClientFlag = 0x02;
pub const PER_CLIENT_FLAG_AUTO_RESET_CONTROLS      : PerClientFlag = 0x04;
pub const PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED: PerClientFlag = 0x08;
pub const PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE: PerClientFlag = 0x10;

pub type BehaviorType = u32;
pub const BEHAVIOR_TYPE_DEFAULT              : BehaviorType = 0x00;
pub const BEHAVIOR_TYPE_LOCK                 : BehaviorType = 0x01;
pub const BEHAVIOR_TYPE_RADIO_GROUP          : BehaviorType = 0x02;
pub const BEHAVIOR_TYPE_OVERLAY_1            : BehaviorType = 0x03;
pub const BEHAVIOR_TYPE_OVERLAY_2            : BehaviorType = 0x04;
pub const BEHAVIOR_TYPE_PERMAMENT_LOCK       : BehaviorType = 0x81;
pub const BEHAVIOR_TYPE_PERMAMENT_RADIO_GROUP: BehaviorType = 0x82;
pub const BEHAVIOR_TYPE_PERMAMENT_OVERLAY_1  : BehaviorType = 0x83;
pub const BEHAVIOR_TYPE_PERMAMENT_OVERLAY_2  : BehaviorType = 0x84;

pub type String8 = xcb_xkb_string8_t;

pub type DoodadType = u32;
pub const DOODAD_TYPE_OUTLINE  : DoodadType = 0x01;
pub const DOODAD_TYPE_SOLID    : DoodadType = 0x02;
pub const DOODAD_TYPE_TEXT     : DoodadType = 0x03;
pub const DOODAD_TYPE_INDICATOR: DoodadType = 0x04;
pub const DOODAD_TYPE_LOGO     : DoodadType = 0x05;

pub type Error = u32;
pub const ERROR_BAD_DEVICE: Error = 0xff;
pub const ERROR_BAD_CLASS : Error = 0xfe;
pub const ERROR_BAD_ID    : Error = 0xfd;

pub struct KeyboardError {
    pub base: base::Error<xcb_xkb_keyboard_error_t>
}

pub type Sa = u32;
pub const SA_CLEAR_LOCKS     : Sa = 0x01;
pub const SA_LATCH_TO_LOCK   : Sa = 0x02;
pub const SA_USE_MOD_MAP_MODS: Sa = 0x04;
pub const SA_GROUP_ABSOLUTE  : Sa = 0x04;

pub type SaType = u32;
pub const SA_TYPE_NO_ACTION      : SaType = 0x00;
pub const SA_TYPE_SET_MODS       : SaType = 0x01;
pub const SA_TYPE_LATCH_MODS     : SaType = 0x02;
pub const SA_TYPE_LOCK_MODS      : SaType = 0x03;
pub const SA_TYPE_SET_GROUP      : SaType = 0x04;
pub const SA_TYPE_LATCH_GROUP    : SaType = 0x05;
pub const SA_TYPE_LOCK_GROUP     : SaType = 0x06;
pub const SA_TYPE_MOVE_PTR       : SaType = 0x07;
pub const SA_TYPE_PTR_BTN        : SaType = 0x08;
pub const SA_TYPE_LOCK_PTR_BTN   : SaType = 0x09;
pub const SA_TYPE_SET_PTR_DFLT   : SaType = 0x0a;
pub const SA_TYPE_ISO_LOCK       : SaType = 0x0b;
pub const SA_TYPE_TERMINATE      : SaType = 0x0c;
pub const SA_TYPE_SWITCH_SCREEN  : SaType = 0x0d;
pub const SA_TYPE_SET_CONTROLS   : SaType = 0x0e;
pub const SA_TYPE_LOCK_CONTROLS  : SaType = 0x0f;
pub const SA_TYPE_ACTION_MESSAGE : SaType = 0x10;
pub const SA_TYPE_REDIRECT_KEY   : SaType = 0x11;
pub const SA_TYPE_DEVICE_BTN     : SaType = 0x12;
pub const SA_TYPE_LOCK_DEVICE_BTN: SaType = 0x13;
pub const SA_TYPE_DEVICE_VALUATOR: SaType = 0x14;

pub type SaMovePtrFlag = u32;
pub const SA_MOVE_PTR_FLAG_NO_ACCELERATION: SaMovePtrFlag = 0x01;
pub const SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_X: SaMovePtrFlag = 0x02;
pub const SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_Y: SaMovePtrFlag = 0x04;

pub type SaSetPtrDfltFlag = u32;
pub const SA_SET_PTR_DFLT_FLAG_DFLT_BTN_ABSOLUTE : SaSetPtrDfltFlag = 0x04;
pub const SA_SET_PTR_DFLT_FLAG_AFFECT_DFLT_BUTTON: SaSetPtrDfltFlag = 0x01;

pub type SaIsoLockFlag = u32;
pub const SA_ISO_LOCK_FLAG_NO_LOCK          : SaIsoLockFlag = 0x01;
pub const SA_ISO_LOCK_FLAG_NO_UNLOCK        : SaIsoLockFlag = 0x02;
pub const SA_ISO_LOCK_FLAG_USE_MOD_MAP_MODS : SaIsoLockFlag = 0x04;
pub const SA_ISO_LOCK_FLAG_GROUP_ABSOLUTE   : SaIsoLockFlag = 0x04;
pub const SA_ISO_LOCK_FLAG_ISO_DFLT_IS_GROUP: SaIsoLockFlag = 0x08;

pub type SaIsoLockNoAffect = u32;
pub const SA_ISO_LOCK_NO_AFFECT_CTRLS: SaIsoLockNoAffect = 0x08;
pub const SA_ISO_LOCK_NO_AFFECT_PTR  : SaIsoLockNoAffect = 0x10;
pub const SA_ISO_LOCK_NO_AFFECT_GROUP: SaIsoLockNoAffect = 0x20;
pub const SA_ISO_LOCK_NO_AFFECT_MODS : SaIsoLockNoAffect = 0x40;

pub type SwitchScreenFlag = u32;
pub const SWITCH_SCREEN_FLAG_APPLICATION: SwitchScreenFlag = 0x01;
pub const SWITCH_SCREEN_FLAG_ABSOLUTE   : SwitchScreenFlag = 0x04;

pub type BoolCtrlsHigh = u32;
pub const BOOL_CTRLS_HIGH_ACCESS_X_FEEDBACK: BoolCtrlsHigh = 0x01;
pub const BOOL_CTRLS_HIGH_AUDIBLE_BELL     : BoolCtrlsHigh = 0x02;
pub const BOOL_CTRLS_HIGH_OVERLAY_1        : BoolCtrlsHigh = 0x04;
pub const BOOL_CTRLS_HIGH_OVERLAY_2        : BoolCtrlsHigh = 0x08;
pub const BOOL_CTRLS_HIGH_IGNORE_GROUP_LOCK: BoolCtrlsHigh = 0x10;

pub type BoolCtrlsLow = u32;
pub const BOOL_CTRLS_LOW_REPEAT_KEYS     : BoolCtrlsLow = 0x01;
pub const BOOL_CTRLS_LOW_SLOW_KEYS       : BoolCtrlsLow = 0x02;
pub const BOOL_CTRLS_LOW_BOUNCE_KEYS     : BoolCtrlsLow = 0x04;
pub const BOOL_CTRLS_LOW_STICKY_KEYS     : BoolCtrlsLow = 0x08;
pub const BOOL_CTRLS_LOW_MOUSE_KEYS      : BoolCtrlsLow = 0x10;
pub const BOOL_CTRLS_LOW_MOUSE_KEYS_ACCEL: BoolCtrlsLow = 0x20;
pub const BOOL_CTRLS_LOW_ACCESS_X_KEYS   : BoolCtrlsLow = 0x40;
pub const BOOL_CTRLS_LOW_ACCESS_X_TIMEOUT: BoolCtrlsLow = 0x80;

pub type ActionMessageFlag = u32;
pub const ACTION_MESSAGE_FLAG_ON_PRESS     : ActionMessageFlag = 0x01;
pub const ACTION_MESSAGE_FLAG_ON_RELEASE   : ActionMessageFlag = 0x02;
pub const ACTION_MESSAGE_FLAG_GEN_KEY_EVENT: ActionMessageFlag = 0x04;

pub type LockDeviceFlags = u32;
pub const LOCK_DEVICE_FLAGS_NO_LOCK  : LockDeviceFlags = 0x01;
pub const LOCK_DEVICE_FLAGS_NO_UNLOCK: LockDeviceFlags = 0x02;

pub type SaValWhat = u32;
pub const SA_VAL_WHAT_IGNORE_VAL      : SaValWhat = 0x00;
pub const SA_VAL_WHAT_SET_VAL_MIN     : SaValWhat = 0x01;
pub const SA_VAL_WHAT_SET_VAL_CENTER  : SaValWhat = 0x02;
pub const SA_VAL_WHAT_SET_VAL_MAX     : SaValWhat = 0x03;
pub const SA_VAL_WHAT_SET_VAL_RELATIVE: SaValWhat = 0x04;
pub const SA_VAL_WHAT_SET_VAL_ABSOLUTE: SaValWhat = 0x05;



#[derive(Copy, Clone)]
pub struct IndicatorMap {
    pub base: xcb_xkb_indicator_map_t,
}

impl IndicatorMap {
    #[allow(unused_unsafe)]
    pub fn new(flags:       u8,
               whichGroups: u8,
               groups:      u8,
               whichMods:   u8,
               mods:        u8,
               realMods:    u8,
               vmods:       u16,
               ctrls:       u32)
            -> IndicatorMap {
        unsafe {
            IndicatorMap {
                base: xcb_xkb_indicator_map_t {
                    flags:       flags,
                    whichGroups: whichGroups,
                    groups:      groups,
                    whichMods:   whichMods,
                    mods:        mods,
                    realMods:    realMods,
                    vmods:       vmods,
                    ctrls:       ctrls,
                }
            }
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn whichGroups(&self) -> u8 {
        unsafe {
            self.base.whichGroups
        }
    }
    pub fn groups(&self) -> u8 {
        unsafe {
            self.base.groups
        }
    }
    pub fn whichMods(&self) -> u8 {
        unsafe {
            self.base.whichMods
        }
    }
    pub fn mods(&self) -> u8 {
        unsafe {
            self.base.mods
        }
    }
    pub fn realMods(&self) -> u8 {
        unsafe {
            self.base.realMods
        }
    }
    pub fn vmods(&self) -> u16 {
        unsafe {
            self.base.vmods
        }
    }
    pub fn ctrls(&self) -> u32 {
        unsafe {
            self.base.ctrls
        }
    }
}

pub type IndicatorMapIterator = xcb_xkb_indicator_map_iterator_t;

impl Iterator for IndicatorMapIterator {
    type Item = IndicatorMap;
    fn next(&mut self) -> std::option::Option<IndicatorMap> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_indicator_map_iterator_t;
                let data = (*iter).data;
                xcb_xkb_indicator_map_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct ModDef {
    pub base: xcb_xkb_mod_def_t,
}

impl ModDef {
    #[allow(unused_unsafe)]
    pub fn new(mask:     u8,
               realMods: u8,
               vmods:    u16)
            -> ModDef {
        unsafe {
            ModDef {
                base: xcb_xkb_mod_def_t {
                    mask:     mask,
                    realMods: realMods,
                    vmods:    vmods,
                }
            }
        }
    }
    pub fn mask(&self) -> u8 {
        unsafe {
            self.base.mask
        }
    }
    pub fn realMods(&self) -> u8 {
        unsafe {
            self.base.realMods
        }
    }
    pub fn vmods(&self) -> u16 {
        unsafe {
            self.base.vmods
        }
    }
}

pub type ModDefIterator = xcb_xkb_mod_def_iterator_t;

impl Iterator for ModDefIterator {
    type Item = ModDef;
    fn next(&mut self) -> std::option::Option<ModDef> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_mod_def_iterator_t;
                let data = (*iter).data;
                xcb_xkb_mod_def_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

pub type KeyName<'a> = base::StructPtr<'a, xcb_xkb_key_name_t>;

impl<'a> KeyName<'a> {
    pub fn name(&self) -> &[c_char] {
        unsafe {
            &(*self.ptr).name
        }
    }
}

pub type KeyNameIterator<'a> = xcb_xkb_key_name_iterator_t<'a>;

impl<'a> Iterator for KeyNameIterator<'a> {
    type Item = KeyName<'a>;
    fn next(&mut self) -> std::option::Option<KeyName<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_key_name_iterator_t;
                let data = (*iter).data;
                xcb_xkb_key_name_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type KeyAlias<'a> = base::StructPtr<'a, xcb_xkb_key_alias_t>;

impl<'a> KeyAlias<'a> {
    pub fn real(&self) -> &[c_char] {
        unsafe {
            &(*self.ptr).real
        }
    }
    pub fn alias(&self) -> &[c_char] {
        unsafe {
            &(*self.ptr).alias
        }
    }
}

pub type KeyAliasIterator<'a> = xcb_xkb_key_alias_iterator_t<'a>;

impl<'a> Iterator for KeyAliasIterator<'a> {
    type Item = KeyAlias<'a>;
    fn next(&mut self) -> std::option::Option<KeyAlias<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_key_alias_iterator_t;
                let data = (*iter).data;
                xcb_xkb_key_alias_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type CountedString16<'a> = base::StructPtr<'a, xcb_xkb_counted_string_16_t>;

impl<'a> CountedString16<'a> {
    pub fn length(&self) -> u16 {
        unsafe {
            (*self.ptr).length
        }
    }
    pub fn string(&self) -> &str {
        unsafe {
            let field = self.ptr;
            let len = xcb_xkb_counted_string_16_string_length(field) as usize;
            let data = xcb_xkb_counted_string_16_string(field);
            let slice = std::slice::from_raw_parts(data as *const u8, len);
            // should we check what comes from X?
            std::str::from_utf8_unchecked(&slice)
        }
    }
    pub fn alignment_pad<T>(&self) -> &[T] {
        unsafe {
            let field = self.ptr;
            let len = xcb_xkb_counted_string_16_alignment_pad_length(field) as usize;
            let data = xcb_xkb_counted_string_16_alignment_pad(field);
            debug_assert_eq!(len % std::mem::size_of::<T>(), 0);
            std::slice::from_raw_parts(data as *const T, len / std::mem::size_of::<T>())
        }
    }
}

pub type CountedString16Iterator<'a> = xcb_xkb_counted_string_16_iterator_t<'a>;

impl<'a> Iterator for CountedString16Iterator<'a> {
    type Item = CountedString16<'a>;
    fn next(&mut self) -> std::option::Option<CountedString16<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_counted_string_16_iterator_t;
                let data = (*iter).data;
                xcb_xkb_counted_string_16_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct KtMapEntry {
    pub base: xcb_xkb_kt_map_entry_t,
}

impl KtMapEntry {
    #[allow(unused_unsafe)]
    pub fn new(active:     bool,
               mods_mask:  u8,
               level:      u8,
               mods_mods:  u8,
               mods_vmods: u16)
            -> KtMapEntry {
        unsafe {
            KtMapEntry {
                base: xcb_xkb_kt_map_entry_t {
                    active:     if active { 1 } else { 0 },
                    mods_mask:  mods_mask,
                    level:      level,
                    mods_mods:  mods_mods,
                    mods_vmods: mods_vmods,
                    pad0:       [0; 2],
                }
            }
        }
    }
    pub fn active(&self) -> bool {
        unsafe {
            self.base.active != 0
        }
    }
    pub fn mods_mask(&self) -> u8 {
        unsafe {
            self.base.mods_mask
        }
    }
    pub fn level(&self) -> u8 {
        unsafe {
            self.base.level
        }
    }
    pub fn mods_mods(&self) -> u8 {
        unsafe {
            self.base.mods_mods
        }
    }
    pub fn mods_vmods(&self) -> u16 {
        unsafe {
            self.base.mods_vmods
        }
    }
}

pub type KtMapEntryIterator = xcb_xkb_kt_map_entry_iterator_t;

impl Iterator for KtMapEntryIterator {
    type Item = KtMapEntry;
    fn next(&mut self) -> std::option::Option<KtMapEntry> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_kt_map_entry_iterator_t;
                let data = (*iter).data;
                xcb_xkb_kt_map_entry_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

pub type KeyType<'a> = base::StructPtr<'a, xcb_xkb_key_type_t>;

impl<'a> KeyType<'a> {
    pub fn mods_mask(&self) -> u8 {
        unsafe {
            (*self.ptr).mods_mask
        }
    }
    pub fn mods_mods(&self) -> u8 {
        unsafe {
            (*self.ptr).mods_mods
        }
    }
    pub fn mods_vmods(&self) -> u16 {
        unsafe {
            (*self.ptr).mods_vmods
        }
    }
    pub fn numLevels(&self) -> u8 {
        unsafe {
            (*self.ptr).numLevels
        }
    }
    pub fn nMapEntries(&self) -> u8 {
        unsafe {
            (*self.ptr).nMapEntries
        }
    }
    pub fn hasPreserve(&self) -> bool {
        unsafe {
            (*self.ptr).hasPreserve != 0
        }
    }
    pub fn map(&self) -> KtMapEntryIterator {
        unsafe {
            xcb_xkb_key_type_map_iterator(self.ptr)
        }
    }
    pub fn preserve(&self) -> ModDefIterator {
        unsafe {
            xcb_xkb_key_type_preserve_iterator(self.ptr)
        }
    }
}

pub type KeyTypeIterator<'a> = xcb_xkb_key_type_iterator_t<'a>;

impl<'a> Iterator for KeyTypeIterator<'a> {
    type Item = KeyType<'a>;
    fn next(&mut self) -> std::option::Option<KeyType<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_key_type_iterator_t;
                let data = (*iter).data;
                xcb_xkb_key_type_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type KeySymMap<'a> = base::StructPtr<'a, xcb_xkb_key_sym_map_t>;

impl<'a> KeySymMap<'a> {
    pub fn kt_index(&self) -> &[u8] {
        unsafe {
            &(*self.ptr).kt_index
        }
    }
    pub fn groupInfo(&self) -> u8 {
        unsafe {
            (*self.ptr).groupInfo
        }
    }
    pub fn width(&self) -> u8 {
        unsafe {
            (*self.ptr).width
        }
    }
    pub fn nSyms(&self) -> u16 {
        unsafe {
            (*self.ptr).nSyms
        }
    }
    pub fn syms(&self) -> &[xproto::Keysym] {
        unsafe {
            let field = self.ptr;
            let len = xcb_xkb_key_sym_map_syms_length(field) as usize;
            let data = xcb_xkb_key_sym_map_syms(field);
            std::slice::from_raw_parts(data, len)
        }
    }
}

pub type KeySymMapIterator<'a> = xcb_xkb_key_sym_map_iterator_t<'a>;

impl<'a> Iterator for KeySymMapIterator<'a> {
    type Item = KeySymMap<'a>;
    fn next(&mut self) -> std::option::Option<KeySymMap<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_key_sym_map_iterator_t;
                let data = (*iter).data;
                xcb_xkb_key_sym_map_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct CommonBehavior {
    pub base: xcb_xkb_common_behavior_t,
}

impl CommonBehavior {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               data:  u8)
            -> CommonBehavior {
        unsafe {
            CommonBehavior {
                base: xcb_xkb_common_behavior_t {
                    type_: type_,
                    data:  data,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn data(&self) -> u8 {
        unsafe {
            self.base.data
        }
    }
}

pub type CommonBehaviorIterator = xcb_xkb_common_behavior_iterator_t;

impl Iterator for CommonBehaviorIterator {
    type Item = CommonBehavior;
    fn next(&mut self) -> std::option::Option<CommonBehavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_common_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_common_behavior_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct DefaultBehavior {
    pub base: xcb_xkb_default_behavior_t,
}

impl DefaultBehavior {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8)
            -> DefaultBehavior {
        unsafe {
            DefaultBehavior {
                base: xcb_xkb_default_behavior_t {
                    type_: type_,
                    pad0:  0,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
}

pub type DefaultBehaviorIterator = xcb_xkb_default_behavior_iterator_t;

impl Iterator for DefaultBehaviorIterator {
    type Item = DefaultBehavior;
    fn next(&mut self) -> std::option::Option<DefaultBehavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_default_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_default_behavior_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct LockBehavior {
    pub base: xcb_xkb_lock_behavior_t,
}

impl LockBehavior {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8)
            -> LockBehavior {
        unsafe {
            LockBehavior {
                base: xcb_xkb_lock_behavior_t {
                    type_: type_,
                    pad0:  0,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
}

pub type LockBehaviorIterator = xcb_xkb_lock_behavior_iterator_t;

impl Iterator for LockBehaviorIterator {
    type Item = LockBehavior;
    fn next(&mut self) -> std::option::Option<LockBehavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_lock_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_lock_behavior_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct RadioGroupBehavior {
    pub base: xcb_xkb_radio_group_behavior_t,
}

impl RadioGroupBehavior {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               group: u8)
            -> RadioGroupBehavior {
        unsafe {
            RadioGroupBehavior {
                base: xcb_xkb_radio_group_behavior_t {
                    type_: type_,
                    group: group,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn group(&self) -> u8 {
        unsafe {
            self.base.group
        }
    }
}

pub type RadioGroupBehaviorIterator = xcb_xkb_radio_group_behavior_iterator_t;

impl Iterator for RadioGroupBehaviorIterator {
    type Item = RadioGroupBehavior;
    fn next(&mut self) -> std::option::Option<RadioGroupBehavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_radio_group_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_radio_group_behavior_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct OverlayBehavior {
    pub base: xcb_xkb_overlay_behavior_t,
}

impl OverlayBehavior {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               key:   xproto::Keycode)
            -> OverlayBehavior {
        unsafe {
            OverlayBehavior {
                base: xcb_xkb_overlay_behavior_t {
                    type_: type_,
                    key:   key,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn key(&self) -> xproto::Keycode {
        unsafe {
            self.base.key
        }
    }
}

pub type OverlayBehaviorIterator = xcb_xkb_overlay_behavior_iterator_t;

impl Iterator for OverlayBehaviorIterator {
    type Item = OverlayBehavior;
    fn next(&mut self) -> std::option::Option<OverlayBehavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_overlay_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_overlay_behavior_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct PermamentLockBehavior {
    pub base: xcb_xkb_permament_lock_behavior_t,
}

impl PermamentLockBehavior {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8)
            -> PermamentLockBehavior {
        unsafe {
            PermamentLockBehavior {
                base: xcb_xkb_permament_lock_behavior_t {
                    type_: type_,
                    pad0:  0,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
}

pub type PermamentLockBehaviorIterator = xcb_xkb_permament_lock_behavior_iterator_t;

impl Iterator for PermamentLockBehaviorIterator {
    type Item = PermamentLockBehavior;
    fn next(&mut self) -> std::option::Option<PermamentLockBehavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_permament_lock_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_permament_lock_behavior_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct PermamentRadioGroupBehavior {
    pub base: xcb_xkb_permament_radio_group_behavior_t,
}

impl PermamentRadioGroupBehavior {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               group: u8)
            -> PermamentRadioGroupBehavior {
        unsafe {
            PermamentRadioGroupBehavior {
                base: xcb_xkb_permament_radio_group_behavior_t {
                    type_: type_,
                    group: group,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn group(&self) -> u8 {
        unsafe {
            self.base.group
        }
    }
}

pub type PermamentRadioGroupBehaviorIterator = xcb_xkb_permament_radio_group_behavior_iterator_t;

impl Iterator for PermamentRadioGroupBehaviorIterator {
    type Item = PermamentRadioGroupBehavior;
    fn next(&mut self) -> std::option::Option<PermamentRadioGroupBehavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_permament_radio_group_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_permament_radio_group_behavior_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct PermamentOverlayBehavior {
    pub base: xcb_xkb_permament_overlay_behavior_t,
}

impl PermamentOverlayBehavior {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               key:   xproto::Keycode)
            -> PermamentOverlayBehavior {
        unsafe {
            PermamentOverlayBehavior {
                base: xcb_xkb_permament_overlay_behavior_t {
                    type_: type_,
                    key:   key,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn key(&self) -> xproto::Keycode {
        unsafe {
            self.base.key
        }
    }
}

pub type PermamentOverlayBehaviorIterator = xcb_xkb_permament_overlay_behavior_iterator_t;

impl Iterator for PermamentOverlayBehaviorIterator {
    type Item = PermamentOverlayBehavior;
    fn next(&mut self) -> std::option::Option<PermamentOverlayBehavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_permament_overlay_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_permament_overlay_behavior_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

pub type Behavior = xcb_xkb_behavior_t;

impl Behavior {
    pub fn common(&self) -> CommonBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const CommonBehavior;
            *_ptr
        }
    }
    pub fn from_common(common: CommonBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut CommonBehavior;
            *res_ptr = common;
            res
        }
    }
    pub fn default(&self) -> DefaultBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const DefaultBehavior;
            *_ptr
        }
    }
    pub fn from_default(default: DefaultBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut DefaultBehavior;
            *res_ptr = default;
            res
        }
    }
    pub fn lock(&self) -> LockBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const LockBehavior;
            *_ptr
        }
    }
    pub fn from_lock(lock: LockBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut LockBehavior;
            *res_ptr = lock;
            res
        }
    }
    pub fn radioGroup(&self) -> RadioGroupBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const RadioGroupBehavior;
            *_ptr
        }
    }
    pub fn from_radioGroup(radioGroup: RadioGroupBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut RadioGroupBehavior;
            *res_ptr = radioGroup;
            res
        }
    }
    pub fn overlay1(&self) -> OverlayBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const OverlayBehavior;
            *_ptr
        }
    }
    pub fn from_overlay1(overlay1: OverlayBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut OverlayBehavior;
            *res_ptr = overlay1;
            res
        }
    }
    pub fn overlay2(&self) -> OverlayBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const OverlayBehavior;
            *_ptr
        }
    }
    pub fn from_overlay2(overlay2: OverlayBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut OverlayBehavior;
            *res_ptr = overlay2;
            res
        }
    }
    pub fn permamentLock(&self) -> PermamentLockBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const PermamentLockBehavior;
            *_ptr
        }
    }
    pub fn from_permamentLock(permamentLock: PermamentLockBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut PermamentLockBehavior;
            *res_ptr = permamentLock;
            res
        }
    }
    pub fn permamentRadioGroup(&self) -> PermamentRadioGroupBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const PermamentRadioGroupBehavior;
            *_ptr
        }
    }
    pub fn from_permamentRadioGroup(permamentRadioGroup: PermamentRadioGroupBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut PermamentRadioGroupBehavior;
            *res_ptr = permamentRadioGroup;
            res
        }
    }
    pub fn permamentOverlay1(&self) -> PermamentOverlayBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const PermamentOverlayBehavior;
            *_ptr
        }
    }
    pub fn from_permamentOverlay1(permamentOverlay1: PermamentOverlayBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut PermamentOverlayBehavior;
            *res_ptr = permamentOverlay1;
            res
        }
    }
    pub fn permamentOverlay2(&self) -> PermamentOverlayBehavior {
        unsafe {
            let _ptr = self.data.as_ptr() as *const PermamentOverlayBehavior;
            *_ptr
        }
    }
    pub fn from_permamentOverlay2(permamentOverlay2: PermamentOverlayBehavior) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut PermamentOverlayBehavior;
            *res_ptr = permamentOverlay2;
            res
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            let _ptr = self.data.as_ptr() as *const u8;
            *_ptr
        }
    }
    pub fn from_type_(type_: u8) -> Behavior {
        unsafe {
            let mut res = Behavior { data: [0; 2] };
            let res_ptr = res.data.as_mut_ptr() as *mut u8;
            *res_ptr = type_;
            res
        }
    }
}

pub type BehaviorIterator = xcb_xkb_behavior_iterator_t;

impl Iterator for BehaviorIterator {
    type Item = Behavior;
    fn next(&mut self) -> std::option::Option<Behavior> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_behavior_next(iter);
                Some(*data)
            }
        }
    }
}

pub type SetBehavior<'a> = base::StructPtr<'a, xcb_xkb_set_behavior_t>;

impl<'a> SetBehavior<'a> {
    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).keycode
        }
    }
    pub fn behavior(&'a self) -> &'a Behavior {
        unsafe {
            &(*self.ptr).behavior
        }
    }
}

pub type SetBehaviorIterator<'a> = xcb_xkb_set_behavior_iterator_t<'a>;

impl<'a> Iterator for SetBehaviorIterator<'a> {
    type Item = SetBehavior<'a>;
    fn next(&mut self) -> std::option::Option<SetBehavior<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_set_behavior_iterator_t;
                let data = (*iter).data;
                xcb_xkb_set_behavior_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SetExplicit {
    pub base: xcb_xkb_set_explicit_t,
}

impl SetExplicit {
    #[allow(unused_unsafe)]
    pub fn new(keycode:  xproto::Keycode,
               explicit: u8)
            -> SetExplicit {
        unsafe {
            SetExplicit {
                base: xcb_xkb_set_explicit_t {
                    keycode:  keycode,
                    explicit: explicit,
                }
            }
        }
    }
    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            self.base.keycode
        }
    }
    pub fn explicit(&self) -> u8 {
        unsafe {
            self.base.explicit
        }
    }
}

pub type SetExplicitIterator = xcb_xkb_set_explicit_iterator_t;

impl Iterator for SetExplicitIterator {
    type Item = SetExplicit;
    fn next(&mut self) -> std::option::Option<SetExplicit> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_set_explicit_iterator_t;
                let data = (*iter).data;
                xcb_xkb_set_explicit_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct KeyModMap {
    pub base: xcb_xkb_key_mod_map_t,
}

impl KeyModMap {
    #[allow(unused_unsafe)]
    pub fn new(keycode: xproto::Keycode,
               mods:    u8)
            -> KeyModMap {
        unsafe {
            KeyModMap {
                base: xcb_xkb_key_mod_map_t {
                    keycode: keycode,
                    mods:    mods,
                }
            }
        }
    }
    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            self.base.keycode
        }
    }
    pub fn mods(&self) -> u8 {
        unsafe {
            self.base.mods
        }
    }
}

pub type KeyModMapIterator = xcb_xkb_key_mod_map_iterator_t;

impl Iterator for KeyModMapIterator {
    type Item = KeyModMap;
    fn next(&mut self) -> std::option::Option<KeyModMap> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_key_mod_map_iterator_t;
                let data = (*iter).data;
                xcb_xkb_key_mod_map_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct KeyVModMap {
    pub base: xcb_xkb_key_v_mod_map_t,
}

impl KeyVModMap {
    #[allow(unused_unsafe)]
    pub fn new(keycode: xproto::Keycode,
               vmods:   u16)
            -> KeyVModMap {
        unsafe {
            KeyVModMap {
                base: xcb_xkb_key_v_mod_map_t {
                    keycode: keycode,
                    pad0:    0,
                    vmods:   vmods,
                }
            }
        }
    }
    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            self.base.keycode
        }
    }
    pub fn vmods(&self) -> u16 {
        unsafe {
            self.base.vmods
        }
    }
}

pub type KeyVModMapIterator = xcb_xkb_key_v_mod_map_iterator_t;

impl Iterator for KeyVModMapIterator {
    type Item = KeyVModMap;
    fn next(&mut self) -> std::option::Option<KeyVModMap> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_key_v_mod_map_iterator_t;
                let data = (*iter).data;
                xcb_xkb_key_v_mod_map_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct KtSetMapEntry {
    pub base: xcb_xkb_kt_set_map_entry_t,
}

impl KtSetMapEntry {
    #[allow(unused_unsafe)]
    pub fn new(level:       u8,
               realMods:    u8,
               virtualMods: u16)
            -> KtSetMapEntry {
        unsafe {
            KtSetMapEntry {
                base: xcb_xkb_kt_set_map_entry_t {
                    level:       level,
                    realMods:    realMods,
                    virtualMods: virtualMods,
                }
            }
        }
    }
    pub fn level(&self) -> u8 {
        unsafe {
            self.base.level
        }
    }
    pub fn realMods(&self) -> u8 {
        unsafe {
            self.base.realMods
        }
    }
    pub fn virtualMods(&self) -> u16 {
        unsafe {
            self.base.virtualMods
        }
    }
}

pub type KtSetMapEntryIterator = xcb_xkb_kt_set_map_entry_iterator_t;

impl Iterator for KtSetMapEntryIterator {
    type Item = KtSetMapEntry;
    fn next(&mut self) -> std::option::Option<KtSetMapEntry> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_kt_set_map_entry_iterator_t;
                let data = (*iter).data;
                xcb_xkb_kt_set_map_entry_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

pub type SetKeyType<'a> = base::StructPtr<'a, xcb_xkb_set_key_type_t>;

impl<'a> SetKeyType<'a> {
    pub fn mask(&self) -> u8 {
        unsafe {
            (*self.ptr).mask
        }
    }
    pub fn realMods(&self) -> u8 {
        unsafe {
            (*self.ptr).realMods
        }
    }
    pub fn virtualMods(&self) -> u16 {
        unsafe {
            (*self.ptr).virtualMods
        }
    }
    pub fn numLevels(&self) -> u8 {
        unsafe {
            (*self.ptr).numLevels
        }
    }
    pub fn nMapEntries(&self) -> u8 {
        unsafe {
            (*self.ptr).nMapEntries
        }
    }
    pub fn preserve(&self) -> bool {
        unsafe {
            (*self.ptr).preserve != 0
        }
    }
    pub fn entries(&self) -> KtSetMapEntryIterator {
        unsafe {
            xcb_xkb_set_key_type_entries_iterator(self.ptr)
        }
    }
    pub fn preserve_entries(&self) -> KtSetMapEntryIterator {
        unsafe {
            xcb_xkb_set_key_type_preserve_entries_iterator(self.ptr)
        }
    }
}

pub type SetKeyTypeIterator<'a> = xcb_xkb_set_key_type_iterator_t<'a>;

impl<'a> Iterator for SetKeyTypeIterator<'a> {
    type Item = SetKeyType<'a>;
    fn next(&mut self) -> std::option::Option<SetKeyType<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_set_key_type_iterator_t;
                let data = (*iter).data;
                xcb_xkb_set_key_type_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type Outline<'a> = base::StructPtr<'a, xcb_xkb_outline_t>;

impl<'a> Outline<'a> {
    pub fn nPoints(&self) -> u8 {
        unsafe {
            (*self.ptr).nPoints
        }
    }
    pub fn cornerRadius(&self) -> u8 {
        unsafe {
            (*self.ptr).cornerRadius
        }
    }
    pub fn points(&self) -> xproto::PointIterator {
        unsafe {
            xcb_xkb_outline_points_iterator(self.ptr)
        }
    }
}

pub type OutlineIterator<'a> = xcb_xkb_outline_iterator_t<'a>;

impl<'a> Iterator for OutlineIterator<'a> {
    type Item = Outline<'a>;
    fn next(&mut self) -> std::option::Option<Outline<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_outline_iterator_t;
                let data = (*iter).data;
                xcb_xkb_outline_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type Shape<'a> = base::StructPtr<'a, xcb_xkb_shape_t>;

impl<'a> Shape<'a> {
    pub fn name(&self) -> xproto::Atom {
        unsafe {
            (*self.ptr).name
        }
    }
    pub fn nOutlines(&self) -> u8 {
        unsafe {
            (*self.ptr).nOutlines
        }
    }
    pub fn primaryNdx(&self) -> u8 {
        unsafe {
            (*self.ptr).primaryNdx
        }
    }
    pub fn approxNdx(&self) -> u8 {
        unsafe {
            (*self.ptr).approxNdx
        }
    }
    pub fn outlines(&self) -> OutlineIterator<'a> {
        unsafe {
            xcb_xkb_shape_outlines_iterator(self.ptr)
        }
    }
}

pub type ShapeIterator<'a> = xcb_xkb_shape_iterator_t<'a>;

impl<'a> Iterator for ShapeIterator<'a> {
    type Item = Shape<'a>;
    fn next(&mut self) -> std::option::Option<Shape<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_shape_iterator_t;
                let data = (*iter).data;
                xcb_xkb_shape_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type Key<'a> = base::StructPtr<'a, xcb_xkb_key_t>;

impl<'a> Key<'a> {
    pub fn name(&self) -> &[String8] {
        unsafe {
            &(*self.ptr).name
        }
    }
    pub fn gap(&self) -> i16 {
        unsafe {
            (*self.ptr).gap
        }
    }
    pub fn shapeNdx(&self) -> u8 {
        unsafe {
            (*self.ptr).shapeNdx
        }
    }
    pub fn colorNdx(&self) -> u8 {
        unsafe {
            (*self.ptr).colorNdx
        }
    }
}

pub type KeyIterator<'a> = xcb_xkb_key_iterator_t<'a>;

impl<'a> Iterator for KeyIterator<'a> {
    type Item = Key<'a>;
    fn next(&mut self) -> std::option::Option<Key<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_key_iterator_t;
                let data = (*iter).data;
                xcb_xkb_key_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type OverlayKey<'a> = base::StructPtr<'a, xcb_xkb_overlay_key_t>;

impl<'a> OverlayKey<'a> {
    pub fn over(&self) -> &[String8] {
        unsafe {
            &(*self.ptr).over
        }
    }
    pub fn under(&self) -> &[String8] {
        unsafe {
            &(*self.ptr).under
        }
    }
}

pub type OverlayKeyIterator<'a> = xcb_xkb_overlay_key_iterator_t<'a>;

impl<'a> Iterator for OverlayKeyIterator<'a> {
    type Item = OverlayKey<'a>;
    fn next(&mut self) -> std::option::Option<OverlayKey<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_overlay_key_iterator_t;
                let data = (*iter).data;
                xcb_xkb_overlay_key_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type OverlayRow<'a> = base::StructPtr<'a, xcb_xkb_overlay_row_t>;

impl<'a> OverlayRow<'a> {
    pub fn rowUnder(&self) -> u8 {
        unsafe {
            (*self.ptr).rowUnder
        }
    }
    pub fn nKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeys
        }
    }
    pub fn keys(&self) -> OverlayKeyIterator<'a> {
        unsafe {
            xcb_xkb_overlay_row_keys_iterator(self.ptr)
        }
    }
}

pub type OverlayRowIterator<'a> = xcb_xkb_overlay_row_iterator_t<'a>;

impl<'a> Iterator for OverlayRowIterator<'a> {
    type Item = OverlayRow<'a>;
    fn next(&mut self) -> std::option::Option<OverlayRow<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_overlay_row_iterator_t;
                let data = (*iter).data;
                xcb_xkb_overlay_row_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type Overlay<'a> = base::StructPtr<'a, xcb_xkb_overlay_t>;

impl<'a> Overlay<'a> {
    pub fn name(&self) -> xproto::Atom {
        unsafe {
            (*self.ptr).name
        }
    }
    pub fn nRows(&self) -> u8 {
        unsafe {
            (*self.ptr).nRows
        }
    }
    pub fn rows(&self) -> OverlayRowIterator<'a> {
        unsafe {
            xcb_xkb_overlay_rows_iterator(self.ptr)
        }
    }
}

pub type OverlayIterator<'a> = xcb_xkb_overlay_iterator_t<'a>;

impl<'a> Iterator for OverlayIterator<'a> {
    type Item = Overlay<'a>;
    fn next(&mut self) -> std::option::Option<Overlay<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_overlay_iterator_t;
                let data = (*iter).data;
                xcb_xkb_overlay_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type Row<'a> = base::StructPtr<'a, xcb_xkb_row_t>;

impl<'a> Row<'a> {
    pub fn top(&self) -> i16 {
        unsafe {
            (*self.ptr).top
        }
    }
    pub fn left(&self) -> i16 {
        unsafe {
            (*self.ptr).left
        }
    }
    pub fn nKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeys
        }
    }
    pub fn vertical(&self) -> bool {
        unsafe {
            (*self.ptr).vertical != 0
        }
    }
    pub fn keys(&self) -> KeyIterator<'a> {
        unsafe {
            xcb_xkb_row_keys_iterator(self.ptr)
        }
    }
}

pub type RowIterator<'a> = xcb_xkb_row_iterator_t<'a>;

impl<'a> Iterator for RowIterator<'a> {
    type Item = Row<'a>;
    fn next(&mut self) -> std::option::Option<Row<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_row_iterator_t;
                let data = (*iter).data;
                xcb_xkb_row_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type Listing<'a> = base::StructPtr<'a, xcb_xkb_listing_t>;

impl<'a> Listing<'a> {
    pub fn flags(&self) -> u16 {
        unsafe {
            (*self.ptr).flags
        }
    }
    pub fn length(&self) -> u16 {
        unsafe {
            (*self.ptr).length
        }
    }
    pub fn string(&self) -> &[String8] {
        unsafe {
            let field = self.ptr;
            let len = xcb_xkb_listing_string_length(field) as usize;
            let data = xcb_xkb_listing_string(field);
            std::slice::from_raw_parts(data, len)
        }
    }
}

pub type ListingIterator<'a> = xcb_xkb_listing_iterator_t<'a>;

impl<'a> Iterator for ListingIterator<'a> {
    type Item = Listing<'a>;
    fn next(&mut self) -> std::option::Option<Listing<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_listing_iterator_t;
                let data = (*iter).data;
                xcb_xkb_listing_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type DeviceLedInfo<'a> = base::StructPtr<'a, xcb_xkb_device_led_info_t>;

impl<'a> DeviceLedInfo<'a> {
    pub fn ledClass(&self) -> LedClassSpec {
        unsafe {
            (*self.ptr).ledClass
        }
    }
    pub fn ledID(&self) -> IdSpec {
        unsafe {
            (*self.ptr).ledID
        }
    }
    pub fn namesPresent(&self) -> u32 {
        unsafe {
            (*self.ptr).namesPresent
        }
    }
    pub fn mapsPresent(&self) -> u32 {
        unsafe {
            (*self.ptr).mapsPresent
        }
    }
    pub fn physIndicators(&self) -> u32 {
        unsafe {
            (*self.ptr).physIndicators
        }
    }
    pub fn state(&self) -> u32 {
        unsafe {
            (*self.ptr).state
        }
    }
    pub fn names(&self) -> &[xproto::Atom] {
        unsafe {
            let field = self.ptr;
            let len = xcb_xkb_device_led_info_names_length(field) as usize;
            let data = xcb_xkb_device_led_info_names(field);
            std::slice::from_raw_parts(data, len)
        }
    }
    pub fn maps(&self) -> IndicatorMapIterator {
        unsafe {
            xcb_xkb_device_led_info_maps_iterator(self.ptr)
        }
    }
}

pub type DeviceLedInfoIterator<'a> = xcb_xkb_device_led_info_iterator_t<'a>;

impl<'a> Iterator for DeviceLedInfoIterator<'a> {
    type Item = DeviceLedInfo<'a>;
    fn next(&mut self) -> std::option::Option<DeviceLedInfo<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_device_led_info_iterator_t;
                let data = (*iter).data;
                xcb_xkb_device_led_info_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub const KEYBOARD: u8 = 0;

#[derive(Copy, Clone)]
pub struct SaNoAction {
    pub base: xcb_xkb_sa_no_action_t,
}

impl SaNoAction {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8)
            -> SaNoAction {
        unsafe {
            SaNoAction {
                base: xcb_xkb_sa_no_action_t {
                    type_: type_,
                    pad0:  [0; 7],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
}

pub type SaNoActionIterator = xcb_xkb_sa_no_action_iterator_t;

impl Iterator for SaNoActionIterator {
    type Item = SaNoAction;
    fn next(&mut self) -> std::option::Option<SaNoAction> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_no_action_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_no_action_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaSetMods {
    pub base: xcb_xkb_sa_set_mods_t,
}

impl SaSetMods {
    #[allow(unused_unsafe)]
    pub fn new(type_:     u8,
               flags:     u8,
               mask:      u8,
               realMods:  u8,
               vmodsHigh: u8,
               vmodsLow:  u8)
            -> SaSetMods {
        unsafe {
            SaSetMods {
                base: xcb_xkb_sa_set_mods_t {
                    type_:     type_,
                    flags:     flags,
                    mask:      mask,
                    realMods:  realMods,
                    vmodsHigh: vmodsHigh,
                    vmodsLow:  vmodsLow,
                    pad0:      [0; 2],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn mask(&self) -> u8 {
        unsafe {
            self.base.mask
        }
    }
    pub fn realMods(&self) -> u8 {
        unsafe {
            self.base.realMods
        }
    }
    pub fn vmodsHigh(&self) -> u8 {
        unsafe {
            self.base.vmodsHigh
        }
    }
    pub fn vmodsLow(&self) -> u8 {
        unsafe {
            self.base.vmodsLow
        }
    }
}

pub type SaSetModsIterator = xcb_xkb_sa_set_mods_iterator_t;

impl Iterator for SaSetModsIterator {
    type Item = SaSetMods;
    fn next(&mut self) -> std::option::Option<SaSetMods> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_set_mods_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_set_mods_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaLatchMods {
    pub base: xcb_xkb_sa_latch_mods_t,
}

impl SaLatchMods {
    #[allow(unused_unsafe)]
    pub fn new(type_:     u8,
               flags:     u8,
               mask:      u8,
               realMods:  u8,
               vmodsHigh: u8,
               vmodsLow:  u8)
            -> SaLatchMods {
        unsafe {
            SaLatchMods {
                base: xcb_xkb_sa_latch_mods_t {
                    type_:     type_,
                    flags:     flags,
                    mask:      mask,
                    realMods:  realMods,
                    vmodsHigh: vmodsHigh,
                    vmodsLow:  vmodsLow,
                    pad0:      [0; 2],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn mask(&self) -> u8 {
        unsafe {
            self.base.mask
        }
    }
    pub fn realMods(&self) -> u8 {
        unsafe {
            self.base.realMods
        }
    }
    pub fn vmodsHigh(&self) -> u8 {
        unsafe {
            self.base.vmodsHigh
        }
    }
    pub fn vmodsLow(&self) -> u8 {
        unsafe {
            self.base.vmodsLow
        }
    }
}

pub type SaLatchModsIterator = xcb_xkb_sa_latch_mods_iterator_t;

impl Iterator for SaLatchModsIterator {
    type Item = SaLatchMods;
    fn next(&mut self) -> std::option::Option<SaLatchMods> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_latch_mods_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_latch_mods_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaLockMods {
    pub base: xcb_xkb_sa_lock_mods_t,
}

impl SaLockMods {
    #[allow(unused_unsafe)]
    pub fn new(type_:     u8,
               flags:     u8,
               mask:      u8,
               realMods:  u8,
               vmodsHigh: u8,
               vmodsLow:  u8)
            -> SaLockMods {
        unsafe {
            SaLockMods {
                base: xcb_xkb_sa_lock_mods_t {
                    type_:     type_,
                    flags:     flags,
                    mask:      mask,
                    realMods:  realMods,
                    vmodsHigh: vmodsHigh,
                    vmodsLow:  vmodsLow,
                    pad0:      [0; 2],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn mask(&self) -> u8 {
        unsafe {
            self.base.mask
        }
    }
    pub fn realMods(&self) -> u8 {
        unsafe {
            self.base.realMods
        }
    }
    pub fn vmodsHigh(&self) -> u8 {
        unsafe {
            self.base.vmodsHigh
        }
    }
    pub fn vmodsLow(&self) -> u8 {
        unsafe {
            self.base.vmodsLow
        }
    }
}

pub type SaLockModsIterator = xcb_xkb_sa_lock_mods_iterator_t;

impl Iterator for SaLockModsIterator {
    type Item = SaLockMods;
    fn next(&mut self) -> std::option::Option<SaLockMods> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_lock_mods_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_lock_mods_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaSetGroup {
    pub base: xcb_xkb_sa_set_group_t,
}

impl SaSetGroup {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               flags: u8,
               group: i8)
            -> SaSetGroup {
        unsafe {
            SaSetGroup {
                base: xcb_xkb_sa_set_group_t {
                    type_: type_,
                    flags: flags,
                    group: group,
                    pad0:  [0; 5],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn group(&self) -> i8 {
        unsafe {
            self.base.group
        }
    }
}

pub type SaSetGroupIterator = xcb_xkb_sa_set_group_iterator_t;

impl Iterator for SaSetGroupIterator {
    type Item = SaSetGroup;
    fn next(&mut self) -> std::option::Option<SaSetGroup> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_set_group_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_set_group_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaLatchGroup {
    pub base: xcb_xkb_sa_latch_group_t,
}

impl SaLatchGroup {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               flags: u8,
               group: i8)
            -> SaLatchGroup {
        unsafe {
            SaLatchGroup {
                base: xcb_xkb_sa_latch_group_t {
                    type_: type_,
                    flags: flags,
                    group: group,
                    pad0:  [0; 5],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn group(&self) -> i8 {
        unsafe {
            self.base.group
        }
    }
}

pub type SaLatchGroupIterator = xcb_xkb_sa_latch_group_iterator_t;

impl Iterator for SaLatchGroupIterator {
    type Item = SaLatchGroup;
    fn next(&mut self) -> std::option::Option<SaLatchGroup> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_latch_group_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_latch_group_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaLockGroup {
    pub base: xcb_xkb_sa_lock_group_t,
}

impl SaLockGroup {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               flags: u8,
               group: i8)
            -> SaLockGroup {
        unsafe {
            SaLockGroup {
                base: xcb_xkb_sa_lock_group_t {
                    type_: type_,
                    flags: flags,
                    group: group,
                    pad0:  [0; 5],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn group(&self) -> i8 {
        unsafe {
            self.base.group
        }
    }
}

pub type SaLockGroupIterator = xcb_xkb_sa_lock_group_iterator_t;

impl Iterator for SaLockGroupIterator {
    type Item = SaLockGroup;
    fn next(&mut self) -> std::option::Option<SaLockGroup> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_lock_group_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_lock_group_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaMovePtr {
    pub base: xcb_xkb_sa_move_ptr_t,
}

impl SaMovePtr {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8,
               flags: u8,
               xHigh: i8,
               xLow:  u8,
               yHigh: i8,
               yLow:  u8)
            -> SaMovePtr {
        unsafe {
            SaMovePtr {
                base: xcb_xkb_sa_move_ptr_t {
                    type_: type_,
                    flags: flags,
                    xHigh: xHigh,
                    xLow:  xLow,
                    yHigh: yHigh,
                    yLow:  yLow,
                    pad0:  [0; 2],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn xHigh(&self) -> i8 {
        unsafe {
            self.base.xHigh
        }
    }
    pub fn xLow(&self) -> u8 {
        unsafe {
            self.base.xLow
        }
    }
    pub fn yHigh(&self) -> i8 {
        unsafe {
            self.base.yHigh
        }
    }
    pub fn yLow(&self) -> u8 {
        unsafe {
            self.base.yLow
        }
    }
}

pub type SaMovePtrIterator = xcb_xkb_sa_move_ptr_iterator_t;

impl Iterator for SaMovePtrIterator {
    type Item = SaMovePtr;
    fn next(&mut self) -> std::option::Option<SaMovePtr> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_move_ptr_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_move_ptr_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaPtrBtn {
    pub base: xcb_xkb_sa_ptr_btn_t,
}

impl SaPtrBtn {
    #[allow(unused_unsafe)]
    pub fn new(type_:  u8,
               flags:  u8,
               count:  u8,
               button: u8)
            -> SaPtrBtn {
        unsafe {
            SaPtrBtn {
                base: xcb_xkb_sa_ptr_btn_t {
                    type_:  type_,
                    flags:  flags,
                    count:  count,
                    button: button,
                    pad0:   [0; 4],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn count(&self) -> u8 {
        unsafe {
            self.base.count
        }
    }
    pub fn button(&self) -> u8 {
        unsafe {
            self.base.button
        }
    }
}

pub type SaPtrBtnIterator = xcb_xkb_sa_ptr_btn_iterator_t;

impl Iterator for SaPtrBtnIterator {
    type Item = SaPtrBtn;
    fn next(&mut self) -> std::option::Option<SaPtrBtn> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_ptr_btn_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_ptr_btn_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaLockPtrBtn {
    pub base: xcb_xkb_sa_lock_ptr_btn_t,
}

impl SaLockPtrBtn {
    #[allow(unused_unsafe)]
    pub fn new(type_:  u8,
               flags:  u8,
               button: u8)
            -> SaLockPtrBtn {
        unsafe {
            SaLockPtrBtn {
                base: xcb_xkb_sa_lock_ptr_btn_t {
                    type_:  type_,
                    flags:  flags,
                    pad0:   0,
                    button: button,
                    pad1:   [0; 4],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn button(&self) -> u8 {
        unsafe {
            self.base.button
        }
    }
}

pub type SaLockPtrBtnIterator = xcb_xkb_sa_lock_ptr_btn_iterator_t;

impl Iterator for SaLockPtrBtnIterator {
    type Item = SaLockPtrBtn;
    fn next(&mut self) -> std::option::Option<SaLockPtrBtn> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_lock_ptr_btn_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_lock_ptr_btn_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaSetPtrDflt {
    pub base: xcb_xkb_sa_set_ptr_dflt_t,
}

impl SaSetPtrDflt {
    #[allow(unused_unsafe)]
    pub fn new(type_:  u8,
               flags:  u8,
               affect: u8,
               value:  i8)
            -> SaSetPtrDflt {
        unsafe {
            SaSetPtrDflt {
                base: xcb_xkb_sa_set_ptr_dflt_t {
                    type_:  type_,
                    flags:  flags,
                    affect: affect,
                    value:  value,
                    pad0:   [0; 4],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn affect(&self) -> u8 {
        unsafe {
            self.base.affect
        }
    }
    pub fn value(&self) -> i8 {
        unsafe {
            self.base.value
        }
    }
}

pub type SaSetPtrDfltIterator = xcb_xkb_sa_set_ptr_dflt_iterator_t;

impl Iterator for SaSetPtrDfltIterator {
    type Item = SaSetPtrDflt;
    fn next(&mut self) -> std::option::Option<SaSetPtrDflt> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_set_ptr_dflt_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_set_ptr_dflt_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaIsoLock {
    pub base: xcb_xkb_sa_iso_lock_t,
}

impl SaIsoLock {
    #[allow(unused_unsafe)]
    pub fn new(type_:     u8,
               flags:     u8,
               mask:      u8,
               realMods:  u8,
               group:     i8,
               affect:    u8,
               vmodsHigh: u8,
               vmodsLow:  u8)
            -> SaIsoLock {
        unsafe {
            SaIsoLock {
                base: xcb_xkb_sa_iso_lock_t {
                    type_:     type_,
                    flags:     flags,
                    mask:      mask,
                    realMods:  realMods,
                    group:     group,
                    affect:    affect,
                    vmodsHigh: vmodsHigh,
                    vmodsLow:  vmodsLow,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn mask(&self) -> u8 {
        unsafe {
            self.base.mask
        }
    }
    pub fn realMods(&self) -> u8 {
        unsafe {
            self.base.realMods
        }
    }
    pub fn group(&self) -> i8 {
        unsafe {
            self.base.group
        }
    }
    pub fn affect(&self) -> u8 {
        unsafe {
            self.base.affect
        }
    }
    pub fn vmodsHigh(&self) -> u8 {
        unsafe {
            self.base.vmodsHigh
        }
    }
    pub fn vmodsLow(&self) -> u8 {
        unsafe {
            self.base.vmodsLow
        }
    }
}

pub type SaIsoLockIterator = xcb_xkb_sa_iso_lock_iterator_t;

impl Iterator for SaIsoLockIterator {
    type Item = SaIsoLock;
    fn next(&mut self) -> std::option::Option<SaIsoLock> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_iso_lock_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_iso_lock_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaTerminate {
    pub base: xcb_xkb_sa_terminate_t,
}

impl SaTerminate {
    #[allow(unused_unsafe)]
    pub fn new(type_: u8)
            -> SaTerminate {
        unsafe {
            SaTerminate {
                base: xcb_xkb_sa_terminate_t {
                    type_: type_,
                    pad0:  [0; 7],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
}

pub type SaTerminateIterator = xcb_xkb_sa_terminate_iterator_t;

impl Iterator for SaTerminateIterator {
    type Item = SaTerminate;
    fn next(&mut self) -> std::option::Option<SaTerminate> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_terminate_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_terminate_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaSwitchScreen {
    pub base: xcb_xkb_sa_switch_screen_t,
}

impl SaSwitchScreen {
    #[allow(unused_unsafe)]
    pub fn new(type_:     u8,
               flags:     u8,
               newScreen: i8)
            -> SaSwitchScreen {
        unsafe {
            SaSwitchScreen {
                base: xcb_xkb_sa_switch_screen_t {
                    type_:     type_,
                    flags:     flags,
                    newScreen: newScreen,
                    pad0:      [0; 5],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn newScreen(&self) -> i8 {
        unsafe {
            self.base.newScreen
        }
    }
}

pub type SaSwitchScreenIterator = xcb_xkb_sa_switch_screen_iterator_t;

impl Iterator for SaSwitchScreenIterator {
    type Item = SaSwitchScreen;
    fn next(&mut self) -> std::option::Option<SaSwitchScreen> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_switch_screen_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_switch_screen_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaSetControls {
    pub base: xcb_xkb_sa_set_controls_t,
}

impl SaSetControls {
    #[allow(unused_unsafe)]
    pub fn new(type_:         u8,
               boolCtrlsHigh: u8,
               boolCtrlsLow:  u8)
            -> SaSetControls {
        unsafe {
            SaSetControls {
                base: xcb_xkb_sa_set_controls_t {
                    type_:         type_,
                    pad0:          [0; 3],
                    boolCtrlsHigh: boolCtrlsHigh,
                    boolCtrlsLow:  boolCtrlsLow,
                    pad1:          [0; 2],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn boolCtrlsHigh(&self) -> u8 {
        unsafe {
            self.base.boolCtrlsHigh
        }
    }
    pub fn boolCtrlsLow(&self) -> u8 {
        unsafe {
            self.base.boolCtrlsLow
        }
    }
}

pub type SaSetControlsIterator = xcb_xkb_sa_set_controls_iterator_t;

impl Iterator for SaSetControlsIterator {
    type Item = SaSetControls;
    fn next(&mut self) -> std::option::Option<SaSetControls> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_set_controls_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_set_controls_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaLockControls {
    pub base: xcb_xkb_sa_lock_controls_t,
}

impl SaLockControls {
    #[allow(unused_unsafe)]
    pub fn new(type_:         u8,
               boolCtrlsHigh: u8,
               boolCtrlsLow:  u8)
            -> SaLockControls {
        unsafe {
            SaLockControls {
                base: xcb_xkb_sa_lock_controls_t {
                    type_:         type_,
                    pad0:          [0; 3],
                    boolCtrlsHigh: boolCtrlsHigh,
                    boolCtrlsLow:  boolCtrlsLow,
                    pad1:          [0; 2],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn boolCtrlsHigh(&self) -> u8 {
        unsafe {
            self.base.boolCtrlsHigh
        }
    }
    pub fn boolCtrlsLow(&self) -> u8 {
        unsafe {
            self.base.boolCtrlsLow
        }
    }
}

pub type SaLockControlsIterator = xcb_xkb_sa_lock_controls_iterator_t;

impl Iterator for SaLockControlsIterator {
    type Item = SaLockControls;
    fn next(&mut self) -> std::option::Option<SaLockControls> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_lock_controls_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_lock_controls_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

pub type SaActionMessage<'a> = base::StructPtr<'a, xcb_xkb_sa_action_message_t>;

impl<'a> SaActionMessage<'a> {
    pub fn type_(&self) -> u8 {
        unsafe {
            (*self.ptr).type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            (*self.ptr).flags
        }
    }
    pub fn message(&self) -> &[u8] {
        unsafe {
            &(*self.ptr).message
        }
    }
}

pub type SaActionMessageIterator<'a> = xcb_xkb_sa_action_message_iterator_t<'a>;

impl<'a> Iterator for SaActionMessageIterator<'a> {
    type Item = SaActionMessage<'a>;
    fn next(&mut self) -> std::option::Option<SaActionMessage<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_action_message_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_action_message_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaRedirectKey {
    pub base: xcb_xkb_sa_redirect_key_t,
}

impl SaRedirectKey {
    #[allow(unused_unsafe)]
    pub fn new(type_:         u8,
               newkey:        xproto::Keycode,
               mask:          u8,
               realModifiers: u8,
               vmodsMaskHigh: u8,
               vmodsMaskLow:  u8,
               vmodsHigh:     u8,
               vmodsLow:      u8)
            -> SaRedirectKey {
        unsafe {
            SaRedirectKey {
                base: xcb_xkb_sa_redirect_key_t {
                    type_:         type_,
                    newkey:        newkey,
                    mask:          mask,
                    realModifiers: realModifiers,
                    vmodsMaskHigh: vmodsMaskHigh,
                    vmodsMaskLow:  vmodsMaskLow,
                    vmodsHigh:     vmodsHigh,
                    vmodsLow:      vmodsLow,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn newkey(&self) -> xproto::Keycode {
        unsafe {
            self.base.newkey
        }
    }
    pub fn mask(&self) -> u8 {
        unsafe {
            self.base.mask
        }
    }
    pub fn realModifiers(&self) -> u8 {
        unsafe {
            self.base.realModifiers
        }
    }
    pub fn vmodsMaskHigh(&self) -> u8 {
        unsafe {
            self.base.vmodsMaskHigh
        }
    }
    pub fn vmodsMaskLow(&self) -> u8 {
        unsafe {
            self.base.vmodsMaskLow
        }
    }
    pub fn vmodsHigh(&self) -> u8 {
        unsafe {
            self.base.vmodsHigh
        }
    }
    pub fn vmodsLow(&self) -> u8 {
        unsafe {
            self.base.vmodsLow
        }
    }
}

pub type SaRedirectKeyIterator = xcb_xkb_sa_redirect_key_iterator_t;

impl Iterator for SaRedirectKeyIterator {
    type Item = SaRedirectKey;
    fn next(&mut self) -> std::option::Option<SaRedirectKey> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_redirect_key_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_redirect_key_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaDeviceBtn {
    pub base: xcb_xkb_sa_device_btn_t,
}

impl SaDeviceBtn {
    #[allow(unused_unsafe)]
    pub fn new(type_:  u8,
               flags:  u8,
               count:  u8,
               button: u8,
               device: u8)
            -> SaDeviceBtn {
        unsafe {
            SaDeviceBtn {
                base: xcb_xkb_sa_device_btn_t {
                    type_:  type_,
                    flags:  flags,
                    count:  count,
                    button: button,
                    device: device,
                    pad0:   [0; 3],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn count(&self) -> u8 {
        unsafe {
            self.base.count
        }
    }
    pub fn button(&self) -> u8 {
        unsafe {
            self.base.button
        }
    }
    pub fn device(&self) -> u8 {
        unsafe {
            self.base.device
        }
    }
}

pub type SaDeviceBtnIterator = xcb_xkb_sa_device_btn_iterator_t;

impl Iterator for SaDeviceBtnIterator {
    type Item = SaDeviceBtn;
    fn next(&mut self) -> std::option::Option<SaDeviceBtn> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_device_btn_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_device_btn_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaLockDeviceBtn {
    pub base: xcb_xkb_sa_lock_device_btn_t,
}

impl SaLockDeviceBtn {
    #[allow(unused_unsafe)]
    pub fn new(type_:  u8,
               flags:  u8,
               button: u8,
               device: u8)
            -> SaLockDeviceBtn {
        unsafe {
            SaLockDeviceBtn {
                base: xcb_xkb_sa_lock_device_btn_t {
                    type_:  type_,
                    flags:  flags,
                    pad0:   0,
                    button: button,
                    device: device,
                    pad1:   [0; 3],
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            self.base.flags
        }
    }
    pub fn button(&self) -> u8 {
        unsafe {
            self.base.button
        }
    }
    pub fn device(&self) -> u8 {
        unsafe {
            self.base.device
        }
    }
}

pub type SaLockDeviceBtnIterator = xcb_xkb_sa_lock_device_btn_iterator_t;

impl Iterator for SaLockDeviceBtnIterator {
    type Item = SaLockDeviceBtn;
    fn next(&mut self) -> std::option::Option<SaLockDeviceBtn> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_lock_device_btn_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_lock_device_btn_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SaDeviceValuator {
    pub base: xcb_xkb_sa_device_valuator_t,
}

impl SaDeviceValuator {
    #[allow(unused_unsafe)]
    pub fn new(type_:     u8,
               device:    u8,
               val1what:  u8,
               val1index: u8,
               val1value: u8,
               val2what:  u8,
               val2index: u8,
               val2value: u8)
            -> SaDeviceValuator {
        unsafe {
            SaDeviceValuator {
                base: xcb_xkb_sa_device_valuator_t {
                    type_:     type_,
                    device:    device,
                    val1what:  val1what,
                    val1index: val1index,
                    val1value: val1value,
                    val2what:  val2what,
                    val2index: val2index,
                    val2value: val2value,
                }
            }
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            self.base.type_
        }
    }
    pub fn device(&self) -> u8 {
        unsafe {
            self.base.device
        }
    }
    pub fn val1what(&self) -> u8 {
        unsafe {
            self.base.val1what
        }
    }
    pub fn val1index(&self) -> u8 {
        unsafe {
            self.base.val1index
        }
    }
    pub fn val1value(&self) -> u8 {
        unsafe {
            self.base.val1value
        }
    }
    pub fn val2what(&self) -> u8 {
        unsafe {
            self.base.val2what
        }
    }
    pub fn val2index(&self) -> u8 {
        unsafe {
            self.base.val2index
        }
    }
    pub fn val2value(&self) -> u8 {
        unsafe {
            self.base.val2value
        }
    }
}

pub type SaDeviceValuatorIterator = xcb_xkb_sa_device_valuator_iterator_t;

impl Iterator for SaDeviceValuatorIterator {
    type Item = SaDeviceValuator;
    fn next(&mut self) -> std::option::Option<SaDeviceValuator> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sa_device_valuator_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sa_device_valuator_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

pub type SiAction<'a> = base::StructPtr<'a, xcb_xkb_si_action_t>;

impl<'a> SiAction<'a> {
    pub fn type_(&self) -> u8 {
        unsafe {
            (*self.ptr).type_
        }
    }
    pub fn data(&self) -> &[u8] {
        unsafe {
            &(*self.ptr).data
        }
    }
}

pub type SiActionIterator<'a> = xcb_xkb_si_action_iterator_t<'a>;

impl<'a> Iterator for SiActionIterator<'a> {
    type Item = SiAction<'a>;
    fn next(&mut self) -> std::option::Option<SiAction<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_si_action_iterator_t;
                let data = (*iter).data;
                xcb_xkb_si_action_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type SymInterpret<'a> = base::StructPtr<'a, xcb_xkb_sym_interpret_t>;

impl<'a> SymInterpret<'a> {
    pub fn sym(&self) -> xproto::Keysym {
        unsafe {
            (*self.ptr).sym
        }
    }
    pub fn mods(&self) -> u8 {
        unsafe {
            (*self.ptr).mods
        }
    }
    pub fn match_(&self) -> u8 {
        unsafe {
            (*self.ptr).match_
        }
    }
    pub fn virtualMod(&self) -> u8 {
        unsafe {
            (*self.ptr).virtualMod
        }
    }
    pub fn flags(&self) -> u8 {
        unsafe {
            (*self.ptr).flags
        }
    }
    pub fn action(&self) -> SiAction {
        unsafe {
            std::mem::transmute(&(*self.ptr).action)
        }
    }
}

pub type SymInterpretIterator<'a> = xcb_xkb_sym_interpret_iterator_t<'a>;

impl<'a> Iterator for SymInterpretIterator<'a> {
    type Item = SymInterpret<'a>;
    fn next(&mut self) -> std::option::Option<SymInterpret<'a>> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_sym_interpret_iterator_t;
                let data = (*iter).data;
                xcb_xkb_sym_interpret_next(iter);
                Some(std::mem::transmute(data))
            }
        }
    }
}

pub type Action = xcb_xkb_action_t;

impl Action {
    pub fn noaction(&self) -> SaNoAction {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaNoAction;
            *_ptr
        }
    }
    pub fn from_noaction(noaction: SaNoAction) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaNoAction;
            *res_ptr = noaction;
            res
        }
    }
    pub fn setmods(&self) -> SaSetMods {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaSetMods;
            *_ptr
        }
    }
    pub fn from_setmods(setmods: SaSetMods) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaSetMods;
            *res_ptr = setmods;
            res
        }
    }
    pub fn latchmods(&self) -> SaLatchMods {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaLatchMods;
            *_ptr
        }
    }
    pub fn from_latchmods(latchmods: SaLatchMods) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaLatchMods;
            *res_ptr = latchmods;
            res
        }
    }
    pub fn lockmods(&self) -> SaLockMods {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaLockMods;
            *_ptr
        }
    }
    pub fn from_lockmods(lockmods: SaLockMods) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaLockMods;
            *res_ptr = lockmods;
            res
        }
    }
    pub fn setgroup(&self) -> SaSetGroup {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaSetGroup;
            *_ptr
        }
    }
    pub fn from_setgroup(setgroup: SaSetGroup) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaSetGroup;
            *res_ptr = setgroup;
            res
        }
    }
    pub fn latchgroup(&self) -> SaLatchGroup {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaLatchGroup;
            *_ptr
        }
    }
    pub fn from_latchgroup(latchgroup: SaLatchGroup) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaLatchGroup;
            *res_ptr = latchgroup;
            res
        }
    }
    pub fn lockgroup(&self) -> SaLockGroup {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaLockGroup;
            *_ptr
        }
    }
    pub fn from_lockgroup(lockgroup: SaLockGroup) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaLockGroup;
            *res_ptr = lockgroup;
            res
        }
    }
    pub fn moveptr(&self) -> SaMovePtr {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaMovePtr;
            *_ptr
        }
    }
    pub fn from_moveptr(moveptr: SaMovePtr) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaMovePtr;
            *res_ptr = moveptr;
            res
        }
    }
    pub fn ptrbtn(&self) -> SaPtrBtn {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaPtrBtn;
            *_ptr
        }
    }
    pub fn from_ptrbtn(ptrbtn: SaPtrBtn) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaPtrBtn;
            *res_ptr = ptrbtn;
            res
        }
    }
    pub fn lockptrbtn(&self) -> SaLockPtrBtn {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaLockPtrBtn;
            *_ptr
        }
    }
    pub fn from_lockptrbtn(lockptrbtn: SaLockPtrBtn) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaLockPtrBtn;
            *res_ptr = lockptrbtn;
            res
        }
    }
    pub fn setptrdflt(&self) -> SaSetPtrDflt {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaSetPtrDflt;
            *_ptr
        }
    }
    pub fn from_setptrdflt(setptrdflt: SaSetPtrDflt) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaSetPtrDflt;
            *res_ptr = setptrdflt;
            res
        }
    }
    pub fn isolock(&self) -> SaIsoLock {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaIsoLock;
            *_ptr
        }
    }
    pub fn from_isolock(isolock: SaIsoLock) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaIsoLock;
            *res_ptr = isolock;
            res
        }
    }
    pub fn terminate(&self) -> SaTerminate {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaTerminate;
            *_ptr
        }
    }
    pub fn from_terminate(terminate: SaTerminate) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaTerminate;
            *res_ptr = terminate;
            res
        }
    }
    pub fn switchscreen(&self) -> SaSwitchScreen {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaSwitchScreen;
            *_ptr
        }
    }
    pub fn from_switchscreen(switchscreen: SaSwitchScreen) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaSwitchScreen;
            *res_ptr = switchscreen;
            res
        }
    }
    pub fn setcontrols(&self) -> SaSetControls {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaSetControls;
            *_ptr
        }
    }
    pub fn from_setcontrols(setcontrols: SaSetControls) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaSetControls;
            *res_ptr = setcontrols;
            res
        }
    }
    pub fn lockcontrols(&self) -> SaLockControls {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaLockControls;
            *_ptr
        }
    }
    pub fn from_lockcontrols(lockcontrols: SaLockControls) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaLockControls;
            *res_ptr = lockcontrols;
            res
        }
    }
    pub fn message<'a>(&'a self) -> SaActionMessage<'a> {
        unsafe {
            std::mem::transmute(self)
        }
    }
    pub fn redirect(&self) -> SaRedirectKey {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaRedirectKey;
            *_ptr
        }
    }
    pub fn from_redirect(redirect: SaRedirectKey) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaRedirectKey;
            *res_ptr = redirect;
            res
        }
    }
    pub fn devbtn(&self) -> SaDeviceBtn {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaDeviceBtn;
            *_ptr
        }
    }
    pub fn from_devbtn(devbtn: SaDeviceBtn) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaDeviceBtn;
            *res_ptr = devbtn;
            res
        }
    }
    pub fn lockdevbtn(&self) -> SaLockDeviceBtn {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaLockDeviceBtn;
            *_ptr
        }
    }
    pub fn from_lockdevbtn(lockdevbtn: SaLockDeviceBtn) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaLockDeviceBtn;
            *res_ptr = lockdevbtn;
            res
        }
    }
    pub fn devval(&self) -> SaDeviceValuator {
        unsafe {
            let _ptr = self.data.as_ptr() as *const SaDeviceValuator;
            *_ptr
        }
    }
    pub fn from_devval(devval: SaDeviceValuator) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut SaDeviceValuator;
            *res_ptr = devval;
            res
        }
    }
    pub fn type_(&self) -> u8 {
        unsafe {
            let _ptr = self.data.as_ptr() as *const u8;
            *_ptr
        }
    }
    pub fn from_type_(type_: u8) -> Action {
        unsafe {
            let mut res = Action { data: [0; 8] };
            let res_ptr = res.data.as_mut_ptr() as *mut u8;
            *res_ptr = type_;
            res
        }
    }
}

pub type ActionIterator = xcb_xkb_action_iterator_t;

impl Iterator for ActionIterator {
    type Item = Action;
    fn next(&mut self) -> std::option::Option<Action> {
        if self.rem == 0 { None }
        else {
            unsafe {
                let iter = self as *mut xcb_xkb_action_iterator_t;
                let data = (*iter).data;
                xcb_xkb_action_next(iter);
                Some(*data)
            }
        }
    }
}

pub const USE_EXTENSION: u8 = 0;

impl base::CookieSeq for xcb_xkb_use_extension_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type UseExtensionCookie<'a> = base::Cookie<'a, xcb_xkb_use_extension_cookie_t>;

impl<'a> UseExtensionCookie<'a> {
    pub fn get_reply(self) -> Result<UseExtensionReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            UseExtensionReply {
                ptr: xcb_xkb_use_extension_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type UseExtensionReply = base::Reply<xcb_xkb_use_extension_reply_t>;

impl UseExtensionReply {
    pub fn supported(&self) -> bool {
        unsafe {
            (*self.ptr).supported != 0
        }
    }
    pub fn serverMajor(&self) -> u16 {
        unsafe {
            (*self.ptr).serverMajor
        }
    }
    pub fn serverMinor(&self) -> u16 {
        unsafe {
            (*self.ptr).serverMinor
        }
    }
}

pub fn use_extension<'a>(c          : &'a base::Connection,
                         wantedMajor: u16,
                         wantedMinor: u16)
        -> UseExtensionCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_use_extension(c.get_raw_conn(),
                                           wantedMajor as u16,  // 0
                                           wantedMinor as u16);  // 1
        UseExtensionCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn use_extension_unchecked<'a>(c          : &'a base::Connection,
                                   wantedMajor: u16,
                                   wantedMinor: u16)
        -> UseExtensionCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_use_extension_unchecked(c.get_raw_conn(),
                                                     wantedMajor as u16,  // 0
                                                     wantedMinor as u16);  // 1
        UseExtensionCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub type SelectEventsDetails<'a> = base::StructPtr<'a, xcb_xkb_select_events_details_t>;

pub const SELECT_EVENTS: u8 = 1;

pub fn select_events<'a>(c          : &'a base::Connection,
                         deviceSpec : DeviceSpec,
                         affectWhich: u16,
                         clear      : u16,
                         selectAll  : u16,
                         affectMap  : u16,
                         map        : u16,
                         details    : std::option::Option<SelectEventsDetails>)
        -> base::VoidCookie<'a> {
    unsafe {
        let details_ptr = match details {
            Some(p) => p.ptr as *const xcb_xkb_select_events_details_t,
            None => std::ptr::null()
        };
        let cookie = xcb_xkb_select_events(c.get_raw_conn(),
                                           deviceSpec as xcb_xkb_device_spec_t,  // 0
                                           affectWhich as u16,  // 1
                                           clear as u16,  // 2
                                           selectAll as u16,  // 3
                                           affectMap as u16,  // 4
                                           map as u16,  // 5
                                           details_ptr);  // 6
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn select_events_checked<'a>(c          : &'a base::Connection,
                                 deviceSpec : DeviceSpec,
                                 affectWhich: u16,
                                 clear      : u16,
                                 selectAll  : u16,
                                 affectMap  : u16,
                                 map        : u16,
                                 details    : std::option::Option<SelectEventsDetails>)
        -> base::VoidCookie<'a> {
    unsafe {
        let details_ptr = match details {
            Some(p) => p.ptr as *const xcb_xkb_select_events_details_t,
            None => std::ptr::null()
        };
        let cookie = xcb_xkb_select_events_checked(c.get_raw_conn(),
                                                   deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                   affectWhich as u16,  // 1
                                                   clear as u16,  // 2
                                                   selectAll as u16,  // 3
                                                   affectMap as u16,  // 4
                                                   map as u16,  // 5
                                                   details_ptr);  // 6
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const BELL: u8 = 3;

pub fn bell<'a>(c         : &'a base::Connection,
                deviceSpec: DeviceSpec,
                bellClass : BellClassSpec,
                bellID    : IdSpec,
                percent   : i8,
                forceSound: bool,
                eventOnly : bool,
                pitch     : i16,
                duration  : i16,
                name      : xproto::Atom,
                window    : xproto::Window)
        -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_bell(c.get_raw_conn(),
                                  deviceSpec as xcb_xkb_device_spec_t,  // 0
                                  bellClass as xcb_xkb_bell_class_spec_t,  // 1
                                  bellID as xcb_xkb_id_spec_t,  // 2
                                  percent as i8,  // 3
                                  forceSound as u8,  // 4
                                  eventOnly as u8,  // 5
                                  pitch as i16,  // 6
                                  duration as i16,  // 7
                                  name as xcb_atom_t,  // 8
                                  window as xcb_window_t);  // 9
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn bell_checked<'a>(c         : &'a base::Connection,
                        deviceSpec: DeviceSpec,
                        bellClass : BellClassSpec,
                        bellID    : IdSpec,
                        percent   : i8,
                        forceSound: bool,
                        eventOnly : bool,
                        pitch     : i16,
                        duration  : i16,
                        name      : xproto::Atom,
                        window    : xproto::Window)
        -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_bell_checked(c.get_raw_conn(),
                                          deviceSpec as xcb_xkb_device_spec_t,  // 0
                                          bellClass as xcb_xkb_bell_class_spec_t,  // 1
                                          bellID as xcb_xkb_id_spec_t,  // 2
                                          percent as i8,  // 3
                                          forceSound as u8,  // 4
                                          eventOnly as u8,  // 5
                                          pitch as i16,  // 6
                                          duration as i16,  // 7
                                          name as xcb_atom_t,  // 8
                                          window as xcb_window_t);  // 9
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const GET_STATE: u8 = 4;

impl base::CookieSeq for xcb_xkb_get_state_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetStateCookie<'a> = base::Cookie<'a, xcb_xkb_get_state_cookie_t>;

impl<'a> GetStateCookie<'a> {
    pub fn get_reply(self) -> Result<GetStateReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetStateReply {
                ptr: xcb_xkb_get_state_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetStateReply = base::Reply<xcb_xkb_get_state_reply_t>;

impl GetStateReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn mods(&self) -> u8 {
        unsafe {
            (*self.ptr).mods
        }
    }
    pub fn baseMods(&self) -> u8 {
        unsafe {
            (*self.ptr).baseMods
        }
    }
    pub fn latchedMods(&self) -> u8 {
        unsafe {
            (*self.ptr).latchedMods
        }
    }
    pub fn lockedMods(&self) -> u8 {
        unsafe {
            (*self.ptr).lockedMods
        }
    }
    pub fn group(&self) -> u8 {
        unsafe {
            (*self.ptr).group
        }
    }
    pub fn lockedGroup(&self) -> u8 {
        unsafe {
            (*self.ptr).lockedGroup
        }
    }
    pub fn baseGroup(&self) -> i16 {
        unsafe {
            (*self.ptr).baseGroup
        }
    }
    pub fn latchedGroup(&self) -> i16 {
        unsafe {
            (*self.ptr).latchedGroup
        }
    }
    pub fn compatState(&self) -> u8 {
        unsafe {
            (*self.ptr).compatState
        }
    }
    pub fn grabMods(&self) -> u8 {
        unsafe {
            (*self.ptr).grabMods
        }
    }
    pub fn compatGrabMods(&self) -> u8 {
        unsafe {
            (*self.ptr).compatGrabMods
        }
    }
    pub fn lookupMods(&self) -> u8 {
        unsafe {
            (*self.ptr).lookupMods
        }
    }
    pub fn compatLookupMods(&self) -> u8 {
        unsafe {
            (*self.ptr).compatLookupMods
        }
    }
    pub fn ptrBtnState(&self) -> u16 {
        unsafe {
            (*self.ptr).ptrBtnState
        }
    }
}

pub fn get_state<'a>(c         : &'a base::Connection,
                     deviceSpec: DeviceSpec)
        -> GetStateCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_state(c.get_raw_conn(),
                                       deviceSpec as xcb_xkb_device_spec_t);  // 0
        GetStateCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_state_unchecked<'a>(c         : &'a base::Connection,
                               deviceSpec: DeviceSpec)
        -> GetStateCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_state_unchecked(c.get_raw_conn(),
                                                 deviceSpec as xcb_xkb_device_spec_t);  // 0
        GetStateCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const LATCH_LOCK_STATE: u8 = 5;

pub fn latch_lock_state<'a>(c               : &'a base::Connection,
                            deviceSpec      : DeviceSpec,
                            affectModLocks  : u8,
                            modLocks        : u8,
                            lockGroup       : bool,
                            groupLock       : u8,
                            affectModLatches: u8,
                            latchGroup      : bool,
                            groupLatch      : u16)
        -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_latch_lock_state(c.get_raw_conn(),
                                              deviceSpec as xcb_xkb_device_spec_t,  // 0
                                              affectModLocks as u8,  // 1
                                              modLocks as u8,  // 2
                                              lockGroup as u8,  // 3
                                              groupLock as u8,  // 4
                                              affectModLatches as u8,  // 5
                                              latchGroup as u8,  // 6
                                              groupLatch as u16);  // 7
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn latch_lock_state_checked<'a>(c               : &'a base::Connection,
                                    deviceSpec      : DeviceSpec,
                                    affectModLocks  : u8,
                                    modLocks        : u8,
                                    lockGroup       : bool,
                                    groupLock       : u8,
                                    affectModLatches: u8,
                                    latchGroup      : bool,
                                    groupLatch      : u16)
        -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_latch_lock_state_checked(c.get_raw_conn(),
                                                      deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                      affectModLocks as u8,  // 1
                                                      modLocks as u8,  // 2
                                                      lockGroup as u8,  // 3
                                                      groupLock as u8,  // 4
                                                      affectModLatches as u8,  // 5
                                                      latchGroup as u8,  // 6
                                                      groupLatch as u16);  // 7
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const GET_CONTROLS: u8 = 6;

impl base::CookieSeq for xcb_xkb_get_controls_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetControlsCookie<'a> = base::Cookie<'a, xcb_xkb_get_controls_cookie_t>;

impl<'a> GetControlsCookie<'a> {
    pub fn get_reply(self) -> Result<GetControlsReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetControlsReply {
                ptr: xcb_xkb_get_controls_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetControlsReply = base::Reply<xcb_xkb_get_controls_reply_t>;

impl GetControlsReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn mouseKeysDfltBtn(&self) -> u8 {
        unsafe {
            (*self.ptr).mouseKeysDfltBtn
        }
    }
    pub fn numGroups(&self) -> u8 {
        unsafe {
            (*self.ptr).numGroups
        }
    }
    pub fn groupsWrap(&self) -> u8 {
        unsafe {
            (*self.ptr).groupsWrap
        }
    }
    pub fn internalModsMask(&self) -> u8 {
        unsafe {
            (*self.ptr).internalModsMask
        }
    }
    pub fn ignoreLockModsMask(&self) -> u8 {
        unsafe {
            (*self.ptr).ignoreLockModsMask
        }
    }
    pub fn internalModsRealMods(&self) -> u8 {
        unsafe {
            (*self.ptr).internalModsRealMods
        }
    }
    pub fn ignoreLockModsRealMods(&self) -> u8 {
        unsafe {
            (*self.ptr).ignoreLockModsRealMods
        }
    }
    pub fn internalModsVmods(&self) -> u16 {
        unsafe {
            (*self.ptr).internalModsVmods
        }
    }
    pub fn ignoreLockModsVmods(&self) -> u16 {
        unsafe {
            (*self.ptr).ignoreLockModsVmods
        }
    }
    pub fn repeatDelay(&self) -> u16 {
        unsafe {
            (*self.ptr).repeatDelay
        }
    }
    pub fn repeatInterval(&self) -> u16 {
        unsafe {
            (*self.ptr).repeatInterval
        }
    }
    pub fn slowKeysDelay(&self) -> u16 {
        unsafe {
            (*self.ptr).slowKeysDelay
        }
    }
    pub fn debounceDelay(&self) -> u16 {
        unsafe {
            (*self.ptr).debounceDelay
        }
    }
    pub fn mouseKeysDelay(&self) -> u16 {
        unsafe {
            (*self.ptr).mouseKeysDelay
        }
    }
    pub fn mouseKeysInterval(&self) -> u16 {
        unsafe {
            (*self.ptr).mouseKeysInterval
        }
    }
    pub fn mouseKeysTimeToMax(&self) -> u16 {
        unsafe {
            (*self.ptr).mouseKeysTimeToMax
        }
    }
    pub fn mouseKeysMaxSpeed(&self) -> u16 {
        unsafe {
            (*self.ptr).mouseKeysMaxSpeed
        }
    }
    pub fn mouseKeysCurve(&self) -> i16 {
        unsafe {
            (*self.ptr).mouseKeysCurve
        }
    }
    pub fn accessXOption(&self) -> u16 {
        unsafe {
            (*self.ptr).accessXOption
        }
    }
    pub fn accessXTimeout(&self) -> u16 {
        unsafe {
            (*self.ptr).accessXTimeout
        }
    }
    pub fn accessXTimeoutOptionsMask(&self) -> u16 {
        unsafe {
            (*self.ptr).accessXTimeoutOptionsMask
        }
    }
    pub fn accessXTimeoutOptionsValues(&self) -> u16 {
        unsafe {
            (*self.ptr).accessXTimeoutOptionsValues
        }
    }
    pub fn accessXTimeoutMask(&self) -> u32 {
        unsafe {
            (*self.ptr).accessXTimeoutMask
        }
    }
    pub fn accessXTimeoutValues(&self) -> u32 {
        unsafe {
            (*self.ptr).accessXTimeoutValues
        }
    }
    pub fn enabledControls(&self) -> u32 {
        unsafe {
            (*self.ptr).enabledControls
        }
    }
    pub fn perKeyRepeat(&self) -> &[u8] {
        unsafe {
            &(*self.ptr).perKeyRepeat
        }
    }
}

pub fn get_controls<'a>(c         : &'a base::Connection,
                        deviceSpec: DeviceSpec)
        -> GetControlsCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_controls(c.get_raw_conn(),
                                          deviceSpec as xcb_xkb_device_spec_t);  // 0
        GetControlsCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_controls_unchecked<'a>(c         : &'a base::Connection,
                                  deviceSpec: DeviceSpec)
        -> GetControlsCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_controls_unchecked(c.get_raw_conn(),
                                                    deviceSpec as xcb_xkb_device_spec_t);  // 0
        GetControlsCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const SET_CONTROLS: u8 = 7;

pub fn set_controls<'a>(c                          : &'a base::Connection,
                        deviceSpec                 : DeviceSpec,
                        affectInternalRealMods     : u8,
                        internalRealMods           : u8,
                        affectIgnoreLockRealMods   : u8,
                        ignoreLockRealMods         : u8,
                        affectInternalVirtualMods  : u16,
                        internalVirtualMods        : u16,
                        affectIgnoreLockVirtualMods: u16,
                        ignoreLockVirtualMods      : u16,
                        mouseKeysDfltBtn           : u8,
                        groupsWrap                 : u8,
                        accessXOptions             : u16,
                        affectEnabledControls      : u32,
                        enabledControls            : u32,
                        changeControls             : u32,
                        repeatDelay                : u16,
                        repeatInterval             : u16,
                        slowKeysDelay              : u16,
                        debounceDelay              : u16,
                        mouseKeysDelay             : u16,
                        mouseKeysInterval          : u16,
                        mouseKeysTimeToMax         : u16,
                        mouseKeysMaxSpeed          : u16,
                        mouseKeysCurve             : i16,
                        accessXTimeout             : u16,
                        accessXTimeoutMask         : u32,
                        accessXTimeoutValues       : u32,
                        accessXTimeoutOptionsMask  : u16,
                        accessXTimeoutOptionsValues: u16,
                        perKeyRepeat               : &[u8])
        -> base::VoidCookie<'a> {
    unsafe {
        let perKeyRepeat_ptr = perKeyRepeat.as_ptr();
        let cookie = xcb_xkb_set_controls(c.get_raw_conn(),
                                          deviceSpec as xcb_xkb_device_spec_t,  // 0
                                          affectInternalRealMods as u8,  // 1
                                          internalRealMods as u8,  // 2
                                          affectIgnoreLockRealMods as u8,  // 3
                                          ignoreLockRealMods as u8,  // 4
                                          affectInternalVirtualMods as u16,  // 5
                                          internalVirtualMods as u16,  // 6
                                          affectIgnoreLockVirtualMods as u16,  // 7
                                          ignoreLockVirtualMods as u16,  // 8
                                          mouseKeysDfltBtn as u8,  // 9
                                          groupsWrap as u8,  // 10
                                          accessXOptions as u16,  // 11
                                          affectEnabledControls as u32,  // 12
                                          enabledControls as u32,  // 13
                                          changeControls as u32,  // 14
                                          repeatDelay as u16,  // 15
                                          repeatInterval as u16,  // 16
                                          slowKeysDelay as u16,  // 17
                                          debounceDelay as u16,  // 18
                                          mouseKeysDelay as u16,  // 19
                                          mouseKeysInterval as u16,  // 20
                                          mouseKeysTimeToMax as u16,  // 21
                                          mouseKeysMaxSpeed as u16,  // 22
                                          mouseKeysCurve as i16,  // 23
                                          accessXTimeout as u16,  // 24
                                          accessXTimeoutMask as u32,  // 25
                                          accessXTimeoutValues as u32,  // 26
                                          accessXTimeoutOptionsMask as u16,  // 27
                                          accessXTimeoutOptionsValues as u16,  // 28
                                          perKeyRepeat_ptr as *const u8);  // 29
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn set_controls_checked<'a>(c                          : &'a base::Connection,
                                deviceSpec                 : DeviceSpec,
                                affectInternalRealMods     : u8,
                                internalRealMods           : u8,
                                affectIgnoreLockRealMods   : u8,
                                ignoreLockRealMods         : u8,
                                affectInternalVirtualMods  : u16,
                                internalVirtualMods        : u16,
                                affectIgnoreLockVirtualMods: u16,
                                ignoreLockVirtualMods      : u16,
                                mouseKeysDfltBtn           : u8,
                                groupsWrap                 : u8,
                                accessXOptions             : u16,
                                affectEnabledControls      : u32,
                                enabledControls            : u32,
                                changeControls             : u32,
                                repeatDelay                : u16,
                                repeatInterval             : u16,
                                slowKeysDelay              : u16,
                                debounceDelay              : u16,
                                mouseKeysDelay             : u16,
                                mouseKeysInterval          : u16,
                                mouseKeysTimeToMax         : u16,
                                mouseKeysMaxSpeed          : u16,
                                mouseKeysCurve             : i16,
                                accessXTimeout             : u16,
                                accessXTimeoutMask         : u32,
                                accessXTimeoutValues       : u32,
                                accessXTimeoutOptionsMask  : u16,
                                accessXTimeoutOptionsValues: u16,
                                perKeyRepeat               : &[u8])
        -> base::VoidCookie<'a> {
    unsafe {
        let perKeyRepeat_ptr = perKeyRepeat.as_ptr();
        let cookie = xcb_xkb_set_controls_checked(c.get_raw_conn(),
                                                  deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                  affectInternalRealMods as u8,  // 1
                                                  internalRealMods as u8,  // 2
                                                  affectIgnoreLockRealMods as u8,  // 3
                                                  ignoreLockRealMods as u8,  // 4
                                                  affectInternalVirtualMods as u16,  // 5
                                                  internalVirtualMods as u16,  // 6
                                                  affectIgnoreLockVirtualMods as u16,  // 7
                                                  ignoreLockVirtualMods as u16,  // 8
                                                  mouseKeysDfltBtn as u8,  // 9
                                                  groupsWrap as u8,  // 10
                                                  accessXOptions as u16,  // 11
                                                  affectEnabledControls as u32,  // 12
                                                  enabledControls as u32,  // 13
                                                  changeControls as u32,  // 14
                                                  repeatDelay as u16,  // 15
                                                  repeatInterval as u16,  // 16
                                                  slowKeysDelay as u16,  // 17
                                                  debounceDelay as u16,  // 18
                                                  mouseKeysDelay as u16,  // 19
                                                  mouseKeysInterval as u16,  // 20
                                                  mouseKeysTimeToMax as u16,  // 21
                                                  mouseKeysMaxSpeed as u16,  // 22
                                                  mouseKeysCurve as i16,  // 23
                                                  accessXTimeout as u16,  // 24
                                                  accessXTimeoutMask as u32,  // 25
                                                  accessXTimeoutValues as u32,  // 26
                                                  accessXTimeoutOptionsMask as u16,  // 27
                                                  accessXTimeoutOptionsValues as u16,  // 28
                                                  perKeyRepeat_ptr as *const u8);  // 29
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const GET_MAP: u8 = 8;

impl base::CookieSeq for xcb_xkb_get_map_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetMapCookie<'a> = base::Cookie<'a, xcb_xkb_get_map_cookie_t>;

impl<'a> GetMapCookie<'a> {
    pub fn get_reply(self) -> Result<GetMapReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetMapReply {
                ptr: xcb_xkb_get_map_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetMapMap<'a> = base::StructPtr<'a, xcb_xkb_get_map_map_t>;

pub type GetMapReply = base::Reply<xcb_xkb_get_map_reply_t>;

impl GetMapReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn minKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).minKeyCode
        }
    }
    pub fn maxKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).maxKeyCode
        }
    }
    pub fn present(&self) -> u16 {
        unsafe {
            (*self.ptr).present
        }
    }
    pub fn firstType(&self) -> u8 {
        unsafe {
            (*self.ptr).firstType
        }
    }
    pub fn nTypes(&self) -> u8 {
        unsafe {
            (*self.ptr).nTypes
        }
    }
    pub fn totalTypes(&self) -> u8 {
        unsafe {
            (*self.ptr).totalTypes
        }
    }
    pub fn firstKeySym(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKeySym
        }
    }
    pub fn totalSyms(&self) -> u16 {
        unsafe {
            (*self.ptr).totalSyms
        }
    }
    pub fn nKeySyms(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeySyms
        }
    }
    pub fn firstKeyAction(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKeyAction
        }
    }
    pub fn totalActions(&self) -> u16 {
        unsafe {
            (*self.ptr).totalActions
        }
    }
    pub fn nKeyActions(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeyActions
        }
    }
    pub fn firstKeyBehavior(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKeyBehavior
        }
    }
    pub fn nKeyBehaviors(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeyBehaviors
        }
    }
    pub fn totalKeyBehaviors(&self) -> u8 {
        unsafe {
            (*self.ptr).totalKeyBehaviors
        }
    }
    pub fn firstKeyExplicit(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKeyExplicit
        }
    }
    pub fn nKeyExplicit(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeyExplicit
        }
    }
    pub fn totalKeyExplicit(&self) -> u8 {
        unsafe {
            (*self.ptr).totalKeyExplicit
        }
    }
    pub fn firstModMapKey(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstModMapKey
        }
    }
    pub fn nModMapKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).nModMapKeys
        }
    }
    pub fn totalModMapKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).totalModMapKeys
        }
    }
    pub fn firstVModMapKey(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstVModMapKey
        }
    }
    pub fn nVModMapKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).nVModMapKeys
        }
    }
    pub fn totalVModMapKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).totalVModMapKeys
        }
    }
    pub fn virtualMods(&self) -> u16 {
        unsafe {
            (*self.ptr).virtualMods
        }
    }
}

pub fn get_map<'a>(c               : &'a base::Connection,
                   deviceSpec      : DeviceSpec,
                   full            : u16,
                   partial         : u16,
                   firstType       : u8,
                   nTypes          : u8,
                   firstKeySym     : xproto::Keycode,
                   nKeySyms        : u8,
                   firstKeyAction  : xproto::Keycode,
                   nKeyActions     : u8,
                   firstKeyBehavior: xproto::Keycode,
                   nKeyBehaviors   : u8,
                   virtualMods     : u16,
                   firstKeyExplicit: xproto::Keycode,
                   nKeyExplicit    : u8,
                   firstModMapKey  : xproto::Keycode,
                   nModMapKeys     : u8,
                   firstVModMapKey : xproto::Keycode,
                   nVModMapKeys    : u8)
        -> GetMapCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_map(c.get_raw_conn(),
                                     deviceSpec as xcb_xkb_device_spec_t,  // 0
                                     full as u16,  // 1
                                     partial as u16,  // 2
                                     firstType as u8,  // 3
                                     nTypes as u8,  // 4
                                     firstKeySym as xcb_keycode_t,  // 5
                                     nKeySyms as u8,  // 6
                                     firstKeyAction as xcb_keycode_t,  // 7
                                     nKeyActions as u8,  // 8
                                     firstKeyBehavior as xcb_keycode_t,  // 9
                                     nKeyBehaviors as u8,  // 10
                                     virtualMods as u16,  // 11
                                     firstKeyExplicit as xcb_keycode_t,  // 12
                                     nKeyExplicit as u8,  // 13
                                     firstModMapKey as xcb_keycode_t,  // 14
                                     nModMapKeys as u8,  // 15
                                     firstVModMapKey as xcb_keycode_t,  // 16
                                     nVModMapKeys as u8);  // 17
        GetMapCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_map_unchecked<'a>(c               : &'a base::Connection,
                             deviceSpec      : DeviceSpec,
                             full            : u16,
                             partial         : u16,
                             firstType       : u8,
                             nTypes          : u8,
                             firstKeySym     : xproto::Keycode,
                             nKeySyms        : u8,
                             firstKeyAction  : xproto::Keycode,
                             nKeyActions     : u8,
                             firstKeyBehavior: xproto::Keycode,
                             nKeyBehaviors   : u8,
                             virtualMods     : u16,
                             firstKeyExplicit: xproto::Keycode,
                             nKeyExplicit    : u8,
                             firstModMapKey  : xproto::Keycode,
                             nModMapKeys     : u8,
                             firstVModMapKey : xproto::Keycode,
                             nVModMapKeys    : u8)
        -> GetMapCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_map_unchecked(c.get_raw_conn(),
                                               deviceSpec as xcb_xkb_device_spec_t,  // 0
                                               full as u16,  // 1
                                               partial as u16,  // 2
                                               firstType as u8,  // 3
                                               nTypes as u8,  // 4
                                               firstKeySym as xcb_keycode_t,  // 5
                                               nKeySyms as u8,  // 6
                                               firstKeyAction as xcb_keycode_t,  // 7
                                               nKeyActions as u8,  // 8
                                               firstKeyBehavior as xcb_keycode_t,  // 9
                                               nKeyBehaviors as u8,  // 10
                                               virtualMods as u16,  // 11
                                               firstKeyExplicit as xcb_keycode_t,  // 12
                                               nKeyExplicit as u8,  // 13
                                               firstModMapKey as xcb_keycode_t,  // 14
                                               nModMapKeys as u8,  // 15
                                               firstVModMapKey as xcb_keycode_t,  // 16
                                               nVModMapKeys as u8);  // 17
        GetMapCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub type SetMapValues<'a> = base::StructPtr<'a, xcb_xkb_set_map_values_t>;

pub const SET_MAP: u8 = 9;

pub fn set_map<'a>(c                : &'a base::Connection,
                   deviceSpec       : DeviceSpec,
                   present          : u16,
                   flags            : u16,
                   minKeyCode       : xproto::Keycode,
                   maxKeyCode       : xproto::Keycode,
                   firstType        : u8,
                   nTypes           : u8,
                   firstKeySym      : xproto::Keycode,
                   nKeySyms         : u8,
                   totalSyms        : u16,
                   firstKeyAction   : xproto::Keycode,
                   nKeyActions      : u8,
                   totalActions     : u16,
                   firstKeyBehavior : xproto::Keycode,
                   nKeyBehaviors    : u8,
                   totalKeyBehaviors: u8,
                   firstKeyExplicit : xproto::Keycode,
                   nKeyExplicit     : u8,
                   totalKeyExplicit : u8,
                   firstModMapKey   : xproto::Keycode,
                   nModMapKeys      : u8,
                   totalModMapKeys  : u8,
                   firstVModMapKey  : xproto::Keycode,
                   nVModMapKeys     : u8,
                   totalVModMapKeys : u8,
                   virtualMods      : u16,
                   values           : std::option::Option<SetMapValues>)
        -> base::VoidCookie<'a> {
    unsafe {
        let values_ptr = match values {
            Some(p) => p.ptr as *const xcb_xkb_set_map_values_t,
            None => std::ptr::null()
        };
        let cookie = xcb_xkb_set_map(c.get_raw_conn(),
                                     deviceSpec as xcb_xkb_device_spec_t,  // 0
                                     present as u16,  // 1
                                     flags as u16,  // 2
                                     minKeyCode as xcb_keycode_t,  // 3
                                     maxKeyCode as xcb_keycode_t,  // 4
                                     firstType as u8,  // 5
                                     nTypes as u8,  // 6
                                     firstKeySym as xcb_keycode_t,  // 7
                                     nKeySyms as u8,  // 8
                                     totalSyms as u16,  // 9
                                     firstKeyAction as xcb_keycode_t,  // 10
                                     nKeyActions as u8,  // 11
                                     totalActions as u16,  // 12
                                     firstKeyBehavior as xcb_keycode_t,  // 13
                                     nKeyBehaviors as u8,  // 14
                                     totalKeyBehaviors as u8,  // 15
                                     firstKeyExplicit as xcb_keycode_t,  // 16
                                     nKeyExplicit as u8,  // 17
                                     totalKeyExplicit as u8,  // 18
                                     firstModMapKey as xcb_keycode_t,  // 19
                                     nModMapKeys as u8,  // 20
                                     totalModMapKeys as u8,  // 21
                                     firstVModMapKey as xcb_keycode_t,  // 22
                                     nVModMapKeys as u8,  // 23
                                     totalVModMapKeys as u8,  // 24
                                     virtualMods as u16,  // 25
                                     values_ptr);  // 26
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn set_map_checked<'a>(c                : &'a base::Connection,
                           deviceSpec       : DeviceSpec,
                           present          : u16,
                           flags            : u16,
                           minKeyCode       : xproto::Keycode,
                           maxKeyCode       : xproto::Keycode,
                           firstType        : u8,
                           nTypes           : u8,
                           firstKeySym      : xproto::Keycode,
                           nKeySyms         : u8,
                           totalSyms        : u16,
                           firstKeyAction   : xproto::Keycode,
                           nKeyActions      : u8,
                           totalActions     : u16,
                           firstKeyBehavior : xproto::Keycode,
                           nKeyBehaviors    : u8,
                           totalKeyBehaviors: u8,
                           firstKeyExplicit : xproto::Keycode,
                           nKeyExplicit     : u8,
                           totalKeyExplicit : u8,
                           firstModMapKey   : xproto::Keycode,
                           nModMapKeys      : u8,
                           totalModMapKeys  : u8,
                           firstVModMapKey  : xproto::Keycode,
                           nVModMapKeys     : u8,
                           totalVModMapKeys : u8,
                           virtualMods      : u16,
                           values           : std::option::Option<SetMapValues>)
        -> base::VoidCookie<'a> {
    unsafe {
        let values_ptr = match values {
            Some(p) => p.ptr as *const xcb_xkb_set_map_values_t,
            None => std::ptr::null()
        };
        let cookie = xcb_xkb_set_map_checked(c.get_raw_conn(),
                                             deviceSpec as xcb_xkb_device_spec_t,  // 0
                                             present as u16,  // 1
                                             flags as u16,  // 2
                                             minKeyCode as xcb_keycode_t,  // 3
                                             maxKeyCode as xcb_keycode_t,  // 4
                                             firstType as u8,  // 5
                                             nTypes as u8,  // 6
                                             firstKeySym as xcb_keycode_t,  // 7
                                             nKeySyms as u8,  // 8
                                             totalSyms as u16,  // 9
                                             firstKeyAction as xcb_keycode_t,  // 10
                                             nKeyActions as u8,  // 11
                                             totalActions as u16,  // 12
                                             firstKeyBehavior as xcb_keycode_t,  // 13
                                             nKeyBehaviors as u8,  // 14
                                             totalKeyBehaviors as u8,  // 15
                                             firstKeyExplicit as xcb_keycode_t,  // 16
                                             nKeyExplicit as u8,  // 17
                                             totalKeyExplicit as u8,  // 18
                                             firstModMapKey as xcb_keycode_t,  // 19
                                             nModMapKeys as u8,  // 20
                                             totalModMapKeys as u8,  // 21
                                             firstVModMapKey as xcb_keycode_t,  // 22
                                             nVModMapKeys as u8,  // 23
                                             totalVModMapKeys as u8,  // 24
                                             virtualMods as u16,  // 25
                                             values_ptr);  // 26
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const GET_COMPAT_MAP: u8 = 10;

impl base::CookieSeq for xcb_xkb_get_compat_map_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetCompatMapCookie<'a> = base::Cookie<'a, xcb_xkb_get_compat_map_cookie_t>;

impl<'a> GetCompatMapCookie<'a> {
    pub fn get_reply(self) -> Result<GetCompatMapReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetCompatMapReply {
                ptr: xcb_xkb_get_compat_map_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetCompatMapReply = base::Reply<xcb_xkb_get_compat_map_reply_t>;

impl GetCompatMapReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn groupsRtrn(&self) -> u8 {
        unsafe {
            (*self.ptr).groupsRtrn
        }
    }
    pub fn firstSIRtrn(&self) -> u16 {
        unsafe {
            (*self.ptr).firstSIRtrn
        }
    }
    pub fn nSIRtrn(&self) -> u16 {
        unsafe {
            (*self.ptr).nSIRtrn
        }
    }
    pub fn nTotalSI(&self) -> u16 {
        unsafe {
            (*self.ptr).nTotalSI
        }
    }
    pub fn si_rtrn(&self) -> SymInterpretIterator {
        unsafe {
            xcb_xkb_get_compat_map_si_rtrn_iterator(self.ptr)
        }
    }
    pub fn group_rtrn(&self) -> ModDefIterator {
        unsafe {
            xcb_xkb_get_compat_map_group_rtrn_iterator(self.ptr)
        }
    }
}

pub fn get_compat_map<'a>(c         : &'a base::Connection,
                          deviceSpec: DeviceSpec,
                          groups    : u8,
                          getAllSI  : bool,
                          firstSI   : u16,
                          nSI       : u16)
        -> GetCompatMapCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_compat_map(c.get_raw_conn(),
                                            deviceSpec as xcb_xkb_device_spec_t,  // 0
                                            groups as u8,  // 1
                                            getAllSI as u8,  // 2
                                            firstSI as u16,  // 3
                                            nSI as u16);  // 4
        GetCompatMapCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_compat_map_unchecked<'a>(c         : &'a base::Connection,
                                    deviceSpec: DeviceSpec,
                                    groups    : u8,
                                    getAllSI  : bool,
                                    firstSI   : u16,
                                    nSI       : u16)
        -> GetCompatMapCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_compat_map_unchecked(c.get_raw_conn(),
                                                      deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                      groups as u8,  // 1
                                                      getAllSI as u8,  // 2
                                                      firstSI as u16,  // 3
                                                      nSI as u16);  // 4
        GetCompatMapCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const SET_COMPAT_MAP: u8 = 11;

pub fn set_compat_map<'a>(c               : &'a base::Connection,
                          deviceSpec      : DeviceSpec,
                          recomputeActions: bool,
                          truncateSI      : bool,
                          firstSI         : u16,
                          si              : &[SymInterpret],
                          groupMaps       : &[ModDef])
        -> base::VoidCookie<'a> {
    unsafe {
        let si_len = si.len();
        let si_ptr = si.as_ptr();
        let groupMaps_len = groupMaps.len();
        let groupMaps_ptr = groupMaps.as_ptr();
        let cookie = xcb_xkb_set_compat_map(c.get_raw_conn(),
                                            deviceSpec as xcb_xkb_device_spec_t,  // 0
                                            recomputeActions as u8,  // 1
                                            truncateSI as u8,  // 2
                                            groupMaps_len as u8,  // 3
                                            firstSI as u16,  // 4
                                            si_len as u16,  // 5
                                            si_ptr as *const xcb_xkb_sym_interpret_t,  // 6
                                            groupMaps_ptr as *const xcb_xkb_mod_def_t);  // 7
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn set_compat_map_checked<'a>(c               : &'a base::Connection,
                                  deviceSpec      : DeviceSpec,
                                  recomputeActions: bool,
                                  truncateSI      : bool,
                                  firstSI         : u16,
                                  si              : &[SymInterpret],
                                  groupMaps       : &[ModDef])
        -> base::VoidCookie<'a> {
    unsafe {
        let si_len = si.len();
        let si_ptr = si.as_ptr();
        let groupMaps_len = groupMaps.len();
        let groupMaps_ptr = groupMaps.as_ptr();
        let cookie = xcb_xkb_set_compat_map_checked(c.get_raw_conn(),
                                                    deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                    recomputeActions as u8,  // 1
                                                    truncateSI as u8,  // 2
                                                    groupMaps_len as u8,  // 3
                                                    firstSI as u16,  // 4
                                                    si_len as u16,  // 5
                                                    si_ptr as *const xcb_xkb_sym_interpret_t,  // 6
                                                    groupMaps_ptr as *const xcb_xkb_mod_def_t);  // 7
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const GET_INDICATOR_STATE: u8 = 12;

impl base::CookieSeq for xcb_xkb_get_indicator_state_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetIndicatorStateCookie<'a> = base::Cookie<'a, xcb_xkb_get_indicator_state_cookie_t>;

impl<'a> GetIndicatorStateCookie<'a> {
    pub fn get_reply(self) -> Result<GetIndicatorStateReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetIndicatorStateReply {
                ptr: xcb_xkb_get_indicator_state_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetIndicatorStateReply = base::Reply<xcb_xkb_get_indicator_state_reply_t>;

impl GetIndicatorStateReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn state(&self) -> u32 {
        unsafe {
            (*self.ptr).state
        }
    }
}

pub fn get_indicator_state<'a>(c         : &'a base::Connection,
                               deviceSpec: DeviceSpec)
        -> GetIndicatorStateCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_indicator_state(c.get_raw_conn(),
                                                 deviceSpec as xcb_xkb_device_spec_t);  // 0
        GetIndicatorStateCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_indicator_state_unchecked<'a>(c         : &'a base::Connection,
                                         deviceSpec: DeviceSpec)
        -> GetIndicatorStateCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_indicator_state_unchecked(c.get_raw_conn(),
                                                           deviceSpec as xcb_xkb_device_spec_t);  // 0
        GetIndicatorStateCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const GET_INDICATOR_MAP: u8 = 13;

impl base::CookieSeq for xcb_xkb_get_indicator_map_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetIndicatorMapCookie<'a> = base::Cookie<'a, xcb_xkb_get_indicator_map_cookie_t>;

impl<'a> GetIndicatorMapCookie<'a> {
    pub fn get_reply(self) -> Result<GetIndicatorMapReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetIndicatorMapReply {
                ptr: xcb_xkb_get_indicator_map_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetIndicatorMapReply = base::Reply<xcb_xkb_get_indicator_map_reply_t>;

impl GetIndicatorMapReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn which(&self) -> u32 {
        unsafe {
            (*self.ptr).which
        }
    }
    pub fn realIndicators(&self) -> u32 {
        unsafe {
            (*self.ptr).realIndicators
        }
    }
    pub fn nIndicators(&self) -> u8 {
        unsafe {
            (*self.ptr).nIndicators
        }
    }
    pub fn maps(&self) -> IndicatorMapIterator {
        unsafe {
            xcb_xkb_get_indicator_map_maps_iterator(self.ptr)
        }
    }
}

pub fn get_indicator_map<'a>(c         : &'a base::Connection,
                             deviceSpec: DeviceSpec,
                             which     : u32)
        -> GetIndicatorMapCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_indicator_map(c.get_raw_conn(),
                                               deviceSpec as xcb_xkb_device_spec_t,  // 0
                                               which as u32);  // 1
        GetIndicatorMapCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_indicator_map_unchecked<'a>(c         : &'a base::Connection,
                                       deviceSpec: DeviceSpec,
                                       which     : u32)
        -> GetIndicatorMapCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_indicator_map_unchecked(c.get_raw_conn(),
                                                         deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                         which as u32);  // 1
        GetIndicatorMapCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const SET_INDICATOR_MAP: u8 = 14;

pub fn set_indicator_map<'a>(c         : &'a base::Connection,
                             deviceSpec: DeviceSpec,
                             maps      : &[IndicatorMap])
        -> base::VoidCookie<'a> {
    unsafe {
        let maps_len = maps.len();
        let maps_ptr = maps.as_ptr();
        let cookie = xcb_xkb_set_indicator_map(c.get_raw_conn(),
                                               deviceSpec as xcb_xkb_device_spec_t,  // 0
                                               maps_len as u32,  // 1
                                               maps_ptr as *const xcb_xkb_indicator_map_t);  // 2
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn set_indicator_map_checked<'a>(c         : &'a base::Connection,
                                     deviceSpec: DeviceSpec,
                                     maps      : &[IndicatorMap])
        -> base::VoidCookie<'a> {
    unsafe {
        let maps_len = maps.len();
        let maps_ptr = maps.as_ptr();
        let cookie = xcb_xkb_set_indicator_map_checked(c.get_raw_conn(),
                                                       deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                       maps_len as u32,  // 1
                                                       maps_ptr as *const xcb_xkb_indicator_map_t);  // 2
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const GET_NAMED_INDICATOR: u8 = 15;

impl base::CookieSeq for xcb_xkb_get_named_indicator_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetNamedIndicatorCookie<'a> = base::Cookie<'a, xcb_xkb_get_named_indicator_cookie_t>;

impl<'a> GetNamedIndicatorCookie<'a> {
    pub fn get_reply(self) -> Result<GetNamedIndicatorReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetNamedIndicatorReply {
                ptr: xcb_xkb_get_named_indicator_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetNamedIndicatorReply = base::Reply<xcb_xkb_get_named_indicator_reply_t>;

impl GetNamedIndicatorReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn indicator(&self) -> xproto::Atom {
        unsafe {
            (*self.ptr).indicator
        }
    }
    pub fn found(&self) -> bool {
        unsafe {
            (*self.ptr).found != 0
        }
    }
    pub fn on(&self) -> bool {
        unsafe {
            (*self.ptr).on != 0
        }
    }
    pub fn realIndicator(&self) -> bool {
        unsafe {
            (*self.ptr).realIndicator != 0
        }
    }
    pub fn ndx(&self) -> u8 {
        unsafe {
            (*self.ptr).ndx
        }
    }
    pub fn map_flags(&self) -> u8 {
        unsafe {
            (*self.ptr).map_flags
        }
    }
    pub fn map_whichGroups(&self) -> u8 {
        unsafe {
            (*self.ptr).map_whichGroups
        }
    }
    pub fn map_groups(&self) -> u8 {
        unsafe {
            (*self.ptr).map_groups
        }
    }
    pub fn map_whichMods(&self) -> u8 {
        unsafe {
            (*self.ptr).map_whichMods
        }
    }
    pub fn map_mods(&self) -> u8 {
        unsafe {
            (*self.ptr).map_mods
        }
    }
    pub fn map_realMods(&self) -> u8 {
        unsafe {
            (*self.ptr).map_realMods
        }
    }
    pub fn map_vmod(&self) -> u16 {
        unsafe {
            (*self.ptr).map_vmod
        }
    }
    pub fn map_ctrls(&self) -> u32 {
        unsafe {
            (*self.ptr).map_ctrls
        }
    }
    pub fn supported(&self) -> bool {
        unsafe {
            (*self.ptr).supported != 0
        }
    }
}

pub fn get_named_indicator<'a>(c         : &'a base::Connection,
                               deviceSpec: DeviceSpec,
                               ledClass  : LedClassSpec,
                               ledID     : IdSpec,
                               indicator : xproto::Atom)
        -> GetNamedIndicatorCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_named_indicator(c.get_raw_conn(),
                                                 deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                 ledClass as xcb_xkb_led_class_spec_t,  // 1
                                                 ledID as xcb_xkb_id_spec_t,  // 2
                                                 indicator as xcb_atom_t);  // 3
        GetNamedIndicatorCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_named_indicator_unchecked<'a>(c         : &'a base::Connection,
                                         deviceSpec: DeviceSpec,
                                         ledClass  : LedClassSpec,
                                         ledID     : IdSpec,
                                         indicator : xproto::Atom)
        -> GetNamedIndicatorCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_named_indicator_unchecked(c.get_raw_conn(),
                                                           deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                           ledClass as xcb_xkb_led_class_spec_t,  // 1
                                                           ledID as xcb_xkb_id_spec_t,  // 2
                                                           indicator as xcb_atom_t);  // 3
        GetNamedIndicatorCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const SET_NAMED_INDICATOR: u8 = 16;

pub fn set_named_indicator<'a>(c              : &'a base::Connection,
                               deviceSpec     : DeviceSpec,
                               ledClass       : LedClassSpec,
                               ledID          : IdSpec,
                               indicator      : xproto::Atom,
                               setState       : bool,
                               on             : bool,
                               setMap         : bool,
                               createMap      : bool,
                               map_flags      : u8,
                               map_whichGroups: u8,
                               map_groups     : u8,
                               map_whichMods  : u8,
                               map_realMods   : u8,
                               map_vmods      : u16,
                               map_ctrls      : u32)
        -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_set_named_indicator(c.get_raw_conn(),
                                                 deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                 ledClass as xcb_xkb_led_class_spec_t,  // 1
                                                 ledID as xcb_xkb_id_spec_t,  // 2
                                                 indicator as xcb_atom_t,  // 3
                                                 setState as u8,  // 4
                                                 on as u8,  // 5
                                                 setMap as u8,  // 6
                                                 createMap as u8,  // 7
                                                 map_flags as u8,  // 8
                                                 map_whichGroups as u8,  // 9
                                                 map_groups as u8,  // 10
                                                 map_whichMods as u8,  // 11
                                                 map_realMods as u8,  // 12
                                                 map_vmods as u16,  // 13
                                                 map_ctrls as u32);  // 14
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn set_named_indicator_checked<'a>(c              : &'a base::Connection,
                                       deviceSpec     : DeviceSpec,
                                       ledClass       : LedClassSpec,
                                       ledID          : IdSpec,
                                       indicator      : xproto::Atom,
                                       setState       : bool,
                                       on             : bool,
                                       setMap         : bool,
                                       createMap      : bool,
                                       map_flags      : u8,
                                       map_whichGroups: u8,
                                       map_groups     : u8,
                                       map_whichMods  : u8,
                                       map_realMods   : u8,
                                       map_vmods      : u16,
                                       map_ctrls      : u32)
        -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_set_named_indicator_checked(c.get_raw_conn(),
                                                         deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                         ledClass as xcb_xkb_led_class_spec_t,  // 1
                                                         ledID as xcb_xkb_id_spec_t,  // 2
                                                         indicator as xcb_atom_t,  // 3
                                                         setState as u8,  // 4
                                                         on as u8,  // 5
                                                         setMap as u8,  // 6
                                                         createMap as u8,  // 7
                                                         map_flags as u8,  // 8
                                                         map_whichGroups as u8,  // 9
                                                         map_groups as u8,  // 10
                                                         map_whichMods as u8,  // 11
                                                         map_realMods as u8,  // 12
                                                         map_vmods as u16,  // 13
                                                         map_ctrls as u32);  // 14
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const GET_NAMES: u8 = 17;

impl base::CookieSeq for xcb_xkb_get_names_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetNamesCookie<'a> = base::Cookie<'a, xcb_xkb_get_names_cookie_t>;

impl<'a> GetNamesCookie<'a> {
    pub fn get_reply(self) -> Result<GetNamesReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetNamesReply {
                ptr: xcb_xkb_get_names_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetNamesValueList<'a> = base::StructPtr<'a, xcb_xkb_get_names_value_list_t>;

pub type GetNamesReply = base::Reply<xcb_xkb_get_names_reply_t>;

impl GetNamesReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn which(&self) -> u32 {
        unsafe {
            (*self.ptr).which
        }
    }
    pub fn minKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).minKeyCode
        }
    }
    pub fn maxKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).maxKeyCode
        }
    }
    pub fn nTypes(&self) -> u8 {
        unsafe {
            (*self.ptr).nTypes
        }
    }
    pub fn groupNames(&self) -> u8 {
        unsafe {
            (*self.ptr).groupNames
        }
    }
    pub fn virtualMods(&self) -> u16 {
        unsafe {
            (*self.ptr).virtualMods
        }
    }
    pub fn firstKey(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKey
        }
    }
    pub fn nKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeys
        }
    }
    pub fn indicators(&self) -> u32 {
        unsafe {
            (*self.ptr).indicators
        }
    }
    pub fn nRadioGroups(&self) -> u8 {
        unsafe {
            (*self.ptr).nRadioGroups
        }
    }
    pub fn nKeyAliases(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeyAliases
        }
    }
    pub fn nKTLevels(&self) -> u16 {
        unsafe {
            (*self.ptr).nKTLevels
        }
    }
}

pub fn get_names<'a>(c         : &'a base::Connection,
                     deviceSpec: DeviceSpec,
                     which     : u32)
        -> GetNamesCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_names(c.get_raw_conn(),
                                       deviceSpec as xcb_xkb_device_spec_t,  // 0
                                       which as u32);  // 1
        GetNamesCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_names_unchecked<'a>(c         : &'a base::Connection,
                               deviceSpec: DeviceSpec,
                               which     : u32)
        -> GetNamesCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_names_unchecked(c.get_raw_conn(),
                                                 deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                 which as u32);  // 1
        GetNamesCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub type SetNamesValues<'a> = base::StructPtr<'a, xcb_xkb_set_names_values_t>;

pub const SET_NAMES: u8 = 18;

pub fn set_names<'a>(c                : &'a base::Connection,
                     deviceSpec       : DeviceSpec,
                     virtualMods      : u16,
                     which            : u32,
                     firstType        : u8,
                     nTypes           : u8,
                     firstKTLevelt    : u8,
                     nKTLevels        : u8,
                     indicators       : u32,
                     groupNames       : u8,
                     nRadioGroups     : u8,
                     firstKey         : xproto::Keycode,
                     nKeys            : u8,
                     nKeyAliases      : u8,
                     totalKTLevelNames: u16,
                     values           : std::option::Option<SetNamesValues>)
        -> base::VoidCookie<'a> {
    unsafe {
        let values_ptr = match values {
            Some(p) => p.ptr as *const xcb_xkb_set_names_values_t,
            None => std::ptr::null()
        };
        let cookie = xcb_xkb_set_names(c.get_raw_conn(),
                                       deviceSpec as xcb_xkb_device_spec_t,  // 0
                                       virtualMods as u16,  // 1
                                       which as u32,  // 2
                                       firstType as u8,  // 3
                                       nTypes as u8,  // 4
                                       firstKTLevelt as u8,  // 5
                                       nKTLevels as u8,  // 6
                                       indicators as u32,  // 7
                                       groupNames as u8,  // 8
                                       nRadioGroups as u8,  // 9
                                       firstKey as xcb_keycode_t,  // 10
                                       nKeys as u8,  // 11
                                       nKeyAliases as u8,  // 12
                                       totalKTLevelNames as u16,  // 13
                                       values_ptr);  // 14
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn set_names_checked<'a>(c                : &'a base::Connection,
                             deviceSpec       : DeviceSpec,
                             virtualMods      : u16,
                             which            : u32,
                             firstType        : u8,
                             nTypes           : u8,
                             firstKTLevelt    : u8,
                             nKTLevels        : u8,
                             indicators       : u32,
                             groupNames       : u8,
                             nRadioGroups     : u8,
                             firstKey         : xproto::Keycode,
                             nKeys            : u8,
                             nKeyAliases      : u8,
                             totalKTLevelNames: u16,
                             values           : std::option::Option<SetNamesValues>)
        -> base::VoidCookie<'a> {
    unsafe {
        let values_ptr = match values {
            Some(p) => p.ptr as *const xcb_xkb_set_names_values_t,
            None => std::ptr::null()
        };
        let cookie = xcb_xkb_set_names_checked(c.get_raw_conn(),
                                               deviceSpec as xcb_xkb_device_spec_t,  // 0
                                               virtualMods as u16,  // 1
                                               which as u32,  // 2
                                               firstType as u8,  // 3
                                               nTypes as u8,  // 4
                                               firstKTLevelt as u8,  // 5
                                               nKTLevels as u8,  // 6
                                               indicators as u32,  // 7
                                               groupNames as u8,  // 8
                                               nRadioGroups as u8,  // 9
                                               firstKey as xcb_keycode_t,  // 10
                                               nKeys as u8,  // 11
                                               nKeyAliases as u8,  // 12
                                               totalKTLevelNames as u16,  // 13
                                               values_ptr);  // 14
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const PER_CLIENT_FLAGS: u8 = 21;

impl base::CookieSeq for xcb_xkb_per_client_flags_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type PerClientFlagsCookie<'a> = base::Cookie<'a, xcb_xkb_per_client_flags_cookie_t>;

impl<'a> PerClientFlagsCookie<'a> {
    pub fn get_reply(self) -> Result<PerClientFlagsReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            PerClientFlagsReply {
                ptr: xcb_xkb_per_client_flags_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type PerClientFlagsReply = base::Reply<xcb_xkb_per_client_flags_reply_t>;

impl PerClientFlagsReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn supported(&self) -> u32 {
        unsafe {
            (*self.ptr).supported
        }
    }
    pub fn value(&self) -> u32 {
        unsafe {
            (*self.ptr).value
        }
    }
    pub fn autoCtrls(&self) -> u32 {
        unsafe {
            (*self.ptr).autoCtrls
        }
    }
    pub fn autoCtrlsValues(&self) -> u32 {
        unsafe {
            (*self.ptr).autoCtrlsValues
        }
    }
}

pub fn per_client_flags<'a>(c              : &'a base::Connection,
                            deviceSpec     : DeviceSpec,
                            change         : u32,
                            value          : u32,
                            ctrlsToChange  : u32,
                            autoCtrls      : u32,
                            autoCtrlsValues: u32)
        -> PerClientFlagsCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_per_client_flags(c.get_raw_conn(),
                                              deviceSpec as xcb_xkb_device_spec_t,  // 0
                                              change as u32,  // 1
                                              value as u32,  // 2
                                              ctrlsToChange as u32,  // 3
                                              autoCtrls as u32,  // 4
                                              autoCtrlsValues as u32);  // 5
        PerClientFlagsCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn per_client_flags_unchecked<'a>(c              : &'a base::Connection,
                                      deviceSpec     : DeviceSpec,
                                      change         : u32,
                                      value          : u32,
                                      ctrlsToChange  : u32,
                                      autoCtrls      : u32,
                                      autoCtrlsValues: u32)
        -> PerClientFlagsCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_per_client_flags_unchecked(c.get_raw_conn(),
                                                        deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                        change as u32,  // 1
                                                        value as u32,  // 2
                                                        ctrlsToChange as u32,  // 3
                                                        autoCtrls as u32,  // 4
                                                        autoCtrlsValues as u32);  // 5
        PerClientFlagsCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const LIST_COMPONENTS: u8 = 22;

impl base::CookieSeq for xcb_xkb_list_components_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type ListComponentsCookie<'a> = base::Cookie<'a, xcb_xkb_list_components_cookie_t>;

impl<'a> ListComponentsCookie<'a> {
    pub fn get_reply(self) -> Result<ListComponentsReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            ListComponentsReply {
                ptr: xcb_xkb_list_components_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type ListComponentsReply = base::Reply<xcb_xkb_list_components_reply_t>;

impl ListComponentsReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn nKeymaps(&self) -> u16 {
        unsafe {
            (*self.ptr).nKeymaps
        }
    }
    pub fn nKeycodes(&self) -> u16 {
        unsafe {
            (*self.ptr).nKeycodes
        }
    }
    pub fn nTypes(&self) -> u16 {
        unsafe {
            (*self.ptr).nTypes
        }
    }
    pub fn nCompatMaps(&self) -> u16 {
        unsafe {
            (*self.ptr).nCompatMaps
        }
    }
    pub fn nSymbols(&self) -> u16 {
        unsafe {
            (*self.ptr).nSymbols
        }
    }
    pub fn nGeometries(&self) -> u16 {
        unsafe {
            (*self.ptr).nGeometries
        }
    }
    pub fn extra(&self) -> u16 {
        unsafe {
            (*self.ptr).extra
        }
    }
    pub fn keymaps(&self) -> ListingIterator {
        unsafe {
            xcb_xkb_list_components_keymaps_iterator(self.ptr)
        }
    }
    pub fn keycodes(&self) -> ListingIterator {
        unsafe {
            xcb_xkb_list_components_keycodes_iterator(self.ptr)
        }
    }
    pub fn types(&self) -> ListingIterator {
        unsafe {
            xcb_xkb_list_components_types_iterator(self.ptr)
        }
    }
    pub fn compatMaps(&self) -> ListingIterator {
        unsafe {
            xcb_xkb_list_components_compat_maps_iterator(self.ptr)
        }
    }
    pub fn symbols(&self) -> ListingIterator {
        unsafe {
            xcb_xkb_list_components_symbols_iterator(self.ptr)
        }
    }
    pub fn geometries(&self) -> ListingIterator {
        unsafe {
            xcb_xkb_list_components_geometries_iterator(self.ptr)
        }
    }
}

pub fn list_components<'a>(c         : &'a base::Connection,
                           deviceSpec: DeviceSpec,
                           maxNames  : u16)
        -> ListComponentsCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_list_components(c.get_raw_conn(),
                                             deviceSpec as xcb_xkb_device_spec_t,  // 0
                                             maxNames as u16);  // 1
        ListComponentsCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn list_components_unchecked<'a>(c         : &'a base::Connection,
                                     deviceSpec: DeviceSpec,
                                     maxNames  : u16)
        -> ListComponentsCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_list_components_unchecked(c.get_raw_conn(),
                                                       deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                       maxNames as u16);  // 1
        ListComponentsCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const GET_KBD_BY_NAME: u8 = 23;

impl base::CookieSeq for xcb_xkb_get_kbd_by_name_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetKbdByNameCookie<'a> = base::Cookie<'a, xcb_xkb_get_kbd_by_name_cookie_t>;

impl<'a> GetKbdByNameCookie<'a> {
    pub fn get_reply(self) -> Result<GetKbdByNameReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetKbdByNameReply {
                ptr: xcb_xkb_get_kbd_by_name_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetKbdByNameReplies<'a> = base::StructPtr<'a, xcb_xkb_get_kbd_by_name_replies_t>;

pub type GetKbdByNameRepliesTypesMap<'a> = base::StructPtr<'a, xcb_xkb_get_kbd_by_name_replies_types_map_t>;

pub type GetKbdByNameRepliesKeyNamesValueList<'a> = base::StructPtr<'a, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t>;

pub type GetKbdByNameReply = base::Reply<xcb_xkb_get_kbd_by_name_reply_t>;

impl GetKbdByNameReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn minKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).minKeyCode
        }
    }
    pub fn maxKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).maxKeyCode
        }
    }
    pub fn loaded(&self) -> bool {
        unsafe {
            (*self.ptr).loaded != 0
        }
    }
    pub fn newKeyboard(&self) -> bool {
        unsafe {
            (*self.ptr).newKeyboard != 0
        }
    }
    pub fn found(&self) -> u16 {
        unsafe {
            (*self.ptr).found
        }
    }
    pub fn reported(&self) -> u16 {
        unsafe {
            (*self.ptr).reported
        }
    }
}

pub fn get_kbd_by_name<'a>(c         : &'a base::Connection,
                           deviceSpec: DeviceSpec,
                           need      : u16,
                           want      : u16,
                           load      : bool)
        -> GetKbdByNameCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_kbd_by_name(c.get_raw_conn(),
                                             deviceSpec as xcb_xkb_device_spec_t,  // 0
                                             need as u16,  // 1
                                             want as u16,  // 2
                                             load as u8);  // 3
        GetKbdByNameCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_kbd_by_name_unchecked<'a>(c         : &'a base::Connection,
                                     deviceSpec: DeviceSpec,
                                     need      : u16,
                                     want      : u16,
                                     load      : bool)
        -> GetKbdByNameCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_kbd_by_name_unchecked(c.get_raw_conn(),
                                                       deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                       need as u16,  // 1
                                                       want as u16,  // 2
                                                       load as u8);  // 3
        GetKbdByNameCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const GET_DEVICE_INFO: u8 = 24;

impl base::CookieSeq for xcb_xkb_get_device_info_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type GetDeviceInfoCookie<'a> = base::Cookie<'a, xcb_xkb_get_device_info_cookie_t>;

impl<'a> GetDeviceInfoCookie<'a> {
    pub fn get_reply(self) -> Result<GetDeviceInfoReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            GetDeviceInfoReply {
                ptr: xcb_xkb_get_device_info_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type GetDeviceInfoReply = base::Reply<xcb_xkb_get_device_info_reply_t>;

impl GetDeviceInfoReply {
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn present(&self) -> u16 {
        unsafe {
            (*self.ptr).present
        }
    }
    pub fn supported(&self) -> u16 {
        unsafe {
            (*self.ptr).supported
        }
    }
    pub fn unsupported(&self) -> u16 {
        unsafe {
            (*self.ptr).unsupported
        }
    }
    pub fn nDeviceLedFBs(&self) -> u16 {
        unsafe {
            (*self.ptr).nDeviceLedFBs
        }
    }
    pub fn firstBtnWanted(&self) -> u8 {
        unsafe {
            (*self.ptr).firstBtnWanted
        }
    }
    pub fn nBtnsWanted(&self) -> u8 {
        unsafe {
            (*self.ptr).nBtnsWanted
        }
    }
    pub fn firstBtnRtrn(&self) -> u8 {
        unsafe {
            (*self.ptr).firstBtnRtrn
        }
    }
    pub fn nBtnsRtrn(&self) -> u8 {
        unsafe {
            (*self.ptr).nBtnsRtrn
        }
    }
    pub fn totalBtns(&self) -> u8 {
        unsafe {
            (*self.ptr).totalBtns
        }
    }
    pub fn hasOwnState(&self) -> bool {
        unsafe {
            (*self.ptr).hasOwnState != 0
        }
    }
    pub fn dfltKbdFB(&self) -> u16 {
        unsafe {
            (*self.ptr).dfltKbdFB
        }
    }
    pub fn dfltLedFB(&self) -> u16 {
        unsafe {
            (*self.ptr).dfltLedFB
        }
    }
    pub fn devType(&self) -> xproto::Atom {
        unsafe {
            (*self.ptr).devType
        }
    }
    pub fn nameLen(&self) -> u16 {
        unsafe {
            (*self.ptr).nameLen
        }
    }
    pub fn name(&self) -> &[String8] {
        unsafe {
            let field = self.ptr;
            let len = xcb_xkb_get_device_info_name_length(field) as usize;
            let data = xcb_xkb_get_device_info_name(field);
            std::slice::from_raw_parts(data, len)
        }
    }
    pub fn btnActions(&self) -> ActionIterator {
        unsafe {
            xcb_xkb_get_device_info_btn_actions_iterator(self.ptr)
        }
    }
    pub fn leds(&self) -> DeviceLedInfoIterator {
        unsafe {
            xcb_xkb_get_device_info_leds_iterator(self.ptr)
        }
    }
}

pub fn get_device_info<'a>(c          : &'a base::Connection,
                           deviceSpec : DeviceSpec,
                           wanted     : u16,
                           allButtons : bool,
                           firstButton: u8,
                           nButtons   : u8,
                           ledClass   : LedClassSpec,
                           ledID      : IdSpec)
        -> GetDeviceInfoCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_device_info(c.get_raw_conn(),
                                             deviceSpec as xcb_xkb_device_spec_t,  // 0
                                             wanted as u16,  // 1
                                             allButtons as u8,  // 2
                                             firstButton as u8,  // 3
                                             nButtons as u8,  // 4
                                             ledClass as xcb_xkb_led_class_spec_t,  // 5
                                             ledID as xcb_xkb_id_spec_t);  // 6
        GetDeviceInfoCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn get_device_info_unchecked<'a>(c          : &'a base::Connection,
                                     deviceSpec : DeviceSpec,
                                     wanted     : u16,
                                     allButtons : bool,
                                     firstButton: u8,
                                     nButtons   : u8,
                                     ledClass   : LedClassSpec,
                                     ledID      : IdSpec)
        -> GetDeviceInfoCookie<'a> {
    unsafe {
        let cookie = xcb_xkb_get_device_info_unchecked(c.get_raw_conn(),
                                                       deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                       wanted as u16,  // 1
                                                       allButtons as u8,  // 2
                                                       firstButton as u8,  // 3
                                                       nButtons as u8,  // 4
                                                       ledClass as xcb_xkb_led_class_spec_t,  // 5
                                                       ledID as xcb_xkb_id_spec_t);  // 6
        GetDeviceInfoCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const SET_DEVICE_INFO: u8 = 25;

pub fn set_device_info<'a>(c         : &'a base::Connection,
                           deviceSpec: DeviceSpec,
                           firstBtn  : u8,
                           change    : u16,
                           btnActions: &[Action],
                           leds      : &[DeviceLedInfo])
        -> base::VoidCookie<'a> {
    unsafe {
        let btnActions_len = btnActions.len();
        let btnActions_ptr = btnActions.as_ptr();
        let leds_len = leds.len();
        let leds_ptr = leds.as_ptr();
        let cookie = xcb_xkb_set_device_info(c.get_raw_conn(),
                                             deviceSpec as xcb_xkb_device_spec_t,  // 0
                                             firstBtn as u8,  // 1
                                             btnActions_len as u8,  // 2
                                             change as u16,  // 3
                                             leds_len as u16,  // 4
                                             btnActions_ptr as *const xcb_xkb_action_t,  // 5
                                             leds_ptr as *const xcb_xkb_device_led_info_t);  // 6
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub fn set_device_info_checked<'a>(c         : &'a base::Connection,
                                   deviceSpec: DeviceSpec,
                                   firstBtn  : u8,
                                   change    : u16,
                                   btnActions: &[Action],
                                   leds      : &[DeviceLedInfo])
        -> base::VoidCookie<'a> {
    unsafe {
        let btnActions_len = btnActions.len();
        let btnActions_ptr = btnActions.as_ptr();
        let leds_len = leds.len();
        let leds_ptr = leds.as_ptr();
        let cookie = xcb_xkb_set_device_info_checked(c.get_raw_conn(),
                                                     deviceSpec as xcb_xkb_device_spec_t,  // 0
                                                     firstBtn as u8,  // 1
                                                     btnActions_len as u8,  // 2
                                                     change as u16,  // 3
                                                     leds_len as u16,  // 4
                                                     btnActions_ptr as *const xcb_xkb_action_t,  // 5
                                                     leds_ptr as *const xcb_xkb_device_led_info_t);  // 6
        base::VoidCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub const SET_DEBUGGING_FLAGS: u8 = 101;

impl base::CookieSeq for xcb_xkb_set_debugging_flags_cookie_t {
    fn sequence(&self) -> c_uint { self.sequence }
}

pub type SetDebuggingFlagsCookie<'a> = base::Cookie<'a, xcb_xkb_set_debugging_flags_cookie_t>;

impl<'a> SetDebuggingFlagsCookie<'a> {
    pub fn get_reply(self) -> Result<SetDebuggingFlagsReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {&mut err} else {std::ptr::null_mut()};
        let reply = unsafe {
            SetDebuggingFlagsReply {
                ptr: xcb_xkb_set_debugging_flags_reply (self.conn.get_raw_conn(), self.cookie, err_ptr)
            }
        };
        let checked = self.checked;
        std::mem::forget(self); // won't call discard on cookie

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok (reply),
            (false, true, true) => Ok (reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c))
        }
    }
}

pub type SetDebuggingFlagsReply = base::Reply<xcb_xkb_set_debugging_flags_reply_t>;

impl SetDebuggingFlagsReply {
    pub fn currentFlags(&self) -> u32 {
        unsafe {
            (*self.ptr).currentFlags
        }
    }
    pub fn currentCtrls(&self) -> u32 {
        unsafe {
            (*self.ptr).currentCtrls
        }
    }
    pub fn supportedFlags(&self) -> u32 {
        unsafe {
            (*self.ptr).supportedFlags
        }
    }
    pub fn supportedCtrls(&self) -> u32 {
        unsafe {
            (*self.ptr).supportedCtrls
        }
    }
}

pub fn set_debugging_flags<'a>(c          : &'a base::Connection,
                               affectFlags: u32,
                               flags      : u32,
                               affectCtrls: u32,
                               ctrls      : u32,
                               message    : &[String8])
        -> SetDebuggingFlagsCookie<'a> {
    unsafe {
        let message_len = message.len();
        let message_ptr = message.as_ptr();
        let cookie = xcb_xkb_set_debugging_flags(c.get_raw_conn(),
                                                 message_len as u16,  // 0
                                                 affectFlags as u32,  // 1
                                                 flags as u32,  // 2
                                                 affectCtrls as u32,  // 3
                                                 ctrls as u32,  // 4
                                                 message_ptr as *const xcb_xkb_string8_t);  // 5
        SetDebuggingFlagsCookie {
            cookie:  cookie,
            conn:    c,
            checked: true
        }
    }
}

pub fn set_debugging_flags_unchecked<'a>(c          : &'a base::Connection,
                                         affectFlags: u32,
                                         flags      : u32,
                                         affectCtrls: u32,
                                         ctrls      : u32,
                                         message    : &[String8])
        -> SetDebuggingFlagsCookie<'a> {
    unsafe {
        let message_len = message.len();
        let message_ptr = message.as_ptr();
        let cookie = xcb_xkb_set_debugging_flags_unchecked(c.get_raw_conn(),
                                                           message_len as u16,  // 0
                                                           affectFlags as u32,  // 1
                                                           flags as u32,  // 2
                                                           affectCtrls as u32,  // 3
                                                           ctrls as u32,  // 4
                                                           message_ptr as *const xcb_xkb_string8_t);  // 5
        SetDebuggingFlagsCookie {
            cookie:  cookie,
            conn:    c,
            checked: false
        }
    }
}

pub const NEW_KEYBOARD_NOTIFY: u8 = 0;

pub type NewKeyboardNotifyEvent = base::Event<xcb_xkb_new_keyboard_notify_event_t>;

impl NewKeyboardNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn oldDeviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).oldDeviceID
        }
    }
    pub fn minKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).minKeyCode
        }
    }
    pub fn maxKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).maxKeyCode
        }
    }
    pub fn oldMinKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).oldMinKeyCode
        }
    }
    pub fn oldMaxKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).oldMaxKeyCode
        }
    }
    pub fn requestMajor(&self) -> u8 {
        unsafe {
            (*self.ptr).requestMajor
        }
    }
    pub fn requestMinor(&self) -> u8 {
        unsafe {
            (*self.ptr).requestMinor
        }
    }
    pub fn changed(&self) -> u16 {
        unsafe {
            (*self.ptr).changed
        }
    }
    /// Constructs a new NewKeyboardNotifyEvent
    /// `response_type` will be set automatically to NEW_KEYBOARD_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               oldDeviceID: u8,
               minKeyCode: xproto::Keycode,
               maxKeyCode: xproto::Keycode,
               oldMinKeyCode: xproto::Keycode,
               oldMaxKeyCode: xproto::Keycode,
               requestMajor: u8,
               requestMinor: u8,
               changed: u16)
            -> NewKeyboardNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_new_keyboard_notify_event_t;
            (*raw).response_type = NEW_KEYBOARD_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).oldDeviceID = oldDeviceID;
            (*raw).minKeyCode = minKeyCode;
            (*raw).maxKeyCode = maxKeyCode;
            (*raw).oldMinKeyCode = oldMinKeyCode;
            (*raw).oldMaxKeyCode = oldMaxKeyCode;
            (*raw).requestMajor = requestMajor;
            (*raw).requestMinor = requestMinor;
            (*raw).changed = changed;
            NewKeyboardNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const MAP_NOTIFY: u8 = 1;

pub type MapNotifyEvent = base::Event<xcb_xkb_map_notify_event_t>;

impl MapNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn ptrBtnActions(&self) -> u8 {
        unsafe {
            (*self.ptr).ptrBtnActions
        }
    }
    pub fn changed(&self) -> u16 {
        unsafe {
            (*self.ptr).changed
        }
    }
    pub fn minKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).minKeyCode
        }
    }
    pub fn maxKeyCode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).maxKeyCode
        }
    }
    pub fn firstType(&self) -> u8 {
        unsafe {
            (*self.ptr).firstType
        }
    }
    pub fn nTypes(&self) -> u8 {
        unsafe {
            (*self.ptr).nTypes
        }
    }
    pub fn firstKeySym(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKeySym
        }
    }
    pub fn nKeySyms(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeySyms
        }
    }
    pub fn firstKeyAct(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKeyAct
        }
    }
    pub fn nKeyActs(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeyActs
        }
    }
    pub fn firstKeyBehavior(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKeyBehavior
        }
    }
    pub fn nKeyBehavior(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeyBehavior
        }
    }
    pub fn firstKeyExplicit(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKeyExplicit
        }
    }
    pub fn nKeyExplicit(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeyExplicit
        }
    }
    pub fn firstModMapKey(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstModMapKey
        }
    }
    pub fn nModMapKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).nModMapKeys
        }
    }
    pub fn firstVModMapKey(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstVModMapKey
        }
    }
    pub fn nVModMapKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).nVModMapKeys
        }
    }
    pub fn virtualMods(&self) -> u16 {
        unsafe {
            (*self.ptr).virtualMods
        }
    }
    /// Constructs a new MapNotifyEvent
    /// `response_type` will be set automatically to MAP_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               ptrBtnActions: u8,
               changed: u16,
               minKeyCode: xproto::Keycode,
               maxKeyCode: xproto::Keycode,
               firstType: u8,
               nTypes: u8,
               firstKeySym: xproto::Keycode,
               nKeySyms: u8,
               firstKeyAct: xproto::Keycode,
               nKeyActs: u8,
               firstKeyBehavior: xproto::Keycode,
               nKeyBehavior: u8,
               firstKeyExplicit: xproto::Keycode,
               nKeyExplicit: u8,
               firstModMapKey: xproto::Keycode,
               nModMapKeys: u8,
               firstVModMapKey: xproto::Keycode,
               nVModMapKeys: u8,
               virtualMods: u16)
            -> MapNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_map_notify_event_t;
            (*raw).response_type = MAP_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).ptrBtnActions = ptrBtnActions;
            (*raw).changed = changed;
            (*raw).minKeyCode = minKeyCode;
            (*raw).maxKeyCode = maxKeyCode;
            (*raw).firstType = firstType;
            (*raw).nTypes = nTypes;
            (*raw).firstKeySym = firstKeySym;
            (*raw).nKeySyms = nKeySyms;
            (*raw).firstKeyAct = firstKeyAct;
            (*raw).nKeyActs = nKeyActs;
            (*raw).firstKeyBehavior = firstKeyBehavior;
            (*raw).nKeyBehavior = nKeyBehavior;
            (*raw).firstKeyExplicit = firstKeyExplicit;
            (*raw).nKeyExplicit = nKeyExplicit;
            (*raw).firstModMapKey = firstModMapKey;
            (*raw).nModMapKeys = nModMapKeys;
            (*raw).firstVModMapKey = firstVModMapKey;
            (*raw).nVModMapKeys = nVModMapKeys;
            (*raw).virtualMods = virtualMods;
            MapNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const STATE_NOTIFY: u8 = 2;

pub type StateNotifyEvent = base::Event<xcb_xkb_state_notify_event_t>;

impl StateNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn mods(&self) -> u8 {
        unsafe {
            (*self.ptr).mods
        }
    }
    pub fn baseMods(&self) -> u8 {
        unsafe {
            (*self.ptr).baseMods
        }
    }
    pub fn latchedMods(&self) -> u8 {
        unsafe {
            (*self.ptr).latchedMods
        }
    }
    pub fn lockedMods(&self) -> u8 {
        unsafe {
            (*self.ptr).lockedMods
        }
    }
    pub fn group(&self) -> u8 {
        unsafe {
            (*self.ptr).group
        }
    }
    pub fn baseGroup(&self) -> i16 {
        unsafe {
            (*self.ptr).baseGroup
        }
    }
    pub fn latchedGroup(&self) -> i16 {
        unsafe {
            (*self.ptr).latchedGroup
        }
    }
    pub fn lockedGroup(&self) -> u8 {
        unsafe {
            (*self.ptr).lockedGroup
        }
    }
    pub fn compatState(&self) -> u8 {
        unsafe {
            (*self.ptr).compatState
        }
    }
    pub fn grabMods(&self) -> u8 {
        unsafe {
            (*self.ptr).grabMods
        }
    }
    pub fn compatGrabMods(&self) -> u8 {
        unsafe {
            (*self.ptr).compatGrabMods
        }
    }
    pub fn lookupMods(&self) -> u8 {
        unsafe {
            (*self.ptr).lookupMods
        }
    }
    pub fn compatLoockupMods(&self) -> u8 {
        unsafe {
            (*self.ptr).compatLoockupMods
        }
    }
    pub fn ptrBtnState(&self) -> u16 {
        unsafe {
            (*self.ptr).ptrBtnState
        }
    }
    pub fn changed(&self) -> u16 {
        unsafe {
            (*self.ptr).changed
        }
    }
    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).keycode
        }
    }
    pub fn eventType(&self) -> u8 {
        unsafe {
            (*self.ptr).eventType
        }
    }
    pub fn requestMajor(&self) -> u8 {
        unsafe {
            (*self.ptr).requestMajor
        }
    }
    pub fn requestMinor(&self) -> u8 {
        unsafe {
            (*self.ptr).requestMinor
        }
    }
    /// Constructs a new StateNotifyEvent
    /// `response_type` will be set automatically to STATE_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               mods: u8,
               baseMods: u8,
               latchedMods: u8,
               lockedMods: u8,
               group: u8,
               baseGroup: i16,
               latchedGroup: i16,
               lockedGroup: u8,
               compatState: u8,
               grabMods: u8,
               compatGrabMods: u8,
               lookupMods: u8,
               compatLoockupMods: u8,
               ptrBtnState: u16,
               changed: u16,
               keycode: xproto::Keycode,
               eventType: u8,
               requestMajor: u8,
               requestMinor: u8)
            -> StateNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_state_notify_event_t;
            (*raw).response_type = STATE_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).mods = mods;
            (*raw).baseMods = baseMods;
            (*raw).latchedMods = latchedMods;
            (*raw).lockedMods = lockedMods;
            (*raw).group = group;
            (*raw).baseGroup = baseGroup;
            (*raw).latchedGroup = latchedGroup;
            (*raw).lockedGroup = lockedGroup;
            (*raw).compatState = compatState;
            (*raw).grabMods = grabMods;
            (*raw).compatGrabMods = compatGrabMods;
            (*raw).lookupMods = lookupMods;
            (*raw).compatLoockupMods = compatLoockupMods;
            (*raw).ptrBtnState = ptrBtnState;
            (*raw).changed = changed;
            (*raw).keycode = keycode;
            (*raw).eventType = eventType;
            (*raw).requestMajor = requestMajor;
            (*raw).requestMinor = requestMinor;
            StateNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const CONTROLS_NOTIFY: u8 = 3;

pub type ControlsNotifyEvent = base::Event<xcb_xkb_controls_notify_event_t>;

impl ControlsNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn numGroups(&self) -> u8 {
        unsafe {
            (*self.ptr).numGroups
        }
    }
    pub fn changedControls(&self) -> u32 {
        unsafe {
            (*self.ptr).changedControls
        }
    }
    pub fn enabledControls(&self) -> u32 {
        unsafe {
            (*self.ptr).enabledControls
        }
    }
    pub fn enabledControlChanges(&self) -> u32 {
        unsafe {
            (*self.ptr).enabledControlChanges
        }
    }
    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).keycode
        }
    }
    pub fn eventType(&self) -> u8 {
        unsafe {
            (*self.ptr).eventType
        }
    }
    pub fn requestMajor(&self) -> u8 {
        unsafe {
            (*self.ptr).requestMajor
        }
    }
    pub fn requestMinor(&self) -> u8 {
        unsafe {
            (*self.ptr).requestMinor
        }
    }
    /// Constructs a new ControlsNotifyEvent
    /// `response_type` will be set automatically to CONTROLS_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               numGroups: u8,
               changedControls: u32,
               enabledControls: u32,
               enabledControlChanges: u32,
               keycode: xproto::Keycode,
               eventType: u8,
               requestMajor: u8,
               requestMinor: u8)
            -> ControlsNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_controls_notify_event_t;
            (*raw).response_type = CONTROLS_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).numGroups = numGroups;
            (*raw).changedControls = changedControls;
            (*raw).enabledControls = enabledControls;
            (*raw).enabledControlChanges = enabledControlChanges;
            (*raw).keycode = keycode;
            (*raw).eventType = eventType;
            (*raw).requestMajor = requestMajor;
            (*raw).requestMinor = requestMinor;
            ControlsNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const INDICATOR_STATE_NOTIFY: u8 = 4;

pub type IndicatorStateNotifyEvent = base::Event<xcb_xkb_indicator_state_notify_event_t>;

impl IndicatorStateNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn state(&self) -> u32 {
        unsafe {
            (*self.ptr).state
        }
    }
    pub fn stateChanged(&self) -> u32 {
        unsafe {
            (*self.ptr).stateChanged
        }
    }
    /// Constructs a new IndicatorStateNotifyEvent
    /// `response_type` will be set automatically to INDICATOR_STATE_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               state: u32,
               stateChanged: u32)
            -> IndicatorStateNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_indicator_state_notify_event_t;
            (*raw).response_type = INDICATOR_STATE_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).state = state;
            (*raw).stateChanged = stateChanged;
            IndicatorStateNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const INDICATOR_MAP_NOTIFY: u8 = 5;

pub type IndicatorMapNotifyEvent = base::Event<xcb_xkb_indicator_map_notify_event_t>;

impl IndicatorMapNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn state(&self) -> u32 {
        unsafe {
            (*self.ptr).state
        }
    }
    pub fn mapChanged(&self) -> u32 {
        unsafe {
            (*self.ptr).mapChanged
        }
    }
    /// Constructs a new IndicatorMapNotifyEvent
    /// `response_type` will be set automatically to INDICATOR_MAP_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               state: u32,
               mapChanged: u32)
            -> IndicatorMapNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_indicator_map_notify_event_t;
            (*raw).response_type = INDICATOR_MAP_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).state = state;
            (*raw).mapChanged = mapChanged;
            IndicatorMapNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const NAMES_NOTIFY: u8 = 6;

pub type NamesNotifyEvent = base::Event<xcb_xkb_names_notify_event_t>;

impl NamesNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn changed(&self) -> u16 {
        unsafe {
            (*self.ptr).changed
        }
    }
    pub fn firstType(&self) -> u8 {
        unsafe {
            (*self.ptr).firstType
        }
    }
    pub fn nTypes(&self) -> u8 {
        unsafe {
            (*self.ptr).nTypes
        }
    }
    pub fn firstLevelName(&self) -> u8 {
        unsafe {
            (*self.ptr).firstLevelName
        }
    }
    pub fn nLevelNames(&self) -> u8 {
        unsafe {
            (*self.ptr).nLevelNames
        }
    }
    pub fn nRadioGroups(&self) -> u8 {
        unsafe {
            (*self.ptr).nRadioGroups
        }
    }
    pub fn nKeyAliases(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeyAliases
        }
    }
    pub fn changedGroupNames(&self) -> u8 {
        unsafe {
            (*self.ptr).changedGroupNames
        }
    }
    pub fn changedVirtualMods(&self) -> u16 {
        unsafe {
            (*self.ptr).changedVirtualMods
        }
    }
    pub fn firstKey(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).firstKey
        }
    }
    pub fn nKeys(&self) -> u8 {
        unsafe {
            (*self.ptr).nKeys
        }
    }
    pub fn changedIndicators(&self) -> u32 {
        unsafe {
            (*self.ptr).changedIndicators
        }
    }
    /// Constructs a new NamesNotifyEvent
    /// `response_type` will be set automatically to NAMES_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               changed: u16,
               firstType: u8,
               nTypes: u8,
               firstLevelName: u8,
               nLevelNames: u8,
               nRadioGroups: u8,
               nKeyAliases: u8,
               changedGroupNames: u8,
               changedVirtualMods: u16,
               firstKey: xproto::Keycode,
               nKeys: u8,
               changedIndicators: u32)
            -> NamesNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_names_notify_event_t;
            (*raw).response_type = NAMES_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).changed = changed;
            (*raw).firstType = firstType;
            (*raw).nTypes = nTypes;
            (*raw).firstLevelName = firstLevelName;
            (*raw).nLevelNames = nLevelNames;
            (*raw).nRadioGroups = nRadioGroups;
            (*raw).nKeyAliases = nKeyAliases;
            (*raw).changedGroupNames = changedGroupNames;
            (*raw).changedVirtualMods = changedVirtualMods;
            (*raw).firstKey = firstKey;
            (*raw).nKeys = nKeys;
            (*raw).changedIndicators = changedIndicators;
            NamesNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const COMPAT_MAP_NOTIFY: u8 = 7;

pub type CompatMapNotifyEvent = base::Event<xcb_xkb_compat_map_notify_event_t>;

impl CompatMapNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn changedGroups(&self) -> u8 {
        unsafe {
            (*self.ptr).changedGroups
        }
    }
    pub fn firstSI(&self) -> u16 {
        unsafe {
            (*self.ptr).firstSI
        }
    }
    pub fn nSI(&self) -> u16 {
        unsafe {
            (*self.ptr).nSI
        }
    }
    pub fn nTotalSI(&self) -> u16 {
        unsafe {
            (*self.ptr).nTotalSI
        }
    }
    /// Constructs a new CompatMapNotifyEvent
    /// `response_type` will be set automatically to COMPAT_MAP_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               changedGroups: u8,
               firstSI: u16,
               nSI: u16,
               nTotalSI: u16)
            -> CompatMapNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_compat_map_notify_event_t;
            (*raw).response_type = COMPAT_MAP_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).changedGroups = changedGroups;
            (*raw).firstSI = firstSI;
            (*raw).nSI = nSI;
            (*raw).nTotalSI = nTotalSI;
            CompatMapNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const BELL_NOTIFY: u8 = 8;

pub type BellNotifyEvent = base::Event<xcb_xkb_bell_notify_event_t>;

impl BellNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn bellClass(&self) -> u8 {
        unsafe {
            (*self.ptr).bellClass
        }
    }
    pub fn bellID(&self) -> u8 {
        unsafe {
            (*self.ptr).bellID
        }
    }
    pub fn percent(&self) -> u8 {
        unsafe {
            (*self.ptr).percent
        }
    }
    pub fn pitch(&self) -> u16 {
        unsafe {
            (*self.ptr).pitch
        }
    }
    pub fn duration(&self) -> u16 {
        unsafe {
            (*self.ptr).duration
        }
    }
    pub fn name(&self) -> xproto::Atom {
        unsafe {
            (*self.ptr).name
        }
    }
    pub fn window(&self) -> xproto::Window {
        unsafe {
            (*self.ptr).window
        }
    }
    pub fn eventOnly(&self) -> bool {
        unsafe {
            (*self.ptr).eventOnly != 0
        }
    }
    /// Constructs a new BellNotifyEvent
    /// `response_type` will be set automatically to BELL_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               bellClass: u8,
               bellID: u8,
               percent: u8,
               pitch: u16,
               duration: u16,
               name: xproto::Atom,
               window: xproto::Window,
               eventOnly: bool)
            -> BellNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_bell_notify_event_t;
            (*raw).response_type = BELL_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).bellClass = bellClass;
            (*raw).bellID = bellID;
            (*raw).percent = percent;
            (*raw).pitch = pitch;
            (*raw).duration = duration;
            (*raw).name = name;
            (*raw).window = window;
            (*raw).eventOnly = if eventOnly { 1 } else { 0 };
            BellNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const ACTION_MESSAGE: u8 = 9;

pub type ActionMessageEvent = base::Event<xcb_xkb_action_message_event_t>;

impl ActionMessageEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).keycode
        }
    }
    pub fn press(&self) -> bool {
        unsafe {
            (*self.ptr).press != 0
        }
    }
    pub fn keyEventFollows(&self) -> bool {
        unsafe {
            (*self.ptr).keyEventFollows != 0
        }
    }
    pub fn mods(&self) -> u8 {
        unsafe {
            (*self.ptr).mods
        }
    }
    pub fn group(&self) -> u8 {
        unsafe {
            (*self.ptr).group
        }
    }
    pub fn message(&self) -> &[String8] {
        unsafe {
            &(*self.ptr).message
        }
    }
    /// Constructs a new ActionMessageEvent
    /// `response_type` will be set automatically to ACTION_MESSAGE
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               keycode: xproto::Keycode,
               press: bool,
               keyEventFollows: bool,
               mods: u8,
               group: u8,
               message: [String8; 8])
            -> ActionMessageEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_action_message_event_t;
            (*raw).response_type = ACTION_MESSAGE;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).keycode = keycode;
            (*raw).press = if press { 1 } else { 0 };
            (*raw).keyEventFollows = if keyEventFollows { 1 } else { 0 };
            (*raw).mods = mods;
            (*raw).group = group;
            (*raw).message = message;
            ActionMessageEvent {
                ptr: raw
            }
        }
    }
}

pub const ACCESS_X_NOTIFY: u8 = 10;

pub type AccessXNotifyEvent = base::Event<xcb_xkb_access_x_notify_event_t>;

impl AccessXNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            (*self.ptr).keycode
        }
    }
    pub fn detailt(&self) -> u16 {
        unsafe {
            (*self.ptr).detailt
        }
    }
    pub fn slowKeysDelay(&self) -> u16 {
        unsafe {
            (*self.ptr).slowKeysDelay
        }
    }
    pub fn debounceDelay(&self) -> u16 {
        unsafe {
            (*self.ptr).debounceDelay
        }
    }
    /// Constructs a new AccessXNotifyEvent
    /// `response_type` will be set automatically to ACCESS_X_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               keycode: xproto::Keycode,
               detailt: u16,
               slowKeysDelay: u16,
               debounceDelay: u16)
            -> AccessXNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_access_x_notify_event_t;
            (*raw).response_type = ACCESS_X_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).keycode = keycode;
            (*raw).detailt = detailt;
            (*raw).slowKeysDelay = slowKeysDelay;
            (*raw).debounceDelay = debounceDelay;
            AccessXNotifyEvent {
                ptr: raw
            }
        }
    }
}

pub const EXTENSION_DEVICE_NOTIFY: u8 = 11;

pub type ExtensionDeviceNotifyEvent = base::Event<xcb_xkb_extension_device_notify_event_t>;

impl ExtensionDeviceNotifyEvent {
    pub fn xkbType(&self) -> u8 {
        unsafe {
            (*self.ptr).xkbType
        }
    }
    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            (*self.ptr).time
        }
    }
    pub fn deviceID(&self) -> u8 {
        unsafe {
            (*self.ptr).deviceID
        }
    }
    pub fn reason(&self) -> u16 {
        unsafe {
            (*self.ptr).reason
        }
    }
    pub fn ledClass(&self) -> u16 {
        unsafe {
            (*self.ptr).ledClass
        }
    }
    pub fn ledID(&self) -> u16 {
        unsafe {
            (*self.ptr).ledID
        }
    }
    pub fn ledsDefined(&self) -> u32 {
        unsafe {
            (*self.ptr).ledsDefined
        }
    }
    pub fn ledState(&self) -> u32 {
        unsafe {
            (*self.ptr).ledState
        }
    }
    pub fn firstButton(&self) -> u8 {
        unsafe {
            (*self.ptr).firstButton
        }
    }
    pub fn nButtons(&self) -> u8 {
        unsafe {
            (*self.ptr).nButtons
        }
    }
    pub fn supported(&self) -> u16 {
        unsafe {
            (*self.ptr).supported
        }
    }
    pub fn unsupported(&self) -> u16 {
        unsafe {
            (*self.ptr).unsupported
        }
    }
    /// Constructs a new ExtensionDeviceNotifyEvent
    /// `response_type` will be set automatically to EXTENSION_DEVICE_NOTIFY
    pub fn new(xkbType: u8,
               time: xproto::Timestamp,
               deviceID: u8,
               reason: u16,
               ledClass: u16,
               ledID: u16,
               ledsDefined: u32,
               ledState: u32,
               firstButton: u8,
               nButtons: u8,
               supported: u16,
               unsupported: u16)
            -> ExtensionDeviceNotifyEvent {
        unsafe {
            let raw = libc::malloc(32 as usize) as *mut xcb_xkb_extension_device_notify_event_t;
            (*raw).response_type = EXTENSION_DEVICE_NOTIFY;
            (*raw).xkbType = xkbType;
            (*raw).time = time;
            (*raw).deviceID = deviceID;
            (*raw).reason = reason;
            (*raw).ledClass = ledClass;
            (*raw).ledID = ledID;
            (*raw).ledsDefined = ledsDefined;
            (*raw).ledState = ledState;
            (*raw).firstButton = firstButton;
            (*raw).nButtons = nButtons;
            (*raw).supported = supported;
            (*raw).unsupported = unsupported;
            ExtensionDeviceNotifyEvent {
                ptr: raw
            }
        }
    }
}
