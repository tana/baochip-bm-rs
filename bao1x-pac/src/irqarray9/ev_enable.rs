#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `scif_rx` reader - Write a ``1`` to enable the ``scif_rx`` Event"]
pub type ScifRxR = crate::BitReader;
#[doc = "Field `scif_rx` writer - Write a ``1`` to enable the ``scif_rx`` Event"]
pub type ScifRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_tx` reader - Write a ``1`` to enable the ``scif_tx`` Event"]
pub type ScifTxR = crate::BitReader;
#[doc = "Field `scif_tx` writer - Write a ``1`` to enable the ``scif_tx`` Event"]
pub type ScifTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_rx_char` reader - Write a ``1`` to enable the ``scif_rx_char`` Event"]
pub type ScifRxCharR = crate::BitReader;
#[doc = "Field `scif_rx_char` writer - Write a ``1`` to enable the ``scif_rx_char`` Event"]
pub type ScifRxCharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_err` reader - Write a ``1`` to enable the ``scif_err`` Event"]
pub type ScifErrR = crate::BitReader;
#[doc = "Field `scif_err` writer - Write a ``1`` to enable the ``scif_err`` Event"]
pub type ScifErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_rx` reader - Write a ``1`` to enable the ``spis0_rx`` Event"]
pub type Spis0RxR = crate::BitReader;
#[doc = "Field `spis0_rx` writer - Write a ``1`` to enable the ``spis0_rx`` Event"]
pub type Spis0RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_tx` reader - Write a ``1`` to enable the ``spis0_tx`` Event"]
pub type Spis0TxR = crate::BitReader;
#[doc = "Field `spis0_tx` writer - Write a ``1`` to enable the ``spis0_tx`` Event"]
pub type Spis0TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_eot` reader - Write a ``1`` to enable the ``spis0_eot`` Event"]
pub type Spis0EotR = crate::BitReader;
#[doc = "Field `spis0_eot` writer - Write a ``1`` to enable the ``spis0_eot`` Event"]
pub type Spis0EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b9s7` reader - Write a ``1`` to enable the ``nc_b9s7`` Event"]
pub type NcB9s7R = crate::BitReader;
#[doc = "Field `nc_b9s7` writer - Write a ``1`` to enable the ``nc_b9s7`` Event"]
pub type NcB9s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_rx` reader - Write a ``1`` to enable the ``spis1_rx`` Event"]
pub type Spis1RxR = crate::BitReader;
#[doc = "Field `spis1_rx` writer - Write a ``1`` to enable the ``spis1_rx`` Event"]
pub type Spis1RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_tx` reader - Write a ``1`` to enable the ``spis1_tx`` Event"]
pub type Spis1TxR = crate::BitReader;
#[doc = "Field `spis1_tx` writer - Write a ``1`` to enable the ``spis1_tx`` Event"]
pub type Spis1TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_eot` reader - Write a ``1`` to enable the ``spis1_eot`` Event"]
pub type Spis1EotR = crate::BitReader;
#[doc = "Field `spis1_eot` writer - Write a ``1`` to enable the ``spis1_eot`` Event"]
pub type Spis1EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b9s11` reader - Write a ``1`` to enable the ``nc_b9s11`` Event"]
pub type NcB9s11R = crate::BitReader;
#[doc = "Field `nc_b9s11` writer - Write a ``1`` to enable the ``nc_b9s11`` Event"]
pub type NcB9s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm0_ev` reader - Write a ``1`` to enable the ``pwm0_ev`` Event"]
pub type Pwm0EvR = crate::BitReader;
#[doc = "Field `pwm0_ev` writer - Write a ``1`` to enable the ``pwm0_ev`` Event"]
pub type Pwm0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm1_ev` reader - Write a ``1`` to enable the ``pwm1_ev`` Event"]
pub type Pwm1EvR = crate::BitReader;
#[doc = "Field `pwm1_ev` writer - Write a ``1`` to enable the ``pwm1_ev`` Event"]
pub type Pwm1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm2_ev` reader - Write a ``1`` to enable the ``pwm2_ev`` Event"]
pub type Pwm2EvR = crate::BitReader;
#[doc = "Field `pwm2_ev` writer - Write a ``1`` to enable the ``pwm2_ev`` Event"]
pub type Pwm2EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm3_ev` reader - Write a ``1`` to enable the ``pwm3_ev`` Event"]
pub type Pwm3EvR = crate::BitReader;
#[doc = "Field `pwm3_ev` writer - Write a ``1`` to enable the ``pwm3_ev`` Event"]
pub type Pwm3EvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``scif_rx`` Event"]
    #[inline(always)]
    pub fn scif_rx(&self) -> ScifRxR {
        ScifRxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``scif_tx`` Event"]
    #[inline(always)]
    pub fn scif_tx(&self) -> ScifTxR {
        ScifTxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``scif_rx_char`` Event"]
    #[inline(always)]
    pub fn scif_rx_char(&self) -> ScifRxCharR {
        ScifRxCharR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``scif_err`` Event"]
    #[inline(always)]
    pub fn scif_err(&self) -> ScifErrR {
        ScifErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``spis0_rx`` Event"]
    #[inline(always)]
    pub fn spis0_rx(&self) -> Spis0RxR {
        Spis0RxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``spis0_tx`` Event"]
    #[inline(always)]
    pub fn spis0_tx(&self) -> Spis0TxR {
        Spis0TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``spis0_eot`` Event"]
    #[inline(always)]
    pub fn spis0_eot(&self) -> Spis0EotR {
        Spis0EotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b9s7`` Event"]
    #[inline(always)]
    pub fn nc_b9s7(&self) -> NcB9s7R {
        NcB9s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``spis1_rx`` Event"]
    #[inline(always)]
    pub fn spis1_rx(&self) -> Spis1RxR {
        Spis1RxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``spis1_tx`` Event"]
    #[inline(always)]
    pub fn spis1_tx(&self) -> Spis1TxR {
        Spis1TxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``spis1_eot`` Event"]
    #[inline(always)]
    pub fn spis1_eot(&self) -> Spis1EotR {
        Spis1EotR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b9s11`` Event"]
    #[inline(always)]
    pub fn nc_b9s11(&self) -> NcB9s11R {
        NcB9s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``pwm0_ev`` Event"]
    #[inline(always)]
    pub fn pwm0_ev(&self) -> Pwm0EvR {
        Pwm0EvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``pwm1_ev`` Event"]
    #[inline(always)]
    pub fn pwm1_ev(&self) -> Pwm1EvR {
        Pwm1EvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``pwm2_ev`` Event"]
    #[inline(always)]
    pub fn pwm2_ev(&self) -> Pwm2EvR {
        Pwm2EvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``pwm3_ev`` Event"]
    #[inline(always)]
    pub fn pwm3_ev(&self) -> Pwm3EvR {
        Pwm3EvR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``scif_rx`` Event"]
    #[inline(always)]
    pub fn scif_rx(&mut self) -> ScifRxW<'_, EvEnableSpec> {
        ScifRxW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``scif_tx`` Event"]
    #[inline(always)]
    pub fn scif_tx(&mut self) -> ScifTxW<'_, EvEnableSpec> {
        ScifTxW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``scif_rx_char`` Event"]
    #[inline(always)]
    pub fn scif_rx_char(&mut self) -> ScifRxCharW<'_, EvEnableSpec> {
        ScifRxCharW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``scif_err`` Event"]
    #[inline(always)]
    pub fn scif_err(&mut self) -> ScifErrW<'_, EvEnableSpec> {
        ScifErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``spis0_rx`` Event"]
    #[inline(always)]
    pub fn spis0_rx(&mut self) -> Spis0RxW<'_, EvEnableSpec> {
        Spis0RxW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``spis0_tx`` Event"]
    #[inline(always)]
    pub fn spis0_tx(&mut self) -> Spis0TxW<'_, EvEnableSpec> {
        Spis0TxW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``spis0_eot`` Event"]
    #[inline(always)]
    pub fn spis0_eot(&mut self) -> Spis0EotW<'_, EvEnableSpec> {
        Spis0EotW::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b9s7`` Event"]
    #[inline(always)]
    pub fn nc_b9s7(&mut self) -> NcB9s7W<'_, EvEnableSpec> {
        NcB9s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``spis1_rx`` Event"]
    #[inline(always)]
    pub fn spis1_rx(&mut self) -> Spis1RxW<'_, EvEnableSpec> {
        Spis1RxW::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``spis1_tx`` Event"]
    #[inline(always)]
    pub fn spis1_tx(&mut self) -> Spis1TxW<'_, EvEnableSpec> {
        Spis1TxW::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``spis1_eot`` Event"]
    #[inline(always)]
    pub fn spis1_eot(&mut self) -> Spis1EotW<'_, EvEnableSpec> {
        Spis1EotW::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b9s11`` Event"]
    #[inline(always)]
    pub fn nc_b9s11(&mut self) -> NcB9s11W<'_, EvEnableSpec> {
        NcB9s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``pwm0_ev`` Event"]
    #[inline(always)]
    pub fn pwm0_ev(&mut self) -> Pwm0EvW<'_, EvEnableSpec> {
        Pwm0EvW::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``pwm1_ev`` Event"]
    #[inline(always)]
    pub fn pwm1_ev(&mut self) -> Pwm1EvW<'_, EvEnableSpec> {
        Pwm1EvW::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``pwm2_ev`` Event"]
    #[inline(always)]
    pub fn pwm2_ev(&mut self) -> Pwm2EvW<'_, EvEnableSpec> {
        Pwm2EvW::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``pwm3_ev`` Event"]
    #[inline(always)]
    pub fn pwm3_ev(&mut self) -> Pwm3EvW<'_, EvEnableSpec> {
        Pwm3EvW::new(self, 15)
    }
}
#[doc = "`1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
