#[doc = "Register `CR_AESENA` reader"]
pub type R = crate::R<CrAesenaSpec>;
#[doc = "Register `CR_AESENA` writer"]
pub type W = crate::W<CrAesenaSpec>;
#[doc = "Field `cr_aesena` reader - cr_aesena read/write control register"]
pub type CrAesenaR = crate::BitReader;
#[doc = "Field `cr_aesena` writer - cr_aesena read/write control register"]
pub type CrAesenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - cr_aesena read/write control register"]
    #[inline(always)]
    pub fn cr_aesena(&self) -> CrAesenaR {
        CrAesenaR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cr_aesena read/write control register"]
    #[inline(always)]
    pub fn cr_aesena(&mut self) -> CrAesenaW<'_, CrAesenaSpec> {
        CrAesenaW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L201 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L201>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_aesena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_aesena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrAesenaSpec;
impl crate::RegisterSpec for CrAesenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_aesena::R`](R) reader structure"]
impl crate::Readable for CrAesenaSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_aesena::W`](W) writer structure"]
impl crate::Writable for CrAesenaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_AESENA to value 0"]
impl crate::Resettable for CrAesenaSpec {}
