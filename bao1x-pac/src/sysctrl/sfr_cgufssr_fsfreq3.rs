#[doc = "Register `SFR_CGUFSSR_FSFREQ3` reader"]
pub type R = crate::R<SfrCgufssrFsfreq3Spec>;
#[doc = "Register `SFR_CGUFSSR_FSFREQ3` writer"]
pub type W = crate::W<SfrCgufssrFsfreq3Spec>;
#[doc = "Field `fsfreq3` reader - fsfreq read only status register"]
pub type Fsfreq3R = crate::FieldReader<u32>;
#[doc = "Field `fsfreq3` writer - fsfreq read only status register"]
pub type Fsfreq3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fsfreq read only status register"]
    #[inline(always)]
    pub fn fsfreq3(&self) -> Fsfreq3R {
        Fsfreq3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fsfreq read only status register"]
    #[inline(always)]
    pub fn fsfreq3(&mut self) -> Fsfreq3W<'_, SfrCgufssrFsfreq3Spec> {
        Fsfreq3W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufssr_fsfreq3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufssr_fsfreq3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufssrFsfreq3Spec;
impl crate::RegisterSpec for SfrCgufssrFsfreq3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufssr_fsfreq3::R`](R) reader structure"]
impl crate::Readable for SfrCgufssrFsfreq3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufssr_fsfreq3::W`](W) writer structure"]
impl crate::Writable for SfrCgufssrFsfreq3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFSSR_FSFREQ3 to value 0"]
impl crate::Resettable for SfrCgufssrFsfreq3Spec {}
