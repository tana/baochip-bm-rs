#[doc = "Register `SFR_CFG1` reader"]
pub type R = crate::R<SfrCfg1Spec>;
#[doc = "Register `SFR_CFG1` writer"]
pub type W = crate::W<SfrCfg1Spec>;
#[doc = "Field `cfg_step` reader - cfg_step read/write control register"]
pub type CfgStepR = crate::FieldReader;
#[doc = "Field `cfg_step` writer - cfg_step read/write control register"]
pub type CfgStepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cfg_filter` reader - cfg_filter read/write control register"]
pub type CfgFilterR = crate::FieldReader;
#[doc = "Field `cfg_filter` writer - cfg_filter read/write control register"]
pub type CfgFilterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cfg_cnt1ms` reader - cfg_cnt1ms read/write control register"]
pub type CfgCnt1msR = crate::FieldReader;
#[doc = "Field `cfg_cnt1ms` writer - cfg_cnt1ms read/write control register"]
pub type CfgCnt1msW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_step read/write control register"]
    #[inline(always)]
    pub fn cfg_step(&self) -> CfgStepR {
        CfgStepR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - cfg_filter read/write control register"]
    #[inline(always)]
    pub fn cfg_filter(&self) -> CfgFilterR {
        CfgFilterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - cfg_cnt1ms read/write control register"]
    #[inline(always)]
    pub fn cfg_cnt1ms(&self) -> CfgCnt1msR {
        CfgCnt1msR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_step read/write control register"]
    #[inline(always)]
    pub fn cfg_step(&mut self) -> CfgStepW<'_, SfrCfg1Spec> {
        CfgStepW::new(self, 0)
    }
    #[doc = "Bits 8:15 - cfg_filter read/write control register"]
    #[inline(always)]
    pub fn cfg_filter(&mut self) -> CfgFilterW<'_, SfrCfg1Spec> {
        CfgFilterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - cfg_cnt1ms read/write control register"]
    #[inline(always)]
    pub fn cfg_cnt1ms(&mut self) -> CfgCnt1msW<'_, SfrCfg1Spec> {
        CfgCnt1msW::new(self, 16)
    }
}
#[doc = "See `dkpc.sv#L168 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L168>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfg1Spec;
impl crate::RegisterSpec for SfrCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg1::R`](R) reader structure"]
impl crate::Readable for SfrCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg1::W`](W) writer structure"]
impl crate::Writable for SfrCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG1 to value 0"]
impl crate::Resettable for SfrCfg1Spec {}
