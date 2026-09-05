#[doc = "Register `SFR_CFG_SCHM_CR_CFG_SCHMSEL2` reader"]
pub type R = crate::R<SfrCfgSchmCrCfgSchmsel2Spec>;
#[doc = "Register `SFR_CFG_SCHM_CR_CFG_SCHMSEL2` writer"]
pub type W = crate::W<SfrCfgSchmCrCfgSchmsel2Spec>;
#[doc = "Field `cr_cfg_schmsel2` reader - cr_cfg_schmsel read/write control register"]
pub type CrCfgSchmsel2R = crate::FieldReader<u16>;
#[doc = "Field `cr_cfg_schmsel2` writer - cr_cfg_schmsel read/write control register"]
pub type CrCfgSchmsel2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_cfg_schmsel read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_schmsel2(&self) -> CrCfgSchmsel2R {
        CrCfgSchmsel2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_cfg_schmsel read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_schmsel2(&mut self) -> CrCfgSchmsel2W<'_, SfrCfgSchmCrCfgSchmsel2Spec> {
        CrCfgSchmsel2W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_schm_cr_cfg_schmsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_schm_cr_cfg_schmsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgSchmCrCfgSchmsel2Spec;
impl crate::RegisterSpec for SfrCfgSchmCrCfgSchmsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_schm_cr_cfg_schmsel2::R`](R) reader structure"]
impl crate::Readable for SfrCfgSchmCrCfgSchmsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_schm_cr_cfg_schmsel2::W`](W) writer structure"]
impl crate::Writable for SfrCfgSchmCrCfgSchmsel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_SCHM_CR_CFG_SCHMSEL2 to value 0"]
impl crate::Resettable for SfrCfgSchmCrCfgSchmsel2Spec {}
