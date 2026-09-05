#[doc = "Register `CR_WRITE_PROTECT` reader"]
pub type R = crate::R<CrWriteProtectSpec>;
#[doc = "Register `CR_WRITE_PROTECT` writer"]
pub type W = crate::W<CrWriteProtectSpec>;
#[doc = "Field `cr_write_protect` reader - cr_write_protect read/write control register"]
pub type CrWriteProtectR = crate::BitReader;
#[doc = "Field `cr_write_protect` writer - cr_write_protect read/write control register"]
pub type CrWriteProtectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - cr_write_protect read/write control register"]
    #[inline(always)]
    pub fn cr_write_protect(&self) -> CrWriteProtectR {
        CrWriteProtectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cr_write_protect read/write control register"]
    #[inline(always)]
    pub fn cr_write_protect(&mut self) -> CrWriteProtectW<'_, CrWriteProtectSpec> {
        CrWriteProtectW::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L134 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L134>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_write_protect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_write_protect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrWriteProtectSpec;
impl crate::RegisterSpec for CrWriteProtectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_write_protect::R`](R) reader structure"]
impl crate::Readable for CrWriteProtectSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_write_protect::W`](W) writer structure"]
impl crate::Writable for CrWriteProtectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_WRITE_PROTECT to value 0"]
impl crate::Resettable for CrWriteProtectSpec {}
