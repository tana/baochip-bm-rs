#[doc = "Register `REG_AU_CFG` reader"]
pub type R = crate::R<RegAuCfgSpec>;
#[doc = "Register `REG_AU_CFG` writer"]
pub type W = crate::W<RegAuCfgSpec>;
#[doc = "Field `r_au_use_signed` reader - r_au_use_signed"]
pub type RAuUseSignedR = crate::BitReader;
#[doc = "Field `r_au_use_signed` writer - r_au_use_signed"]
pub type RAuUseSignedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_au_bypass` reader - r_au_bypass"]
pub type RAuBypassR = crate::BitReader;
#[doc = "Field `r_au_bypass` writer - r_au_bypass"]
pub type RAuBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_au_mode` reader - r_au_mode"]
pub type RAuModeR = crate::FieldReader;
#[doc = "Field `r_au_mode` writer - r_au_mode"]
pub type RAuModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_au_shift` reader - r_au_shift"]
pub type RAuShiftR = crate::FieldReader;
#[doc = "Field `r_au_shift` writer - r_au_shift"]
pub type RAuShiftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - r_au_use_signed"]
    #[inline(always)]
    pub fn r_au_use_signed(&self) -> RAuUseSignedR {
        RAuUseSignedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_au_bypass"]
    #[inline(always)]
    pub fn r_au_bypass(&self) -> RAuBypassR {
        RAuBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - r_au_mode"]
    #[inline(always)]
    pub fn r_au_mode(&self) -> RAuModeR {
        RAuModeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - r_au_shift"]
    #[inline(always)]
    pub fn r_au_shift(&self) -> RAuShiftR {
        RAuShiftR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - r_au_use_signed"]
    #[inline(always)]
    pub fn r_au_use_signed(&mut self) -> RAuUseSignedW<'_, RegAuCfgSpec> {
        RAuUseSignedW::new(self, 0)
    }
    #[doc = "Bit 1 - r_au_bypass"]
    #[inline(always)]
    pub fn r_au_bypass(&mut self) -> RAuBypassW<'_, RegAuCfgSpec> {
        RAuBypassW::new(self, 1)
    }
    #[doc = "Bits 8:11 - r_au_mode"]
    #[inline(always)]
    pub fn r_au_mode(&mut self) -> RAuModeW<'_, RegAuCfgSpec> {
        RAuModeW::new(self, 8)
    }
    #[doc = "Bits 16:20 - r_au_shift"]
    #[inline(always)]
    pub fn r_au_shift(&mut self) -> RAuShiftW<'_, RegAuCfgSpec> {
        RAuShiftW::new(self, 16)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_au_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_au_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAuCfgSpec;
impl crate::RegisterSpec for RegAuCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_au_cfg::R`](R) reader structure"]
impl crate::Readable for RegAuCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_au_cfg::W`](W) writer structure"]
impl crate::Writable for RegAuCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AU_CFG to value 0"]
impl crate::Resettable for RegAuCfgSpec {}
