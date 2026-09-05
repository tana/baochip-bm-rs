#[doc = "Register `SFR_TXF3` reader"]
pub type R = crate::R<SfrTxf3Spec>;
#[doc = "Register `SFR_TXF3` writer"]
pub type W = crate::W<SfrTxf3Spec>;
#[doc = "Field `fdin` reader - fdin read/write control register"]
pub type FdinR = crate::FieldReader<u32>;
#[doc = "Field `fdin` writer - fdin read/write control register"]
pub type FdinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fdin read/write control register"]
    #[inline(always)]
    pub fn fdin(&self) -> FdinR {
        FdinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fdin read/write control register"]
    #[inline(always)]
    pub fn fdin(&mut self) -> FdinW<'_, SfrTxf3Spec> {
        FdinW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L496 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L496>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_txf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_txf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTxf3Spec;
impl crate::RegisterSpec for SfrTxf3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_txf3::R`](R) reader structure"]
impl crate::Readable for SfrTxf3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_txf3::W`](W) writer structure"]
impl crate::Writable for SfrTxf3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TXF3 to value 0"]
impl crate::Resettable for SfrTxf3Spec {}
