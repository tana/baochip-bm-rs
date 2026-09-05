#[doc = "Register `REG_TIM0_CH3_LUT` reader"]
pub type R = crate::R<RegTim0Ch3LutSpec>;
#[doc = "Register `REG_TIM0_CH3_LUT` writer"]
pub type W = crate::W<RegTim0Ch3LutSpec>;
#[doc = "Field `r_timer0_ch3_lut` reader - r_timer0_ch3_lut"]
pub type RTimer0Ch3LutR = crate::FieldReader<u16>;
#[doc = "Field `r_timer0_ch3_lut` writer - r_timer0_ch3_lut"]
pub type RTimer0Ch3LutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_timer0_ch3_flt` reader - r_timer0_ch3_flt"]
pub type RTimer0Ch3FltR = crate::FieldReader;
#[doc = "Field `r_timer0_ch3_flt` writer - r_timer0_ch3_flt"]
pub type RTimer0Ch3FltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - r_timer0_ch3_lut"]
    #[inline(always)]
    pub fn r_timer0_ch3_lut(&self) -> RTimer0Ch3LutR {
        RTimer0Ch3LutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - r_timer0_ch3_flt"]
    #[inline(always)]
    pub fn r_timer0_ch3_flt(&self) -> RTimer0Ch3FltR {
        RTimer0Ch3FltR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_timer0_ch3_lut"]
    #[inline(always)]
    pub fn r_timer0_ch3_lut(&mut self) -> RTimer0Ch3LutW<'_, RegTim0Ch3LutSpec> {
        RTimer0Ch3LutW::new(self, 0)
    }
    #[doc = "Bits 16:17 - r_timer0_ch3_flt"]
    #[inline(always)]
    pub fn r_timer0_ch3_flt(&mut self) -> RTimer0Ch3FltW<'_, RegTim0Ch3LutSpec> {
        RTimer0Ch3FltW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch3_lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch3_lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim0Ch3LutSpec;
impl crate::RegisterSpec for RegTim0Ch3LutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim0_ch3_lut::R`](R) reader structure"]
impl crate::Readable for RegTim0Ch3LutSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim0_ch3_lut::W`](W) writer structure"]
impl crate::Writable for RegTim0Ch3LutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM0_CH3_LUT to value 0"]
impl crate::Resettable for RegTim0Ch3LutSpec {}
