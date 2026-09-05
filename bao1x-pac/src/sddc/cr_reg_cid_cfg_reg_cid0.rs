#[doc = "Register `CR_REG_CID_CFG_REG_CID0` reader"]
pub type R = crate::R<CrRegCidCfgRegCid0Spec>;
#[doc = "Register `CR_REG_CID_CFG_REG_CID0` writer"]
pub type W = crate::W<CrRegCidCfgRegCid0Spec>;
#[doc = "Field `cfg_reg_cid0` reader - cr_reg_cid read/write control register"]
pub type CfgRegCid0R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_cid0` writer - cr_reg_cid read/write control register"]
pub type CfgRegCid0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_cid read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_cid0(&self) -> CfgRegCid0R {
        CfgRegCid0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_cid read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_cid0(&mut self) -> CfgRegCid0W<'_, CrRegCidCfgRegCid0Spec> {
        CfgRegCid0W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_cid_cfg_reg_cid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_cid_cfg_reg_cid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegCidCfgRegCid0Spec;
impl crate::RegisterSpec for CrRegCidCfgRegCid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_cid_cfg_reg_cid0::R`](R) reader structure"]
impl crate::Readable for CrRegCidCfgRegCid0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_cid_cfg_reg_cid0::W`](W) writer structure"]
impl crate::Writable for CrRegCidCfgRegCid0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_CID_CFG_REG_CID0 to value 0"]
impl crate::Resettable for CrRegCidCfgRegCid0Spec {}
