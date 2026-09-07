#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `scif_rx` reader - `1` when a \"scif_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type ScifRxR = crate::BitReader;
#[doc = "Field `scif_rx` writer - `1` when a \"scif_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type ScifRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_tx` reader - `1` when a \"scif_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type ScifTxR = crate::BitReader;
#[doc = "Field `scif_tx` writer - `1` when a \"scif_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type ScifTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_rx_char` reader - `1` when a \"scif_rx_char\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type ScifRxCharR = crate::BitReader;
#[doc = "Field `scif_rx_char` writer - `1` when a \"scif_rx_char\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type ScifRxCharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_err` reader - `1` when a \"scif_err\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type ScifErrR = crate::BitReader;
#[doc = "Field `scif_err` writer - `1` when a \"scif_err\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type ScifErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_rx` reader - `1` when a \"spis0_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis0RxR = crate::BitReader;
#[doc = "Field `spis0_rx` writer - `1` when a \"spis0_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis0RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_tx` reader - `1` when a \"spis0_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis0TxR = crate::BitReader;
#[doc = "Field `spis0_tx` writer - `1` when a \"spis0_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis0TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_eot` reader - `1` when a \"spis0_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis0EotR = crate::BitReader;
#[doc = "Field `spis0_eot` writer - `1` when a \"spis0_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis0EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b9s7` reader - `1` when a \"nc_b9s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB9s7R = crate::BitReader;
#[doc = "Field `nc_b9s7` writer - `1` when a \"nc_b9s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB9s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_rx` reader - `1` when a \"spis1_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis1RxR = crate::BitReader;
#[doc = "Field `spis1_rx` writer - `1` when a \"spis1_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis1RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_tx` reader - `1` when a \"spis1_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis1TxR = crate::BitReader;
#[doc = "Field `spis1_tx` writer - `1` when a \"spis1_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis1TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_eot` reader - `1` when a \"spis1_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis1EotR = crate::BitReader;
#[doc = "Field `spis1_eot` writer - `1` when a \"spis1_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spis1EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b9s11` reader - `1` when a \"nc_b9s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB9s11R = crate::BitReader;
#[doc = "Field `nc_b9s11` writer - `1` when a \"nc_b9s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB9s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm0_ev` reader - `1` when a \"pwm0_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pwm0EvR = crate::BitReader;
#[doc = "Field `pwm0_ev` writer - `1` when a \"pwm0_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pwm0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm1_ev` reader - `1` when a \"pwm1_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pwm1EvR = crate::BitReader;
#[doc = "Field `pwm1_ev` writer - `1` when a \"pwm1_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pwm1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm2_ev` reader - `1` when a \"pwm2_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pwm2EvR = crate::BitReader;
#[doc = "Field `pwm2_ev` writer - `1` when a \"pwm2_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pwm2EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm3_ev` reader - `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pwm3EvR = crate::BitReader;
#[doc = "Field `pwm3_ev` writer - `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pwm3EvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `1` when a \"scif_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn scif_rx(&self) -> ScifRxR {
        ScifRxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - `1` when a \"scif_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn scif_tx(&self) -> ScifTxR {
        ScifTxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - `1` when a \"scif_rx_char\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn scif_rx_char(&self) -> ScifRxCharR {
        ScifRxCharR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - `1` when a \"scif_err\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn scif_err(&self) -> ScifErrR {
        ScifErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - `1` when a \"spis0_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis0_rx(&self) -> Spis0RxR {
        Spis0RxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - `1` when a \"spis0_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis0_tx(&self) -> Spis0TxR {
        Spis0TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - `1` when a \"spis0_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis0_eot(&self) -> Spis0EotR {
        Spis0EotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - `1` when a \"nc_b9s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b9s7(&self) -> NcB9s7R {
        NcB9s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - `1` when a \"spis1_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis1_rx(&self) -> Spis1RxR {
        Spis1RxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - `1` when a \"spis1_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis1_tx(&self) -> Spis1TxR {
        Spis1TxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - `1` when a \"spis1_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis1_eot(&self) -> Spis1EotR {
        Spis1EotR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b9s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b9s11(&self) -> NcB9s11R {
        NcB9s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - `1` when a \"pwm0_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pwm0_ev(&self) -> Pwm0EvR {
        Pwm0EvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - `1` when a \"pwm1_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pwm1_ev(&self) -> Pwm1EvR {
        Pwm1EvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - `1` when a \"pwm2_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pwm2_ev(&self) -> Pwm2EvR {
        Pwm2EvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pwm3_ev(&self) -> Pwm3EvR {
        Pwm3EvR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` when a \"scif_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn scif_rx(&mut self) -> ScifRxW<'_, EvPendingSpec> {
        ScifRxW::new(self, 0)
    }
    #[doc = "Bit 1 - `1` when a \"scif_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn scif_tx(&mut self) -> ScifTxW<'_, EvPendingSpec> {
        ScifTxW::new(self, 1)
    }
    #[doc = "Bit 2 - `1` when a \"scif_rx_char\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn scif_rx_char(&mut self) -> ScifRxCharW<'_, EvPendingSpec> {
        ScifRxCharW::new(self, 2)
    }
    #[doc = "Bit 3 - `1` when a \"scif_err\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn scif_err(&mut self) -> ScifErrW<'_, EvPendingSpec> {
        ScifErrW::new(self, 3)
    }
    #[doc = "Bit 4 - `1` when a \"spis0_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis0_rx(&mut self) -> Spis0RxW<'_, EvPendingSpec> {
        Spis0RxW::new(self, 4)
    }
    #[doc = "Bit 5 - `1` when a \"spis0_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis0_tx(&mut self) -> Spis0TxW<'_, EvPendingSpec> {
        Spis0TxW::new(self, 5)
    }
    #[doc = "Bit 6 - `1` when a \"spis0_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis0_eot(&mut self) -> Spis0EotW<'_, EvPendingSpec> {
        Spis0EotW::new(self, 6)
    }
    #[doc = "Bit 7 - `1` when a \"nc_b9s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b9s7(&mut self) -> NcB9s7W<'_, EvPendingSpec> {
        NcB9s7W::new(self, 7)
    }
    #[doc = "Bit 8 - `1` when a \"spis1_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis1_rx(&mut self) -> Spis1RxW<'_, EvPendingSpec> {
        Spis1RxW::new(self, 8)
    }
    #[doc = "Bit 9 - `1` when a \"spis1_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis1_tx(&mut self) -> Spis1TxW<'_, EvPendingSpec> {
        Spis1TxW::new(self, 9)
    }
    #[doc = "Bit 10 - `1` when a \"spis1_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spis1_eot(&mut self) -> Spis1EotW<'_, EvPendingSpec> {
        Spis1EotW::new(self, 10)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b9s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b9s11(&mut self) -> NcB9s11W<'_, EvPendingSpec> {
        NcB9s11W::new(self, 11)
    }
    #[doc = "Bit 12 - `1` when a \"pwm0_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pwm0_ev(&mut self) -> Pwm0EvW<'_, EvPendingSpec> {
        Pwm0EvW::new(self, 12)
    }
    #[doc = "Bit 13 - `1` when a \"pwm1_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pwm1_ev(&mut self) -> Pwm1EvW<'_, EvPendingSpec> {
        Pwm1EvW::new(self, 13)
    }
    #[doc = "Bit 14 - `1` when a \"pwm2_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pwm2_ev(&mut self) -> Pwm2EvW<'_, EvPendingSpec> {
        Pwm2EvW::new(self, 14)
    }
    #[doc = "Bit 15 - `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pwm3_ev(&mut self) -> Pwm3EvW<'_, EvPendingSpec> {
        Pwm3EvW::new(self, 15)
    }
}
#[doc = "`1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvPendingSpec;
impl crate::RegisterSpec for EvPendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_pending::R`](R) reader structure"]
impl crate::Readable for EvPendingSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_pending::W`](W) writer structure"]
impl crate::Writable for EvPendingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_PENDING to value 0"]
impl crate::Resettable for EvPendingSpec {}
