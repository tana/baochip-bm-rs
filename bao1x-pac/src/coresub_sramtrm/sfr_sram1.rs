#[doc = "Register `SFR_SRAM1` reader"]
pub type R = crate::R<SfrSram1Spec>;
#[doc = "Register `SFR_SRAM1` writer"]
pub type W = crate::W<SfrSram1Spec>;
#[doc = "Field `sfr_sram1` reader - sfr_sram1 read/write control register"]
pub type SfrSram1R = crate::FieldReader;
#[doc = "Field `sfr_sram1` writer - sfr_sram1 read/write control register"]
pub type SfrSram1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_sram1 read/write control register"]
    #[inline(always)]
    pub fn sfr_sram1(&self) -> SfrSram1R {
        SfrSram1R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_sram1 read/write control register"]
    #[inline(always)]
    pub fn sfr_sram1(&mut self) -> SfrSram1W<'_, SfrSram1Spec> {
        SfrSram1W::new(self, 0)
    }
}
#[doc = "See `coresub_sramtrm.sv#L58 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L58>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sram1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sram1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSram1Spec;
impl crate::RegisterSpec for SfrSram1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sram1::R`](R) reader structure"]
impl crate::Readable for SfrSram1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sram1::W`](W) writer structure"]
impl crate::Writable for SfrSram1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SRAM1 to value 0"]
impl crate::Resettable for SfrSram1Spec {}
