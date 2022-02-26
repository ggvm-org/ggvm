#[derive(Debug, PartialEq)]
pub(crate) enum JmpTarget {
    Addr(i64),
    Label(String),
}

macro_rules! impl_from_jmp_target {
    ($from_ty:ty => Label) => {
        impl From<$from_ty> for JmpTarget {
            fn from(v: $from_ty) -> Self {
                Self::Label(v.to_string())
            }
        }
    };
    ($from_ty:ty => Addr) => {
        impl From<$from_ty> for JmpTarget {
            fn from(v: $from_ty) -> Self {
                Self::Addr(i64::from(v))
            }
        }
    };
}

macro_rules! impl_from_jmp_target_label {
    ($($from_ty:ty),+) => {
        $(impl_from_jmp_target!($from_ty => Label);)+
    };
}

macro_rules! impl_from_jmp_target_addr {
    ($($from_ty:ty),+) => {
        $(impl_from_jmp_target!($from_ty => Addr);)+
    };
}

impl_from_jmp_target_label!(&str, String);
impl_from_jmp_target_addr!(u8, u16, u32, i8, i16, i32, i64);
