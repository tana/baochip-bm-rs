#[doc = "Register `REG_TX_CFG` reader"]
pub type R = crate::R<RegTxCfgSpec>;
#[doc = "Register `REG_TX_CFG` writer"]
pub type W = crate::W<RegTxCfgSpec>;
#[doc = "Field `r_tx_continuous` reader - r_tx_continuous"]
pub type RTxContinuousR = crate::BitReader;
#[doc = "Field `r_tx_continuous` writer - r_tx_continuous"]
pub type RTxContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tx_en` reader - r_tx_en"]
pub type RTxEnR = crate::BitReader;
#[doc = "Field `r_tx_en` writer - r_tx_en"]
pub type RTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tx_clr` reader - r_tx_clr"]
pub type RTxClrR = crate::BitReader;
#[doc = "Field `r_tx_clr` writer - r_tx_clr"]
pub type RTxClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_tx_continuous"]
    #[inline(always)]
    pub fn r_tx_continuous(&self) -> RTxContinuousR {
        RTxContinuousR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - r_tx_en"]
    #[inline(always)]
    pub fn r_tx_en(&self) -> RTxEnR {
        RTxEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - r_tx_clr"]
    #[inline(always)]
    pub fn r_tx_clr(&self) -> RTxClrR {
        RTxClrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_tx_continuous"]
    #[inline(always)]
    pub fn r_tx_continuous(&mut self) -> RTxContinuousW<'_, RegTxCfgSpec> {
        RTxContinuousW::new(self, 0)
    }
    #[doc = "Bit 4 - r_tx_en"]
    #[inline(always)]
    pub fn r_tx_en(&mut self) -> RTxEnW<'_, RegTxCfgSpec> {
        RTxEnW::new(self, 4)
    }
    #[doc = "Bit 6 - r_tx_clr"]
    #[inline(always)]
    pub fn r_tx_clr(&mut self) -> RTxClrW<'_, RegTxCfgSpec> {
        RTxClrW::new(self, 6)
    }
}
#[doc = "See `udma_spis_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_spis_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxCfgSpec;
impl crate::RegisterSpec for RegTxCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tx_cfg::R`](R) reader structure"]
impl crate::Readable for RegTxCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tx_cfg::W`](W) writer structure"]
impl crate::Writable for RegTxCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TX_CFG to value 0"]
impl crate::Resettable for RegTxCfgSpec {}
