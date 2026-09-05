#[doc = "Register `SFR_CFG_DRVSEL_CR_CFG_DRVSEL1` reader"]
pub type R = crate::R<SfrCfgDrvselCrCfgDrvsel1Spec>;
#[doc = "Register `SFR_CFG_DRVSEL_CR_CFG_DRVSEL1` writer"]
pub type W = crate::W<SfrCfgDrvselCrCfgDrvsel1Spec>;
#[doc = "Field `cr_cfg_drvsel1` reader - cr_cfg_drvsel read/write control register"]
pub type CrCfgDrvsel1R = crate::FieldReader<u32>;
#[doc = "Field `cr_cfg_drvsel1` writer - cr_cfg_drvsel read/write control register"]
pub type CrCfgDrvsel1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_cfg_drvsel read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_drvsel1(&self) -> CrCfgDrvsel1R {
        CrCfgDrvsel1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_cfg_drvsel read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_drvsel1(&mut self) -> CrCfgDrvsel1W<'_, SfrCfgDrvselCrCfgDrvsel1Spec> {
        CrCfgDrvsel1W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_drvsel_cr_cfg_drvsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_drvsel_cr_cfg_drvsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgDrvselCrCfgDrvsel1Spec;
impl crate::RegisterSpec for SfrCfgDrvselCrCfgDrvsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_drvsel_cr_cfg_drvsel1::R`](R) reader structure"]
impl crate::Readable for SfrCfgDrvselCrCfgDrvsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_drvsel_cr_cfg_drvsel1::W`](W) writer structure"]
impl crate::Writable for SfrCfgDrvselCrCfgDrvsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_DRVSEL_CR_CFG_DRVSEL1 to value 0"]
impl crate::Resettable for SfrCfgDrvselCrCfgDrvsel1Spec {}
