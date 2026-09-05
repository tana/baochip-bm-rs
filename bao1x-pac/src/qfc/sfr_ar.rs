#[doc = "Register `SFR_AR` reader"]
pub type R = crate::R<SfrArSpec>;
#[doc = "Register `SFR_AR` writer"]
pub type W = crate::W<SfrArSpec>;
#[doc = "Field `sfr_ar` reader - sfr_ar performs action on write of value: 0x5a"]
pub type SfrArR = crate::FieldReader<u32>;
#[doc = "Field `sfr_ar` writer - sfr_ar performs action on write of value: 0x5a"]
pub type SfrArW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_ar performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfr_ar(&self) -> SfrArR {
        SfrArR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_ar performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfr_ar(&mut self) -> SfrArW<'_, SfrArSpec> {
        SfrArW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L190 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L190>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrArSpec;
impl crate::RegisterSpec for SfrArSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ar::R`](R) reader structure"]
impl crate::Readable for SfrArSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ar::W`](W) writer structure"]
impl crate::Writable for SfrArSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_AR to value 0"]
impl crate::Resettable for SfrArSpec {}
