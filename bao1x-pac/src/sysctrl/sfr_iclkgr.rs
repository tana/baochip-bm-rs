#[doc = "Register `SFR_ICLKGR` reader"]
pub type R = crate::R<SfrIclkgrSpec>;
#[doc = "Register `SFR_ICLKGR` writer"]
pub type W = crate::W<SfrIclkgrSpec>;
#[doc = "Field `iclksubgate` reader - iclksubgate read only status register"]
pub type IclksubgateR = crate::FieldReader;
#[doc = "Field `iclksubgate` writer - iclksubgate read only status register"]
pub type IclksubgateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - iclksubgate read only status register"]
    #[inline(always)]
    pub fn iclksubgate(&self) -> IclksubgateR {
        IclksubgateR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - iclksubgate read only status register"]
    #[inline(always)]
    pub fn iclksubgate(&mut self) -> IclksubgateW<'_, SfrIclkgrSpec> {
        IclksubgateW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L796 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L796>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_iclkgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_iclkgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIclkgrSpec;
impl crate::RegisterSpec for SfrIclkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_iclkgr::R`](R) reader structure"]
impl crate::Readable for SfrIclkgrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_iclkgr::W`](W) writer structure"]
impl crate::Writable for SfrIclkgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ICLKGR to value 0"]
impl crate::Resettable for SfrIclkgrSpec {}
