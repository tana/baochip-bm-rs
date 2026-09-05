#[doc = "Register `CR_REG_CSD_CFG_REG_CSD2` reader"]
pub type R = crate::R<CrRegCsdCfgRegCsd2Spec>;
#[doc = "Register `CR_REG_CSD_CFG_REG_CSD2` writer"]
pub type W = crate::W<CrRegCsdCfgRegCsd2Spec>;
#[doc = "Field `cfg_reg_csd2` reader - cr_reg_csd read/write control register"]
pub type CfgRegCsd2R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_csd2` writer - cr_reg_csd read/write control register"]
pub type CfgRegCsd2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_csd read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_csd2(&self) -> CfgRegCsd2R {
        CfgRegCsd2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_csd read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_csd2(&mut self) -> CfgRegCsd2W<'_, CrRegCsdCfgRegCsd2Spec> {
        CfgRegCsd2W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_csd_cfg_reg_csd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_csd_cfg_reg_csd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegCsdCfgRegCsd2Spec;
impl crate::RegisterSpec for CrRegCsdCfgRegCsd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_csd_cfg_reg_csd2::R`](R) reader structure"]
impl crate::Readable for CrRegCsdCfgRegCsd2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_csd_cfg_reg_csd2::W`](W) writer structure"]
impl crate::Writable for CrRegCsdCfgRegCsd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_CSD_CFG_REG_CSD2 to value 0"]
impl crate::Resettable for CrRegCsdCfgRegCsd2Spec {}
