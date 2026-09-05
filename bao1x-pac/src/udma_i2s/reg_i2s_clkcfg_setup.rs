#[doc = "Register `REG_I2S_CLKCFG_SETUP` reader"]
pub type R = crate::R<RegI2sClkcfgSetupSpec>;
#[doc = "Register `REG_I2S_CLKCFG_SETUP` writer"]
pub type W = crate::W<RegI2sClkcfgSetupSpec>;
#[doc = "Field `r_master_gen_clk_div` reader - r_master_gen_clk_div"]
pub type RMasterGenClkDivR = crate::FieldReader;
#[doc = "Field `r_master_gen_clk_div` writer - r_master_gen_clk_div"]
pub type RMasterGenClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_slave_gen_clk_div` reader - r_slave_gen_clk_div"]
pub type RSlaveGenClkDivR = crate::FieldReader;
#[doc = "Field `r_slave_gen_clk_div` writer - r_slave_gen_clk_div"]
pub type RSlaveGenClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_common_gen_clk_div` reader - r_common_gen_clk_div"]
pub type RCommonGenClkDivR = crate::FieldReader;
#[doc = "Field `r_common_gen_clk_div` writer - r_common_gen_clk_div"]
pub type RCommonGenClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_slave_clk_en` reader - r_slave_clk_en"]
pub type RSlaveClkEnR = crate::BitReader;
#[doc = "Field `r_slave_clk_en` writer - r_slave_clk_en"]
pub type RSlaveClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_master_clk_en` reader - r_master_clk_en"]
pub type RMasterClkEnR = crate::BitReader;
#[doc = "Field `r_master_clk_en` writer - r_master_clk_en"]
pub type RMasterClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pdm_clk_en` reader - r_pdm_clk_en"]
pub type RPdmClkEnR = crate::BitReader;
#[doc = "Field `r_pdm_clk_en` writer - r_pdm_clk_en"]
pub type RPdmClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_slave_sel_ext` reader - r_slave_sel_ext"]
pub type RSlaveSelExtR = crate::BitReader;
#[doc = "Field `r_slave_sel_ext` writer - r_slave_sel_ext"]
pub type RSlaveSelExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_slave_sel_num` reader - r_slave_sel_num"]
pub type RSlaveSelNumR = crate::BitReader;
#[doc = "Field `r_slave_sel_num` writer - r_slave_sel_num"]
pub type RSlaveSelNumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_master_sel_ext` reader - r_master_sel_ext"]
pub type RMasterSelExtR = crate::BitReader;
#[doc = "Field `r_master_sel_ext` writer - r_master_sel_ext"]
pub type RMasterSelExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_master_sel_num` reader - r_master_sel_num"]
pub type RMasterSelNumR = crate::BitReader;
#[doc = "Field `r_master_sel_num` writer - r_master_sel_num"]
pub type RMasterSelNumW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - r_master_gen_clk_div"]
    #[inline(always)]
    pub fn r_master_gen_clk_div(&self) -> RMasterGenClkDivR {
        RMasterGenClkDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - r_slave_gen_clk_div"]
    #[inline(always)]
    pub fn r_slave_gen_clk_div(&self) -> RSlaveGenClkDivR {
        RSlaveGenClkDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - r_common_gen_clk_div"]
    #[inline(always)]
    pub fn r_common_gen_clk_div(&self) -> RCommonGenClkDivR {
        RCommonGenClkDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - r_slave_clk_en"]
    #[inline(always)]
    pub fn r_slave_clk_en(&self) -> RSlaveClkEnR {
        RSlaveClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - r_master_clk_en"]
    #[inline(always)]
    pub fn r_master_clk_en(&self) -> RMasterClkEnR {
        RMasterClkEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - r_pdm_clk_en"]
    #[inline(always)]
    pub fn r_pdm_clk_en(&self) -> RPdmClkEnR {
        RPdmClkEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - r_slave_sel_ext"]
    #[inline(always)]
    pub fn r_slave_sel_ext(&self) -> RSlaveSelExtR {
        RSlaveSelExtR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - r_slave_sel_num"]
    #[inline(always)]
    pub fn r_slave_sel_num(&self) -> RSlaveSelNumR {
        RSlaveSelNumR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - r_master_sel_ext"]
    #[inline(always)]
    pub fn r_master_sel_ext(&self) -> RMasterSelExtR {
        RMasterSelExtR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - r_master_sel_num"]
    #[inline(always)]
    pub fn r_master_sel_num(&self) -> RMasterSelNumR {
        RMasterSelNumR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - r_master_gen_clk_div"]
    #[inline(always)]
    pub fn r_master_gen_clk_div(&mut self) -> RMasterGenClkDivW<'_, RegI2sClkcfgSetupSpec> {
        RMasterGenClkDivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - r_slave_gen_clk_div"]
    #[inline(always)]
    pub fn r_slave_gen_clk_div(&mut self) -> RSlaveGenClkDivW<'_, RegI2sClkcfgSetupSpec> {
        RSlaveGenClkDivW::new(self, 8)
    }
    #[doc = "Bits 16:23 - r_common_gen_clk_div"]
    #[inline(always)]
    pub fn r_common_gen_clk_div(&mut self) -> RCommonGenClkDivW<'_, RegI2sClkcfgSetupSpec> {
        RCommonGenClkDivW::new(self, 16)
    }
    #[doc = "Bit 24 - r_slave_clk_en"]
    #[inline(always)]
    pub fn r_slave_clk_en(&mut self) -> RSlaveClkEnW<'_, RegI2sClkcfgSetupSpec> {
        RSlaveClkEnW::new(self, 24)
    }
    #[doc = "Bit 25 - r_master_clk_en"]
    #[inline(always)]
    pub fn r_master_clk_en(&mut self) -> RMasterClkEnW<'_, RegI2sClkcfgSetupSpec> {
        RMasterClkEnW::new(self, 25)
    }
    #[doc = "Bit 26 - r_pdm_clk_en"]
    #[inline(always)]
    pub fn r_pdm_clk_en(&mut self) -> RPdmClkEnW<'_, RegI2sClkcfgSetupSpec> {
        RPdmClkEnW::new(self, 26)
    }
    #[doc = "Bit 28 - r_slave_sel_ext"]
    #[inline(always)]
    pub fn r_slave_sel_ext(&mut self) -> RSlaveSelExtW<'_, RegI2sClkcfgSetupSpec> {
        RSlaveSelExtW::new(self, 28)
    }
    #[doc = "Bit 29 - r_slave_sel_num"]
    #[inline(always)]
    pub fn r_slave_sel_num(&mut self) -> RSlaveSelNumW<'_, RegI2sClkcfgSetupSpec> {
        RSlaveSelNumW::new(self, 29)
    }
    #[doc = "Bit 30 - r_master_sel_ext"]
    #[inline(always)]
    pub fn r_master_sel_ext(&mut self) -> RMasterSelExtW<'_, RegI2sClkcfgSetupSpec> {
        RMasterSelExtW::new(self, 30)
    }
    #[doc = "Bit 31 - r_master_sel_num"]
    #[inline(always)]
    pub fn r_master_sel_num(&mut self) -> RMasterSelNumW<'_, RegI2sClkcfgSetupSpec> {
        RMasterSelNumW::new(self, 31)
    }
}
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2s_clkcfg_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2s_clkcfg_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegI2sClkcfgSetupSpec;
impl crate::RegisterSpec for RegI2sClkcfgSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_i2s_clkcfg_setup::R`](R) reader structure"]
impl crate::Readable for RegI2sClkcfgSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_i2s_clkcfg_setup::W`](W) writer structure"]
impl crate::Writable for RegI2sClkcfgSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_I2S_CLKCFG_SETUP to value 0"]
impl crate::Resettable for RegI2sClkcfgSetupSpec {}
