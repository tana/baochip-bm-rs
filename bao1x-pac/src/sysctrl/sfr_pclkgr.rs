#[doc = "Register `SFR_PCLKGR` reader"]
pub type R = crate::R<SfrPclkgrSpec>;
#[doc = "Register `SFR_PCLKGR` writer"]
pub type W = crate::W<SfrPclkgrSpec>;
#[doc = "Field `pclksubgate` reader - pclksubgate read only status register"]
pub type PclksubgateR = crate::FieldReader;
#[doc = "Field `pclksubgate` writer - pclksubgate read only status register"]
pub type PclksubgateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - pclksubgate read only status register"]
    #[inline(always)]
    pub fn pclksubgate(&self) -> PclksubgateR {
        PclksubgateR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - pclksubgate read only status register"]
    #[inline(always)]
    pub fn pclksubgate(&mut self) -> PclksubgateW<'_, SfrPclkgrSpec> {
        PclksubgateW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L797 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L797>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pclkgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pclkgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPclkgrSpec;
impl crate::RegisterSpec for SfrPclkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pclkgr::R`](R) reader structure"]
impl crate::Readable for SfrPclkgrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pclkgr::W`](W) writer structure"]
impl crate::Writable for SfrPclkgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PCLKGR to value 0"]
impl crate::Resettable for SfrPclkgrSpec {}
