#[doc = "Register `SFR_PP` reader"]
pub type R = crate::R<SfrPpSpec>;
#[doc = "Register `SFR_PP` writer"]
pub type W = crate::W<SfrPpSpec>;
#[doc = "Field `sfr_pp` reader - sfr_pp read/write control register"]
pub type SfrPpR = crate::FieldReader<u32>;
#[doc = "Field `sfr_pp` writer - sfr_pp read/write control register"]
pub type SfrPpW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - sfr_pp read/write control register"]
    #[inline(always)]
    pub fn sfr_pp(&self) -> SfrPpR {
        SfrPpR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - sfr_pp read/write control register"]
    #[inline(always)]
    pub fn sfr_pp(&mut self) -> SfrPpW<'_, SfrPpSpec> {
        SfrPpW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L107>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPpSpec;
impl crate::RegisterSpec for SfrPpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pp::R`](R) reader structure"]
impl crate::Readable for SfrPpSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pp::W`](W) writer structure"]
impl crate::Writable for SfrPpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PP to value 0"]
impl crate::Resettable for SfrPpSpec {}
