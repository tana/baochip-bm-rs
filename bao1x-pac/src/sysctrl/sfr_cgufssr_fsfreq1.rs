#[doc = "Register `SFR_CGUFSSR_FSFREQ1` reader"]
pub type R = crate::R<SfrCgufssrFsfreq1Spec>;
#[doc = "Register `SFR_CGUFSSR_FSFREQ1` writer"]
pub type W = crate::W<SfrCgufssrFsfreq1Spec>;
#[doc = "Field `fsfreq1` reader - fsfreq read only status register"]
pub type Fsfreq1R = crate::FieldReader<u32>;
#[doc = "Field `fsfreq1` writer - fsfreq read only status register"]
pub type Fsfreq1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fsfreq read only status register"]
    #[inline(always)]
    pub fn fsfreq1(&self) -> Fsfreq1R {
        Fsfreq1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fsfreq read only status register"]
    #[inline(always)]
    pub fn fsfreq1(&mut self) -> Fsfreq1W<'_, SfrCgufssrFsfreq1Spec> {
        Fsfreq1W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufssr_fsfreq1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufssr_fsfreq1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufssrFsfreq1Spec;
impl crate::RegisterSpec for SfrCgufssrFsfreq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufssr_fsfreq1::R`](R) reader structure"]
impl crate::Readable for SfrCgufssrFsfreq1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufssr_fsfreq1::W`](W) writer structure"]
impl crate::Writable for SfrCgufssrFsfreq1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFSSR_FSFREQ1 to value 0"]
impl crate::Resettable for SfrCgufssrFsfreq1Spec {}
