#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `uart2_rx_dupe` reader - Write a ``1`` to enable the ``uart2_rx_dupe`` Event"]
pub type Uart2RxDupeR = crate::BitReader;
#[doc = "Field `uart2_rx_dupe` writer - Write a ``1`` to enable the ``uart2_rx_dupe`` Event"]
pub type Uart2RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_tx_dupe` reader - Write a ``1`` to enable the ``uart2_tx_dupe`` Event"]
pub type Uart2TxDupeR = crate::BitReader;
#[doc = "Field `uart2_tx_dupe` writer - Write a ``1`` to enable the ``uart2_tx_dupe`` Event"]
pub type Uart2TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_rx_char_dupe` reader - Write a ``1`` to enable the ``uart2_rx_char_dupe`` Event"]
pub type Uart2RxCharDupeR = crate::BitReader;
#[doc = "Field `uart2_rx_char_dupe` writer - Write a ``1`` to enable the ``uart2_rx_char_dupe`` Event"]
pub type Uart2RxCharDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_err_dupe` reader - Write a ``1`` to enable the ``uart2_err_dupe`` Event"]
pub type Uart2ErrDupeR = crate::BitReader;
#[doc = "Field `uart2_err_dupe` writer - Write a ``1`` to enable the ``uart2_err_dupe`` Event"]
pub type Uart2ErrDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_rx_dupe` reader - Write a ``1`` to enable the ``uart3_rx_dupe`` Event"]
pub type Uart3RxDupeR = crate::BitReader;
#[doc = "Field `uart3_rx_dupe` writer - Write a ``1`` to enable the ``uart3_rx_dupe`` Event"]
pub type Uart3RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_tx_dupe` reader - Write a ``1`` to enable the ``uart3_tx_dupe`` Event"]
pub type Uart3TxDupeR = crate::BitReader;
#[doc = "Field `uart3_tx_dupe` writer - Write a ``1`` to enable the ``uart3_tx_dupe`` Event"]
pub type Uart3TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_rx_char_dupe` reader - Write a ``1`` to enable the ``uart3_rx_char_dupe`` Event"]
pub type Uart3RxCharDupeR = crate::BitReader;
#[doc = "Field `uart3_rx_char_dupe` writer - Write a ``1`` to enable the ``uart3_rx_char_dupe`` Event"]
pub type Uart3RxCharDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_err_dupe` reader - Write a ``1`` to enable the ``uart3_err_dupe`` Event"]
pub type Uart3ErrDupeR = crate::BitReader;
#[doc = "Field `uart3_err_dupe` writer - Write a ``1`` to enable the ``uart3_err_dupe`` Event"]
pub type Uart3ErrDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trng_done_dupe` reader - Write a ``1`` to enable the ``trng_done_dupe`` Event"]
pub type TrngDoneDupeR = crate::BitReader;
#[doc = "Field `trng_done_dupe` writer - Write a ``1`` to enable the ``trng_done_dupe`` Event"]
pub type TrngDoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b14s9` reader - Write a ``1`` to enable the ``nc_b14s9`` Event"]
pub type NcB14s9R = crate::BitReader;
#[doc = "Field `nc_b14s9` writer - Write a ``1`` to enable the ``nc_b14s9`` Event"]
pub type NcB14s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b14s10` reader - Write a ``1`` to enable the ``nc_b14s10`` Event"]
pub type NcB14s10R = crate::BitReader;
#[doc = "Field `nc_b14s10` writer - Write a ``1`` to enable the ``nc_b14s10`` Event"]
pub type NcB14s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b14s11` reader - Write a ``1`` to enable the ``nc_b14s11`` Event"]
pub type NcB14s11R = crate::BitReader;
#[doc = "Field `nc_b14s11` writer - Write a ``1`` to enable the ``nc_b14s11`` Event"]
pub type NcB14s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b14s12` reader - Write a ``1`` to enable the ``nc_b14s12`` Event"]
pub type NcB14s12R = crate::BitReader;
#[doc = "Field `nc_b14s12` writer - Write a ``1`` to enable the ``nc_b14s12`` Event"]
pub type NcB14s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b14s13` reader - Write a ``1`` to enable the ``nc_b14s13`` Event"]
pub type NcB14s13R = crate::BitReader;
#[doc = "Field `nc_b14s13` writer - Write a ``1`` to enable the ``nc_b14s13`` Event"]
pub type NcB14s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b14s14` reader - Write a ``1`` to enable the ``nc_b14s14`` Event"]
pub type NcB14s14R = crate::BitReader;
#[doc = "Field `nc_b14s14` writer - Write a ``1`` to enable the ``nc_b14s14`` Event"]
pub type NcB14s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b14s15` reader - Write a ``1`` to enable the ``nc_b14s15`` Event"]
pub type NcB14s15R = crate::BitReader;
#[doc = "Field `nc_b14s15` writer - Write a ``1`` to enable the ``nc_b14s15`` Event"]
pub type NcB14s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``uart2_rx_dupe`` Event"]
    #[inline(always)]
    pub fn uart2_rx_dupe(&self) -> Uart2RxDupeR {
        Uart2RxDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``uart2_tx_dupe`` Event"]
    #[inline(always)]
    pub fn uart2_tx_dupe(&self) -> Uart2TxDupeR {
        Uart2TxDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``uart2_rx_char_dupe`` Event"]
    #[inline(always)]
    pub fn uart2_rx_char_dupe(&self) -> Uart2RxCharDupeR {
        Uart2RxCharDupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``uart2_err_dupe`` Event"]
    #[inline(always)]
    pub fn uart2_err_dupe(&self) -> Uart2ErrDupeR {
        Uart2ErrDupeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``uart3_rx_dupe`` Event"]
    #[inline(always)]
    pub fn uart3_rx_dupe(&self) -> Uart3RxDupeR {
        Uart3RxDupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``uart3_tx_dupe`` Event"]
    #[inline(always)]
    pub fn uart3_tx_dupe(&self) -> Uart3TxDupeR {
        Uart3TxDupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``uart3_rx_char_dupe`` Event"]
    #[inline(always)]
    pub fn uart3_rx_char_dupe(&self) -> Uart3RxCharDupeR {
        Uart3RxCharDupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``uart3_err_dupe`` Event"]
    #[inline(always)]
    pub fn uart3_err_dupe(&self) -> Uart3ErrDupeR {
        Uart3ErrDupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``trng_done_dupe`` Event"]
    #[inline(always)]
    pub fn trng_done_dupe(&self) -> TrngDoneDupeR {
        TrngDoneDupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b14s9`` Event"]
    #[inline(always)]
    pub fn nc_b14s9(&self) -> NcB14s9R {
        NcB14s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b14s10`` Event"]
    #[inline(always)]
    pub fn nc_b14s10(&self) -> NcB14s10R {
        NcB14s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b14s11`` Event"]
    #[inline(always)]
    pub fn nc_b14s11(&self) -> NcB14s11R {
        NcB14s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b14s12`` Event"]
    #[inline(always)]
    pub fn nc_b14s12(&self) -> NcB14s12R {
        NcB14s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b14s13`` Event"]
    #[inline(always)]
    pub fn nc_b14s13(&self) -> NcB14s13R {
        NcB14s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b14s14`` Event"]
    #[inline(always)]
    pub fn nc_b14s14(&self) -> NcB14s14R {
        NcB14s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b14s15`` Event"]
    #[inline(always)]
    pub fn nc_b14s15(&self) -> NcB14s15R {
        NcB14s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``uart2_rx_dupe`` Event"]
    #[inline(always)]
    pub fn uart2_rx_dupe(&mut self) -> Uart2RxDupeW<'_, EvEnableSpec> {
        Uart2RxDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``uart2_tx_dupe`` Event"]
    #[inline(always)]
    pub fn uart2_tx_dupe(&mut self) -> Uart2TxDupeW<'_, EvEnableSpec> {
        Uart2TxDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``uart2_rx_char_dupe`` Event"]
    #[inline(always)]
    pub fn uart2_rx_char_dupe(&mut self) -> Uart2RxCharDupeW<'_, EvEnableSpec> {
        Uart2RxCharDupeW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``uart2_err_dupe`` Event"]
    #[inline(always)]
    pub fn uart2_err_dupe(&mut self) -> Uart2ErrDupeW<'_, EvEnableSpec> {
        Uart2ErrDupeW::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``uart3_rx_dupe`` Event"]
    #[inline(always)]
    pub fn uart3_rx_dupe(&mut self) -> Uart3RxDupeW<'_, EvEnableSpec> {
        Uart3RxDupeW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``uart3_tx_dupe`` Event"]
    #[inline(always)]
    pub fn uart3_tx_dupe(&mut self) -> Uart3TxDupeW<'_, EvEnableSpec> {
        Uart3TxDupeW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``uart3_rx_char_dupe`` Event"]
    #[inline(always)]
    pub fn uart3_rx_char_dupe(&mut self) -> Uart3RxCharDupeW<'_, EvEnableSpec> {
        Uart3RxCharDupeW::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``uart3_err_dupe`` Event"]
    #[inline(always)]
    pub fn uart3_err_dupe(&mut self) -> Uart3ErrDupeW<'_, EvEnableSpec> {
        Uart3ErrDupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``trng_done_dupe`` Event"]
    #[inline(always)]
    pub fn trng_done_dupe(&mut self) -> TrngDoneDupeW<'_, EvEnableSpec> {
        TrngDoneDupeW::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b14s9`` Event"]
    #[inline(always)]
    pub fn nc_b14s9(&mut self) -> NcB14s9W<'_, EvEnableSpec> {
        NcB14s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b14s10`` Event"]
    #[inline(always)]
    pub fn nc_b14s10(&mut self) -> NcB14s10W<'_, EvEnableSpec> {
        NcB14s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b14s11`` Event"]
    #[inline(always)]
    pub fn nc_b14s11(&mut self) -> NcB14s11W<'_, EvEnableSpec> {
        NcB14s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b14s12`` Event"]
    #[inline(always)]
    pub fn nc_b14s12(&mut self) -> NcB14s12W<'_, EvEnableSpec> {
        NcB14s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b14s13`` Event"]
    #[inline(always)]
    pub fn nc_b14s13(&mut self) -> NcB14s13W<'_, EvEnableSpec> {
        NcB14s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b14s14`` Event"]
    #[inline(always)]
    pub fn nc_b14s14(&mut self) -> NcB14s14W<'_, EvEnableSpec> {
        NcB14s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b14s15`` Event"]
    #[inline(always)]
    pub fn nc_b14s15(&mut self) -> NcB14s15W<'_, EvEnableSpec> {
        NcB14s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b14s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvEnableSpec;
impl crate::RegisterSpec for EvEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_enable::R`](R) reader structure"]
impl crate::Readable for EvEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_enable::W`](W) writer structure"]
impl crate::Writable for EvEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_ENABLE to value 0"]
impl crate::Resettable for EvEnableSpec {}
