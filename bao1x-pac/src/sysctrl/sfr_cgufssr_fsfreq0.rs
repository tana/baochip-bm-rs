#[doc = "Register `SFR_CGUFSSR_FSFREQ0` reader"]
pub type R = crate::R<SfrCgufssrFsfreq0Spec>;
#[doc = "Register `SFR_CGUFSSR_FSFREQ0` writer"]
pub type W = crate::W<SfrCgufssrFsfreq0Spec>;
#[doc = "Field `fsfreq0` reader - fsfreq read only status register"]
pub type Fsfreq0R = crate::FieldReader<u32>;
#[doc = "Field `fsfreq0` writer - fsfreq read only status register"]
pub type Fsfreq0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fsfreq read only status register"]
    #[inline(always)]
    pub fn fsfreq0(&self) -> Fsfreq0R {
        Fsfreq0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fsfreq read only status register"]
    #[inline(always)]
    pub fn fsfreq0(&mut self) -> Fsfreq0W<'_, SfrCgufssrFsfreq0Spec> {
        Fsfreq0W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufssr_fsfreq0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufssr_fsfreq0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufssrFsfreq0Spec;
impl crate::RegisterSpec for SfrCgufssrFsfreq0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufssr_fsfreq0::R`](R) reader structure"]
impl crate::Readable for SfrCgufssrFsfreq0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufssr_fsfreq0::W`](W) writer structure"]
impl crate::Writable for SfrCgufssrFsfreq0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFSSR_FSFREQ0 to value 0"]
impl crate::Resettable for SfrCgufssrFsfreq0Spec {}
