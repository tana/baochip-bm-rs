#[doc = "Register `REG_I2S_PDM_SETUP` reader"]
pub type R = crate::R<RegI2sPdmSetupSpec>;
#[doc = "Register `REG_I2S_PDM_SETUP` writer"]
pub type W = crate::W<RegI2sPdmSetupSpec>;
#[doc = "Field `r_slave_pdm_shift` reader - r_slave_pdm_shift"]
pub type RSlavePdmShiftR = crate::FieldReader;
#[doc = "Field `r_slave_pdm_shift` writer - r_slave_pdm_shift"]
pub type RSlavePdmShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_slave_pdm_decimation` reader - r_slave_pdm_decimation"]
pub type RSlavePdmDecimationR = crate::FieldReader<u16>;
#[doc = "Field `r_slave_pdm_decimation` writer - r_slave_pdm_decimation"]
pub type RSlavePdmDecimationW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `r_slave_pdm_mode` reader - r_slave_pdm_mode"]
pub type RSlavePdmModeR = crate::FieldReader;
#[doc = "Field `r_slave_pdm_mode` writer - r_slave_pdm_mode"]
pub type RSlavePdmModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_slave_pdm_en` reader - r_slave_pdm_en"]
pub type RSlavePdmEnR = crate::BitReader;
#[doc = "Field `r_slave_pdm_en` writer - r_slave_pdm_en"]
pub type RSlavePdmEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - r_slave_pdm_shift"]
    #[inline(always)]
    pub fn r_slave_pdm_shift(&self) -> RSlavePdmShiftR {
        RSlavePdmShiftR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:12 - r_slave_pdm_decimation"]
    #[inline(always)]
    pub fn r_slave_pdm_decimation(&self) -> RSlavePdmDecimationR {
        RSlavePdmDecimationR::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 13:14 - r_slave_pdm_mode"]
    #[inline(always)]
    pub fn r_slave_pdm_mode(&self) -> RSlavePdmModeR {
        RSlavePdmModeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 31 - r_slave_pdm_en"]
    #[inline(always)]
    pub fn r_slave_pdm_en(&self) -> RSlavePdmEnR {
        RSlavePdmEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - r_slave_pdm_shift"]
    #[inline(always)]
    pub fn r_slave_pdm_shift(&mut self) -> RSlavePdmShiftW<'_, RegI2sPdmSetupSpec> {
        RSlavePdmShiftW::new(self, 0)
    }
    #[doc = "Bits 3:12 - r_slave_pdm_decimation"]
    #[inline(always)]
    pub fn r_slave_pdm_decimation(&mut self) -> RSlavePdmDecimationW<'_, RegI2sPdmSetupSpec> {
        RSlavePdmDecimationW::new(self, 3)
    }
    #[doc = "Bits 13:14 - r_slave_pdm_mode"]
    #[inline(always)]
    pub fn r_slave_pdm_mode(&mut self) -> RSlavePdmModeW<'_, RegI2sPdmSetupSpec> {
        RSlavePdmModeW::new(self, 13)
    }
    #[doc = "Bit 31 - r_slave_pdm_en"]
    #[inline(always)]
    pub fn r_slave_pdm_en(&mut self) -> RSlavePdmEnW<'_, RegI2sPdmSetupSpec> {
        RSlavePdmEnW::new(self, 31)
    }
}
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2s_pdm_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2s_pdm_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegI2sPdmSetupSpec;
impl crate::RegisterSpec for RegI2sPdmSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_i2s_pdm_setup::R`](R) reader structure"]
impl crate::Readable for RegI2sPdmSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_i2s_pdm_setup::W`](W) writer structure"]
impl crate::Writable for RegI2sPdmSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_I2S_PDM_SETUP to value 0"]
impl crate::Resettable for RegI2sPdmSetupSpec {}
