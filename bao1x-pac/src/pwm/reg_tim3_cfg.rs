#[doc = "Register `REG_TIM3_CFG` reader"]
pub type R = crate::R<RegTim3CfgSpec>;
#[doc = "Register `REG_TIM3_CFG` writer"]
pub type W = crate::W<RegTim3CfgSpec>;
#[doc = "Field `r_timer3_in_sel` reader - r_timer3_in_sel"]
pub type RTimer3InSelR = crate::FieldReader;
#[doc = "Field `r_timer3_in_sel` writer - r_timer3_in_sel"]
pub type RTimer3InSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_timer3_in_mode` reader - r_timer3_in_mode"]
pub type RTimer3InModeR = crate::FieldReader;
#[doc = "Field `r_timer3_in_mode` writer - r_timer3_in_mode"]
pub type RTimer3InModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_timer3_in_clk` reader - r_timer3_in_clk"]
pub type RTimer3InClkR = crate::BitReader;
#[doc = "Field `r_timer3_in_clk` writer - r_timer3_in_clk"]
pub type RTimer3InClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer3_saw` reader - r_timer3_saw"]
pub type RTimer3SawR = crate::BitReader;
#[doc = "Field `r_timer3_saw` writer - r_timer3_saw"]
pub type RTimer3SawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer3_presc` reader - r_timer3_presc"]
pub type RTimer3PrescR = crate::FieldReader;
#[doc = "Field `r_timer3_presc` writer - r_timer3_presc"]
pub type RTimer3PrescW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - r_timer3_in_sel"]
    #[inline(always)]
    pub fn r_timer3_in_sel(&self) -> RTimer3InSelR {
        RTimer3InSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - r_timer3_in_mode"]
    #[inline(always)]
    pub fn r_timer3_in_mode(&self) -> RTimer3InModeR {
        RTimer3InModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - r_timer3_in_clk"]
    #[inline(always)]
    pub fn r_timer3_in_clk(&self) -> RTimer3InClkR {
        RTimer3InClkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - r_timer3_saw"]
    #[inline(always)]
    pub fn r_timer3_saw(&self) -> RTimer3SawR {
        RTimer3SawR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - r_timer3_presc"]
    #[inline(always)]
    pub fn r_timer3_presc(&self) -> RTimer3PrescR {
        RTimer3PrescR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - r_timer3_in_sel"]
    #[inline(always)]
    pub fn r_timer3_in_sel(&mut self) -> RTimer3InSelW<'_, RegTim3CfgSpec> {
        RTimer3InSelW::new(self, 0)
    }
    #[doc = "Bits 8:10 - r_timer3_in_mode"]
    #[inline(always)]
    pub fn r_timer3_in_mode(&mut self) -> RTimer3InModeW<'_, RegTim3CfgSpec> {
        RTimer3InModeW::new(self, 8)
    }
    #[doc = "Bit 11 - r_timer3_in_clk"]
    #[inline(always)]
    pub fn r_timer3_in_clk(&mut self) -> RTimer3InClkW<'_, RegTim3CfgSpec> {
        RTimer3InClkW::new(self, 11)
    }
    #[doc = "Bit 12 - r_timer3_saw"]
    #[inline(always)]
    pub fn r_timer3_saw(&mut self) -> RTimer3SawW<'_, RegTim3CfgSpec> {
        RTimer3SawW::new(self, 12)
    }
    #[doc = "Bits 16:23 - r_timer3_presc"]
    #[inline(always)]
    pub fn r_timer3_presc(&mut self) -> RTimer3PrescW<'_, RegTim3CfgSpec> {
        RTimer3PrescW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim3CfgSpec;
impl crate::RegisterSpec for RegTim3CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim3_cfg::R`](R) reader structure"]
impl crate::Readable for RegTim3CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim3_cfg::W`](W) writer structure"]
impl crate::Writable for RegTim3CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM3_CFG to value 0"]
impl crate::Resettable for RegTim3CfgSpec {}
