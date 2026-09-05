#[doc = "Register `REG_RSP1` reader"]
pub type R = crate::R<RegRsp1Spec>;
#[doc = "Register `REG_RSP1` writer"]
pub type W = crate::W<RegRsp1Spec>;
#[doc = "Field `cfg_rsp_data_i_63_32` reader - cfg_rsp_data_i_63_32"]
pub type CfgRspDataI63_32R = crate::FieldReader<u32>;
#[doc = "Field `cfg_rsp_data_i_63_32` writer - cfg_rsp_data_i_63_32"]
pub type CfgRspDataI63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cfg_rsp_data_i_63_32"]
    #[inline(always)]
    pub fn cfg_rsp_data_i_63_32(&self) -> CfgRspDataI63_32R {
        CfgRspDataI63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cfg_rsp_data_i_63_32"]
    #[inline(always)]
    pub fn cfg_rsp_data_i_63_32(&mut self) -> CfgRspDataI63_32W<'_, RegRsp1Spec> {
        CfgRspDataI63_32W::new(self, 0)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRsp1Spec;
impl crate::RegisterSpec for RegRsp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rsp1::R`](R) reader structure"]
impl crate::Readable for RegRsp1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rsp1::W`](W) writer structure"]
impl crate::Writable for RegRsp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RSP1 to value 0"]
impl crate::Resettable for RegRsp1Spec {}
