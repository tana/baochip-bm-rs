#[doc = "Register `SFR_TXD` reader"]
pub type R = crate::R<SfrTxdSpec>;
#[doc = "Register `SFR_TXD` writer"]
pub type W = crate::W<SfrTxdSpec>;
#[doc = "Field `sfr_txd` reader - sfr_txd read/write control register"]
pub type SfrTxdR = crate::FieldReader;
#[doc = "Field `sfr_txd` writer - sfr_txd read/write control register"]
pub type SfrTxdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfr_txd read/write control register"]
    #[inline(always)]
    pub fn sfr_txd(&self) -> SfrTxdR {
        SfrTxdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfr_txd read/write control register"]
    #[inline(always)]
    pub fn sfr_txd(&mut self) -> SfrTxdW<'_, SfrTxdSpec> {
        SfrTxdW::new(self, 0)
    }
}
#[doc = "See `duart.sv#L42 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L42>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_txd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_txd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTxdSpec;
impl crate::RegisterSpec for SfrTxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_txd::R`](R) reader structure"]
impl crate::Readable for SfrTxdSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_txd::W`](W) writer structure"]
impl crate::Writable for SfrTxdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TXD to value 0"]
impl crate::Resettable for SfrTxdSpec {}
