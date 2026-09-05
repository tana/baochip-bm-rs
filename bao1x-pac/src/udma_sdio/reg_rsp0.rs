#[doc = "Register `REG_RSP0` reader"]
pub type R = crate::R<RegRsp0Spec>;
#[doc = "Register `REG_RSP0` writer"]
pub type W = crate::W<RegRsp0Spec>;
#[doc = "Field `cfg_rsp_data_i_31_0` reader - cfg_rsp_data_i_31_0"]
pub type CfgRspDataI31_0R = crate::FieldReader<u32>;
#[doc = "Field `cfg_rsp_data_i_31_0` writer - cfg_rsp_data_i_31_0"]
pub type CfgRspDataI31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cfg_rsp_data_i_31_0"]
    #[inline(always)]
    pub fn cfg_rsp_data_i_31_0(&self) -> CfgRspDataI31_0R {
        CfgRspDataI31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cfg_rsp_data_i_31_0"]
    #[inline(always)]
    pub fn cfg_rsp_data_i_31_0(&mut self) -> CfgRspDataI31_0W<'_, RegRsp0Spec> {
        CfgRspDataI31_0W::new(self, 0)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRsp0Spec;
impl crate::RegisterSpec for RegRsp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rsp0::R`](R) reader structure"]
impl crate::Readable for RegRsp0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rsp0::W`](W) writer structure"]
impl crate::Writable for RegRsp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RSP0 to value 0"]
impl crate::Resettable for RegRsp0Spec {}
