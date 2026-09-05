#[doc = "Register `SFR_LDSR` reader"]
pub type R = crate::R<SfrLdsrSpec>;
#[doc = "Register `SFR_LDSR` writer"]
pub type W = crate::W<SfrLdsrSpec>;
#[doc = "Field `sr_ldsr` reader - sr_ldsr read only status register"]
pub type SrLdsrR = crate::FieldReader;
#[doc = "Field `sr_ldsr` writer - sr_ldsr read only status register"]
pub type SrLdsrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sr_ldsr read only status register"]
    #[inline(always)]
    pub fn sr_ldsr(&self) -> SrLdsrR {
        SrLdsrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sr_ldsr read only status register"]
    #[inline(always)]
    pub fn sr_ldsr(&mut self) -> SrLdsrW<'_, SfrLdsrSpec> {
        SrLdsrW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L69 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L69>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrLdsrSpec;
impl crate::RegisterSpec for SfrLdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ldsr::R`](R) reader structure"]
impl crate::Readable for SfrLdsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ldsr::W`](W) writer structure"]
impl crate::Writable for SfrLdsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_LDSR to value 0"]
impl crate::Resettable for SfrLdsrSpec {}
