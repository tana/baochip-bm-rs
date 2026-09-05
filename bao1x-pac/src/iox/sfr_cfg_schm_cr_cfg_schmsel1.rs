#[doc = "Register `SFR_CFG_SCHM_CR_CFG_SCHMSEL1` reader"]
pub type R = crate::R<SfrCfgSchmCrCfgSchmsel1Spec>;
#[doc = "Register `SFR_CFG_SCHM_CR_CFG_SCHMSEL1` writer"]
pub type W = crate::W<SfrCfgSchmCrCfgSchmsel1Spec>;
#[doc = "Field `cr_cfg_schmsel1` reader - cr_cfg_schmsel read/write control register"]
pub type CrCfgSchmsel1R = crate::FieldReader<u16>;
#[doc = "Field `cr_cfg_schmsel1` writer - cr_cfg_schmsel read/write control register"]
pub type CrCfgSchmsel1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_cfg_schmsel read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_schmsel1(&self) -> CrCfgSchmsel1R {
        CrCfgSchmsel1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_cfg_schmsel read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_schmsel1(&mut self) -> CrCfgSchmsel1W<'_, SfrCfgSchmCrCfgSchmsel1Spec> {
        CrCfgSchmsel1W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_schm_cr_cfg_schmsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_schm_cr_cfg_schmsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgSchmCrCfgSchmsel1Spec;
impl crate::RegisterSpec for SfrCfgSchmCrCfgSchmsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_schm_cr_cfg_schmsel1::R`](R) reader structure"]
impl crate::Readable for SfrCfgSchmCrCfgSchmsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_schm_cr_cfg_schmsel1::W`](W) writer structure"]
impl crate::Writable for SfrCfgSchmCrCfgSchmsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_SCHM_CR_CFG_SCHMSEL1 to value 0"]
impl crate::Resettable for SfrCfgSchmCrCfgSchmsel1Spec {}
