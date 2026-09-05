#[doc = "Register `SFR_DRGEN` reader"]
pub type R = crate::R<SfrDrgenSpec>;
#[doc = "Register `SFR_DRGEN` writer"]
pub type W = crate::W<SfrDrgenSpec>;
#[doc = "Field `sfr_drgen` reader - sfr_drgen read/write control register"]
pub type SfrDrgenR = crate::FieldReader<u32>;
#[doc = "Field `sfr_drgen` writer - sfr_drgen read/write control register"]
pub type SfrDrgenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_drgen read/write control register"]
    #[inline(always)]
    pub fn sfr_drgen(&self) -> SfrDrgenR {
        SfrDrgenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_drgen read/write control register"]
    #[inline(always)]
    pub fn sfr_drgen(&mut self) -> SfrDrgenW<'_, SfrDrgenSpec> {
        SfrDrgenW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L240 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L240>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_drgen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_drgen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDrgenSpec;
impl crate::RegisterSpec for SfrDrgenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_drgen::R`](R) reader structure"]
impl crate::Readable for SfrDrgenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_drgen::W`](W) writer structure"]
impl crate::Writable for SfrDrgenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DRGEN to value 0"]
impl crate::Resettable for SfrDrgenSpec {}
