#[doc = "Register `CR_XIP_WIDTH` reader"]
pub type R = crate::R<CrXipWidthSpec>;
#[doc = "Register `CR_XIP_WIDTH` writer"]
pub type W = crate::W<CrXipWidthSpec>;
#[doc = "Field `cr_xip_width` reader - cr_xip_width read/write control register"]
pub type CrXipWidthR = crate::FieldReader;
#[doc = "Field `cr_xip_width` writer - cr_xip_width read/write control register"]
pub type CrXipWidthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - cr_xip_width read/write control register"]
    #[inline(always)]
    pub fn cr_xip_width(&self) -> CrXipWidthR {
        CrXipWidthR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - cr_xip_width read/write control register"]
    #[inline(always)]
    pub fn cr_xip_width(&mut self) -> CrXipWidthW<'_, CrXipWidthSpec> {
        CrXipWidthW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L195 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L195>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrXipWidthSpec;
impl crate::RegisterSpec for CrXipWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_xip_width::R`](R) reader structure"]
impl crate::Readable for CrXipWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_xip_width::W`](W) writer structure"]
impl crate::Writable for CrXipWidthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_XIP_WIDTH to value 0"]
impl crate::Resettable for CrXipWidthSpec {}
