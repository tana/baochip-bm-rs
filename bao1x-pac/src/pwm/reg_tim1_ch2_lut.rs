#[doc = "Register `REG_TIM1_CH2_LUT` reader"]
pub type R = crate::R<RegTim1Ch2LutSpec>;
#[doc = "Register `REG_TIM1_CH2_LUT` writer"]
pub type W = crate::W<RegTim1Ch2LutSpec>;
#[doc = "Field `r_timer1_ch2_lut` reader - r_timer1_ch2_lut"]
pub type RTimer1Ch2LutR = crate::FieldReader<u16>;
#[doc = "Field `r_timer1_ch2_lut` writer - r_timer1_ch2_lut"]
pub type RTimer1Ch2LutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_timer1_ch2_flt` reader - r_timer1_ch2_flt"]
pub type RTimer1Ch2FltR = crate::FieldReader;
#[doc = "Field `r_timer1_ch2_flt` writer - r_timer1_ch2_flt"]
pub type RTimer1Ch2FltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - r_timer1_ch2_lut"]
    #[inline(always)]
    pub fn r_timer1_ch2_lut(&self) -> RTimer1Ch2LutR {
        RTimer1Ch2LutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - r_timer1_ch2_flt"]
    #[inline(always)]
    pub fn r_timer1_ch2_flt(&self) -> RTimer1Ch2FltR {
        RTimer1Ch2FltR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_timer1_ch2_lut"]
    #[inline(always)]
    pub fn r_timer1_ch2_lut(&mut self) -> RTimer1Ch2LutW<'_, RegTim1Ch2LutSpec> {
        RTimer1Ch2LutW::new(self, 0)
    }
    #[doc = "Bits 16:17 - r_timer1_ch2_flt"]
    #[inline(always)]
    pub fn r_timer1_ch2_flt(&mut self) -> RTimer1Ch2FltW<'_, RegTim1Ch2LutSpec> {
        RTimer1Ch2FltW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch2_lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch2_lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim1Ch2LutSpec;
impl crate::RegisterSpec for RegTim1Ch2LutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim1_ch2_lut::R`](R) reader structure"]
impl crate::Readable for RegTim1Ch2LutSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim1_ch2_lut::W`](W) writer structure"]
impl crate::Writable for RegTim1Ch2LutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM1_CH2_LUT to value 0"]
impl crate::Resettable for RegTim1Ch2LutSpec {}
