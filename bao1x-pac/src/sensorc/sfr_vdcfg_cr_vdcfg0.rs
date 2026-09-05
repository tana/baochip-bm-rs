#[doc = "Register `SFR_VDCFG_CR_VDCFG0` reader"]
pub type R = crate::R<SfrVdcfgCrVdcfg0Spec>;
#[doc = "Register `SFR_VDCFG_CR_VDCFG0` writer"]
pub type W = crate::W<SfrVdcfgCrVdcfg0Spec>;
#[doc = "Field `cr_vdcfg0` reader - cr_vdcfg read/write control register"]
pub type CrVdcfg0R = crate::FieldReader;
#[doc = "Field `cr_vdcfg0` writer - cr_vdcfg read/write control register"]
pub type CrVdcfg0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - cr_vdcfg read/write control register"]
    #[inline(always)]
    pub fn cr_vdcfg0(&self) -> CrVdcfg0R {
        CrVdcfg0R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - cr_vdcfg read/write control register"]
    #[inline(always)]
    pub fn cr_vdcfg0(&mut self) -> CrVdcfg0W<'_, SfrVdcfgCrVdcfg0Spec> {
        CrVdcfg0W::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrVdcfgCrVdcfg0Spec;
impl crate::RegisterSpec for SfrVdcfgCrVdcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_vdcfg_cr_vdcfg0::R`](R) reader structure"]
impl crate::Readable for SfrVdcfgCrVdcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_vdcfg_cr_vdcfg0::W`](W) writer structure"]
impl crate::Writable for SfrVdcfgCrVdcfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_VDCFG_CR_VDCFG0 to value 0"]
impl crate::Resettable for SfrVdcfgCrVdcfg0Spec {}
