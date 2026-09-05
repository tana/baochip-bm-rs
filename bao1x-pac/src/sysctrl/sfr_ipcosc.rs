#[doc = "Register `SFR_IPCOSC` reader"]
pub type R = crate::R<SfrIpcoscSpec>;
#[doc = "Register `SFR_IPCOSC` writer"]
pub type W = crate::W<SfrIpcoscSpec>;
#[doc = "Field `sfr_ipcosc` reader - sfr_ipcosc read/write control register"]
pub type SfrIpcoscR = crate::FieldReader;
#[doc = "Field `sfr_ipcosc` writer - sfr_ipcosc read/write control register"]
pub type SfrIpcoscW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - sfr_ipcosc read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcosc(&self) -> SfrIpcoscR {
        SfrIpcoscR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - sfr_ipcosc read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcosc(&mut self) -> SfrIpcoscW<'_, SfrIpcoscSpec> {
        SfrIpcoscW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L813 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L813>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcosc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcosc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIpcoscSpec;
impl crate::RegisterSpec for SfrIpcoscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ipcosc::R`](R) reader structure"]
impl crate::Readable for SfrIpcoscSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ipcosc::W`](W) writer structure"]
impl crate::Writable for SfrIpcoscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IPCOSC to value 0"]
impl crate::Resettable for SfrIpcoscSpec {}
