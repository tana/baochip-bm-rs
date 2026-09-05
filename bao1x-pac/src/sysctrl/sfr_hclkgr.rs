#[doc = "Register `SFR_HCLKGR` reader"]
pub type R = crate::R<SfrHclkgrSpec>;
#[doc = "Register `SFR_HCLKGR` writer"]
pub type W = crate::W<SfrHclkgrSpec>;
#[doc = "Field `hclksubgate` reader - hclksubgate read only status register"]
pub type HclksubgateR = crate::FieldReader;
#[doc = "Field `hclksubgate` writer - hclksubgate read only status register"]
pub type HclksubgateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - hclksubgate read only status register"]
    #[inline(always)]
    pub fn hclksubgate(&self) -> HclksubgateR {
        HclksubgateR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - hclksubgate read only status register"]
    #[inline(always)]
    pub fn hclksubgate(&mut self) -> HclksubgateW<'_, SfrHclkgrSpec> {
        HclksubgateW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L795 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L795>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_hclkgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_hclkgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrHclkgrSpec;
impl crate::RegisterSpec for SfrHclkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_hclkgr::R`](R) reader structure"]
impl crate::Readable for SfrHclkgrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_hclkgr::W`](W) writer structure"]
impl crate::Writable for SfrHclkgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_HCLKGR to value 0"]
impl crate::Resettable for SfrHclkgrSpec {}
