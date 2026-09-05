#[doc = "Register `SFR_CGUFSSR_FSFREQ2` reader"]
pub type R = crate::R<SfrCgufssrFsfreq2Spec>;
#[doc = "Register `SFR_CGUFSSR_FSFREQ2` writer"]
pub type W = crate::W<SfrCgufssrFsfreq2Spec>;
#[doc = "Field `fsfreq2` reader - fsfreq read only status register"]
pub type Fsfreq2R = crate::FieldReader<u32>;
#[doc = "Field `fsfreq2` writer - fsfreq read only status register"]
pub type Fsfreq2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fsfreq read only status register"]
    #[inline(always)]
    pub fn fsfreq2(&self) -> Fsfreq2R {
        Fsfreq2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fsfreq read only status register"]
    #[inline(always)]
    pub fn fsfreq2(&mut self) -> Fsfreq2W<'_, SfrCgufssrFsfreq2Spec> {
        Fsfreq2W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufssr_fsfreq2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufssr_fsfreq2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufssrFsfreq2Spec;
impl crate::RegisterSpec for SfrCgufssrFsfreq2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufssr_fsfreq2::R`](R) reader structure"]
impl crate::Readable for SfrCgufssrFsfreq2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufssr_fsfreq2::W`](W) writer structure"]
impl crate::Writable for SfrCgufssrFsfreq2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFSSR_FSFREQ2 to value 0"]
impl crate::Resettable for SfrCgufssrFsfreq2Spec {}
