#[doc = "Register `REG_CMD_OP` reader"]
pub type R = crate::R<RegCmdOpSpec>;
#[doc = "Register `REG_CMD_OP` writer"]
pub type W = crate::W<RegCmdOpSpec>;
#[doc = "Field `r_cmd_rsp_type` reader - r_cmd_rsp_type"]
pub type RCmdRspTypeR = crate::FieldReader;
#[doc = "Field `r_cmd_rsp_type` writer - r_cmd_rsp_type"]
pub type RCmdRspTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_cmd_op` reader - r_cmd_op"]
pub type RCmdOpR = crate::FieldReader;
#[doc = "Field `r_cmd_op` writer - r_cmd_op"]
pub type RCmdOpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_cmd_stopopt` reader - r_cmd_stopopt"]
pub type RCmdStopoptR = crate::FieldReader;
#[doc = "Field `r_cmd_stopopt` writer - r_cmd_stopopt"]
pub type RCmdStopoptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - r_cmd_rsp_type"]
    #[inline(always)]
    pub fn r_cmd_rsp_type(&self) -> RCmdRspTypeR {
        RCmdRspTypeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:13 - r_cmd_op"]
    #[inline(always)]
    pub fn r_cmd_op(&self) -> RCmdOpR {
        RCmdOpR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - r_cmd_stopopt"]
    #[inline(always)]
    pub fn r_cmd_stopopt(&self) -> RCmdStopoptR {
        RCmdStopoptR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - r_cmd_rsp_type"]
    #[inline(always)]
    pub fn r_cmd_rsp_type(&mut self) -> RCmdRspTypeW<'_, RegCmdOpSpec> {
        RCmdRspTypeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - r_cmd_op"]
    #[inline(always)]
    pub fn r_cmd_op(&mut self) -> RCmdOpW<'_, RegCmdOpSpec> {
        RCmdOpW::new(self, 8)
    }
    #[doc = "Bits 16:17 - r_cmd_stopopt"]
    #[inline(always)]
    pub fn r_cmd_stopopt(&mut self) -> RCmdStopoptW<'_, RegCmdOpSpec> {
        RCmdStopoptW::new(self, 16)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmd_op::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmd_op::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCmdOpSpec;
impl crate::RegisterSpec for RegCmdOpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cmd_op::R`](R) reader structure"]
impl crate::Readable for RegCmdOpSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cmd_op::W`](W) writer structure"]
impl crate::Writable for RegCmdOpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CMD_OP to value 0"]
impl crate::Resettable for RegCmdOpSpec {}
