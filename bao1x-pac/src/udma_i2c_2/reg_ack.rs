#[doc = "Register `REG_ACK` reader"]
pub type R = crate::R<RegAckSpec>;
#[doc = "Register `REG_ACK` writer"]
pub type W = crate::W<RegAckSpec>;
#[doc = "Field `r_nack` reader - r_nack"]
pub type RNackR = crate::BitReader;
#[doc = "Field `r_nack` writer - r_nack"]
pub type RNackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_nack"]
    #[inline(always)]
    pub fn r_nack(&self) -> RNackR {
        RNackR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_nack"]
    #[inline(always)]
    pub fn r_nack(&mut self) -> RNackW<'_, RegAckSpec> {
        RNackW::new(self, 0)
    }
}
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAckSpec;
impl crate::RegisterSpec for RegAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ack::R`](R) reader structure"]
impl crate::Readable for RegAckSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ack::W`](W) writer structure"]
impl crate::Writable for RegAckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ACK to value 0"]
impl crate::Resettable for RegAckSpec {}
