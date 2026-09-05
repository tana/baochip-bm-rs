#[doc = "Register `REG_SPIS_SETUP` reader"]
pub type R = crate::R<RegSpisSetupSpec>;
#[doc = "Register `REG_SPIS_SETUP` writer"]
pub type W = crate::W<RegSpisSetupSpec>;
#[doc = "Field `cfgcpol` reader - cfgcpol"]
pub type CfgcpolR = crate::BitReader;
#[doc = "Field `cfgcpol` writer - cfgcpol"]
pub type CfgcpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfgcpha` reader - cfgcpha"]
pub type CfgcphaR = crate::BitReader;
#[doc = "Field `cfgcpha` writer - cfgcpha"]
pub type CfgcphaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - cfgcpol"]
    #[inline(always)]
    pub fn cfgcpol(&self) -> CfgcpolR {
        CfgcpolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cfgcpha"]
    #[inline(always)]
    pub fn cfgcpha(&self) -> CfgcphaR {
        CfgcphaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cfgcpol"]
    #[inline(always)]
    pub fn cfgcpol(&mut self) -> CfgcpolW<'_, RegSpisSetupSpec> {
        CfgcpolW::new(self, 0)
    }
    #[doc = "Bit 1 - cfgcpha"]
    #[inline(always)]
    pub fn cfgcpha(&mut self) -> CfgcphaW<'_, RegSpisSetupSpec> {
        CfgcphaW::new(self, 1)
    }
}
#[doc = "See `udma_spis_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_spis_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_spis_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_spis_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpisSetupSpec;
impl crate::RegisterSpec for RegSpisSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_spis_setup::R`](R) reader structure"]
impl crate::Readable for RegSpisSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_spis_setup::W`](W) writer structure"]
impl crate::Writable for RegSpisSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SPIS_SETUP to value 0"]
impl crate::Resettable for RegSpisSetupSpec {}
