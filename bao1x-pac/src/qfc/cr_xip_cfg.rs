#[doc = "Register `CR_XIP_CFG` reader"]
pub type R = crate::R<CrXipCfgSpec>;
#[doc = "Register `CR_XIP_CFG` writer"]
pub type W = crate::W<CrXipCfgSpec>;
#[doc = "Field `cr_xip_cfg` reader - cr_xip_cfg read/write control register"]
pub type CrXipCfgR = crate::FieldReader<u16>;
#[doc = "Field `cr_xip_cfg` writer - cr_xip_cfg read/write control register"]
pub type CrXipCfgW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - cr_xip_cfg read/write control register"]
    #[inline(always)]
    pub fn cr_xip_cfg(&self) -> CrXipCfgR {
        CrXipCfgR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - cr_xip_cfg read/write control register"]
    #[inline(always)]
    pub fn cr_xip_cfg(&mut self) -> CrXipCfgW<'_, CrXipCfgSpec> {
        CrXipCfgW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L198 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L198>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrXipCfgSpec;
impl crate::RegisterSpec for CrXipCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_xip_cfg::R`](R) reader structure"]
impl crate::Readable for CrXipCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_xip_cfg::W`](W) writer structure"]
impl crate::Writable for CrXipCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_XIP_CFG to value 0"]
impl crate::Resettable for CrXipCfgSpec {}
