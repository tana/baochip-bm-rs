#[doc = "Register `SFR_SCHSTART_AR` reader"]
pub type R = crate::R<SfrSchstartArSpec>;
#[doc = "Register `SFR_SCHSTART_AR` writer"]
pub type W = crate::W<SfrSchstartArSpec>;
#[doc = "Field `sfr_schstart_ar` reader - sfr_schstart_ar performs action on write of value: 0xaa"]
pub type SfrSchstartArR = crate::FieldReader<u32>;
#[doc = "Field `sfr_schstart_ar` writer - sfr_schstart_ar performs action on write of value: 0xaa"]
pub type SfrSchstartArW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_schstart_ar performs action on write of value: 0xaa"]
    #[inline(always)]
    pub fn sfr_schstart_ar(&self) -> SfrSchstartArR {
        SfrSchstartArR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_schstart_ar performs action on write of value: 0xaa"]
    #[inline(always)]
    pub fn sfr_schstart_ar(&mut self) -> SfrSchstartArW<'_, SfrSchstartArSpec> {
        SfrSchstartArW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L95 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L95>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_schstart_ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_schstart_ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSchstartArSpec;
impl crate::RegisterSpec for SfrSchstartArSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_schstart_ar::R`](R) reader structure"]
impl crate::Readable for SfrSchstartArSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_schstart_ar::W`](W) writer structure"]
impl crate::Writable for SfrSchstartArSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SCHSTART_AR to value 0"]
impl crate::Resettable for SfrSchstartArSpec {}
