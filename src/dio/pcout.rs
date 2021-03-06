#[doc = "Reader of register PCOUT"]
pub type R = crate::R<u16, super::PCOUT>;
#[doc = "Writer for register PCOUT"]
pub type W = crate::W<u16, super::PCOUT>;
#[doc = "Register PCOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::PCOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5OUT`"]
pub type P5OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P5OUT`"]
pub struct P5OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `P6OUT`"]
pub type P6OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `P6OUT`"]
pub struct P6OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Output"]
    #[inline(always)]
    pub fn p5out(&self) -> P5OUT_R {
        P5OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Output"]
    #[inline(always)]
    pub fn p6out(&self) -> P6OUT_R {
        P6OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Output"]
    #[inline(always)]
    pub fn p5out(&mut self) -> P5OUT_W {
        P5OUT_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Output"]
    #[inline(always)]
    pub fn p6out(&mut self) -> P6OUT_W {
        P6OUT_W { w: self }
    }
}
