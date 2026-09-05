#[doc = "Register `SFR_SCH_TRANSIZE` reader"]
pub type R = crate::R<SfrSchTransizeSpec>;
#[doc = "Register `SFR_SCH_TRANSIZE` writer"]
pub type W = crate::W<SfrSchTransizeSpec>;
#[doc = "Field `schcr_transize` reader - schcr_transize read/write control register"]
pub type SchcrTransizeR = crate::FieldReader<u32>;
#[doc = "Field `schcr_transize` writer - schcr_transize read/write control register"]
pub type SchcrTransizeW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - schcr_transize read/write control register"]
    #[inline(always)]
    pub fn schcr_transize(&self) -> SchcrTransizeR {
        SchcrTransizeR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - schcr_transize read/write control register"]
    #[inline(always)]
    pub fn schcr_transize(&mut self) -> SchcrTransizeW<'_, SfrSchTransizeSpec> {
        SchcrTransizeW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L109 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L109>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_transize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_transize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSchTransizeSpec;
impl crate::RegisterSpec for SfrSchTransizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sch_transize::R`](R) reader structure"]
impl crate::Readable for SfrSchTransizeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sch_transize::W`](W) writer structure"]
impl crate::Writable for SfrSchTransizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SCH_TRANSIZE to value 0"]
impl crate::Resettable for SfrSchTransizeSpec {}
