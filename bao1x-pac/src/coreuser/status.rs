#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `coreuser` reader - Computed value of the `CoreUser` signal as passed on to the SoC. For debugging."]
pub type CoreuserR = crate::FieldReader;
#[doc = "Field `coreuser` writer - Computed value of the `CoreUser` signal as passed on to the SoC. For debugging."]
pub type CoreuserW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `mm` reader - Computed value of the `mm` signal passed on to the SoC."]
pub type MmR = crate::BitReader;
#[doc = "Field `mm` writer - Computed value of the `mm` signal passed on to the SoC."]
pub type MmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Computed value of the `CoreUser` signal as passed on to the SoC. For debugging."]
    #[inline(always)]
    pub fn coreuser(&self) -> CoreuserR {
        CoreuserR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Computed value of the `mm` signal passed on to the SoC."]
    #[inline(always)]
    pub fn mm(&self) -> MmR {
        MmR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Computed value of the `CoreUser` signal as passed on to the SoC. For debugging."]
    #[inline(always)]
    pub fn coreuser(&mut self) -> CoreuserW<'_, StatusSpec> {
        CoreuserW::new(self, 0)
    }
    #[doc = "Bit 8 - Computed value of the `mm` signal passed on to the SoC."]
    #[inline(always)]
    pub fn mm(&mut self) -> MmW<'_, StatusSpec> {
        MmW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
