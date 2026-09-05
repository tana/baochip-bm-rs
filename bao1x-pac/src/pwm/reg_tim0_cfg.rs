#[doc = "Register `REG_TIM0_CFG` reader"]
pub type R = crate::R<RegTim0CfgSpec>;
#[doc = "Register `REG_TIM0_CFG` writer"]
pub type W = crate::W<RegTim0CfgSpec>;
#[doc = "Field `r_timer0_in_sel` reader - r_timer0_in_sel"]
pub type RTimer0InSelR = crate::FieldReader;
#[doc = "Field `r_timer0_in_sel` writer - r_timer0_in_sel"]
pub type RTimer0InSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_timer0_in_mode` reader - r_timer0_in_mode"]
pub type RTimer0InModeR = crate::FieldReader;
#[doc = "Field `r_timer0_in_mode` writer - r_timer0_in_mode"]
pub type RTimer0InModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_timer0_in_clk` reader - r_timer0_in_clk"]
pub type RTimer0InClkR = crate::BitReader;
#[doc = "Field `r_timer0_in_clk` writer - r_timer0_in_clk"]
pub type RTimer0InClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer0_saw` reader - r_timer0_saw"]
pub type RTimer0SawR = crate::BitReader;
#[doc = "Field `r_timer0_saw` writer - r_timer0_saw"]
pub type RTimer0SawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer0_presc` reader - r_timer0_presc"]
pub type RTimer0PrescR = crate::FieldReader;
#[doc = "Field `r_timer0_presc` writer - r_timer0_presc"]
pub type RTimer0PrescW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - r_timer0_in_sel"]
    #[inline(always)]
    pub fn r_timer0_in_sel(&self) -> RTimer0InSelR {
        RTimer0InSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - r_timer0_in_mode"]
    #[inline(always)]
    pub fn r_timer0_in_mode(&self) -> RTimer0InModeR {
        RTimer0InModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - r_timer0_in_clk"]
    #[inline(always)]
    pub fn r_timer0_in_clk(&self) -> RTimer0InClkR {
        RTimer0InClkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - r_timer0_saw"]
    #[inline(always)]
    pub fn r_timer0_saw(&self) -> RTimer0SawR {
        RTimer0SawR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - r_timer0_presc"]
    #[inline(always)]
    pub fn r_timer0_presc(&self) -> RTimer0PrescR {
        RTimer0PrescR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - r_timer0_in_sel"]
    #[inline(always)]
    pub fn r_timer0_in_sel(&mut self) -> RTimer0InSelW<'_, RegTim0CfgSpec> {
        RTimer0InSelW::new(self, 0)
    }
    #[doc = "Bits 8:10 - r_timer0_in_mode"]
    #[inline(always)]
    pub fn r_timer0_in_mode(&mut self) -> RTimer0InModeW<'_, RegTim0CfgSpec> {
        RTimer0InModeW::new(self, 8)
    }
    #[doc = "Bit 11 - r_timer0_in_clk"]
    #[inline(always)]
    pub fn r_timer0_in_clk(&mut self) -> RTimer0InClkW<'_, RegTim0CfgSpec> {
        RTimer0InClkW::new(self, 11)
    }
    #[doc = "Bit 12 - r_timer0_saw"]
    #[inline(always)]
    pub fn r_timer0_saw(&mut self) -> RTimer0SawW<'_, RegTim0CfgSpec> {
        RTimer0SawW::new(self, 12)
    }
    #[doc = "Bits 16:23 - r_timer0_presc"]
    #[inline(always)]
    pub fn r_timer0_presc(&mut self) -> RTimer0PrescW<'_, RegTim0CfgSpec> {
        RTimer0PrescW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim0CfgSpec;
impl crate::RegisterSpec for RegTim0CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim0_cfg::R`](R) reader structure"]
impl crate::Readable for RegTim0CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim0_cfg::W`](W) writer structure"]
impl crate::Writable for RegTim0CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM0_CFG to value 0"]
impl crate::Resettable for RegTim0CfgSpec {}
