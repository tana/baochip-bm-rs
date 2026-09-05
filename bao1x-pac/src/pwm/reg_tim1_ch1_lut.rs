#[doc = "Register `REG_TIM1_CH1_LUT` reader"]
pub type R = crate::R<RegTim1Ch1LutSpec>;
#[doc = "Register `REG_TIM1_CH1_LUT` writer"]
pub type W = crate::W<RegTim1Ch1LutSpec>;
#[doc = "Field `r_timer1_ch1_lut` reader - r_timer1_ch1_lut"]
pub type RTimer1Ch1LutR = crate::FieldReader<u16>;
#[doc = "Field `r_timer1_ch1_lut` writer - r_timer1_ch1_lut"]
pub type RTimer1Ch1LutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_timer1_ch1_flt` reader - r_timer1_ch1_flt"]
pub type RTimer1Ch1FltR = crate::FieldReader;
#[doc = "Field `r_timer1_ch1_flt` writer - r_timer1_ch1_flt"]
pub type RTimer1Ch1FltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - r_timer1_ch1_lut"]
    #[inline(always)]
    pub fn r_timer1_ch1_lut(&self) -> RTimer1Ch1LutR {
        RTimer1Ch1LutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - r_timer1_ch1_flt"]
    #[inline(always)]
    pub fn r_timer1_ch1_flt(&self) -> RTimer1Ch1FltR {
        RTimer1Ch1FltR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_timer1_ch1_lut"]
    #[inline(always)]
    pub fn r_timer1_ch1_lut(&mut self) -> RTimer1Ch1LutW<'_, RegTim1Ch1LutSpec> {
        RTimer1Ch1LutW::new(self, 0)
    }
    #[doc = "Bits 16:17 - r_timer1_ch1_flt"]
    #[inline(always)]
    pub fn r_timer1_ch1_flt(&mut self) -> RTimer1Ch1FltW<'_, RegTim1Ch1LutSpec> {
        RTimer1Ch1FltW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch1_lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch1_lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim1Ch1LutSpec;
impl crate::RegisterSpec for RegTim1Ch1LutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim1_ch1_lut::R`](R) reader structure"]
impl crate::Readable for RegTim1Ch1LutSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim1_ch1_lut::W`](W) writer structure"]
impl crate::Writable for RegTim1Ch1LutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM1_CH1_LUT to value 0"]
impl crate::Resettable for RegTim1Ch1LutSpec {}
