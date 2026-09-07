#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `i2c0_rx` reader - Level of the ``i2c0_rx`` event"]
pub type I2c0RxR = crate::BitReader;
#[doc = "Field `i2c0_rx` writer - Level of the ``i2c0_rx`` event"]
pub type I2c0RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_tx` reader - Level of the ``i2c0_tx`` event"]
pub type I2c0TxR = crate::BitReader;
#[doc = "Field `i2c0_tx` writer - Level of the ``i2c0_tx`` event"]
pub type I2c0TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_cmd` reader - Level of the ``i2c0_cmd`` event"]
pub type I2c0CmdR = crate::BitReader;
#[doc = "Field `i2c0_cmd` writer - Level of the ``i2c0_cmd`` event"]
pub type I2c0CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_eot` reader - Level of the ``i2c0_eot`` event"]
pub type I2c0EotR = crate::BitReader;
#[doc = "Field `i2c0_eot` writer - Level of the ``i2c0_eot`` event"]
pub type I2c0EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_rx` reader - Level of the ``i2c1_rx`` event"]
pub type I2c1RxR = crate::BitReader;
#[doc = "Field `i2c1_rx` writer - Level of the ``i2c1_rx`` event"]
pub type I2c1RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_tx` reader - Level of the ``i2c1_tx`` event"]
pub type I2c1TxR = crate::BitReader;
#[doc = "Field `i2c1_tx` writer - Level of the ``i2c1_tx`` event"]
pub type I2c1TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_cmd` reader - Level of the ``i2c1_cmd`` event"]
pub type I2c1CmdR = crate::BitReader;
#[doc = "Field `i2c1_cmd` writer - Level of the ``i2c1_cmd`` event"]
pub type I2c1CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_eot` reader - Level of the ``i2c1_eot`` event"]
pub type I2c1EotR = crate::BitReader;
#[doc = "Field `i2c1_eot` writer - Level of the ``i2c1_eot`` event"]
pub type I2c1EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_rx` reader - Level of the ``i2c2_rx`` event"]
pub type I2c2RxR = crate::BitReader;
#[doc = "Field `i2c2_rx` writer - Level of the ``i2c2_rx`` event"]
pub type I2c2RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_tx` reader - Level of the ``i2c2_tx`` event"]
pub type I2c2TxR = crate::BitReader;
#[doc = "Field `i2c2_tx` writer - Level of the ``i2c2_tx`` event"]
pub type I2c2TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_cmd` reader - Level of the ``i2c2_cmd`` event"]
pub type I2c2CmdR = crate::BitReader;
#[doc = "Field `i2c2_cmd` writer - Level of the ``i2c2_cmd`` event"]
pub type I2c2CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_eot` reader - Level of the ``i2c2_eot`` event"]
pub type I2c2EotR = crate::BitReader;
#[doc = "Field `i2c2_eot` writer - Level of the ``i2c2_eot`` event"]
pub type I2c2EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c3_rx` reader - Level of the ``i2c3_rx`` event"]
pub type I2c3RxR = crate::BitReader;
#[doc = "Field `i2c3_rx` writer - Level of the ``i2c3_rx`` event"]
pub type I2c3RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c3_tx` reader - Level of the ``i2c3_tx`` event"]
pub type I2c3TxR = crate::BitReader;
#[doc = "Field `i2c3_tx` writer - Level of the ``i2c3_tx`` event"]
pub type I2c3TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c3_cmd` reader - Level of the ``i2c3_cmd`` event"]
pub type I2c3CmdR = crate::BitReader;
#[doc = "Field `i2c3_cmd` writer - Level of the ``i2c3_cmd`` event"]
pub type I2c3CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c3_eot` reader - Level of the ``i2c3_eot`` event"]
pub type I2c3EotR = crate::BitReader;
#[doc = "Field `i2c3_eot` writer - Level of the ``i2c3_eot`` event"]
pub type I2c3EotW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``i2c0_rx`` event"]
    #[inline(always)]
    pub fn i2c0_rx(&self) -> I2c0RxR {
        I2c0RxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``i2c0_tx`` event"]
    #[inline(always)]
    pub fn i2c0_tx(&self) -> I2c0TxR {
        I2c0TxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``i2c0_cmd`` event"]
    #[inline(always)]
    pub fn i2c0_cmd(&self) -> I2c0CmdR {
        I2c0CmdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``i2c0_eot`` event"]
    #[inline(always)]
    pub fn i2c0_eot(&self) -> I2c0EotR {
        I2c0EotR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``i2c1_rx`` event"]
    #[inline(always)]
    pub fn i2c1_rx(&self) -> I2c1RxR {
        I2c1RxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``i2c1_tx`` event"]
    #[inline(always)]
    pub fn i2c1_tx(&self) -> I2c1TxR {
        I2c1TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``i2c1_cmd`` event"]
    #[inline(always)]
    pub fn i2c1_cmd(&self) -> I2c1CmdR {
        I2c1CmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``i2c1_eot`` event"]
    #[inline(always)]
    pub fn i2c1_eot(&self) -> I2c1EotR {
        I2c1EotR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``i2c2_rx`` event"]
    #[inline(always)]
    pub fn i2c2_rx(&self) -> I2c2RxR {
        I2c2RxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``i2c2_tx`` event"]
    #[inline(always)]
    pub fn i2c2_tx(&self) -> I2c2TxR {
        I2c2TxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``i2c2_cmd`` event"]
    #[inline(always)]
    pub fn i2c2_cmd(&self) -> I2c2CmdR {
        I2c2CmdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``i2c2_eot`` event"]
    #[inline(always)]
    pub fn i2c2_eot(&self) -> I2c2EotR {
        I2c2EotR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``i2c3_rx`` event"]
    #[inline(always)]
    pub fn i2c3_rx(&self) -> I2c3RxR {
        I2c3RxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``i2c3_tx`` event"]
    #[inline(always)]
    pub fn i2c3_tx(&self) -> I2c3TxR {
        I2c3TxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``i2c3_cmd`` event"]
    #[inline(always)]
    pub fn i2c3_cmd(&self) -> I2c3CmdR {
        I2c3CmdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``i2c3_eot`` event"]
    #[inline(always)]
    pub fn i2c3_eot(&self) -> I2c3EotR {
        I2c3EotR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``i2c0_rx`` event"]
    #[inline(always)]
    pub fn i2c0_rx(&mut self) -> I2c0RxW<'_, EvStatusSpec> {
        I2c0RxW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``i2c0_tx`` event"]
    #[inline(always)]
    pub fn i2c0_tx(&mut self) -> I2c0TxW<'_, EvStatusSpec> {
        I2c0TxW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``i2c0_cmd`` event"]
    #[inline(always)]
    pub fn i2c0_cmd(&mut self) -> I2c0CmdW<'_, EvStatusSpec> {
        I2c0CmdW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``i2c0_eot`` event"]
    #[inline(always)]
    pub fn i2c0_eot(&mut self) -> I2c0EotW<'_, EvStatusSpec> {
        I2c0EotW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``i2c1_rx`` event"]
    #[inline(always)]
    pub fn i2c1_rx(&mut self) -> I2c1RxW<'_, EvStatusSpec> {
        I2c1RxW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``i2c1_tx`` event"]
    #[inline(always)]
    pub fn i2c1_tx(&mut self) -> I2c1TxW<'_, EvStatusSpec> {
        I2c1TxW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``i2c1_cmd`` event"]
    #[inline(always)]
    pub fn i2c1_cmd(&mut self) -> I2c1CmdW<'_, EvStatusSpec> {
        I2c1CmdW::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``i2c1_eot`` event"]
    #[inline(always)]
    pub fn i2c1_eot(&mut self) -> I2c1EotW<'_, EvStatusSpec> {
        I2c1EotW::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``i2c2_rx`` event"]
    #[inline(always)]
    pub fn i2c2_rx(&mut self) -> I2c2RxW<'_, EvStatusSpec> {
        I2c2RxW::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``i2c2_tx`` event"]
    #[inline(always)]
    pub fn i2c2_tx(&mut self) -> I2c2TxW<'_, EvStatusSpec> {
        I2c2TxW::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``i2c2_cmd`` event"]
    #[inline(always)]
    pub fn i2c2_cmd(&mut self) -> I2c2CmdW<'_, EvStatusSpec> {
        I2c2CmdW::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``i2c2_eot`` event"]
    #[inline(always)]
    pub fn i2c2_eot(&mut self) -> I2c2EotW<'_, EvStatusSpec> {
        I2c2EotW::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``i2c3_rx`` event"]
    #[inline(always)]
    pub fn i2c3_rx(&mut self) -> I2c3RxW<'_, EvStatusSpec> {
        I2c3RxW::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``i2c3_tx`` event"]
    #[inline(always)]
    pub fn i2c3_tx(&mut self) -> I2c3TxW<'_, EvStatusSpec> {
        I2c3TxW::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``i2c3_cmd`` event"]
    #[inline(always)]
    pub fn i2c3_cmd(&mut self) -> I2c3CmdW<'_, EvStatusSpec> {
        I2c3CmdW::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``i2c3_eot`` event"]
    #[inline(always)]
    pub fn i2c3_eot(&mut self) -> I2c3EotW<'_, EvStatusSpec> {
        I2c3EotW::new(self, 15)
    }
}
#[doc = "`1` when a \"i2c3_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
