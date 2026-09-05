#[doc = "Register `SFR_SCH_AXSTART` reader"]
pub type R = crate::R<SfrSchAxstartSpec>;
#[doc = "Register `SFR_SCH_AXSTART` writer"]
pub type W = crate::W<SfrSchAxstartSpec>;
#[doc = "Field `schcr_axstart` reader - schcr_axstart read/write control register"]
pub type SchcrAxstartR = crate::FieldReader<u32>;
#[doc = "Field `schcr_axstart` writer - schcr_axstart read/write control register"]
pub type SchcrAxstartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - schcr_axstart read/write control register"]
    #[inline(always)]
    pub fn schcr_axstart(&self) -> SchcrAxstartR {
        SchcrAxstartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - schcr_axstart read/write control register"]
    #[inline(always)]
    pub fn schcr_axstart(&mut self) -> SchcrAxstartW<'_, SfrSchAxstartSpec> {
        SchcrAxstartW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L106>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_axstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_axstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSchAxstartSpec;
impl crate::RegisterSpec for SfrSchAxstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sch_axstart::R`](R) reader structure"]
impl crate::Readable for SfrSchAxstartSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sch_axstart::W`](W) writer structure"]
impl crate::Writable for SfrSchAxstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SCH_AXSTART to value 0"]
impl crate::Resettable for SfrSchAxstartSpec {}
