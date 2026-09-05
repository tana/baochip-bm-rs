#[doc = "Register `REG_TIM2_CH0_LUT` reader"]
pub type R = crate::R<RegTim2Ch0LutSpec>;
#[doc = "Register `REG_TIM2_CH0_LUT` writer"]
pub type W = crate::W<RegTim2Ch0LutSpec>;
#[doc = "Field `r_timer2_ch0_lut` reader - r_timer2_ch0_lut"]
pub type RTimer2Ch0LutR = crate::FieldReader<u16>;
#[doc = "Field `r_timer2_ch0_lut` writer - r_timer2_ch0_lut"]
pub type RTimer2Ch0LutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_timer2_ch0_flt` reader - r_timer2_ch0_flt"]
pub type RTimer2Ch0FltR = crate::FieldReader;
#[doc = "Field `r_timer2_ch0_flt` writer - r_timer2_ch0_flt"]
pub type RTimer2Ch0FltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - r_timer2_ch0_lut"]
    #[inline(always)]
    pub fn r_timer2_ch0_lut(&self) -> RTimer2Ch0LutR {
        RTimer2Ch0LutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - r_timer2_ch0_flt"]
    #[inline(always)]
    pub fn r_timer2_ch0_flt(&self) -> RTimer2Ch0FltR {
        RTimer2Ch0FltR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_timer2_ch0_lut"]
    #[inline(always)]
    pub fn r_timer2_ch0_lut(&mut self) -> RTimer2Ch0LutW<'_, RegTim2Ch0LutSpec> {
        RTimer2Ch0LutW::new(self, 0)
    }
    #[doc = "Bits 16:17 - r_timer2_ch0_flt"]
    #[inline(always)]
    pub fn r_timer2_ch0_flt(&mut self) -> RTimer2Ch0FltW<'_, RegTim2Ch0LutSpec> {
        RTimer2Ch0FltW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch0_lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch0_lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim2Ch0LutSpec;
impl crate::RegisterSpec for RegTim2Ch0LutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim2_ch0_lut::R`](R) reader structure"]
impl crate::Readable for RegTim2Ch0LutSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim2_ch0_lut::W`](W) writer structure"]
impl crate::Writable for RegTim2Ch0LutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM2_CH0_LUT to value 0"]
impl crate::Resettable for RegTim2Ch0LutSpec {}
