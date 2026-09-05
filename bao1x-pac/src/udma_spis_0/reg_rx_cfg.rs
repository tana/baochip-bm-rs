#[doc = "Register `REG_RX_CFG` reader"]
pub type R = crate::R<RegRxCfgSpec>;
#[doc = "Register `REG_RX_CFG` writer"]
pub type W = crate::W<RegRxCfgSpec>;
#[doc = "Field `r_rx_continuous` reader - r_rx_continuous"]
pub type RRxContinuousR = crate::BitReader;
#[doc = "Field `r_rx_continuous` writer - r_rx_continuous"]
pub type RRxContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rx_en` reader - r_rx_en"]
pub type RRxEnR = crate::BitReader;
#[doc = "Field `r_rx_en` writer - r_rx_en"]
pub type RRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rx_clr` reader - r_rx_clr"]
pub type RRxClrR = crate::BitReader;
#[doc = "Field `r_rx_clr` writer - r_rx_clr"]
pub type RRxClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_rx_continuous"]
    #[inline(always)]
    pub fn r_rx_continuous(&self) -> RRxContinuousR {
        RRxContinuousR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - r_rx_en"]
    #[inline(always)]
    pub fn r_rx_en(&self) -> RRxEnR {
        RRxEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - r_rx_clr"]
    #[inline(always)]
    pub fn r_rx_clr(&self) -> RRxClrR {
        RRxClrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_rx_continuous"]
    #[inline(always)]
    pub fn r_rx_continuous(&mut self) -> RRxContinuousW<'_, RegRxCfgSpec> {
        RRxContinuousW::new(self, 0)
    }
    #[doc = "Bit 4 - r_rx_en"]
    #[inline(always)]
    pub fn r_rx_en(&mut self) -> RRxEnW<'_, RegRxCfgSpec> {
        RRxEnW::new(self, 4)
    }
    #[doc = "Bit 6 - r_rx_clr"]
    #[inline(always)]
    pub fn r_rx_clr(&mut self) -> RRxClrW<'_, RegRxCfgSpec> {
        RRxClrW::new(self, 6)
    }
}
#[doc = "See `udma_spis_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_spis_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxCfgSpec;
impl crate::RegisterSpec for RegRxCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_cfg::R`](R) reader structure"]
impl crate::Readable for RegRxCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_cfg::W`](W) writer structure"]
impl crate::Writable for RegRxCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_CFG to value 0"]
impl crate::Resettable for RegRxCfgSpec {}
