#[doc = "Register `CR_REG_CSD_CFG_REG_CSD3` reader"]
pub type R = crate::R<CrRegCsdCfgRegCsd3Spec>;
#[doc = "Register `CR_REG_CSD_CFG_REG_CSD3` writer"]
pub type W = crate::W<CrRegCsdCfgRegCsd3Spec>;
#[doc = "Field `cfg_reg_csd3` reader - cr_reg_csd read/write control register"]
pub type CfgRegCsd3R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_csd3` writer - cr_reg_csd read/write control register"]
pub type CfgRegCsd3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_csd read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_csd3(&self) -> CfgRegCsd3R {
        CfgRegCsd3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_csd read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_csd3(&mut self) -> CfgRegCsd3W<'_, CrRegCsdCfgRegCsd3Spec> {
        CfgRegCsd3W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_csd_cfg_reg_csd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_csd_cfg_reg_csd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegCsdCfgRegCsd3Spec;
impl crate::RegisterSpec for CrRegCsdCfgRegCsd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_csd_cfg_reg_csd3::R`](R) reader structure"]
impl crate::Readable for CrRegCsdCfgRegCsd3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_csd_cfg_reg_csd3::W`](W) writer structure"]
impl crate::Writable for CrRegCsdCfgRegCsd3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_CSD_CFG_REG_CSD3 to value 0"]
impl crate::Resettable for CrRegCsdCfgRegCsd3Spec {}
