#[doc = "Register `CR_REG_CID_CFG_REG_CID2` reader"]
pub type R = crate::R<CrRegCidCfgRegCid2Spec>;
#[doc = "Register `CR_REG_CID_CFG_REG_CID2` writer"]
pub type W = crate::W<CrRegCidCfgRegCid2Spec>;
#[doc = "Field `cfg_reg_cid2` reader - cr_reg_cid read/write control register"]
pub type CfgRegCid2R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_cid2` writer - cr_reg_cid read/write control register"]
pub type CfgRegCid2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_cid read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_cid2(&self) -> CfgRegCid2R {
        CfgRegCid2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_cid read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_cid2(&mut self) -> CfgRegCid2W<'_, CrRegCidCfgRegCid2Spec> {
        CfgRegCid2W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_cid_cfg_reg_cid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_cid_cfg_reg_cid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegCidCfgRegCid2Spec;
impl crate::RegisterSpec for CrRegCidCfgRegCid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_cid_cfg_reg_cid2::R`](R) reader structure"]
impl crate::Readable for CrRegCidCfgRegCid2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_cid_cfg_reg_cid2::W`](W) writer structure"]
impl crate::Writable for CrRegCidCfgRegCid2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_CID_CFG_REG_CID2 to value 0"]
impl crate::Resettable for CrRegCidCfgRegCid2Spec {}
