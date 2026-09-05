#[doc = "Register `SFR_MASKSEED` reader"]
pub type R = crate::R<SfrMaskseedSpec>;
#[doc = "Register `SFR_MASKSEED` writer"]
pub type W = crate::W<SfrMaskseedSpec>;
#[doc = "Field `sfr_maskseed` reader - sfr_maskseed read/write control register"]
pub type SfrMaskseedR = crate::FieldReader<u32>;
#[doc = "Field `sfr_maskseed` writer - sfr_maskseed read/write control register"]
pub type SfrMaskseedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_maskseed read/write control register"]
    #[inline(always)]
    pub fn sfr_maskseed(&self) -> SfrMaskseedR {
        SfrMaskseedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_maskseed read/write control register"]
    #[inline(always)]
    pub fn sfr_maskseed(&mut self) -> SfrMaskseedW<'_, SfrMaskseedSpec> {
        SfrMaskseedW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L312 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L312>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_maskseed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_maskseed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMaskseedSpec;
impl crate::RegisterSpec for SfrMaskseedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_maskseed::R`](R) reader structure"]
impl crate::Readable for SfrMaskseedSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_maskseed::W`](W) writer structure"]
impl crate::Writable for SfrMaskseedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MASKSEED to value 0"]
impl crate::Resettable for SfrMaskseedSpec {}
