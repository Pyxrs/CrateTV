use std::{collections::HashSet, ops::Deref, str::FromStr};

use bitcode::{Decode, Encode};
use once_cell::sync::Lazy;
use pstd::collections::insecure::HashMap;

#[rustfmt::skip]
const PERMISSIONS: &[&str] = &[
    /*0*/ "Profile::ViewAny",
    /*1*/ "Profile::EditOwn",
    /*2*/ "Profile::EditAny",

    /*3*/ "Account::Follow",
    /*4*/ "Account::Block",
    /*5*/ "Account::EditOwn",
    /*6*/ "Account::EditAny",

    /*7*/ "StreamKey::ViewOwn",
    /*8*/ "StreamKey::RegenerateOwn",
    /*9*/ "StreamKey::ViewAny",
    /*10*/ "StreamKey::RegenerateAny",
    
    /*11*/ "Stream::ViewAny",
    /*12*/ "Stream::StartOwn",
    /*13*/ "Stream::StopOwn",
    /*14*/ "Stream::EditOwn",
    /*15*/ "Stream::DeleteOwn",
    /*16*/ "Stream::StopAny",
    /*17*/ "Stream::EditAny",
    /*18*/ "Stream::DeleteAny",
];

pub static GUEST_PERMISSIONS: Lazy<HashSet<Permission>> = Lazy::new(|| {
    ["Profile::ViewAny", "Stream::ViewAny"]
        .iter()
        .filter_map(|name| PERMISSION_ID_LOOKUP.get(name).map(|id| Permission(*id)))
        .collect()
});

pub static USER_PERMISSIONS: Lazy<HashSet<Permission>> = Lazy::new(|| {
    [
        "Profile::EditOwn",
        "Account::Follow",
        "Account::Block",
        "Account::EditOwn",
        "StreamKey::ViewOwn",
        "StreamKey::RegenerateOwn",
        "Stream::StartOwn",
        "Stream::StopOwn",
        "Stream::EditOwn",
        "Stream::DeleteOwn",
    ]
    .iter()
    .filter_map(|name| PERMISSION_ID_LOOKUP.get(name).map(|id| Permission(*id)))
    .chain(GUEST_PERMISSIONS.iter().copied())
    .collect()
});

pub static ADMIN_PERMISSIONS: Lazy<HashSet<Permission>> = Lazy::new(|| {
    [
        "Profile::EditAny",
        "Account::EditAny",
        "StreamKey::ViewAny",
        "StreamKey::RegenerateAny",
        "Stream::StopAny",
        "Stream::EditAny",
        "Stream::DeleteAny",
    ]
    .iter()
    .filter_map(|name| PERMISSION_ID_LOOKUP.get(name).map(|id| Permission(*id)))
    .chain(USER_PERMISSIONS.iter().copied())
    .collect()
});

static PERMISSION_ID_LOOKUP: Lazy<HashMap<&'static str, u32>> = Lazy::new(|| {
    PERMISSIONS
        .iter()
        .enumerate()
        .map(|(i, &name)| (name, i as u32))
        .collect()
});

#[derive(Encode, Decode, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Permission(u32);

impl Permission {
    pub fn get_name(&self) -> &'static str {
        PERMISSIONS[self.0 as usize]
    }
}

impl FromStr for Permission {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PERMISSION_ID_LOOKUP.get(s).copied().map(Self).ok_or(())
    }
}

impl Deref for Permission {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
