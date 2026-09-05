#[doc = "Register `REG_RSP3` reader"]
pub type R = crate::R<RegRsp3Spec>;
#[doc = "Register `REG_RSP3` writer"]
pub type W = crate::W<RegRsp3Spec>;
#[doc = "Field `cfg_rsp_data_i_127_96` reader - cfg_rsp_data_i_127_96"]
pub type CfgRspDataI127_96R = crate::FieldReader<u32>;
#[doc = "Field `cfg_rsp_data_i_127_96` writer - cfg_rsp_data_i_127_96"]
pub type CfgRspDataI127_96W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cfg_rsp_data_i_127_96"]
    #[inline(always)]
    pub fn cfg_rsp_data_i_127_96(&self) -> CfgRspDataI127_96R {
        CfgRspDataI127_96R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cfg_rsp_data_i_127_96"]
    #[inline(always)]
    pub fn cfg_rsp_data_i_127_96(&mut self) -> CfgRspDataI127_96W<'_, RegRsp3Spec> {
        CfgRspDataI127_96W::new(self, 0)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRsp3Spec;
impl crate::RegisterSpec for RegRsp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rsp3::R`](R) reader structure"]
impl crate::Readable for RegRsp3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rsp3::W`](W) writer structure"]
impl crate::Writable for RegRsp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RSP3 to value 0"]
impl crate::Resettable for RegRsp3Spec {}
