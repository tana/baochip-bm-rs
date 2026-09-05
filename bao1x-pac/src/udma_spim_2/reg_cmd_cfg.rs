#[doc = "Register `REG_CMD_CFG` reader"]
pub type R = crate::R<RegCmdCfgSpec>;
#[doc = "Register `REG_CMD_CFG` writer"]
pub type W = crate::W<RegCmdCfgSpec>;
#[doc = "Field `r_cmd_continuous` reader - r_cmd_continuous"]
pub type RCmdContinuousR = crate::BitReader;
#[doc = "Field `r_cmd_continuous` writer - r_cmd_continuous"]
pub type RCmdContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cmd_en` reader - r_cmd_en"]
pub type RCmdEnR = crate::BitReader;
#[doc = "Field `r_cmd_en` writer - r_cmd_en"]
pub type RCmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cmd_clr` reader - r_cmd_clr"]
pub type RCmdClrR = crate::BitReader;
#[doc = "Field `r_cmd_clr` writer - r_cmd_clr"]
pub type RCmdClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_cmd_continuous"]
    #[inline(always)]
    pub fn r_cmd_continuous(&self) -> RCmdContinuousR {
        RCmdContinuousR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - r_cmd_en"]
    #[inline(always)]
    pub fn r_cmd_en(&self) -> RCmdEnR {
        RCmdEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - r_cmd_clr"]
    #[inline(always)]
    pub fn r_cmd_clr(&self) -> RCmdClrR {
        RCmdClrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_cmd_continuous"]
    #[inline(always)]
    pub fn r_cmd_continuous(&mut self) -> RCmdContinuousW<'_, RegCmdCfgSpec> {
        RCmdContinuousW::new(self, 0)
    }
    #[doc = "Bit 4 - r_cmd_en"]
    #[inline(always)]
    pub fn r_cmd_en(&mut self) -> RCmdEnW<'_, RegCmdCfgSpec> {
        RCmdEnW::new(self, 4)
    }
    #[doc = "Bit 6 - r_cmd_clr"]
    #[inline(always)]
    pub fn r_cmd_clr(&mut self) -> RCmdClrW<'_, RegCmdCfgSpec> {
        RCmdClrW::new(self, 6)
    }
}
#[doc = "See `udma_spim_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_qspi/rtl/udma_spim_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmd_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmd_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCmdCfgSpec;
impl crate::RegisterSpec for RegCmdCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cmd_cfg::R`](R) reader structure"]
impl crate::Readable for RegCmdCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cmd_cfg::W`](W) writer structure"]
impl crate::Writable for RegCmdCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CMD_CFG to value 0"]
impl crate::Resettable for RegCmdCfgSpec {}
