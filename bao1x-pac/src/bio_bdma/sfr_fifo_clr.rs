#[doc = "Register `SFR_FIFO_CLR` reader"]
pub type R = crate::R<SfrFifoClrSpec>;
#[doc = "Register `SFR_FIFO_CLR` writer"]
pub type W = crate::W<SfrFifoClrSpec>;
#[doc = "Field `sfr_fifo_clr` reader - sfr_fifo_clr read/write control register"]
pub type SfrFifoClrR = crate::FieldReader;
#[doc = "Field `sfr_fifo_clr` writer - sfr_fifo_clr read/write control register"]
pub type SfrFifoClrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sfr_fifo_clr read/write control register"]
    #[inline(always)]
    pub fn sfr_fifo_clr(&self) -> SfrFifoClrR {
        SfrFifoClrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sfr_fifo_clr read/write control register"]
    #[inline(always)]
    pub fn sfr_fifo_clr(&mut self) -> SfrFifoClrW<'_, SfrFifoClrSpec> {
        SfrFifoClrW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L509 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L509>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fifo_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fifo_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFifoClrSpec;
impl crate::RegisterSpec for SfrFifoClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_fifo_clr::R`](R) reader structure"]
impl crate::Readable for SfrFifoClrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_fifo_clr::W`](W) writer structure"]
impl crate::Writable for SfrFifoClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FIFO_CLR to value 0"]
impl crate::Resettable for SfrFifoClrSpec {}
