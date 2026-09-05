#[doc = "Register `SFR_RXF2` reader"]
pub type R = crate::R<SfrRxf2Spec>;
#[doc = "Register `SFR_RXF2` writer"]
pub type W = crate::W<SfrRxf2Spec>;
#[doc = "Field `fdout` reader - fdout read only status register"]
pub type FdoutR = crate::FieldReader<u32>;
#[doc = "Field `fdout` writer - fdout read only status register"]
pub type FdoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fdout read only status register"]
    #[inline(always)]
    pub fn fdout(&self) -> FdoutR {
        FdoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fdout read only status register"]
    #[inline(always)]
    pub fn fdout(&mut self) -> FdoutW<'_, SfrRxf2Spec> {
        FdoutW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L499 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L499>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rxf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rxf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRxf2Spec;
impl crate::RegisterSpec for SfrRxf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rxf2::R`](R) reader structure"]
impl crate::Readable for SfrRxf2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rxf2::W`](W) writer structure"]
impl crate::Writable for SfrRxf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RXF2 to value 0"]
impl crate::Resettable for SfrRxf2Spec {}
