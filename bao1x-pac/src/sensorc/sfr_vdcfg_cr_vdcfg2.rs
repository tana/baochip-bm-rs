#[doc = "Register `SFR_VDCFG_CR_VDCFG2` reader"]
pub type R = crate::R<SfrVdcfgCrVdcfg2Spec>;
#[doc = "Register `SFR_VDCFG_CR_VDCFG2` writer"]
pub type W = crate::W<SfrVdcfgCrVdcfg2Spec>;
#[doc = "Field `cr_vdcfg2` reader - cr_vdcfg read/write control register"]
pub type CrVdcfg2R = crate::FieldReader;
#[doc = "Field `cr_vdcfg2` writer - cr_vdcfg read/write control register"]
pub type CrVdcfg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - cr_vdcfg read/write control register"]
    #[inline(always)]
    pub fn cr_vdcfg2(&self) -> CrVdcfg2R {
        CrVdcfg2R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - cr_vdcfg read/write control register"]
    #[inline(always)]
    pub fn cr_vdcfg2(&mut self) -> CrVdcfg2W<'_, SfrVdcfgCrVdcfg2Spec> {
        CrVdcfg2W::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrVdcfgCrVdcfg2Spec;
impl crate::RegisterSpec for SfrVdcfgCrVdcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_vdcfg_cr_vdcfg2::R`](R) reader structure"]
impl crate::Readable for SfrVdcfgCrVdcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_vdcfg_cr_vdcfg2::W`](W) writer structure"]
impl crate::Writable for SfrVdcfgCrVdcfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_VDCFG_CR_VDCFG2 to value 0"]
impl crate::Resettable for SfrVdcfgCrVdcfg2Spec {}
