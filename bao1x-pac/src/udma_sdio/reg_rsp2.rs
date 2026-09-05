#[doc = "Register `REG_RSP2` reader"]
pub type R = crate::R<RegRsp2Spec>;
#[doc = "Register `REG_RSP2` writer"]
pub type W = crate::W<RegRsp2Spec>;
#[doc = "Field `cfg_rsp_data_i_95_64` reader - cfg_rsp_data_i_95_64"]
pub type CfgRspDataI95_64R = crate::FieldReader<u32>;
#[doc = "Field `cfg_rsp_data_i_95_64` writer - cfg_rsp_data_i_95_64"]
pub type CfgRspDataI95_64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cfg_rsp_data_i_95_64"]
    #[inline(always)]
    pub fn cfg_rsp_data_i_95_64(&self) -> CfgRspDataI95_64R {
        CfgRspDataI95_64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cfg_rsp_data_i_95_64"]
    #[inline(always)]
    pub fn cfg_rsp_data_i_95_64(&mut self) -> CfgRspDataI95_64W<'_, RegRsp2Spec> {
        CfgRspDataI95_64W::new(self, 0)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRsp2Spec;
impl crate::RegisterSpec for RegRsp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rsp2::R`](R) reader structure"]
impl crate::Readable for RegRsp2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rsp2::W`](W) writer structure"]
impl crate::Writable for RegRsp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RSP2 to value 0"]
impl crate::Resettable for RegRsp2Spec {}
