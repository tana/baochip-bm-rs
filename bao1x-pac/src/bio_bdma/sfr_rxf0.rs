#[doc = "Register `SFR_RXF0` reader"]
pub type R = crate::R<SfrRxf0Spec>;
#[doc = "Register `SFR_RXF0` writer"]
pub type W = crate::W<SfrRxf0Spec>;
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
    pub fn fdout(&mut self) -> FdoutW<'_, SfrRxf0Spec> {
        FdoutW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L497 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L497>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rxf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rxf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRxf0Spec;
impl crate::RegisterSpec for SfrRxf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rxf0::R`](R) reader structure"]
impl crate::Readable for SfrRxf0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rxf0::W`](W) writer structure"]
impl crate::Writable for SfrRxf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RXF0 to value 0"]
impl crate::Resettable for SfrRxf0Spec {}
