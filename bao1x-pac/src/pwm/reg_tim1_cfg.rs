#[doc = "Register `REG_TIM1_CFG` reader"]
pub type R = crate::R<RegTim1CfgSpec>;
#[doc = "Register `REG_TIM1_CFG` writer"]
pub type W = crate::W<RegTim1CfgSpec>;
#[doc = "Field `r_timer1_in_sel` reader - r_timer1_in_sel"]
pub type RTimer1InSelR = crate::FieldReader;
#[doc = "Field `r_timer1_in_sel` writer - r_timer1_in_sel"]
pub type RTimer1InSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_timer1_in_mode` reader - r_timer1_in_mode"]
pub type RTimer1InModeR = crate::FieldReader;
#[doc = "Field `r_timer1_in_mode` writer - r_timer1_in_mode"]
pub type RTimer1InModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_timer1_in_clk` reader - r_timer1_in_clk"]
pub type RTimer1InClkR = crate::BitReader;
#[doc = "Field `r_timer1_in_clk` writer - r_timer1_in_clk"]
pub type RTimer1InClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer1_saw` reader - r_timer1_saw"]
pub type RTimer1SawR = crate::BitReader;
#[doc = "Field `r_timer1_saw` writer - r_timer1_saw"]
pub type RTimer1SawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer1_presc` reader - r_timer1_presc"]
pub type RTimer1PrescR = crate::FieldReader;
#[doc = "Field `r_timer1_presc` writer - r_timer1_presc"]
pub type RTimer1PrescW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - r_timer1_in_sel"]
    #[inline(always)]
    pub fn r_timer1_in_sel(&self) -> RTimer1InSelR {
        RTimer1InSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - r_timer1_in_mode"]
    #[inline(always)]
    pub fn r_timer1_in_mode(&self) -> RTimer1InModeR {
        RTimer1InModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - r_timer1_in_clk"]
    #[inline(always)]
    pub fn r_timer1_in_clk(&self) -> RTimer1InClkR {
        RTimer1InClkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - r_timer1_saw"]
    #[inline(always)]
    pub fn r_timer1_saw(&self) -> RTimer1SawR {
        RTimer1SawR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - r_timer1_presc"]
    #[inline(always)]
    pub fn r_timer1_presc(&self) -> RTimer1PrescR {
        RTimer1PrescR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - r_timer1_in_sel"]
    #[inline(always)]
    pub fn r_timer1_in_sel(&mut self) -> RTimer1InSelW<'_, RegTim1CfgSpec> {
        RTimer1InSelW::new(self, 0)
    }
    #[doc = "Bits 8:10 - r_timer1_in_mode"]
    #[inline(always)]
    pub fn r_timer1_in_mode(&mut self) -> RTimer1InModeW<'_, RegTim1CfgSpec> {
        RTimer1InModeW::new(self, 8)
    }
    #[doc = "Bit 11 - r_timer1_in_clk"]
    #[inline(always)]
    pub fn r_timer1_in_clk(&mut self) -> RTimer1InClkW<'_, RegTim1CfgSpec> {
        RTimer1InClkW::new(self, 11)
    }
    #[doc = "Bit 12 - r_timer1_saw"]
    #[inline(always)]
    pub fn r_timer1_saw(&mut self) -> RTimer1SawW<'_, RegTim1CfgSpec> {
        RTimer1SawW::new(self, 12)
    }
    #[doc = "Bits 16:23 - r_timer1_presc"]
    #[inline(always)]
    pub fn r_timer1_presc(&mut self) -> RTimer1PrescW<'_, RegTim1CfgSpec> {
        RTimer1PrescW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim1CfgSpec;
impl crate::RegisterSpec for RegTim1CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim1_cfg::R`](R) reader structure"]
impl crate::Readable for RegTim1CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim1_cfg::W`](W) writer structure"]
impl crate::Writable for RegTim1CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM1_CFG to value 0"]
impl crate::Resettable for RegTim1CfgSpec {}
