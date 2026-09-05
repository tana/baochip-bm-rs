#[doc = "Register `SFR_IOX` reader"]
pub type R = crate::R<SfrIoxSpec>;
#[doc = "Register `SFR_IOX` writer"]
pub type W = crate::W<SfrIoxSpec>;
#[doc = "Field `sfr_iox` reader - sfr_iox read/write control register"]
pub type SfrIoxR = crate::BitReader;
#[doc = "Field `sfr_iox` writer - sfr_iox read/write control register"]
pub type SfrIoxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sfr_iox read/write control register"]
    #[inline(always)]
    pub fn sfr_iox(&self) -> SfrIoxR {
        SfrIoxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sfr_iox read/write control register"]
    #[inline(always)]
    pub fn sfr_iox(&mut self) -> SfrIoxW<'_, SfrIoxSpec> {
        SfrIoxW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L400 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L400>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_iox::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_iox::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIoxSpec;
impl crate::RegisterSpec for SfrIoxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_iox::R`](R) reader structure"]
impl crate::Readable for SfrIoxSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_iox::W`](W) writer structure"]
impl crate::Writable for SfrIoxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IOX to value 0"]
impl crate::Resettable for SfrIoxSpec {}
