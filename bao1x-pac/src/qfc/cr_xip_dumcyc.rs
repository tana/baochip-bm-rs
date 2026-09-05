#[doc = "Register `CR_XIP_DUMCYC` reader"]
pub type R = crate::R<CrXipDumcycSpec>;
#[doc = "Register `CR_XIP_DUMCYC` writer"]
pub type W = crate::W<CrXipDumcycSpec>;
#[doc = "Field `cr_xip_dumcyc` reader - cr_xip_dumcyc read/write control register"]
pub type CrXipDumcycR = crate::FieldReader<u16>;
#[doc = "Field `cr_xip_dumcyc` writer - cr_xip_dumcyc read/write control register"]
pub type CrXipDumcycW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_xip_dumcyc read/write control register"]
    #[inline(always)]
    pub fn cr_xip_dumcyc(&self) -> CrXipDumcycR {
        CrXipDumcycR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_xip_dumcyc read/write control register"]
    #[inline(always)]
    pub fn cr_xip_dumcyc(&mut self) -> CrXipDumcycW<'_, CrXipDumcycSpec> {
        CrXipDumcycW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L197 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L197>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_dumcyc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_dumcyc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrXipDumcycSpec;
impl crate::RegisterSpec for CrXipDumcycSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_xip_dumcyc::R`](R) reader structure"]
impl crate::Readable for CrXipDumcycSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_xip_dumcyc::W`](W) writer structure"]
impl crate::Writable for CrXipDumcycSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_XIP_DUMCYC to value 0"]
impl crate::Resettable for CrXipDumcycSpec {}
