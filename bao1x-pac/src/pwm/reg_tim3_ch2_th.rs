#[doc = "Register `REG_TIM3_CH2_TH` reader"]
pub type R = crate::R<RegTim3Ch2ThSpec>;
#[doc = "Register `REG_TIM3_CH2_TH` writer"]
pub type W = crate::W<RegTim3Ch2ThSpec>;
#[doc = "Field `r_timer3_ch2_th` reader - r_timer3_ch2_th"]
pub type RTimer3Ch2ThR = crate::FieldReader<u16>;
#[doc = "Field `r_timer3_ch2_th` writer - r_timer3_ch2_th"]
pub type RTimer3Ch2ThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_timer3_ch2_mode` reader - r_timer3_ch2_mode"]
pub type RTimer3Ch2ModeR = crate::FieldReader;
#[doc = "Field `r_timer3_ch2_mode` writer - r_timer3_ch2_mode"]
pub type RTimer3Ch2ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - r_timer3_ch2_th"]
    #[inline(always)]
    pub fn r_timer3_ch2_th(&self) -> RTimer3Ch2ThR {
        RTimer3Ch2ThR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - r_timer3_ch2_mode"]
    #[inline(always)]
    pub fn r_timer3_ch2_mode(&self) -> RTimer3Ch2ModeR {
        RTimer3Ch2ModeR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_timer3_ch2_th"]
    #[inline(always)]
    pub fn r_timer3_ch2_th(&mut self) -> RTimer3Ch2ThW<'_, RegTim3Ch2ThSpec> {
        RTimer3Ch2ThW::new(self, 0)
    }
    #[doc = "Bits 16:18 - r_timer3_ch2_mode"]
    #[inline(always)]
    pub fn r_timer3_ch2_mode(&mut self) -> RTimer3Ch2ModeW<'_, RegTim3Ch2ThSpec> {
        RTimer3Ch2ModeW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch2_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch2_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim3Ch2ThSpec;
impl crate::RegisterSpec for RegTim3Ch2ThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim3_ch2_th::R`](R) reader structure"]
impl crate::Readable for RegTim3Ch2ThSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim3_ch2_th::W`](W) writer structure"]
impl crate::Writable for RegTim3Ch2ThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM3_CH2_TH to value 0"]
impl crate::Resettable for RegTim3Ch2ThSpec {}
