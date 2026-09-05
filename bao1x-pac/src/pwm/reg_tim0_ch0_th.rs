#[doc = "Register `REG_TIM0_CH0_TH` reader"]
pub type R = crate::R<RegTim0Ch0ThSpec>;
#[doc = "Register `REG_TIM0_CH0_TH` writer"]
pub type W = crate::W<RegTim0Ch0ThSpec>;
#[doc = "Field `r_timer0_ch0_th` reader - r_timer0_ch0_th"]
pub type RTimer0Ch0ThR = crate::FieldReader<u16>;
#[doc = "Field `r_timer0_ch0_th` writer - r_timer0_ch0_th"]
pub type RTimer0Ch0ThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_timer0_ch0_mode` reader - r_timer0_ch0_mode"]
pub type RTimer0Ch0ModeR = crate::FieldReader;
#[doc = "Field `r_timer0_ch0_mode` writer - r_timer0_ch0_mode"]
pub type RTimer0Ch0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - r_timer0_ch0_th"]
    #[inline(always)]
    pub fn r_timer0_ch0_th(&self) -> RTimer0Ch0ThR {
        RTimer0Ch0ThR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - r_timer0_ch0_mode"]
    #[inline(always)]
    pub fn r_timer0_ch0_mode(&self) -> RTimer0Ch0ModeR {
        RTimer0Ch0ModeR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_timer0_ch0_th"]
    #[inline(always)]
    pub fn r_timer0_ch0_th(&mut self) -> RTimer0Ch0ThW<'_, RegTim0Ch0ThSpec> {
        RTimer0Ch0ThW::new(self, 0)
    }
    #[doc = "Bits 16:18 - r_timer0_ch0_mode"]
    #[inline(always)]
    pub fn r_timer0_ch0_mode(&mut self) -> RTimer0Ch0ModeW<'_, RegTim0Ch0ThSpec> {
        RTimer0Ch0ModeW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch0_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch0_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim0Ch0ThSpec;
impl crate::RegisterSpec for RegTim0Ch0ThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim0_ch0_th::R`](R) reader structure"]
impl crate::Readable for RegTim0Ch0ThSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim0_ch0_th::W`](W) writer structure"]
impl crate::Writable for RegTim0Ch0ThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM0_CH0_TH to value 0"]
impl crate::Resettable for RegTim0Ch0ThSpec {}
