#[doc = "Reader of register ADC14HI1"]
pub type R = crate::R<u32, super::ADC14HI1>;
#[doc = "Writer for register ADC14HI1"]
pub type W = crate::W<u32, super::ADC14HI1>;
#[doc = "Register ADC14HI1 `reset()`'s with value 0x3fff"]
impl crate::ResetValue for super::ADC14HI1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3fff
    }
}
#[doc = "Reader of field `ADC14HI1`"]
pub type ADC14HI1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADC14HI1`"]
pub struct ADC14HI1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14HI1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - High threshold 1"]
    #[inline(always)]
    pub fn adc14hi1(&self) -> ADC14HI1_R {
        ADC14HI1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High threshold 1"]
    #[inline(always)]
    pub fn adc14hi1(&mut self) -> ADC14HI1_W {
        ADC14HI1_W { w: self }
    }
}
