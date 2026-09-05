#[doc = "Register `CR_REV` reader"]
pub type R = crate::R<CrRevSpec>;
#[doc = "Register `CR_REV` writer"]
pub type W = crate::W<CrRevSpec>;
#[doc = "Field `cfg_reg_sd_spec_revision` reader - cfg_reg_sd_spec_revision read/write control register"]
pub type CfgRegSdSpecRevisionR = crate::FieldReader;
#[doc = "Field `cfg_reg_sd_spec_revision` writer - cfg_reg_sd_spec_revision read/write control register"]
pub type CfgRegSdSpecRevisionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cfg_reg_cccr_sdio_revision` reader - cfg_reg_cccr_sdio_revision read/write control register"]
pub type CfgRegCccrSdioRevisionR = crate::FieldReader;
#[doc = "Field `cfg_reg_cccr_sdio_revision` writer - cfg_reg_cccr_sdio_revision read/write control register"]
pub type CfgRegCccrSdioRevisionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_sd_spec_revision read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_spec_revision(&self) -> CfgRegSdSpecRevisionR {
        CfgRegSdSpecRevisionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - cfg_reg_cccr_sdio_revision read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_cccr_sdio_revision(&self) -> CfgRegCccrSdioRevisionR {
        CfgRegCccrSdioRevisionR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_sd_spec_revision read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_spec_revision(&mut self) -> CfgRegSdSpecRevisionW<'_, CrRevSpec> {
        CfgRegSdSpecRevisionW::new(self, 0)
    }
    #[doc = "Bits 8:15 - cfg_reg_cccr_sdio_revision read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_cccr_sdio_revision(&mut self) -> CfgRegCccrSdioRevisionW<'_, CrRevSpec> {
        CfgRegCccrSdioRevisionW::new(self, 8)
    }
}
#[doc = "See `sddc.sv#L118 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L118>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_rev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_rev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRevSpec;
impl crate::RegisterSpec for CrRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_rev::R`](R) reader structure"]
impl crate::Readable for CrRevSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_rev::W`](W) writer structure"]
impl crate::Writable for CrRevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REV to value 0"]
impl crate::Resettable for CrRevSpec {}
