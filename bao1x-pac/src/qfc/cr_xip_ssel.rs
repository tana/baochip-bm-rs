#[doc = "Register `CR_XIP_SSEL` reader"]
pub type R = crate::R<CrXipSselSpec>;
#[doc = "Register `CR_XIP_SSEL` writer"]
pub type W = crate::W<CrXipSselSpec>;
#[doc = "Field `cr_xip_ssel` reader - cr_xip_ssel read/write control register"]
pub type CrXipSselR = crate::FieldReader;
#[doc = "Field `cr_xip_ssel` writer - cr_xip_ssel read/write control register"]
pub type CrXipSselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - cr_xip_ssel read/write control register"]
    #[inline(always)]
    pub fn cr_xip_ssel(&self) -> CrXipSselR {
        CrXipSselR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - cr_xip_ssel read/write control register"]
    #[inline(always)]
    pub fn cr_xip_ssel(&mut self) -> CrXipSselW<'_, CrXipSselSpec> {
        CrXipSselW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L196 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L196>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_ssel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_ssel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrXipSselSpec;
impl crate::RegisterSpec for CrXipSselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_xip_ssel::R`](R) reader structure"]
impl crate::Readable for CrXipSselSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_xip_ssel::W`](W) writer structure"]
impl crate::Writable for CrXipSselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_XIP_SSEL to value 0"]
impl crate::Resettable for CrXipSselSpec {}
