//! Utilties for discontinuity related QEMU plugin API

/// Type of a PC dicontinuity
pub type DisconType = crate::sys::qemu_plugin_discon_type;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
/// Mask for selecting PC discontinuities
pub struct DisconMask(pub(crate) i32);

impl DisconMask {
    /// Mask selecting all discontinuity events
    pub const ALL: Self = Self(DisconType::QEMU_PLUGIN_DISCON_ALL as i32);
}

impl std::ops::BitAnd for DisconMask {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl std::ops::BitAndAssign for DisconMask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl std::ops::BitOr for DisconMask {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl std::ops::BitOrAssign for DisconMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl std::ops::BitAnd<DisconType> for DisconMask {
    type Output = Self;

    fn bitand(mut self, rhs: DisconType) -> Self::Output {
        self &= rhs;
        self
    }
}

impl std::ops::BitAndAssign<DisconType> for DisconMask {
    fn bitand_assign(&mut self, rhs: DisconType) {
        self.0 &= rhs as i32
    }
}

impl std::ops::BitOr<DisconType> for DisconMask {
    type Output = Self;

    fn bitor(mut self, rhs: DisconType) -> Self::Output {
        self |= rhs;
        self
    }
}

impl std::ops::BitOrAssign<DisconType> for DisconMask {
    fn bitor_assign(&mut self, rhs: DisconType) {
        self.0 |= rhs as i32
    }
}
