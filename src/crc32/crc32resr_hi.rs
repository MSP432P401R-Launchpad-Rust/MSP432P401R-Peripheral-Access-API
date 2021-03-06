#[doc = "Reader of register CRC32RESR_HI"]
pub type R = crate::R<u16, super::CRC32RESR_HI>;
#[doc = "Writer for register CRC32RESR_HI"]
pub type W = crate::W<u16, super::CRC32RESR_HI>;
#[doc = "Register CRC32RESR_HI `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CRC32RESR_HI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CRC32RESR_HI`"]
pub type CRC32RESR_HI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC32RESR_HI`"]
pub struct CRC32RESR_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC32RESR_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC32 reverse result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32resr_hi(&self) -> CRC32RESR_HI_R {
        CRC32RESR_HI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 reverse result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32resr_hi(&mut self) -> CRC32RESR_HI_W {
        CRC32RESR_HI_W { w: self }
    }
}
