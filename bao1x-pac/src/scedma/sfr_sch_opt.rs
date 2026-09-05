#[doc = "Register `SFR_SCH_OPT` reader"]
pub type R = crate::R<SfrSchOptSpec>;
#[doc = "Register `SFR_SCH_OPT` writer"]
pub type W = crate::W<SfrSchOptSpec>;
#[doc = "Field `schcr_opt` reader - schcr_opt read/write control register"]
pub type SchcrOptR = crate::FieldReader<u16>;
#[doc = "Field `schcr_opt` writer - schcr_opt read/write control register"]
pub type SchcrOptW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - schcr_opt read/write control register"]
    #[inline(always)]
    pub fn schcr_opt(&self) -> SchcrOptR {
        SchcrOptR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - schcr_opt read/write control register"]
    #[inline(always)]
    pub fn schcr_opt(&mut self) -> SchcrOptW<'_, SfrSchOptSpec> {
        SchcrOptW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L105>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSchOptSpec;
impl crate::RegisterSpec for SfrSchOptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sch_opt::R`](R) reader structure"]
impl crate::Readable for SfrSchOptSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sch_opt::W`](W) writer structure"]
impl crate::Writable for SfrSchOptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SCH_OPT to value 0"]
impl crate::Resettable for SfrSchOptSpec {}
