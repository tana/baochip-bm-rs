#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `scif_rx` reader - Level of the ``scif_rx`` event"]
pub type ScifRxR = crate::BitReader;
#[doc = "Field `scif_rx` writer - Level of the ``scif_rx`` event"]
pub type ScifRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_tx` reader - Level of the ``scif_tx`` event"]
pub type ScifTxR = crate::BitReader;
#[doc = "Field `scif_tx` writer - Level of the ``scif_tx`` event"]
pub type ScifTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_rx_char` reader - Level of the ``scif_rx_char`` event"]
pub type ScifRxCharR = crate::BitReader;
#[doc = "Field `scif_rx_char` writer - Level of the ``scif_rx_char`` event"]
pub type ScifRxCharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scif_err` reader - Level of the ``scif_err`` event"]
pub type ScifErrR = crate::BitReader;
#[doc = "Field `scif_err` writer - Level of the ``scif_err`` event"]
pub type ScifErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_rx` reader - Level of the ``spis0_rx`` event"]
pub type Spis0RxR = crate::BitReader;
#[doc = "Field `spis0_rx` writer - Level of the ``spis0_rx`` event"]
pub type Spis0RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_tx` reader - Level of the ``spis0_tx`` event"]
pub type Spis0TxR = crate::BitReader;
#[doc = "Field `spis0_tx` writer - Level of the ``spis0_tx`` event"]
pub type Spis0TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis0_eot` reader - Level of the ``spis0_eot`` event"]
pub type Spis0EotR = crate::BitReader;
#[doc = "Field `spis0_eot` writer - Level of the ``spis0_eot`` event"]
pub type Spis0EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b9s7` reader - Level of the ``nc_b9s7`` event"]
pub type NcB9s7R = crate::BitReader;
#[doc = "Field `nc_b9s7` writer - Level of the ``nc_b9s7`` event"]
pub type NcB9s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_rx` reader - Level of the ``spis1_rx`` event"]
pub type Spis1RxR = crate::BitReader;
#[doc = "Field `spis1_rx` writer - Level of the ``spis1_rx`` event"]
pub type Spis1RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_tx` reader - Level of the ``spis1_tx`` event"]
pub type Spis1TxR = crate::BitReader;
#[doc = "Field `spis1_tx` writer - Level of the ``spis1_tx`` event"]
pub type Spis1TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spis1_eot` reader - Level of the ``spis1_eot`` event"]
pub type Spis1EotR = crate::BitReader;
#[doc = "Field `spis1_eot` writer - Level of the ``spis1_eot`` event"]
pub type Spis1EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b9s11` reader - Level of the ``nc_b9s11`` event"]
pub type NcB9s11R = crate::BitReader;
#[doc = "Field `nc_b9s11` writer - Level of the ``nc_b9s11`` event"]
pub type NcB9s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm0_ev` reader - Level of the ``pwm0_ev`` event"]
pub type Pwm0EvR = crate::BitReader;
#[doc = "Field `pwm0_ev` writer - Level of the ``pwm0_ev`` event"]
pub type Pwm0EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm1_ev` reader - Level of the ``pwm1_ev`` event"]
pub type Pwm1EvR = crate::BitReader;
#[doc = "Field `pwm1_ev` writer - Level of the ``pwm1_ev`` event"]
pub type Pwm1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm2_ev` reader - Level of the ``pwm2_ev`` event"]
pub type Pwm2EvR = crate::BitReader;
#[doc = "Field `pwm2_ev` writer - Level of the ``pwm2_ev`` event"]
pub type Pwm2EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm3_ev` reader - Level of the ``pwm3_ev`` event"]
pub type Pwm3EvR = crate::BitReader;
#[doc = "Field `pwm3_ev` writer - Level of the ``pwm3_ev`` event"]
pub type Pwm3EvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``scif_rx`` event"]
    #[inline(always)]
    pub fn scif_rx(&self) -> ScifRxR {
        ScifRxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``scif_tx`` event"]
    #[inline(always)]
    pub fn scif_tx(&self) -> ScifTxR {
        ScifTxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``scif_rx_char`` event"]
    #[inline(always)]
    pub fn scif_rx_char(&self) -> ScifRxCharR {
        ScifRxCharR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``scif_err`` event"]
    #[inline(always)]
    pub fn scif_err(&self) -> ScifErrR {
        ScifErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``spis0_rx`` event"]
    #[inline(always)]
    pub fn spis0_rx(&self) -> Spis0RxR {
        Spis0RxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``spis0_tx`` event"]
    #[inline(always)]
    pub fn spis0_tx(&self) -> Spis0TxR {
        Spis0TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``spis0_eot`` event"]
    #[inline(always)]
    pub fn spis0_eot(&self) -> Spis0EotR {
        Spis0EotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``nc_b9s7`` event"]
    #[inline(always)]
    pub fn nc_b9s7(&self) -> NcB9s7R {
        NcB9s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``spis1_rx`` event"]
    #[inline(always)]
    pub fn spis1_rx(&self) -> Spis1RxR {
        Spis1RxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``spis1_tx`` event"]
    #[inline(always)]
    pub fn spis1_tx(&self) -> Spis1TxR {
        Spis1TxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``spis1_eot`` event"]
    #[inline(always)]
    pub fn spis1_eot(&self) -> Spis1EotR {
        Spis1EotR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``nc_b9s11`` event"]
    #[inline(always)]
    pub fn nc_b9s11(&self) -> NcB9s11R {
        NcB9s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``pwm0_ev`` event"]
    #[inline(always)]
    pub fn pwm0_ev(&self) -> Pwm0EvR {
        Pwm0EvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``pwm1_ev`` event"]
    #[inline(always)]
    pub fn pwm1_ev(&self) -> Pwm1EvR {
        Pwm1EvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``pwm2_ev`` event"]
    #[inline(always)]
    pub fn pwm2_ev(&self) -> Pwm2EvR {
        Pwm2EvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``pwm3_ev`` event"]
    #[inline(always)]
    pub fn pwm3_ev(&self) -> Pwm3EvR {
        Pwm3EvR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``scif_rx`` event"]
    #[inline(always)]
    pub fn scif_rx(&mut self) -> ScifRxW<'_, EvStatusSpec> {
        ScifRxW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``scif_tx`` event"]
    #[inline(always)]
    pub fn scif_tx(&mut self) -> ScifTxW<'_, EvStatusSpec> {
        ScifTxW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``scif_rx_char`` event"]
    #[inline(always)]
    pub fn scif_rx_char(&mut self) -> ScifRxCharW<'_, EvStatusSpec> {
        ScifRxCharW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``scif_err`` event"]
    #[inline(always)]
    pub fn scif_err(&mut self) -> ScifErrW<'_, EvStatusSpec> {
        ScifErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``spis0_rx`` event"]
    #[inline(always)]
    pub fn spis0_rx(&mut self) -> Spis0RxW<'_, EvStatusSpec> {
        Spis0RxW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``spis0_tx`` event"]
    #[inline(always)]
    pub fn spis0_tx(&mut self) -> Spis0TxW<'_, EvStatusSpec> {
        Spis0TxW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``spis0_eot`` event"]
    #[inline(always)]
    pub fn spis0_eot(&mut self) -> Spis0EotW<'_, EvStatusSpec> {
        Spis0EotW::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``nc_b9s7`` event"]
    #[inline(always)]
    pub fn nc_b9s7(&mut self) -> NcB9s7W<'_, EvStatusSpec> {
        NcB9s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``spis1_rx`` event"]
    #[inline(always)]
    pub fn spis1_rx(&mut self) -> Spis1RxW<'_, EvStatusSpec> {
        Spis1RxW::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``spis1_tx`` event"]
    #[inline(always)]
    pub fn spis1_tx(&mut self) -> Spis1TxW<'_, EvStatusSpec> {
        Spis1TxW::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``spis1_eot`` event"]
    #[inline(always)]
    pub fn spis1_eot(&mut self) -> Spis1EotW<'_, EvStatusSpec> {
        Spis1EotW::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``nc_b9s11`` event"]
    #[inline(always)]
    pub fn nc_b9s11(&mut self) -> NcB9s11W<'_, EvStatusSpec> {
        NcB9s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``pwm0_ev`` event"]
    #[inline(always)]
    pub fn pwm0_ev(&mut self) -> Pwm0EvW<'_, EvStatusSpec> {
        Pwm0EvW::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``pwm1_ev`` event"]
    #[inline(always)]
    pub fn pwm1_ev(&mut self) -> Pwm1EvW<'_, EvStatusSpec> {
        Pwm1EvW::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``pwm2_ev`` event"]
    #[inline(always)]
    pub fn pwm2_ev(&mut self) -> Pwm2EvW<'_, EvStatusSpec> {
        Pwm2EvW::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``pwm3_ev`` event"]
    #[inline(always)]
    pub fn pwm3_ev(&mut self) -> Pwm3EvW<'_, EvStatusSpec> {
        Pwm3EvW::new(self, 15)
    }
}
#[doc = "`1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvStatusSpec;
impl crate::RegisterSpec for EvStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_status::R`](R) reader structure"]
impl crate::Readable for EvStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_status::W`](W) writer structure"]
impl crate::Writable for EvStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_STATUS to value 0"]
impl crate::Resettable for EvStatusSpec {}
