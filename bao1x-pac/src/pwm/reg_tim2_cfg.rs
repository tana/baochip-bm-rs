#[doc = "Register `REG_TIM2_CFG` reader"]
pub type R = crate::R<RegTim2CfgSpec>;
#[doc = "Register `REG_TIM2_CFG` writer"]
pub type W = crate::W<RegTim2CfgSpec>;
#[doc = "Field `r_timer2_in_sel` reader - r_timer2_in_sel"]
pub type RTimer2InSelR = crate::FieldReader;
#[doc = "Field `r_timer2_in_sel` writer - r_timer2_in_sel"]
pub type RTimer2InSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_timer2_in_mode` reader - r_timer2_in_mode"]
pub type RTimer2InModeR = crate::FieldReader;
#[doc = "Field `r_timer2_in_mode` writer - r_timer2_in_mode"]
pub type RTimer2InModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_timer2_in_clk` reader - r_timer2_in_clk"]
pub type RTimer2InClkR = crate::BitReader;
#[doc = "Field `r_timer2_in_clk` writer - r_timer2_in_clk"]
pub type RTimer2InClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer2_saw` reader - r_timer2_saw"]
pub type RTimer2SawR = crate::BitReader;
#[doc = "Field `r_timer2_saw` writer - r_timer2_saw"]
pub type RTimer2SawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer2_presc` reader - r_timer2_presc"]
pub type RTimer2PrescR = crate::FieldReader;
#[doc = "Field `r_timer2_presc` writer - r_timer2_presc"]
pub type RTimer2PrescW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - r_timer2_in_sel"]
    #[inline(always)]
    pub fn r_timer2_in_sel(&self) -> RTimer2InSelR {
        RTimer2InSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - r_timer2_in_mode"]
    #[inline(always)]
    pub fn r_timer2_in_mode(&self) -> RTimer2InModeR {
        RTimer2InModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - r_timer2_in_clk"]
    #[inline(always)]
    pub fn r_timer2_in_clk(&self) -> RTimer2InClkR {
        RTimer2InClkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - r_timer2_saw"]
    #[inline(always)]
    pub fn r_timer2_saw(&self) -> RTimer2SawR {
        RTimer2SawR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - r_timer2_presc"]
    #[inline(always)]
    pub fn r_timer2_presc(&self) -> RTimer2PrescR {
        RTimer2PrescR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - r_timer2_in_sel"]
    #[inline(always)]
    pub fn r_timer2_in_sel(&mut self) -> RTimer2InSelW<'_, RegTim2CfgSpec> {
        RTimer2InSelW::new(self, 0)
    }
    #[doc = "Bits 8:10 - r_timer2_in_mode"]
    #[inline(always)]
    pub fn r_timer2_in_mode(&mut self) -> RTimer2InModeW<'_, RegTim2CfgSpec> {
        RTimer2InModeW::new(self, 8)
    }
    #[doc = "Bit 11 - r_timer2_in_clk"]
    #[inline(always)]
    pub fn r_timer2_in_clk(&mut self) -> RTimer2InClkW<'_, RegTim2CfgSpec> {
        RTimer2InClkW::new(self, 11)
    }
    #[doc = "Bit 12 - r_timer2_saw"]
    #[inline(always)]
    pub fn r_timer2_saw(&mut self) -> RTimer2SawW<'_, RegTim2CfgSpec> {
        RTimer2SawW::new(self, 12)
    }
    #[doc = "Bits 16:23 - r_timer2_presc"]
    #[inline(always)]
    pub fn r_timer2_presc(&mut self) -> RTimer2PrescW<'_, RegTim2CfgSpec> {
        RTimer2PrescW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim2CfgSpec;
impl crate::RegisterSpec for RegTim2CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim2_cfg::R`](R) reader structure"]
impl crate::Readable for RegTim2CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim2_cfg::W`](W) writer structure"]
impl crate::Writable for RegTim2CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM2_CFG to value 0"]
impl crate::Resettable for RegTim2CfgSpec {}
