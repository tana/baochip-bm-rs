#[doc = "Register `SFR_CGUFDPER` reader"]
pub type R = crate::R<SfrCgufdperSpec>;
#[doc = "Register `SFR_CGUFDPER` writer"]
pub type W = crate::W<SfrCgufdperSpec>;
#[doc = "Field `sfr_cgufdper` reader - sfr_cgufdper read/write control register"]
pub type SfrCgufdperR = crate::FieldReader<u32>;
#[doc = "Field `sfr_cgufdper` writer - sfr_cgufdper read/write control register"]
pub type SfrCgufdperW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_cgufdper read/write control register"]
    #[inline(always)]
    pub fn sfr_cgufdper(&self) -> SfrCgufdperR {
        SfrCgufdperR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_cgufdper read/write control register"]
    #[inline(always)]
    pub fn sfr_cgufdper(&mut self) -> SfrCgufdperW<'_, SfrCgufdperSpec> {
        SfrCgufdperW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L776 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L776>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufdper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufdper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufdperSpec;
impl crate::RegisterSpec for SfrCgufdperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufdper::R`](R) reader structure"]
impl crate::Readable for SfrCgufdperSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufdper::W`](W) writer structure"]
impl crate::Writable for SfrCgufdperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFDPER to value 0"]
impl crate::Resettable for SfrCgufdperSpec {}
