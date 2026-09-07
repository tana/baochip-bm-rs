#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `nc_b12s0` reader - Level of the ``nc_b12s0`` event"]
pub type NcB12s0R = crate::BitReader;
#[doc = "Field `nc_b12s0` writer - Level of the ``nc_b12s0`` event"]
pub type NcB12s0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b12s1` reader - Level of the ``nc_b12s1`` event"]
pub type NcB12s1R = crate::BitReader;
#[doc = "Field `nc_b12s1` writer - Level of the ``nc_b12s1`` event"]
pub type NcB12s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b12s2` reader - Level of the ``nc_b12s2`` event"]
pub type NcB12s2R = crate::BitReader;
#[doc = "Field `nc_b12s2` writer - Level of the ``nc_b12s2`` event"]
pub type NcB12s2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b12s3` reader - Level of the ``nc_b12s3`` event"]
pub type NcB12s3R = crate::BitReader;
#[doc = "Field `nc_b12s3` writer - Level of the ``nc_b12s3`` event"]
pub type NcB12s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b12s4` reader - Level of the ``nc_b12s4`` event"]
pub type NcB12s4R = crate::BitReader;
#[doc = "Field `nc_b12s4` writer - Level of the ``nc_b12s4`` event"]
pub type NcB12s4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b12s5` reader - Level of the ``nc_b12s5`` event"]
pub type NcB12s5R = crate::BitReader;
#[doc = "Field `nc_b12s5` writer - Level of the ``nc_b12s5`` event"]
pub type NcB12s5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b12s6` reader - Level of the ``nc_b12s6`` event"]
pub type NcB12s6R = crate::BitReader;
#[doc = "Field `nc_b12s6` writer - Level of the ``nc_b12s6`` event"]
pub type NcB12s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b12s7` reader - Level of the ``nc_b12s7`` event"]
pub type NcB12s7R = crate::BitReader;
#[doc = "Field `nc_b12s7` writer - Level of the ``nc_b12s7`` event"]
pub type NcB12s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_nack` reader - Level of the ``i2c0_nack`` event"]
pub type I2c0NackR = crate::BitReader;
#[doc = "Field `i2c0_nack` writer - Level of the ``i2c0_nack`` event"]
pub type I2c0NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_nack` reader - Level of the ``i2c1_nack`` event"]
pub type I2c1NackR = crate::BitReader;
#[doc = "Field `i2c1_nack` writer - Level of the ``i2c1_nack`` event"]
pub type I2c1NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_nack` reader - Level of the ``i2c2_nack`` event"]
pub type I2c2NackR = crate::BitReader;
#[doc = "Field `i2c2_nack` writer - Level of the ``i2c2_nack`` event"]
pub type I2c2NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c3_nack` reader - Level of the ``i2c3_nack`` event"]
pub type I2c3NackR = crate::BitReader;
#[doc = "Field `i2c3_nack` writer - Level of the ``i2c3_nack`` event"]
pub type I2c3NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_err` reader - Level of the ``i2c0_err`` event"]
pub type I2c0ErrR = crate::BitReader;
#[doc = "Field `i2c0_err` writer - Level of the ``i2c0_err`` event"]
pub type I2c0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_err` reader - Level of the ``i2c1_err`` event"]
pub type I2c1ErrR = crate::BitReader;
#[doc = "Field `i2c1_err` writer - Level of the ``i2c1_err`` event"]
pub type I2c1ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_err` reader - Level of the ``i2c2_err`` event"]
pub type I2c2ErrR = crate::BitReader;
#[doc = "Field `i2c2_err` writer - Level of the ``i2c2_err`` event"]
pub type I2c2ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c3_err` reader - Level of the ``i2c3_err`` event"]
pub type I2c3ErrR = crate::BitReader;
#[doc = "Field `i2c3_err` writer - Level of the ``i2c3_err`` event"]
pub type I2c3ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``nc_b12s0`` event"]
    #[inline(always)]
    pub fn nc_b12s0(&self) -> NcB12s0R {
        NcB12s0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``nc_b12s1`` event"]
    #[inline(always)]
    pub fn nc_b12s1(&self) -> NcB12s1R {
        NcB12s1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``nc_b12s2`` event"]
    #[inline(always)]
    pub fn nc_b12s2(&self) -> NcB12s2R {
        NcB12s2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``nc_b12s3`` event"]
    #[inline(always)]
    pub fn nc_b12s3(&self) -> NcB12s3R {
        NcB12s3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``nc_b12s4`` event"]
    #[inline(always)]
    pub fn nc_b12s4(&self) -> NcB12s4R {
        NcB12s4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``nc_b12s5`` event"]
    #[inline(always)]
    pub fn nc_b12s5(&self) -> NcB12s5R {
        NcB12s5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``nc_b12s6`` event"]
    #[inline(always)]
    pub fn nc_b12s6(&self) -> NcB12s6R {
        NcB12s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``nc_b12s7`` event"]
    #[inline(always)]
    pub fn nc_b12s7(&self) -> NcB12s7R {
        NcB12s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``i2c0_nack`` event"]
    #[inline(always)]
    pub fn i2c0_nack(&self) -> I2c0NackR {
        I2c0NackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``i2c1_nack`` event"]
    #[inline(always)]
    pub fn i2c1_nack(&self) -> I2c1NackR {
        I2c1NackR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``i2c2_nack`` event"]
    #[inline(always)]
    pub fn i2c2_nack(&self) -> I2c2NackR {
        I2c2NackR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``i2c3_nack`` event"]
    #[inline(always)]
    pub fn i2c3_nack(&self) -> I2c3NackR {
        I2c3NackR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``i2c0_err`` event"]
    #[inline(always)]
    pub fn i2c0_err(&self) -> I2c0ErrR {
        I2c0ErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``i2c1_err`` event"]
    #[inline(always)]
    pub fn i2c1_err(&self) -> I2c1ErrR {
        I2c1ErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``i2c2_err`` event"]
    #[inline(always)]
    pub fn i2c2_err(&self) -> I2c2ErrR {
        I2c2ErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``i2c3_err`` event"]
    #[inline(always)]
    pub fn i2c3_err(&self) -> I2c3ErrR {
        I2c3ErrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``nc_b12s0`` event"]
    #[inline(always)]
    pub fn nc_b12s0(&mut self) -> NcB12s0W<'_, EvStatusSpec> {
        NcB12s0W::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``nc_b12s1`` event"]
    #[inline(always)]
    pub fn nc_b12s1(&mut self) -> NcB12s1W<'_, EvStatusSpec> {
        NcB12s1W::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``nc_b12s2`` event"]
    #[inline(always)]
    pub fn nc_b12s2(&mut self) -> NcB12s2W<'_, EvStatusSpec> {
        NcB12s2W::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``nc_b12s3`` event"]
    #[inline(always)]
    pub fn nc_b12s3(&mut self) -> NcB12s3W<'_, EvStatusSpec> {
        NcB12s3W::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``nc_b12s4`` event"]
    #[inline(always)]
    pub fn nc_b12s4(&mut self) -> NcB12s4W<'_, EvStatusSpec> {
        NcB12s4W::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``nc_b12s5`` event"]
    #[inline(always)]
    pub fn nc_b12s5(&mut self) -> NcB12s5W<'_, EvStatusSpec> {
        NcB12s5W::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``nc_b12s6`` event"]
    #[inline(always)]
    pub fn nc_b12s6(&mut self) -> NcB12s6W<'_, EvStatusSpec> {
        NcB12s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``nc_b12s7`` event"]
    #[inline(always)]
    pub fn nc_b12s7(&mut self) -> NcB12s7W<'_, EvStatusSpec> {
        NcB12s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``i2c0_nack`` event"]
    #[inline(always)]
    pub fn i2c0_nack(&mut self) -> I2c0NackW<'_, EvStatusSpec> {
        I2c0NackW::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``i2c1_nack`` event"]
    #[inline(always)]
    pub fn i2c1_nack(&mut self) -> I2c1NackW<'_, EvStatusSpec> {
        I2c1NackW::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``i2c2_nack`` event"]
    #[inline(always)]
    pub fn i2c2_nack(&mut self) -> I2c2NackW<'_, EvStatusSpec> {
        I2c2NackW::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``i2c3_nack`` event"]
    #[inline(always)]
    pub fn i2c3_nack(&mut self) -> I2c3NackW<'_, EvStatusSpec> {
        I2c3NackW::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``i2c0_err`` event"]
    #[inline(always)]
    pub fn i2c0_err(&mut self) -> I2c0ErrW<'_, EvStatusSpec> {
        I2c0ErrW::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``i2c1_err`` event"]
    #[inline(always)]
    pub fn i2c1_err(&mut self) -> I2c1ErrW<'_, EvStatusSpec> {
        I2c1ErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``i2c2_err`` event"]
    #[inline(always)]
    pub fn i2c2_err(&mut self) -> I2c2ErrW<'_, EvStatusSpec> {
        I2c2ErrW::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``i2c3_err`` event"]
    #[inline(always)]
    pub fn i2c3_err(&mut self) -> I2c3ErrW<'_, EvStatusSpec> {
        I2c3ErrW::new(self, 15)
    }
}
#[doc = "`1` when a \"i2c3_err\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
