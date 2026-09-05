#[doc = "Register `SFR_LDCFG` reader"]
pub type R = crate::R<SfrLdcfgSpec>;
#[doc = "Register `SFR_LDCFG` writer"]
pub type W = crate::W<SfrLdcfgSpec>;
#[doc = "Field `sfr_ldcfg` reader - sfr_ldcfg read/write control register"]
pub type SfrLdcfgR = crate::FieldReader;
#[doc = "Field `sfr_ldcfg` writer - sfr_ldcfg read/write control register"]
pub type SfrLdcfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sfr_ldcfg read/write control register"]
    #[inline(always)]
    pub fn sfr_ldcfg(&self) -> SfrLdcfgR {
        SfrLdcfgR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sfr_ldcfg read/write control register"]
    #[inline(always)]
    pub fn sfr_ldcfg(&mut self) -> SfrLdcfgW<'_, SfrLdcfgSpec> {
        SfrLdcfgW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L70 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L70>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrLdcfgSpec;
impl crate::RegisterSpec for SfrLdcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ldcfg::R`](R) reader structure"]
impl crate::Readable for SfrLdcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ldcfg::W`](W) writer structure"]
impl crate::Writable for SfrLdcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_LDCFG to value 0"]
impl crate::Resettable for SfrLdcfgSpec {}
