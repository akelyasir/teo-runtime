pub(super) const CREATE_U32: u32 = 1;
pub(super) const UPDATE_U32: u32 = 1 << 1;
pub(super) const DELETE_U32: u32 = 1 << 2;
pub(super) const COPY_U32: u32 = 1 << 3;
pub(super) const FIND_U32: u32 = 1 << 4;
pub(super) const FIRST_U32: u32 = 1 << 5;
pub(super) const CONNECT_U32: u32 = 1 << 6;
pub(super) const DISCONNECT_U32: u32 = 1 << 7;
pub(super) const SET_U32: u32 = 1 << 8;
pub(super) const JOIN_U32: u32 = 1 << 9;
pub(super) const COUNT_U32: u32 = 1 << 10;
pub(super) const AGGREGATE_U32: u32 = 1 << 11;
pub(super) const GROUP_BY_U32: u32 = 1 << 12;
pub(super) const CODE_NAME_U32: u32 = 1 << 13;

pub(super) const UPSERT_U32: u32 = CREATE_U32 | UPDATE_U32;
pub(super) const CONNECT_OR_CREATE_U32: u32 = CONNECT_U32 | CREATE_U32;
pub(super) const JOIN_CREATE_U32: u32 = JOIN_U32 | CREATE_U32;
pub(super) const JOIN_DELETE_U32: u32 = JOIN_U32 | DELETE_U32;
pub(super) const FIND_FIRST_U32: u32 = FIND_U32 | FIRST_U32;

pub(super) const ENTRY_U32: u32 = 1 << 14;
pub(super) const NESTED_U32: u32 = 1 << 15;
pub(super) const CODE_POSITION_U32: u32 = 1 << 16;

pub(super) const SINGLE_U32: u32 = 1 << 17;
pub(super) const MANY_U32: u32 = 1 << 18;
pub(super) const CODE_AMOUNT_U32: u32 = 1 << 19;

pub(super) const ALL_NAMES_U32: u32 = CREATE_U32 | UPDATE_U32 | UPSERT_U32 | DELETE_U32 | COPY_U32 | FIND_U32 | FIND_FIRST_U32 | CONNECT_U32 | CONNECT_OR_CREATE_U32 | DISCONNECT_U32 | SET_U32 | JOIN_CREATE_U32 | JOIN_DELETE_U32 | COUNT_U32 | AGGREGATE_U32 | GROUP_BY_U32 | CODE_NAME_U32;
pub(super) const ALL_POSITIONS_U32: u32 = ENTRY_U32 | NESTED_U32 | CODE_POSITION_U32;
pub(super) const ALL_AMOUNTS_U32: u32 = SINGLE_U32 | MANY_U32 | CODE_AMOUNT_U32;

pub(super) const NOT_ALL_NAMES_U32: u32 = !ALL_NAMES_U32;
pub(super) const NOT_ENTRY_NESTED_U32: u32 = !ALL_POSITIONS_U32;
pub(super) const NOT_SINGLE_MANY_U32: u32 = !ALL_AMOUNTS_U32;

pub(super) const FIND_UNIQUE_HANDLER_U32: u32 = FIND_U32 | ENTRY_U32 | SINGLE_U32;
pub(super) const FIND_FIRST_HANDLER_U32: u32 = FIND_FIRST_U32 | ENTRY_U32 | SINGLE_U32;
pub(super) const FIND_MANY_HANDLER_U32: u32 = FIND_U32 | ENTRY_U32 | MANY_U32;
pub(super) const CREATE_HANDLER_U32: u32 = CREATE_U32 | ENTRY_U32 | SINGLE_U32;
pub(super) const UPDATE_HANDLER_U32: u32 = UPDATE_U32 | ENTRY_U32 | SINGLE_U32;
pub(super) const UPSERT_HANDLER_U32: u32 = UPSERT_U32 | ENTRY_U32 | SINGLE_U32;
pub(super) const DELETE_HANDLER_U32: u32 = DELETE_U32 | ENTRY_U32 | SINGLE_U32;
pub(super) const CREATE_MANY_HANDLER_U32: u32 = CREATE_U32 | ENTRY_U32 | MANY_U32;
pub(super) const UPDATE_MANY_HANDLER_U32: u32 = UPDATE_U32 | ENTRY_U32 | MANY_U32;
pub(super) const DELETE_MANY_HANDLER_U32: u32 = DELETE_U32 | ENTRY_U32 | MANY_U32;
pub(super) const COUNT_HANDLER_U32: u32 = COUNT_U32 | ENTRY_U32;
pub(super) const AGGREGATE_HANDLER_U32: u32 = AGGREGATE_U32 | ENTRY_U32;
pub(super) const GROUP_BY_HANDLER_U32: u32 = GROUP_BY_U32 | ENTRY_U32;

pub(super) const NESTED_CREATE_ACTION_U32: u32 = CREATE_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_UPDATE_ACTION_U32: u32 = UPDATE_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_UPSERT_ACTION_U32: u32 = UPSERT_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_DELETE_ACTION_U32: u32 = DELETE_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_COPY_ACTION_U32: u32 = COPY_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_CONNECT_OR_CREATE_ACTION_U32: u32 = CONNECT_OR_CREATE_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_CONNECT_ACTION_U32: u32 = CONNECT_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_DISCONNECT_ACTION_U32: u32 = DISCONNECT_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_SET_ACTION_U32: u32 = SET_U32 | NESTED_U32 | SINGLE_U32;
pub(super) const NESTED_CREATE_MANY_ACTION_U32: u32 = CREATE_U32 | NESTED_U32 | MANY_U32;
pub(super) const NESTED_UPDATE_MANY_ACTION_U32: u32 = UPDATE_U32 | NESTED_U32 | MANY_U32;
pub(super) const NESTED_DELETE_MANY_ACTION_U32: u32 = DELETE_U32 | NESTED_U32 | MANY_U32;