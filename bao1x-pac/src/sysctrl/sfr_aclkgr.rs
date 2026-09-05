#[doc = "Register `SFR_ACLKGR` reader"]
pub type R = crate::R<SfrAclkgrSpec>;
#[doc = "Register `SFR_ACLKGR` writer"]
pub type W = crate::W<SfrAclkgrSpec>;
#[doc = "Field `aclksubgate` reader - aclksubgate read only status register"]
pub type AclksubgateR = crate::FieldReader;
#[doc = "Field `aclksubgate` writer - aclksubgate read only status register"]
pub type AclksubgateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - aclksubgate read only status register"]
    #[inline(always)]
    pub fn aclksubgate(&self) -> AclksubgateR {
        AclksubgateR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - aclksubgate read only status register"]
    #[inline(always)]
    pub fn aclksubgate(&mut self) -> AclksubgateW<'_, SfrAclkgrSpec> {
        AclksubgateW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L794 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L794>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_aclkgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_aclkgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrAclkgrSpec;
impl crate::RegisterSpec for SfrAclkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_aclkgr::R`](R) reader structure"]
impl crate::Readable for SfrAclkgrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_aclkgr::W`](W) writer structure"]
impl crate::Writable for SfrAclkgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ACLKGR to value 0"]
impl crate::Resettable for SfrAclkgrSpec {}
