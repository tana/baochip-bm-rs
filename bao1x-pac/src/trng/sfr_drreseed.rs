#[doc = "Register `SFR_DRRESEED` reader"]
pub type R = crate::R<SfrDrreseedSpec>;
#[doc = "Register `SFR_DRRESEED` writer"]
pub type W = crate::W<SfrDrreseedSpec>;
#[doc = "Field `sfr_drreseed` reader - sfr_drreseed read/write control register"]
pub type SfrDrreseedR = crate::FieldReader<u32>;
#[doc = "Field `sfr_drreseed` writer - sfr_drreseed read/write control register"]
pub type SfrDrreseedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_drreseed read/write control register"]
    #[inline(always)]
    pub fn sfr_drreseed(&self) -> SfrDrreseedR {
        SfrDrreseedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_drreseed read/write control register"]
    #[inline(always)]
    pub fn sfr_drreseed(&mut self) -> SfrDrreseedW<'_, SfrDrreseedSpec> {
        SfrDrreseedW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L241 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L241>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_drreseed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_drreseed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDrreseedSpec;
impl crate::RegisterSpec for SfrDrreseedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_drreseed::R`](R) reader structure"]
impl crate::Readable for SfrDrreseedSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_drreseed::W`](W) writer structure"]
impl crate::Writable for SfrDrreseedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DRRESEED to value 0"]
impl crate::Resettable for SfrDrreseedSpec {}
