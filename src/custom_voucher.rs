use minerva_voucher::{Voucher, VoucherError, Sign, Validate, SignatureAlgorithm, attr::*};
use crate::utils;
use std::convert::TryFrom;

//

pub struct CustomVoucher(Voucher);

impl core::ops::Deref for CustomVoucher {
    type Target = Voucher;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl core::ops::DerefMut for CustomVoucher {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl TryFrom<&[u8]> for CustomVoucher {
    type Error = VoucherError;
    fn try_from(raw: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(Voucher::try_from(raw)?))
    }
}

impl CustomVoucher {
    pub fn new_vrq() -> Self { Self(Voucher::new_vrq()) }
    pub fn set(&mut self, attr: Attr) -> &mut Self {
        self.0.set(attr);
        self
    }
}

impl Sign for CustomVoucher {
    fn sign(&mut self, privkey_pem: &[u8], alg: SignatureAlgorithm) -> Result<&mut Self, VoucherError> {
        if let Err(_) = backend::sign(privkey_pem, alg, self.to_sign(alg)) {
            Err(VoucherError::SigningFailed)
        } else {
            Ok(self)
        }
    }
}

impl Validate for CustomVoucher {
    fn validate(&self, pem: Option<&[u8]>) -> Result<&Self, VoucherError> {
        match backend::validate(pem, self.to_validate()) {
            Ok(true) => Ok(self),
            Ok(false) => Err(VoucherError::ValidationFailed),
            Err(_) => Err(VoucherError::ValidationFailed),
        }
    }
}

//

#[cfg(feature = "minerva-mbedtls")]
use minerva_mbedtls_backend as backend;

#[cfg(feature = "minerva-mbedtls")]
mod minerva_mbedtls_backend {
    use super::*;
    pub use minerva_voucher::sign::sign_with_mbedtls as sign;
    pub use minerva_voucher::validate::validate_with_mbedtls as validate;
}

