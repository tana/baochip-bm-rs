#[doc = "Register `REG_CMD_SIZE` reader"]
pub type R = crate::R<RegCmdSizeSpec>;
#[doc = "Register `REG_CMD_SIZE` writer"]
pub type W = crate::W<RegCmdSizeSpec>;
#[doc = "Field `r_cmd_size` reader - r_cmd_size"]
pub type RCmdSizeR = crate::FieldReader<u16>;
#[doc = "Field `r_cmd_size` writer - r_cmd_size"]
pub type RCmdSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - r_cmd_size"]
    #[inline(always)]
    pub fn r_cmd_size(&self) -> RCmdSizeR {
        RCmdSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_cmd_size"]
    #[inline(always)]
    pub fn r_cmd_size(&mut self) -> RCmdSizeW<'_, RegCmdSizeSpec> {
        RCmdSizeW::new(self, 0)
    }
}
#[doc = "See `udma_spim_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_qspi/rtl/udma_spim_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmd_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmd_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCmdSizeSpec;
impl crate::RegisterSpec for RegCmdSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cmd_size::R`](R) reader structure"]
impl crate::Readable for RegCmdSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cmd_size::W`](W) writer structure"]
impl crate::Writable for RegCmdSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CMD_SIZE to value 0"]
impl crate::Resettable for RegCmdSizeSpec {}
