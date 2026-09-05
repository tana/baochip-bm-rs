#[doc = "Register `REG_I2S_MST_SETUP` reader"]
pub type R = crate::R<RegI2sMstSetupSpec>;
#[doc = "Register `REG_I2S_MST_SETUP` writer"]
pub type W = crate::W<RegI2sMstSetupSpec>;
#[doc = "Field `r_master_i2s_words` reader - r_master_i2s_words"]
pub type RMasterI2sWordsR = crate::FieldReader;
#[doc = "Field `r_master_i2s_words` writer - r_master_i2s_words"]
pub type RMasterI2sWordsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_master_i2s_bits_word` reader - r_master_i2s_bits_word"]
pub type RMasterI2sBitsWordR = crate::FieldReader;
#[doc = "Field `r_master_i2s_bits_word` writer - r_master_i2s_bits_word"]
pub type RMasterI2sBitsWordW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_master_i2s_lsb_first` reader - r_master_i2s_lsb_first"]
pub type RMasterI2sLsbFirstR = crate::BitReader;
#[doc = "Field `r_master_i2s_lsb_first` writer - r_master_i2s_lsb_first"]
pub type RMasterI2sLsbFirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_master_i2s_2ch` reader - r_master_i2s_2ch"]
pub type RMasterI2s2chR = crate::BitReader;
#[doc = "Field `r_master_i2s_2ch` writer - r_master_i2s_2ch"]
pub type RMasterI2s2chW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_master_i2s_en` reader - r_master_i2s_en"]
pub type RMasterI2sEnR = crate::BitReader;
#[doc = "Field `r_master_i2s_en` writer - r_master_i2s_en"]
pub type RMasterI2sEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - r_master_i2s_words"]
    #[inline(always)]
    pub fn r_master_i2s_words(&self) -> RMasterI2sWordsR {
        RMasterI2sWordsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - r_master_i2s_bits_word"]
    #[inline(always)]
    pub fn r_master_i2s_bits_word(&self) -> RMasterI2sBitsWordR {
        RMasterI2sBitsWordR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - r_master_i2s_lsb_first"]
    #[inline(always)]
    pub fn r_master_i2s_lsb_first(&self) -> RMasterI2sLsbFirstR {
        RMasterI2sLsbFirstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - r_master_i2s_2ch"]
    #[inline(always)]
    pub fn r_master_i2s_2ch(&self) -> RMasterI2s2chR {
        RMasterI2s2chR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - r_master_i2s_en"]
    #[inline(always)]
    pub fn r_master_i2s_en(&self) -> RMasterI2sEnR {
        RMasterI2sEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - r_master_i2s_words"]
    #[inline(always)]
    pub fn r_master_i2s_words(&mut self) -> RMasterI2sWordsW<'_, RegI2sMstSetupSpec> {
        RMasterI2sWordsW::new(self, 0)
    }
    #[doc = "Bits 8:12 - r_master_i2s_bits_word"]
    #[inline(always)]
    pub fn r_master_i2s_bits_word(&mut self) -> RMasterI2sBitsWordW<'_, RegI2sMstSetupSpec> {
        RMasterI2sBitsWordW::new(self, 8)
    }
    #[doc = "Bit 16 - r_master_i2s_lsb_first"]
    #[inline(always)]
    pub fn r_master_i2s_lsb_first(&mut self) -> RMasterI2sLsbFirstW<'_, RegI2sMstSetupSpec> {
        RMasterI2sLsbFirstW::new(self, 16)
    }
    #[doc = "Bit 17 - r_master_i2s_2ch"]
    #[inline(always)]
    pub fn r_master_i2s_2ch(&mut self) -> RMasterI2s2chW<'_, RegI2sMstSetupSpec> {
        RMasterI2s2chW::new(self, 17)
    }
    #[doc = "Bit 31 - r_master_i2s_en"]
    #[inline(always)]
    pub fn r_master_i2s_en(&mut self) -> RMasterI2sEnW<'_, RegI2sMstSetupSpec> {
        RMasterI2sEnW::new(self, 31)
    }
}
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2s_mst_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2s_mst_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegI2sMstSetupSpec;
impl crate::RegisterSpec for RegI2sMstSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_i2s_mst_setup::R`](R) reader structure"]
impl crate::Readable for RegI2sMstSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_i2s_mst_setup::W`](W) writer structure"]
impl crate::Writable for RegI2sMstSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_I2S_MST_SETUP to value 0"]
impl crate::Resettable for RegI2sMstSetupSpec {}
