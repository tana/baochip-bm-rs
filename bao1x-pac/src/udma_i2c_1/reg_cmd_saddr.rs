#[doc = "Register `REG_CMD_SADDR` reader"]
pub type R = crate::R<RegCmdSaddrSpec>;
#[doc = "Register `REG_CMD_SADDR` writer"]
pub type W = crate::W<RegCmdSaddrSpec>;
#[doc = "Field `r_cmd_startaddr` reader - r_cmd_startaddr"]
pub type RCmdStartaddrR = crate::FieldReader<u16>;
#[doc = "Field `r_cmd_startaddr` writer - r_cmd_startaddr"]
pub type RCmdStartaddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - r_cmd_startaddr"]
    #[inline(always)]
    pub fn r_cmd_startaddr(&self) -> RCmdStartaddrR {
        RCmdStartaddrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - r_cmd_startaddr"]
    #[inline(always)]
    pub fn r_cmd_startaddr(&mut self) -> RCmdStartaddrW<'_, RegCmdSaddrSpec> {
        RCmdStartaddrW::new(self, 0)
    }
}
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmd_saddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmd_saddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCmdSaddrSpec;
impl crate::RegisterSpec for RegCmdSaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cmd_saddr::R`](R) reader structure"]
impl crate::Readable for RegCmdSaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cmd_saddr::W`](W) writer structure"]
impl crate::Writable for RegCmdSaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CMD_SADDR to value 0"]
impl crate::Resettable for RegCmdSaddrSpec {}
