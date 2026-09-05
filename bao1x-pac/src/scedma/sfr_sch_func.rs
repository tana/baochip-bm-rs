#[doc = "Register `SFR_SCH_FUNC` reader"]
pub type R = crate::R<SfrSchFuncSpec>;
#[doc = "Register `SFR_SCH_FUNC` writer"]
pub type W = crate::W<SfrSchFuncSpec>;
#[doc = "Field `schcr_func` reader - schcr_func read/write control register"]
pub type SchcrFuncR = crate::BitReader;
#[doc = "Field `schcr_func` writer - schcr_func read/write control register"]
pub type SchcrFuncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - schcr_func read/write control register"]
    #[inline(always)]
    pub fn schcr_func(&self) -> SchcrFuncR {
        SchcrFuncR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - schcr_func read/write control register"]
    #[inline(always)]
    pub fn schcr_func(&mut self) -> SchcrFuncW<'_, SfrSchFuncSpec> {
        SchcrFuncW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_func::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_func::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSchFuncSpec;
impl crate::RegisterSpec for SfrSchFuncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sch_func::R`](R) reader structure"]
impl crate::Readable for SfrSchFuncSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sch_func::W`](W) writer structure"]
impl crate::Writable for SfrSchFuncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SCH_FUNC to value 0"]
impl crate::Resettable for SfrSchFuncSpec {}
