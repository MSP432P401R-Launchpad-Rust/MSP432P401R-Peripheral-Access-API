#[doc = "Reader of register DMA_PRIOSET"]
pub type R = crate::R<u32, super::DMA_PRIOSET>;
#[doc = "Writer for register DMA_PRIOSET"]
pub type W = crate::W<u32, super::DMA_PRIOSET>;
#[doc = "Register DMA_PRIOSET `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_PRIOSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Returns the channel priority mask status, or sets the channel priority to high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_A {
    #[doc = "0: DMA channel C is using the default priority level."]
    SET_0_READ = 0,
    #[doc = "1: DMA channel C is using a high priority level."]
    SET_1_READ = 1,
}
impl From<SET_A> for u32 {
    #[inline(always)]
    fn from(variant: SET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SET`"]
pub type SET_R = crate::R<u32, SET_A>;
impl SET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SET_A::SET_0_READ),
            1 => Val(SET_A::SET_1_READ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET_0_READ`"]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        *self == SET_A::SET_0_READ
    }
    #[doc = "Checks if the value of the field is `SET_1_READ`"]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        *self == SET_A::SET_1_READ
    }
}
#[doc = "Returns the channel priority mask status, or sets the channel priority to high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_AW {
    #[doc = "0: No effect.\r\r\nUse the DMA_PRIOCLR Register to set channel C to the default priority level."]
    SET_0_WRITE = 0,
    #[doc = "1: Channel C uses the high priority level.\r\r\nWriting to a bit where a DMA channel is not implemented has no effect."]
    SET_1_WRITE = 1,
}
impl From<SET_AW> for u32 {
    #[inline(always)]
    fn from(variant: SET_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `SET`"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect. Use the DMA_PRIOCLR Register to set channel C to the default priority level."]
    #[inline(always)]
    pub fn set_0_write(self) -> &'a mut W {
        self.variant(SET_AW::SET_0_WRITE)
    }
    #[doc = "Channel C uses the high priority level. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn set_1_write(self) -> &'a mut W {
        self.variant(SET_AW::SET_1_WRITE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Returns the channel priority mask status, or sets the channel priority to high."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Returns the channel priority mask status, or sets the channel priority to high."]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
}
