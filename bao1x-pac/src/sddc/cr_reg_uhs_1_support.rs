#[doc = "Register `CR_REG_UHS_1_SUPPORT` reader"]
pub type R = crate::R<CrRegUhs1SupportSpec>;
#[doc = "Register `CR_REG_UHS_1_SUPPORT` writer"]
pub type W = crate::W<CrRegUhs1SupportSpec>;
#[doc = "Field `cfg_reg_max_current` reader - cfg_reg_max_current read/write control register"]
pub type CfgRegMaxCurrentR = crate::FieldReader<u16>;
#[doc = "Field `cfg_reg_max_current` writer - cfg_reg_max_current read/write control register"]
pub type CfgRegMaxCurrentW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cfg_reg_data_strc_version` reader - cfg_reg_data_strc_version read/write control register"]
pub type CfgRegDataStrcVersionR = crate::FieldReader;
#[doc = "Field `cfg_reg_data_strc_version` writer - cfg_reg_data_strc_version read/write control register"]
pub type CfgRegDataStrcVersionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cfg_reg_uhs_1_support` reader - cfg_reg_uhs_1_support read/write control register"]
pub type CfgRegUhs1SupportR = crate::FieldReader;
#[doc = "Field `cfg_reg_uhs_1_support` writer - cfg_reg_uhs_1_support read/write control register"]
pub type CfgRegUhs1SupportW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - cfg_reg_max_current read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_max_current(&self) -> CfgRegMaxCurrentR {
        CfgRegMaxCurrentR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - cfg_reg_data_strc_version read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_data_strc_version(&self) -> CfgRegDataStrcVersionR {
        CfgRegDataStrcVersionR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - cfg_reg_uhs_1_support read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_uhs_1_support(&self) -> CfgRegUhs1SupportR {
        CfgRegUhs1SupportR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfg_reg_max_current read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_max_current(&mut self) -> CfgRegMaxCurrentW<'_, CrRegUhs1SupportSpec> {
        CfgRegMaxCurrentW::new(self, 0)
    }
    #[doc = "Bits 16:23 - cfg_reg_data_strc_version read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_data_strc_version(
        &mut self,
    ) -> CfgRegDataStrcVersionW<'_, CrRegUhs1SupportSpec> {
        CfgRegDataStrcVersionW::new(self, 16)
    }
    #[doc = "Bits 24:31 - cfg_reg_uhs_1_support read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_uhs_1_support(&mut self) -> CfgRegUhs1SupportW<'_, CrRegUhs1SupportSpec> {
        CfgRegUhs1SupportW::new(self, 24)
    }
}
#[doc = "See `sddc.sv#L159 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L159>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_uhs_1_support::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_uhs_1_support::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegUhs1SupportSpec;
impl crate::RegisterSpec for CrRegUhs1SupportSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_uhs_1_support::R`](R) reader structure"]
impl crate::Readable for CrRegUhs1SupportSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_uhs_1_support::W`](W) writer structure"]
impl crate::Writable for CrRegUhs1SupportSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_UHS_1_SUPPORT to value 0"]
impl crate::Resettable for CrRegUhs1SupportSpec {}
