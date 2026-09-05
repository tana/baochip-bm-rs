#[doc = "Register `SFR_IRQ_EDGE` reader"]
pub type R = crate::R<SfrIrqEdgeSpec>;
#[doc = "Register `SFR_IRQ_EDGE` writer"]
pub type W = crate::W<SfrIrqEdgeSpec>;
#[doc = "Field `sfr_irq_edge` reader - sfr_irq_edge read/write control register"]
pub type SfrIrqEdgeR = crate::FieldReader;
#[doc = "Field `sfr_irq_edge` writer - sfr_irq_edge read/write control register"]
pub type SfrIrqEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sfr_irq_edge read/write control register"]
    #[inline(always)]
    pub fn sfr_irq_edge(&self) -> SfrIrqEdgeR {
        SfrIrqEdgeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sfr_irq_edge read/write control register"]
    #[inline(always)]
    pub fn sfr_irq_edge(&mut self) -> SfrIrqEdgeW<'_, SfrIrqEdgeSpec> {
        SfrIrqEdgeW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L525 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L525>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irq_edge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irq_edge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIrqEdgeSpec;
impl crate::RegisterSpec for SfrIrqEdgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_irq_edge::R`](R) reader structure"]
impl crate::Readable for SfrIrqEdgeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_irq_edge::W`](W) writer structure"]
impl crate::Writable for SfrIrqEdgeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IRQ_EDGE to value 0"]
impl crate::Resettable for SfrIrqEdgeSpec {}
