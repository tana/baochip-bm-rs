#[doc = "Register `PROTECT` reader"]
pub type R = crate::R<ProtectSpec>;
#[doc = "Register `PROTECT` writer"]
pub type W = crate::W<ProtectSpec>;
#[doc = "Field `protect` reader - "]
pub type ProtectR = crate::BitReader;
#[doc = "Field `protect` writer - "]
pub type ProtectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<'_, ProtectSpec> {
        ProtectW::new(self, 0)
    }
}
#[doc = "Writing `1` to this bit prevents any further updates to CoreUser configuration status. Can only be reversed with a system reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`protect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProtectSpec;
impl crate::RegisterSpec for ProtectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`protect::R`](R) reader structure"]
impl crate::Readable for ProtectSpec {}
#[doc = "`write(|w| ..)` method takes [`protect::W`](W) writer structure"]
impl crate::Writable for ProtectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PROTECT to value 0"]
impl crate::Resettable for ProtectSpec {}
