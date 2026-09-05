#[doc = "Register `SFR_MEM_GUTTER` reader"]
pub type R = crate::R<SfrMemGutterSpec>;
#[doc = "Register `SFR_MEM_GUTTER` writer"]
pub type W = crate::W<SfrMemGutterSpec>;
#[doc = "Field `sfr_mem_gutter` reader - sfr_mem_gutter read/write control register"]
pub type SfrMemGutterR = crate::FieldReader<u32>;
#[doc = "Field `sfr_mem_gutter` writer - sfr_mem_gutter read/write control register"]
pub type SfrMemGutterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_mem_gutter read/write control register"]
    #[inline(always)]
    pub fn sfr_mem_gutter(&self) -> SfrMemGutterR {
        SfrMemGutterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_mem_gutter read/write control register"]
    #[inline(always)]
    pub fn sfr_mem_gutter(&mut self) -> SfrMemGutterW<'_, SfrMemGutterSpec> {
        SfrMemGutterW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L535 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L535>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mem_gutter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mem_gutter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMemGutterSpec;
impl crate::RegisterSpec for SfrMemGutterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_mem_gutter::R`](R) reader structure"]
impl crate::Readable for SfrMemGutterSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_mem_gutter::W`](W) writer structure"]
impl crate::Writable for SfrMemGutterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MEM_GUTTER to value 0"]
impl crate::Resettable for SfrMemGutterSpec {}
