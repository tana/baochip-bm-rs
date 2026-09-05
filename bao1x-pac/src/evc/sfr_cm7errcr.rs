#[doc = "Register `SFR_CM7ERRCR` reader"]
pub type R = crate::R<SfrCm7errcrSpec>;
#[doc = "Register `SFR_CM7ERRCR` writer"]
pub type W = crate::W<SfrCm7errcrSpec>;
#[doc = "Field `erren` reader - erren read/write control register"]
pub type ErrenR = crate::FieldReader<u32>;
#[doc = "Field `erren` writer - erren read/write control register"]
pub type ErrenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - erren read/write control register"]
    #[inline(always)]
    pub fn erren(&self) -> ErrenR {
        ErrenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - erren read/write control register"]
    #[inline(always)]
    pub fn erren(&mut self) -> ErrenW<'_, SfrCm7errcrSpec> {
        ErrenW::new(self, 0)
    }
}
#[doc = "See `evc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7errcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7errcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCm7errcrSpec;
impl crate::RegisterSpec for SfrCm7errcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cm7errcr::R`](R) reader structure"]
impl crate::Readable for SfrCm7errcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cm7errcr::W`](W) writer structure"]
impl crate::Writable for SfrCm7errcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CM7ERRCR to value 0"]
impl crate::Resettable for SfrCm7errcrSpec {}
