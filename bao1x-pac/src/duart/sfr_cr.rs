#[doc = "Register `SFR_CR` reader"]
pub type R = crate::R<SfrCrSpec>;
#[doc = "Register `SFR_CR` writer"]
pub type W = crate::W<SfrCrSpec>;
#[doc = "Field `sfr_cr` reader - sfr_cr read/write control register"]
pub type SfrCrR = crate::BitReader;
#[doc = "Field `sfr_cr` writer - sfr_cr read/write control register"]
pub type SfrCrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sfr_cr read/write control register"]
    #[inline(always)]
    pub fn sfr_cr(&self) -> SfrCrR {
        SfrCrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sfr_cr read/write control register"]
    #[inline(always)]
    pub fn sfr_cr(&mut self) -> SfrCrW<'_, SfrCrSpec> {
        SfrCrW::new(self, 0)
    }
}
#[doc = "See `duart.sv#L43 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L43>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrSpec;
impl crate::RegisterSpec for SfrCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cr::R`](R) reader structure"]
impl crate::Readable for SfrCrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cr::W`](W) writer structure"]
impl crate::Writable for SfrCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CR to value 0"]
impl crate::Resettable for SfrCrSpec {}
