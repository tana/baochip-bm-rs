#[doc = "Register `SFR_LDMASK` reader"]
pub type R = crate::R<SfrLdmaskSpec>;
#[doc = "Register `SFR_LDMASK` writer"]
pub type W = crate::W<SfrLdmaskSpec>;
#[doc = "Field `cr_ldmask` reader - cr_ldmask read/write control register"]
pub type CrLdmaskR = crate::FieldReader;
#[doc = "Field `cr_ldmask` writer - cr_ldmask read/write control register"]
pub type CrLdmaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - cr_ldmask read/write control register"]
    #[inline(always)]
    pub fn cr_ldmask(&self) -> CrLdmaskR {
        CrLdmaskR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - cr_ldmask read/write control register"]
    #[inline(always)]
    pub fn cr_ldmask(&mut self) -> CrLdmaskW<'_, SfrLdmaskSpec> {
        CrLdmaskW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L68 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L68>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrLdmaskSpec;
impl crate::RegisterSpec for SfrLdmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ldmask::R`](R) reader structure"]
impl crate::Readable for SfrLdmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ldmask::W`](W) writer structure"]
impl crate::Writable for SfrLdmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_LDMASK to value 0"]
impl crate::Resettable for SfrLdmaskSpec {}
