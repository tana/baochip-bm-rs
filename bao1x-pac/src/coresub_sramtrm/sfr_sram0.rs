#[doc = "Register `SFR_SRAM0` reader"]
pub type R = crate::R<SfrSram0Spec>;
#[doc = "Register `SFR_SRAM0` writer"]
pub type W = crate::W<SfrSram0Spec>;
#[doc = "Field `sfr_sram0` reader - sfr_sram0 read/write control register"]
pub type SfrSram0R = crate::FieldReader;
#[doc = "Field `sfr_sram0` writer - sfr_sram0 read/write control register"]
pub type SfrSram0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_sram0 read/write control register"]
    #[inline(always)]
    pub fn sfr_sram0(&self) -> SfrSram0R {
        SfrSram0R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_sram0 read/write control register"]
    #[inline(always)]
    pub fn sfr_sram0(&mut self) -> SfrSram0W<'_, SfrSram0Spec> {
        SfrSram0W::new(self, 0)
    }
}
#[doc = "See `coresub_sramtrm.sv#L57 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L57>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sram0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sram0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSram0Spec;
impl crate::RegisterSpec for SfrSram0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sram0::R`](R) reader structure"]
impl crate::Readable for SfrSram0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sram0::W`](W) writer structure"]
impl crate::Writable for SfrSram0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SRAM0 to value 0"]
impl crate::Resettable for SfrSram0Spec {}
