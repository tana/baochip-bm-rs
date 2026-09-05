#[doc = "Register `SFR_RRCCR` reader"]
pub type R = crate::R<SfrRrccrSpec>;
#[doc = "Register `SFR_RRCCR` writer"]
pub type W = crate::W<SfrRrccrSpec>;
#[doc = "Field `sfr_rrccr` reader - sfr_rrccr read/write control register"]
pub type SfrRrccrR = crate::FieldReader<u32>;
#[doc = "Field `sfr_rrccr` writer - sfr_rrccr read/write control register"]
pub type SfrRrccrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_rrccr read/write control register"]
    #[inline(always)]
    pub fn sfr_rrccr(&self) -> SfrRrccrR {
        SfrRrccrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_rrccr read/write control register"]
    #[inline(always)]
    pub fn sfr_rrccr(&mut self) -> SfrRrccrW<'_, SfrRrccrSpec> {
        SfrRrccrW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L261 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L261>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrccrSpec;
impl crate::RegisterSpec for SfrRrccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrccr::R`](R) reader structure"]
impl crate::Readable for SfrRrccrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrccr::W`](W) writer structure"]
impl crate::Writable for SfrRrccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCCR to value 0"]
impl crate::Resettable for SfrRrccrSpec {}
