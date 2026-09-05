#[doc = "Register `SFR_PERI_GUTTER` reader"]
pub type R = crate::R<SfrPeriGutterSpec>;
#[doc = "Register `SFR_PERI_GUTTER` writer"]
pub type W = crate::W<SfrPeriGutterSpec>;
#[doc = "Field `sfr_peri_gutter` reader - sfr_peri_gutter read/write control register"]
pub type SfrPeriGutterR = crate::FieldReader<u32>;
#[doc = "Field `sfr_peri_gutter` writer - sfr_peri_gutter read/write control register"]
pub type SfrPeriGutterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_peri_gutter read/write control register"]
    #[inline(always)]
    pub fn sfr_peri_gutter(&self) -> SfrPeriGutterR {
        SfrPeriGutterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_peri_gutter read/write control register"]
    #[inline(always)]
    pub fn sfr_peri_gutter(&mut self) -> SfrPeriGutterW<'_, SfrPeriGutterSpec> {
        SfrPeriGutterW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L536 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L536>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_peri_gutter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_peri_gutter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPeriGutterSpec;
impl crate::RegisterSpec for SfrPeriGutterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_peri_gutter::R`](R) reader structure"]
impl crate::Readable for SfrPeriGutterSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_peri_gutter::W`](W) writer structure"]
impl crate::Writable for SfrPeriGutterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PERI_GUTTER to value 0"]
impl crate::Resettable for SfrPeriGutterSpec {}
