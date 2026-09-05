#[doc = "Register `SFR_SCH_SEGID` reader"]
pub type R = crate::R<SfrSchSegidSpec>;
#[doc = "Register `SFR_SCH_SEGID` writer"]
pub type W = crate::W<SfrSchSegidSpec>;
#[doc = "Field `schcr_segid` reader - schcr_segid read/write control register"]
pub type SchcrSegidR = crate::FieldReader;
#[doc = "Field `schcr_segid` writer - schcr_segid read/write control register"]
pub type SchcrSegidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - schcr_segid read/write control register"]
    #[inline(always)]
    pub fn schcr_segid(&self) -> SchcrSegidR {
        SchcrSegidR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - schcr_segid read/write control register"]
    #[inline(always)]
    pub fn schcr_segid(&mut self) -> SchcrSegidW<'_, SfrSchSegidSpec> {
        SchcrSegidW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L107>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_segid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_segid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSchSegidSpec;
impl crate::RegisterSpec for SfrSchSegidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sch_segid::R`](R) reader structure"]
impl crate::Readable for SfrSchSegidSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sch_segid::W`](W) writer structure"]
impl crate::Writable for SfrSchSegidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SCH_SEGID to value 0"]
impl crate::Resettable for SfrSchSegidSpec {}
