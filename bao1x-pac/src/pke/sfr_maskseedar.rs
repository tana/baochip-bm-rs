#[doc = "Register `SFR_MASKSEEDAR` reader"]
pub type R = crate::R<SfrMaskseedarSpec>;
#[doc = "Register `SFR_MASKSEEDAR` writer"]
pub type W = crate::W<SfrMaskseedarSpec>;
#[doc = "Field `sfr_maskseedar` reader - sfr_maskseedar performs action on write of value: 0x5a"]
pub type SfrMaskseedarR = crate::FieldReader<u32>;
#[doc = "Field `sfr_maskseedar` writer - sfr_maskseedar performs action on write of value: 0x5a"]
pub type SfrMaskseedarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_maskseedar performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfr_maskseedar(&self) -> SfrMaskseedarR {
        SfrMaskseedarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_maskseedar performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfr_maskseedar(&mut self) -> SfrMaskseedarW<'_, SfrMaskseedarSpec> {
        SfrMaskseedarW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L313 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L313>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_maskseedar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_maskseedar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMaskseedarSpec;
impl crate::RegisterSpec for SfrMaskseedarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_maskseedar::R`](R) reader structure"]
impl crate::Readable for SfrMaskseedarSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_maskseedar::W`](W) writer structure"]
impl crate::Writable for SfrMaskseedarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MASKSEEDAR to value 0"]
impl crate::Resettable for SfrMaskseedarSpec {}
