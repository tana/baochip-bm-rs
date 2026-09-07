#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `spim0_rx` reader - Write a ``1`` to enable the ``spim0_rx`` Event"]
pub type Spim0RxR = crate::BitReader;
#[doc = "Field `spim0_rx` writer - Write a ``1`` to enable the ``spim0_rx`` Event"]
pub type Spim0RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim0_tx` reader - Write a ``1`` to enable the ``spim0_tx`` Event"]
pub type Spim0TxR = crate::BitReader;
#[doc = "Field `spim0_tx` writer - Write a ``1`` to enable the ``spim0_tx`` Event"]
pub type Spim0TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim0_cmd` reader - Write a ``1`` to enable the ``spim0_cmd`` Event"]
pub type Spim0CmdR = crate::BitReader;
#[doc = "Field `spim0_cmd` writer - Write a ``1`` to enable the ``spim0_cmd`` Event"]
pub type Spim0CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim0_eot` reader - Write a ``1`` to enable the ``spim0_eot`` Event"]
pub type Spim0EotR = crate::BitReader;
#[doc = "Field `spim0_eot` writer - Write a ``1`` to enable the ``spim0_eot`` Event"]
pub type Spim0EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_rx` reader - Write a ``1`` to enable the ``spim1_rx`` Event"]
pub type Spim1RxR = crate::BitReader;
#[doc = "Field `spim1_rx` writer - Write a ``1`` to enable the ``spim1_rx`` Event"]
pub type Spim1RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_tx` reader - Write a ``1`` to enable the ``spim1_tx`` Event"]
pub type Spim1TxR = crate::BitReader;
#[doc = "Field `spim1_tx` writer - Write a ``1`` to enable the ``spim1_tx`` Event"]
pub type Spim1TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_cmd` reader - Write a ``1`` to enable the ``spim1_cmd`` Event"]
pub type Spim1CmdR = crate::BitReader;
#[doc = "Field `spim1_cmd` writer - Write a ``1`` to enable the ``spim1_cmd`` Event"]
pub type Spim1CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_eot` reader - Write a ``1`` to enable the ``spim1_eot`` Event"]
pub type Spim1EotR = crate::BitReader;
#[doc = "Field `spim1_eot` writer - Write a ``1`` to enable the ``spim1_eot`` Event"]
pub type Spim1EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_rx` reader - Write a ``1`` to enable the ``spim2_rx`` Event"]
pub type Spim2RxR = crate::BitReader;
#[doc = "Field `spim2_rx` writer - Write a ``1`` to enable the ``spim2_rx`` Event"]
pub type Spim2RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_tx` reader - Write a ``1`` to enable the ``spim2_tx`` Event"]
pub type Spim2TxR = crate::BitReader;
#[doc = "Field `spim2_tx` writer - Write a ``1`` to enable the ``spim2_tx`` Event"]
pub type Spim2TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_cmd` reader - Write a ``1`` to enable the ``spim2_cmd`` Event"]
pub type Spim2CmdR = crate::BitReader;
#[doc = "Field `spim2_cmd` writer - Write a ``1`` to enable the ``spim2_cmd`` Event"]
pub type Spim2CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_eot` reader - Write a ``1`` to enable the ``spim2_eot`` Event"]
pub type Spim2EotR = crate::BitReader;
#[doc = "Field `spim2_eot` writer - Write a ``1`` to enable the ``spim2_eot`` Event"]
pub type Spim2EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim3_rx` reader - Write a ``1`` to enable the ``spim3_rx`` Event"]
pub type Spim3RxR = crate::BitReader;
#[doc = "Field `spim3_rx` writer - Write a ``1`` to enable the ``spim3_rx`` Event"]
pub type Spim3RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim3_tx` reader - Write a ``1`` to enable the ``spim3_tx`` Event"]
pub type Spim3TxR = crate::BitReader;
#[doc = "Field `spim3_tx` writer - Write a ``1`` to enable the ``spim3_tx`` Event"]
pub type Spim3TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim3_cmd` reader - Write a ``1`` to enable the ``spim3_cmd`` Event"]
pub type Spim3CmdR = crate::BitReader;
#[doc = "Field `spim3_cmd` writer - Write a ``1`` to enable the ``spim3_cmd`` Event"]
pub type Spim3CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim3_eot` reader - Write a ``1`` to enable the ``spim3_eot`` Event"]
pub type Spim3EotR = crate::BitReader;
#[doc = "Field `spim3_eot` writer - Write a ``1`` to enable the ``spim3_eot`` Event"]
pub type Spim3EotW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``spim0_rx`` Event"]
    #[inline(always)]
    pub fn spim0_rx(&self) -> Spim0RxR {
        Spim0RxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``spim0_tx`` Event"]
    #[inline(always)]
    pub fn spim0_tx(&self) -> Spim0TxR {
        Spim0TxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``spim0_cmd`` Event"]
    #[inline(always)]
    pub fn spim0_cmd(&self) -> Spim0CmdR {
        Spim0CmdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``spim0_eot`` Event"]
    #[inline(always)]
    pub fn spim0_eot(&self) -> Spim0EotR {
        Spim0EotR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``spim1_rx`` Event"]
    #[inline(always)]
    pub fn spim1_rx(&self) -> Spim1RxR {
        Spim1RxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``spim1_tx`` Event"]
    #[inline(always)]
    pub fn spim1_tx(&self) -> Spim1TxR {
        Spim1TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``spim1_cmd`` Event"]
    #[inline(always)]
    pub fn spim1_cmd(&self) -> Spim1CmdR {
        Spim1CmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``spim1_eot`` Event"]
    #[inline(always)]
    pub fn spim1_eot(&self) -> Spim1EotR {
        Spim1EotR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``spim2_rx`` Event"]
    #[inline(always)]
    pub fn spim2_rx(&self) -> Spim2RxR {
        Spim2RxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``spim2_tx`` Event"]
    #[inline(always)]
    pub fn spim2_tx(&self) -> Spim2TxR {
        Spim2TxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``spim2_cmd`` Event"]
    #[inline(always)]
    pub fn spim2_cmd(&self) -> Spim2CmdR {
        Spim2CmdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``spim2_eot`` Event"]
    #[inline(always)]
    pub fn spim2_eot(&self) -> Spim2EotR {
        Spim2EotR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``spim3_rx`` Event"]
    #[inline(always)]
    pub fn spim3_rx(&self) -> Spim3RxR {
        Spim3RxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``spim3_tx`` Event"]
    #[inline(always)]
    pub fn spim3_tx(&self) -> Spim3TxR {
        Spim3TxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``spim3_cmd`` Event"]
    #[inline(always)]
    pub fn spim3_cmd(&self) -> Spim3CmdR {
        Spim3CmdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``spim3_eot`` Event"]
    #[inline(always)]
    pub fn spim3_eot(&self) -> Spim3EotR {
        Spim3EotR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``spim0_rx`` Event"]
    #[inline(always)]
    pub fn spim0_rx(&mut self) -> Spim0RxW<'_, EvEnableSpec> {
        Spim0RxW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``spim0_tx`` Event"]
    #[inline(always)]
    pub fn spim0_tx(&mut self) -> Spim0TxW<'_, EvEnableSpec> {
        Spim0TxW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``spim0_cmd`` Event"]
    #[inline(always)]
    pub fn spim0_cmd(&mut self) -> Spim0CmdW<'_, EvEnableSpec> {
        Spim0CmdW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``spim0_eot`` Event"]
    #[inline(always)]
    pub fn spim0_eot(&mut self) -> Spim0EotW<'_, EvEnableSpec> {
        Spim0EotW::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``spim1_rx`` Event"]
    #[inline(always)]
    pub fn spim1_rx(&mut self) -> Spim1RxW<'_, EvEnableSpec> {
        Spim1RxW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``spim1_tx`` Event"]
    #[inline(always)]
    pub fn spim1_tx(&mut self) -> Spim1TxW<'_, EvEnableSpec> {
        Spim1TxW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``spim1_cmd`` Event"]
    #[inline(always)]
    pub fn spim1_cmd(&mut self) -> Spim1CmdW<'_, EvEnableSpec> {
        Spim1CmdW::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``spim1_eot`` Event"]
    #[inline(always)]
    pub fn spim1_eot(&mut self) -> Spim1EotW<'_, EvEnableSpec> {
        Spim1EotW::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``spim2_rx`` Event"]
    #[inline(always)]
    pub fn spim2_rx(&mut self) -> Spim2RxW<'_, EvEnableSpec> {
        Spim2RxW::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``spim2_tx`` Event"]
    #[inline(always)]
    pub fn spim2_tx(&mut self) -> Spim2TxW<'_, EvEnableSpec> {
        Spim2TxW::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``spim2_cmd`` Event"]
    #[inline(always)]
    pub fn spim2_cmd(&mut self) -> Spim2CmdW<'_, EvEnableSpec> {
        Spim2CmdW::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``spim2_eot`` Event"]
    #[inline(always)]
    pub fn spim2_eot(&mut self) -> Spim2EotW<'_, EvEnableSpec> {
        Spim2EotW::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``spim3_rx`` Event"]
    #[inline(always)]
    pub fn spim3_rx(&mut self) -> Spim3RxW<'_, EvEnableSpec> {
        Spim3RxW::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``spim3_tx`` Event"]
    #[inline(always)]
    pub fn spim3_tx(&mut self) -> Spim3TxW<'_, EvEnableSpec> {
        Spim3TxW::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``spim3_cmd`` Event"]
    #[inline(always)]
    pub fn spim3_cmd(&mut self) -> Spim3CmdW<'_, EvEnableSpec> {
        Spim3CmdW::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``spim3_eot`` Event"]
    #[inline(always)]
    pub fn spim3_eot(&mut self) -> Spim3EotW<'_, EvEnableSpec> {
        Spim3EotW::new(self, 15)
    }
}
#[doc = "`1` when a \"spim3_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
