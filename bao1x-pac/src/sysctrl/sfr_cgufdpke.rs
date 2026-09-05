#[doc = "Register `SFR_CGUFDPKE` reader"]
pub type R = crate::R<SfrCgufdpkeSpec>;
#[doc = "Register `SFR_CGUFDPKE` writer"]
pub type W = crate::W<SfrCgufdpkeSpec>;
#[doc = "Field `sfr_cgufdpke` reader - sfr_cgufdpke read/write control register"]
pub type SfrCgufdpkeR = crate::FieldReader;
#[doc = "Field `sfr_cgufdpke` writer - sfr_cgufdpke read/write control register"]
pub type SfrCgufdpkeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfr_cgufdpke read/write control register"]
    #[inline(always)]
    pub fn sfr_cgufdpke(&self) -> SfrCgufdpkeR {
        SfrCgufdpkeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfr_cgufdpke read/write control register"]
    #[inline(always)]
    pub fn sfr_cgufdpke(&mut self) -> SfrCgufdpkeW<'_, SfrCgufdpkeSpec> {
        SfrCgufdpkeW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L777 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L777>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufdpke::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufdpke::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufdpkeSpec;
impl crate::RegisterSpec for SfrCgufdpkeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufdpke::R`](R) reader structure"]
impl crate::Readable for SfrCgufdpkeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufdpke::W`](W) writer structure"]
impl crate::Writable for SfrCgufdpkeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFDPKE to value 0"]
impl crate::Resettable for SfrCgufdpkeSpec {}
