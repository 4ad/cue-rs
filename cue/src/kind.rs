#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Bottom,
    Null,
    Bool,
    Int,
    Float,
    String,
    Bytes,
    Struct,
    List,
    Number,
    Top,
}

impl Kind {
    pub(crate) unsafe fn from(kind: i32) -> Kind {
        match kind {
            cue_sys::cue_kind_CUE_KIND_BOTTOM => Kind::Bottom,
            cue_sys::cue_kind_CUE_KIND_NULL => Kind::Null,
            cue_sys::cue_kind_CUE_KIND_BOOL => Kind::Bool,
            cue_sys::cue_kind_CUE_KIND_INT => Kind::Int,
            cue_sys::cue_kind_CUE_KIND_FLOAT => Kind::Float,
            cue_sys::cue_kind_CUE_KIND_STRING => Kind::String,
            cue_sys::cue_kind_CUE_KIND_BYTES => Kind::Bytes,
            cue_sys::cue_kind_CUE_KIND_STRUCT => Kind::Struct,
            cue_sys::cue_kind_CUE_KIND_LIST => Kind::List,
            cue_sys::cue_kind_CUE_KIND_NUMBER => Kind::Number,
            cue_sys::cue_kind_CUE_KIND_TOP => Kind::Top,
            _ => panic!("invalid kind"),
        }
    }
}
