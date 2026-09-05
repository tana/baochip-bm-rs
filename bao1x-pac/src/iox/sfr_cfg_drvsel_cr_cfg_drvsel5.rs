#[doc = "Register `SFR_CFG_DRVSEL_CR_CFG_DRVSEL5` reader"]
pub type R = crate::R<SfrCfgDrvselCrCfgDrvsel5Spec>;
#[doc = "Register `SFR_CFG_DRVSEL_CR_CFG_DRVSEL5` writer"]
pub type W = crate::W<SfrCfgDrvselCrCfgDrvsel5Spec>;
#[doc = "Field `cr_cfg_drvsel5` reader - cr_cfg_drvsel read/write control register"]
pub type CrCfgDrvsel5R = crate::FieldReader<u32>;
#[doc = "Field `cr_cfg_drvsel5` writer - cr_cfg_drvsel read/write control register"]
pub type CrCfgDrvsel5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_cfg_drvsel read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_drvsel5(&self) -> CrCfgDrvsel5R {
        CrCfgDrvsel5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_cfg_drvsel read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_drvsel5(&mut self) -> CrCfgDrvsel5W<'_, SfrCfgDrvselCrCfgDrvsel5Spec> {
        CrCfgDrvsel5W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_drvsel_cr_cfg_drvsel5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_drvsel_cr_cfg_drvsel5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgDrvselCrCfgDrvsel5Spec;
impl crate::RegisterSpec for SfrCfgDrvselCrCfgDrvsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_drvsel_cr_cfg_drvsel5::R`](R) reader structure"]
impl crate::Readable for SfrCfgDrvselCrCfgDrvsel5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_drvsel_cr_cfg_drvsel5::W`](W) writer structure"]
impl crate::Writable for SfrCfgDrvselCrCfgDrvsel5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_DRVSEL_CR_CFG_DRVSEL5 to value 0"]
impl crate::Resettable for SfrCfgDrvselCrCfgDrvsel5Spec {}
