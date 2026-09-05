#[doc = "Register `CR_XIP_ADDRMODE` reader"]
pub type R = crate::R<CrXipAddrmodeSpec>;
#[doc = "Register `CR_XIP_ADDRMODE` writer"]
pub type W = crate::W<CrXipAddrmodeSpec>;
#[doc = "Field `cr_xip_addrmode` reader - cr_xip_addrmode read/write control register"]
pub type CrXipAddrmodeR = crate::FieldReader;
#[doc = "Field `cr_xip_addrmode` writer - cr_xip_addrmode read/write control register"]
pub type CrXipAddrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - cr_xip_addrmode read/write control register"]
    #[inline(always)]
    pub fn cr_xip_addrmode(&self) -> CrXipAddrmodeR {
        CrXipAddrmodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - cr_xip_addrmode read/write control register"]
    #[inline(always)]
    pub fn cr_xip_addrmode(&mut self) -> CrXipAddrmodeW<'_, CrXipAddrmodeSpec> {
        CrXipAddrmodeW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L193 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L193>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_addrmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_addrmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrXipAddrmodeSpec;
impl crate::RegisterSpec for CrXipAddrmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_xip_addrmode::R`](R) reader structure"]
impl crate::Readable for CrXipAddrmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_xip_addrmode::W`](W) writer structure"]
impl crate::Writable for CrXipAddrmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_XIP_ADDRMODE to value 0"]
impl crate::Resettable for CrXipAddrmodeSpec {}
