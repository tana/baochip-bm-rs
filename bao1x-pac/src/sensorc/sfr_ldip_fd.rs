#[doc = "Register `SFR_LDIP_FD` reader"]
pub type R = crate::R<SfrLdipFdSpec>;
#[doc = "Register `SFR_LDIP_FD` writer"]
pub type W = crate::W<SfrLdipFdSpec>;
#[doc = "Field `sfr_ldip_fd` reader - sfr_ldip_fd read/write control register"]
pub type SfrLdipFdR = crate::FieldReader<u16>;
#[doc = "Field `sfr_ldip_fd` writer - sfr_ldip_fd read/write control register"]
pub type SfrLdipFdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_ldip_fd read/write control register"]
    #[inline(always)]
    pub fn sfr_ldip_fd(&self) -> SfrLdipFdR {
        SfrLdipFdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_ldip_fd read/write control register"]
    #[inline(always)]
    pub fn sfr_ldip_fd(&mut self) -> SfrLdipFdW<'_, SfrLdipFdSpec> {
        SfrLdipFdW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L78 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L78>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldip_fd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldip_fd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrLdipFdSpec;
impl crate::RegisterSpec for SfrLdipFdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ldip_fd::R`](R) reader structure"]
impl crate::Readable for SfrLdipFdSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ldip_fd::W`](W) writer structure"]
impl crate::Writable for SfrLdipFdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_LDIP_FD to value 0"]
impl crate::Resettable for SfrLdipFdSpec {}
