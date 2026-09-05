#[doc = "Register `SFR_SCH_SEGSTART` reader"]
pub type R = crate::R<SfrSchSegstartSpec>;
#[doc = "Register `SFR_SCH_SEGSTART` writer"]
pub type W = crate::W<SfrSchSegstartSpec>;
#[doc = "Field `schcr_segstart` reader - schcr_segstart read/write control register"]
pub type SchcrSegstartR = crate::FieldReader<u16>;
#[doc = "Field `schcr_segstart` writer - schcr_segstart read/write control register"]
pub type SchcrSegstartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - schcr_segstart read/write control register"]
    #[inline(always)]
    pub fn schcr_segstart(&self) -> SchcrSegstartR {
        SchcrSegstartR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - schcr_segstart read/write control register"]
    #[inline(always)]
    pub fn schcr_segstart(&mut self) -> SchcrSegstartW<'_, SfrSchSegstartSpec> {
        SchcrSegstartW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L108>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_segstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_segstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSchSegstartSpec;
impl crate::RegisterSpec for SfrSchSegstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sch_segstart::R`](R) reader structure"]
impl crate::Readable for SfrSchSegstartSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sch_segstart::W`](W) writer structure"]
impl crate::Writable for SfrSchSegstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SCH_SEGSTART to value 0"]
impl crate::Resettable for SfrSchSegstartSpec {}
